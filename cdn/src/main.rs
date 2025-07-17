use std::{fs, path::PathBuf, process, time::Duration};

use actix_cors::Cors;
use actix_files::NamedFile;
use actix_multipart::form::MultipartForm;
use actix_web::{post, web, App, HttpRequest, HttpResponse, HttpServer};
use mongodb::{bson::doc, Client, error::Error, Collection};
use include_crypt::{include_crypt, EncryptedFile};
use rand::{distributions::Alphanumeric, Rng};
use chrono;
use tokio::runtime::Runtime;

mod mongo;
mod forms;

const PORT: u16 = 2953;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Connecting to the MongoDB");
    let client = create_client().await.unwrap();
    let db = client.database("cdn");
    let collection = db.collection::<mongo::File>("Files");
    let cdn_token: String;

    {
        let collection = db.collection::<mongo::Token>("Token");
        let file = collection.find_one(doc!{ }).await;

        match file {
            Err(err) => {
                println!("Error in find token: {}", err);
                process::exit(0x001);
            },
            Ok(result1) => {
                if result1.is_none() {
                    let random_token: String = rand::thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(54)
                    .map(char::from)
                    .collect();

                    let new_token = mongo::Token {
                        internal: random_token.clone()
                    };
                    _ = collection.insert_one(new_token).await;

                    cdn_token = random_token;
                } else {
                    cdn_token = result1.unwrap().internal;
                }
            }
        }
    }

    let rt = Runtime::new()?;
    {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        let collection_clone = collection.clone();
        rt.spawn(async move {
            loop {
                interval.tick().await;
                delete_old_requests(&collection_clone).await;
            }
        });
    }

    println!("Connected to the MongoDB");

    println!("Initializing http server");
    HttpServer::new(move || {
        let mut dir_files = std::env::current_exe().unwrap();
        dir_files.pop();
        dir_files.push("files");

        if !dir_files.exists() {
            if let Err(err) = fs::create_dir_all(&dir_files) {
                println!("Error in create dir: {}", err);
            }
        }

        App::new()
        .wrap(
            Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .max_age(3600)
        )
        .app_data(web::Data::new(dir_files.clone()))
        .route("/{filename:.*}", web::get().to(assets_render))
        .app_data(web::Data::new(collection.clone()))
        .app_data(web::Data::new(cdn_token.clone()))
        .service(upload_file)
        .service(sign_upload)
    })
    .bind(("127.0.0.1", PORT))?
    .run()
    .await
}


async fn assets_render(req: HttpRequest,
    dir_files: web::Data<PathBuf>,
) -> actix_web::Result<NamedFile> {
    let mut file_parsed = req.match_info().query("filename");

    if file_parsed.contains("..") || file_parsed.is_empty() {
        file_parsed = "index.html";
    }

    let mut render_path = dir_files.to_path_buf();
    render_path.push(file_parsed);

    if !render_path.exists() {
        render_path = dir_files.to_path_buf();
        render_path.push("file_not_found.html");
    }

    Ok(NamedFile::open(render_path)?)
}

#[post("/sign-upload")]
async fn sign_upload(
    req: HttpRequest,
    cdn_token: web::Data<String>,
    collection: web::Data<Collection<mongo::File>>,
    info: web::Json<forms::UploadSign>
) -> HttpResponse {
    let auth_token = match get_auth_token(&req) {
        Some(result) => result,
        _ => {
            return HttpResponse::Forbidden().body("Token is missing");
        }
    };

    if auth_token != cdn_token.as_str() {
        return HttpResponse::Forbidden().body("Token is missing");
    }
    
    let time = chrono::Utc::now().timestamp() + 1800; // 30 mns
    let document = mongo::File {
        token: info.token.clone(),
        path: info.path.clone(),
        size: info.size,
        content_type: info.content_type.clone(),
        expires: time
    };

    match collection.insert_one(document).await {
        Ok(result) => {
            HttpResponse::Ok().body(format!("Create document with uuid {}", result.inserted_id))
        },
        Err(err) => {
            println!("Err: {}", err);
            HttpResponse::Forbidden().body(format!("Can not create document. Err: {}", err))
        }
    }
}

#[post("/upload")]
async fn upload_file(
    dir_files: web::Data<PathBuf>,
    collection: web::Data<Collection<mongo::File>>,
    MultipartForm(form): MultipartForm<forms::UploadForm>
) -> HttpResponse {
    let uploaded_file = form.file;
    let mut dir_files = dir_files.to_path_buf();

    if uploaded_file.size < 1 {
        return HttpResponse::NotAcceptable().body("File not found");
    }

    let token_string = form.token.as_str();
    let file_document = match collection.find_one(doc!{ "token": token_string }).await {
        Ok(result) => result,
        Err(err) => {
            println!("Err: {}", err);
            return HttpResponse::NotAcceptable().body("Token not found, skip");
        }
    };

    if file_document.is_none() {
        return HttpResponse::NotAcceptable().body("Token not found, skip");
    }

    let file_document = file_document.unwrap();

    if file_document.expires < chrono::Utc::now().timestamp() {
        return HttpResponse::Forbidden().body("Session expired");
    }

    if file_document.content_type != uploaded_file.content_type.unwrap().to_string() {
        return HttpResponse::UnsupportedMediaType().body("Content type is missing");
    }

    if file_document.size != uploaded_file.size {
        return HttpResponse::UnsupportedMediaType().body("Size of file is missing");
    }

    if let Err(err) = collection.delete_many(doc!{ "token": token_string }).await {
        println!("Err: {}", err);
    }

    dir_files.push(file_document.path);

    let mut dir_clone = dir_files.clone();
    dir_clone.pop();

    if !dir_clone.exists() {
        if let Err(err) = fs::create_dir_all(&dir_clone) {
            println!("Error in create dir: {}", err);
        }
    }

    drop(dir_clone);

    uploaded_file.file.persist(dir_files).unwrap();

    HttpResponse::Ok().body("Uploaded")
}


fn get_auth_token(req: &HttpRequest) -> Option<&str> {
    req.headers().get("authorization")?.to_str().ok()
}

async fn delete_old_requests(collection: &Collection<mongo::File>) {
    let time = chrono::Utc::now().timestamp();
    _ = collection.delete_many(doc!{"expires": { "$lte": time }}).await;
}

async fn create_client() -> Result<Client, Error>{
    let file: EncryptedFile = include_crypt!("assets/mongodb.txt");
    let decrypted_str = file.decrypt_str();

    if let Err(err) = decrypted_str {
        println!("Error in decrypted mongodb: {}", err);
        process::exit(0x001);
    }

    let client = Client::with_uri_str(decrypted_str.unwrap()).await?;
    Ok(client)
}
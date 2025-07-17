use std::{process, thread, time::Duration};

use include_crypt::{include_crypt, EncryptedFile};
use mongodb::{bson::doc, error as merr, Client, Collection};
use tokio::runtime::Runtime;

mod mongo;
mod cryptor;

#[tokio::main]
async fn main() {
    println!("Connecting to the MongoDB");

    let client = create_client().await.unwrap();
    let db_auth = client.database("auth");
    let db_main = client.database("main");

    let rt = Runtime::new().unwrap();
    {
        let db_clone = db_auth.clone();
        rt.spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(60));

            let collection_temp = db_clone.collection::<mongo::Temp>("Temps");
            let collection_resets = db_clone.collection::<mongo::Reset>("Resets");
            let collection_sessions = db_clone.collection::<mongo::Session>("Sessions");

            loop {
                interval.tick().await;
                delete_old_files(&collection_temp, &collection_resets, &collection_sessions).await;
            }

        });
    }

    println!("Connected to the MongoDB");

    println!("Initializing http server");
}

async fn delete_old_files(
    collection_temp: &Collection<mongo::Temp>,
    collection_resets: &Collection<mongo::Reset>,
    collection_sessions: &Collection<mongo::Session>,
) {
    let time = chrono::Utc::now().timestamp();
    _ = collection_temp.delete_many(doc!{"created": { "$lte": time - 172800 }}).await; // 48 hrs / 60 * 60 * 48
    _ = collection_resets.delete_many(doc!{"created": { "$lte": time - 86400 }}).await; // 24 hrs / 60 * 60 * 24
    _ = collection_sessions.delete_many(doc!{"expires": { "$lte": time }}).await;
}

async fn create_client() -> Result<Client, merr::Error>{
    let file: EncryptedFile = include_crypt!("assets/mongodb.txt");
    let decrypted_str = file.decrypt_str();

    if let Err(err) = decrypted_str {
        println!("Error in decrypted mongodb: {}", err);
        process::exit(0x001);
    }

    let client = Client::with_uri_str(decrypted_str.unwrap()).await?;
    Ok(client)
}
use serde::Deserialize;
use actix_multipart::form::{
    tempfile::TempFile, text::Text, MultipartForm
};

#[derive(Debug, MultipartForm)]
pub struct UploadForm {
    #[multipart(rename = "file")]
    pub file: TempFile,
    pub token: Text<String>,
}

#[derive(Deserialize, Clone)]
pub struct UploadSign {
    pub token: String,
    pub path: String,
    pub size: usize,
    pub content_type: String,
}
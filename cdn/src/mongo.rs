use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct File {
    pub token: String,
    pub path: String,
    pub size: usize,
    pub content_type: String,
    pub expires: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub internal: String
}
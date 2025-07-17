use serde::{Deserialize, Serialize};

/* ------------- auth ------------- */
#[derive(Serialize, Deserialize)]
pub struct Temp {
    pub email: String,
    pub login: String,
    pub pass: String,
    pub nick: String,
    pub created: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Reset {
    pub account: u64,
    pub code: String,
    pub created: i64,
}

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub id: String,
    pub account: u64,
    pub os: String,
    pub browser: String,
    pub location: String,
    pub expires: i64,
    pub hashed_ip: String,
}
/* ------------- auth ------------- */


/* ------------- main ------------- */
#[derive(Serialize, Deserialize)]
pub struct  Account {
    pub id: u64,
    pub tag: String,
    pub email: String,
    pub login: String,
    pub pwd: String,

    pub balance: u32,
    pub nick: String,
    pub avatar: String,
    pub banner: String,

    pub created: i64,
    pub ips: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct  Connects {
    pub id: u64,

    pub discord: String,
    pub steam: String,
    pub telegram: String,
    pub google: String,
    pub github: String,
    pub vk: String,
}
/* ------------- main ------------- */
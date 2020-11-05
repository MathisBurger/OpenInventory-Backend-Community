use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
struct Data {
    database: database,
    access: access
}


#[derive(Serialize, Deserialize)]
struct database {
    username: String,
    password: String,
    database: String,
    host: String
}

#[derive(Serialize, Deserialize)]
struct access {
    #[serde(rename="allow-web")]
    allow_web: bool,
    #[serde(rename="allow-mobile")]
    allow_mobile: bool,
    #[serde(rename="allow-desktop")]
    allow_desktop: bool
}
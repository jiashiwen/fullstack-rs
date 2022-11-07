use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct KV {
    pub Key: String,
    pub Value: String,
}

#[derive(Deserialize, Serialize)]
pub struct Key {
    pub Key: String,
}

#[derive(Deserialize, Serialize)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct UserName {
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ID {
    pub id: String,
}

use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostModel{
    pub title: str,
    pub content: str,
    pub author: str,
    pub likes: i32
}
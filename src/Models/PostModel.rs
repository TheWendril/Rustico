use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostModel{
    pub title: String,
    pub content: String,
    pub author: String,
    pub tag: String,
    pub likes: i32
}
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct PostModel{
    pub id: i8,
    pub title: String,
    pub image: String,
    pub content: String,
    pub author: i8,
    pub tag: String,
    pub likes: i32
}
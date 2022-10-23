use rocket::serde::{Serialize, Deserialize};
use rocket::form::FromForm;

#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(FromForm)]
#[serde(crate = "rocket::serde")]
pub struct PostModel{
    pub title: String,
    pub content: String,
    pub author: String,
    pub likes: i32
}
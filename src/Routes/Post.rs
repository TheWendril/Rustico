use super::super::Models::PostModel::PostModel;
use rocket::serde::{json::Json};


#[get("/post/<post_title>")]
pub fn get_post(post_title: &str) -> String {
    format!("O nome do seu post é: {}!", post_title)
}

#[post("/post", format = "json", data = "<postmodel>")]
pub fn insert_post(postmodel: Json<PostModel>) -> String {

    format!("Sua key é: {}!", postmodel.title)
}

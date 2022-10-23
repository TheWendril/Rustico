use super::super::Models::PostModel::PostModel;
use rocket::form::Form;


#[get("/post/<postTitle>")]
pub fn getpost(postTitle: &str) -> String {
    format!("O nome do seu post é: {}!", postTitle)
}

#[post("/post", data = "<postmodel>")]
pub fn insertPost(postmodel: Form<PostModel>) -> String {

    format!("O nome do seu post é: {}!", postmodel.title)
}

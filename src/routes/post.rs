use models::Post
#[macro_use] extern crate rocket;

#[get("/post/<postTitle>")]
fn getpost(postTitle: &str) -> String {
    format!("O nome do seu post é: {}!", postTitle)
}

#[post("/post", body: Post)]
fn getpost() -> String {
    format!("O nome do seu post é: {}!", postname)
}

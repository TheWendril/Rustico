

#[get("/post/<postTitle>")]
pub fn getpost(postTitle: &str) -> String {
    format!("O nome do seu post é: {}!", postTitle)
}

#[post("/post", data = "<Post>")]
pub fn getpost(post: Json<Post>) -> &'str {
    format!("O nome do seu post é: {}!", postname)
}

#[macro_use] extern crate rocket;

#[get("/post/<postname>")]
fn getpost(postname: &str) -> String {
    format!("O nome do seu post é: {}!", postname)
}

#[post("/post/<postname>")]
fn getpost(postname: &str) -> String {
    format!("O nome do seu post é: {}!", postname)
}

#[get("/")]
fn mainpage() -> &'static str{
    return "Hello World"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![getpost, mainpage])
}

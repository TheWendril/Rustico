#[macro_use] extern crate rocket;
mod Routes;
mod Models;

#[get("/")]
fn mainpage() -> &'static str{
    return "Hello World"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![mainpage, Routes::Post::getpost, Routes::Post::insertPost])
}

#[macro_use] extern crate rocket;
mod Routes;

#[get("/")]
fn mainpage() -> &'static str{
    return "Hello World"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![mainpage])
}

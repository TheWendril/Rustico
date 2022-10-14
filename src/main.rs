#[macro_use] extern crate rocket;

#[get("/")]
fn mainpage() -> &'static str{
    return "Hello World"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![mainpage])
}

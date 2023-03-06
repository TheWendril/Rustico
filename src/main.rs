#[macro_use] extern crate rocket;
mod Routes;
mod Models;
mod DB;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, PUT, DELETE,OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn main_page() -> &'static str{
    return "Rustico API"
}

#[launch]
fn rocket() -> _ {

    rocket::build().mount("/", routes![main_page, Routes::Post::get_post, Routes::Post::insert_post, 
                                       Routes::Post::get_all_posts, Routes::Post::delete_post,
                                       Routes::publisher::get_publisher_by_id, Routes::publisher::post_publisher,
                                       Routes::publisher::get_all_publishers, Routes::publisher::delete_publisher])
                    .attach(CORS)
}

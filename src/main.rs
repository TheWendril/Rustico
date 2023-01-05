

#[macro_use] extern crate rocket;
mod Routes;
mod Models;
mod DB;

#[get("/")]
fn main_page() -> &'static str{
    return "Hello World"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![main_page, Routes::Post::get_post, Routes::Post::insert_post, 
                                       Routes::Post::get_all_posts, Routes::Post::delete_post,
                                       Routes::publisher::get_publisher_by_id, Routes::publisher::post_publisher,
                                       Routes::publisher::get_all_publishers, Routes::publisher::delete_publisher])
}

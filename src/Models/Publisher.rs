use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Publisher{
    id: i8,
    name: String,
    access_level: i8,
    password: String,
    picture: String, 
    bio: String,
    education: String, 
    age: i8
}
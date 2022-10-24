use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Publisher{
    name: String,
    bio: String,
    education: String, 
    age: i16
}
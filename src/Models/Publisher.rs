use rocket::serde::{Serialize, Deserialize, json::Json};

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Publisher{
    name: str,
    education: str, 
    age: i16
}
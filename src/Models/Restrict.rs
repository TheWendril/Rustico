use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RestrictDefault{
    pub key : String
}
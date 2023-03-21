use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct LoginModel{
    pub email: String,
    pub password: String
}


#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TokenModel{
    pub token: String
}
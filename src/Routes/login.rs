use jwt::{Algorithm, Header, Token};
use super::super::Models::login::LoginModel;
use super::super::Models::login::TokenModel;
use super::super::DB::dbconnector;
use rocket::{serde::{json::Json}};
use rocket::serde::json::json;

fn generate_token(email: &str, password: &str) -> Result<String, jwt::errors::Error> {
    let header = Header::new(Algorithm::HS256);
    let claims = json!({
        "email": email,
        "password": password,
        "exp": 60 * 60 * 168 // token expira em 168 horas ou 1 semana 
    });

    Token::new(header, &claims.to_string(), b"secret_key")
        .map(|token| token.compact())
}


#[post("/login", data = "<login>")]
pub fn login(login: Json<LoginModel>) -> Json<TokenModel> {

    let dbcon: dbconnector::DbConnector = dbconnector::DbConnector::new();
    let mut query_string: String = String::from(" SELECT email, password FROM publishers WHERE email = '");   
    query_string.push_str(&login.email);
    query_string.push_str("'");
    
    let mut query_result = dbcon.conn.prepare(&query_string.as_str()).unwrap();

    let mapped_rows = query_result.query_map([], |row| {
        Ok(LoginModel{email: row.get(0)?, password: row.get(1)?})
    }).unwrap();

    let new_login_model: LoginModel = mapped_rows.into_iter()
                                               .next()
                                               .unwrap()
                                               .ok()
                                               .unwrap();    


    if login.email == new_login_model.email && login.password == new_login_model.password {
        let token = generate_token(&login.email, &login.password).unwrap();
        let response = TokenModel{ token: token };
        return Json(response);
    }

    Json(TokenModel{token: "INVÁLIDO".to_string()})
}

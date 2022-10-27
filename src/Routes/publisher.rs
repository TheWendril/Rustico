// Que Deus me perdoe por esse código...
// Escrito por Wendril Avila

use super::super::Models::Publisher::Publisher;
use super::super::DB::dbconnector;
use rocket::{serde::{json::Json}, http::Status};




#[get("/publisher/<publisher_id>")]
pub fn get_publisher_by_id(publisher_id: i8) -> Json<Publisher> {

    let dbcon: dbconnector::DbConnector = dbconnector::DbConnector::new();
    let mut query_string: String = String::from(" SELECT * FROM posts WHERE title = '");   
    query_string.push_str(post_title);
    query_string.push_str("'");
    
    let mut query_result = dbcon.conn.prepare(&query_string.as_str()).unwrap();

    let mapped_rows = query_result.query_map([], |row| {
        Ok(create_post_model(row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?, row.get(4)?, row.get(5)?))
    }).unwrap();

    let new_post_model: PostModel = mapped_rows.into_iter()
                                               .next()
                                               .unwrap()
                                               .ok()
                                               .unwrap();
    return Json(new_post_model)
}
// Que Deus me perdoe por esse código...
// Escrito por Wendril Avila

use super::super::Models::Publisher::{Publisher, PublisherBuilder};
use super::super::DB::dbconnector;
use rocket::{serde::{json::Json}, http::Status};


#[get("/publisher/<publisher_id>")]
pub fn get_publisher_by_id(publisher_id: i8) -> Json<Publisher> {

    let dbcon: dbconnector::DbConnector = dbconnector::DbConnector::new();
    let mut query_string: String = String::from(" SELECT * FROM publishers WHERE id = ");   
    query_string.push_str(publisher_id.as_str());


    let mut query_result = dbcon.conn.prepare(&query_string.as_str()).unwrap();

    let mapped_rows = query_result.query_map([], |row| {
        Ok( PublisherBuilder::new()
                             .id(row.get(0)?)
                             .name(row.get(1)?)
                             .email(row.get(2)?)
                             .access_level(row.get(3)?)
                             .password(row.get(4)?)
                             .picture(row.get(5)?)
                             .bio(row.get(6)?)
                             .education(row.get(7)?)
                             .age(row.get(8)?)
                             .build(row.get(9)?);
        }).unwrap();

    let new_publisher: Publisher = mapped_rows.into_iter()
                                               .next()
                                               .unwrap()
                                               .ok()
                                               .unwrap();
    return Json(new_publisher)
}
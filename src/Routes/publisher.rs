// Que Deus me perdoe por esse código...
// Escrito por Wendril Avila

use super::super::Models::publishermodel::{Publisher, PublisherBuilder};
use super::super::DB::dbconnector;
use rocket::{serde::{json::Json}, http::Status};


fn create_publisher_model(id: i8, name: String, email: String, access_level: i8, password: String, 
                            picture: String, bio: String, education: String, age: i8) -> Publisher{
    Publisher { id: id, name: name, email: email, access_level: access_level, password: password, picture: picture, 
                bio: bio, education: education, age: age }
}

#[get("/publisher/<publisher_id>")]
pub fn get_publisher_by_id(publisher_id: i8) -> Json<Publisher> {

    let dbcon: dbconnector::DbConnector = dbconnector::DbConnector::new();
    let mut query_string: String = String::from(" SELECT * FROM publishers WHERE id = ");   
    query_string.push_str(publisher_id.to_string().as_str());


    let mut query_result = dbcon.conn.prepare(&query_string.as_str()).unwrap();

    let mapped_rows = query_result.query_map([], |row| {

        Ok( 
            Publisher {
                        id: row.get(0)?,
                        name: row.get(1)?,
                        email: row.get(2)?,
                        access_level: row.get(3)?,
                        password: row.get(4)?,
                        picture: row.get(5)?,
                        bio: row.get(6)?,
                        education: row.get(7)?,
                        age: row.get(8)?
                      }
        )
        }).unwrap();

    let new_publisher: Publisher = mapped_rows.into_iter()
                                               .next()
                                               .unwrap()
                                               .ok()
                                               .unwrap();
    return Json(new_publisher)
}



#[post("/publisher", format = "json", data = "<NewPublisher>")]
pub fn post_publisher(NewPublisher: Json<Publisher>) -> Status {

    let dbcon: dbconnector::DbConnector = dbconnector::DbConnector::new();
    let query_execution = dbcon.conn.execute( "INSERT INTO Publishers (id, name, email, access_level, password, picture, bio, education, age) 
                                               VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)", 
                                               (&NewPublisher.id, &NewPublisher.name, &NewPublisher.email, 
                                                &NewPublisher.access_level, &NewPublisher.password, &NewPublisher.picture,
                                                &NewPublisher.bio, &NewPublisher.education, &NewPublisher.age));
    if query_execution.is_err() {return Status::InternalServerError} 
    else {return Status::Ok}
}


#[get("/publisher")]
pub fn get_all_publishers() -> Json<Vec<Publisher>>{

    let dbcon: dbconnector::DbConnector = dbconnector::DbConnector::new();
    let mut statement = dbcon.conn.prepare("SELECT * FROM publishers").unwrap();
    let mut posts_vector: Vec<Publisher> = Vec::new();

    let mapped_rows = statement.query_map([], |row|{
        Ok(create_publisher_model(row.get(0)?, 
                                  row.get(1)?, 
                                  row.get(2)?, 
                                  row.get(3)?, 
                                  "No Access!".to_string(), 
                                  row.get(5)?,
                                  row.get(6)?,
                                  row.get(7)?,
                                  row.get(8)?,))
    });

    for post in mapped_rows.unwrap(){
        posts_vector.push(post.unwrap());
    }

    Json(posts_vector)
}

#[delete("/publisher/<id>")]
pub fn delete_publisher(id: i8) -> Status{

    let dbcon: dbconnector::DbConnector = dbconnector::DbConnector::new();
    let mut query_string: String = String::from(" DELETE FROM posts WHERE id = '");   
    query_string.push_str(id.to_string().as_str());
    query_string.push_str("'");

    let query_result = dbcon.conn.execute(&query_string.as_str(), []);

    if query_result.is_err() {return Status::FailedDependency} 
    else {return Status::Ok}
}
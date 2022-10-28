// Que Deus me perdoe por esse código...
// Escrito por Wendril Avila

use super::super::Models::postmodel::PostModel;
use super::super::DB::dbconnector;
use rocket::{serde::{json::Json}, http::Status};

fn create_post_model(title: String, image: String, content: String, author: i8, tag: String, likes: i32) -> PostModel{
    PostModel { title: title, image: image, content: content, author: author, tag: tag, likes: likes }
}

#[get("/post/<post_title>")]
pub fn get_post(post_title: &str) -> Json<PostModel> {

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

#[post("/post", format = "json", data = "<postmodel>")]
pub fn insert_post(postmodel: Json<PostModel>) -> Status {
    
    let dbcon: dbconnector::DbConnector = dbconnector::DbConnector::new();
    let query_execution = dbcon.conn.execute( "INSERT INTO posts (title, image, content, author, tag, likes) 
                                               VALUES (?1, ?2, ?3, ?4, ?5, ?6)", 
                                               (&postmodel.title, &postmodel.image, &postmodel.content, 
                                                &postmodel.author, &postmodel.tag, &postmodel.likes));
    if query_execution.is_err() {return Status::InternalServerError} 
    else {return Status::Ok}
    
}


#[get("/post")]
pub fn get_all_posts() -> Json<Vec<PostModel>>{

    let dbcon: dbconnector::DbConnector = dbconnector::DbConnector::new();
    let mut statement = dbcon.conn.prepare("SELECT * FROM posts").unwrap();
    let mut posts_vector: Vec<PostModel> = Vec::new();

    let mapped_rows = statement.query_map([], |row|{
        Ok(create_post_model(row.get(0)?, 
                             row.get(1)?, 
                             row.get(2)?, 
                             row.get(3)?, 
                             row.get(4)?, 
                             row.get(5)?))
    });

    for post in mapped_rows.unwrap(){
        posts_vector.push(post.unwrap());
    }

    Json(posts_vector)
}

#[delete("/post/<post_title>")]
pub fn delete_post(post_title: &str) -> Status{

    let dbcon: dbconnector::DbConnector = dbconnector::DbConnector::new();
    let mut query_string: String = String::from(" DELETE FROM posts WHERE title = '");   
    query_string.push_str(post_title);
    query_string.push_str("'");

    let query_result = dbcon.conn.execute(&query_string.as_str(), []);

    if query_result.is_err() {return Status::FailedDependency} 
    else {return Status::Ok}
 
}
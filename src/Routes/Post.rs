// Que Deus me perdoe por esse código...
// Escrito por Wendril Avila

use super::super::Models::PostModel::PostModel;
use super::super::DB::dbconnector;
use rocket::serde::{json::Json};
use rusqlite::{NO_PARAMS};

#[get("/post/<post_title>")]
pub fn get_post(post_title: &str) -> Json<PostModel> {

    let dbcon: dbconnector::DbConnector = dbconnector::DbConnector::new();
    let mut queryString: String = String::from(" SELECT * FROM posts WHERE title = ");   
    queryString.push_str(post_title);
    
    let mut queryResult = dbcon.conn.prepare(&queryString.as_str()).unwrap();
    queryResult.column_count();

    let mappedRows = queryResult.query_map(NO_PARAMS, |row| {
        Ok(PostModel{
            title: row.get(0)?,
            image: row.get(1)?,
            content: row.get(2)?,
            author: row.get(3)?,
            tag: row.get(4)?,
            likes: row.get(5)?
        })
    }).unwrap();

    let mut newPostModel: PostModel = mappedRows.into_iter()
                                                .next()
                                                .unwrap()
                                                .ok()
                                                .unwrap();
    return Json(newPostModel)
}

#[post("/post", format = "json", data = "<postmodel>")]
pub fn insert_post(postmodel: Json<PostModel>) -> String {
    
    let dbcon: dbconnector::DbConnector = dbconnector::DbConnector::new();
    dbcon.conn.execute( "INSERT INTO posts (title, image, content, author, tag, likes) 
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6)", 
                        (&postmodel.title, &postmodel.image, &postmodel.content, 
                         &postmodel.author, &postmodel.tag, &postmodel.likes));

    return format!("Sua key é: {}!", postmodel.title)
}

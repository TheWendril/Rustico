use rusqlite;
use std::env;

pub struct DbConnector{
    pub conn: rusqlite::Connection 
}

impl DbConnector{    
    pub fn new() -> Self{

        let db_path = env::var("DB_PATH").unwrap();
        Self { conn: rusqlite::Connection::open(db_path).unwrap() }
    }
}

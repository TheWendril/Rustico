use rusqlite;

pub struct DbConnector{
    pub conn: rusqlite::Connection 
}

impl DbConnector{    
    pub fn new() -> Self{
        Self { conn: rusqlite::Connection::open("~/rusticodb.db").unwrap() }
    }
}

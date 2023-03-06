use rusqlite;
use dotenv::dotenv;

pub struct DbConnector{
    pub conn: rusqlite::Connection 
}

impl DbConnector{    
    pub fn new() -> Self{

        dotenv().ok();
        let db_path = std::env::var("DBPATH").expect("Especifique o caminho do banco");
        Self { conn: rusqlite::Connection::open(db_path).unwrap() }
    }
}

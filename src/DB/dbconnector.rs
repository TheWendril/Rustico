use rocket::http::private::Connection;
use sqlite;


struct DbConnector{
    
    conn: sqlite::Connection
 
}

impl DbConnector{    

    fn new() -> Self{
           
        Self { conn: sqlite::open("~/rusticodb").unwrap() }
    }

}

use mongodb::{bson::doc, Client};

struct DBConnector{
    client : Client
}


impl DBConnector{
    
    pub fn new() -> Self {
        Self{client: Client::with_uri_str("mongodb+srv://Wendril:<flamengo10>@cluster0.jbjhg.gcp.mongodb.net/?retryWrites=true&w=majority")}
    }
}
use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize)]
#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Publisher{
    id: i8,
    name: String,
    email: String,
    access_level: i8,
    password: String,
    picture: String, 
    bio: String,
    education: String, 
    age: i8
}


impl Publisher for PublisherBuilder{

    pub fn new() -> PublisherBuilder{
        
        PublisherBuilder{
            id: 0, name: "", email: "", access_level: 0, 
            password: "", picture: "", bio: "", education: "", age: 0
        }
    }

    pub fn id(mut self, id: i8) -> PublisherBuilder{
        self.id = id;
        self
    }
    
    pub fn name(mut self, name: String) -> PublisherBuilder{
        self.name = name;
        self
    }

    pub fn email(mut self, email: String) -> PublisherBuilder{
        self.email = email;
        self
    }

    pub fn email(mut self, email: String) -> PublisherBuilder{
        self.email = email;
        self
    }
    
    pub fn access_level(mut self, access_level: i8) -> PublisherBuilder{
        self.access_level = access_level;
        self
    }
    
    pub fn password(mut self, password: String) -> PublisherBuilder{
        self.password = password;
        self
    }

    pub fn picture(mut self, picture: String) -> PublisherBuilder{
        self.picture = picture;
        self
    }

    pub fn bio(mut self, bio: String) -> PublisherBuilder{
        self.bio = bio;
        self
    }

    pub fn education(mut self, education: String) -> PublisherBuilder{
        self.education = education;
        self
    }

    pub fn age(mut self, age: i8) -> PublisherBuilder{
        self.age = age;
        self
    }

    pub fn build(self) -> Publisher{
        
        Publisher{
            id: self.id, name: self.name, email: self.email, access_level: self.access_level, 
            password: self.password, picture: self.picture, bio: self.bio, education: self.education, age: self.age
        }
    }

}
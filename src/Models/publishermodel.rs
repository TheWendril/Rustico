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

// Código Inútil mas que não vou remover por enquanto ----------------->
pub trait PublisherBuilder{
    fn new() -> Publisher;
    fn id(self, id: i8) -> Self;
    fn name(self, name: String) -> Self;
    fn email(self, email: String) -> Self;
    fn access_level(self, access_level: i8) -> Self;
    fn password(self, password: String) -> Self;
    fn picture(self, picture: String) -> Self;
    fn bio(self, bio: String) -> Self;
    fn education(self, education: String) -> Self;
    fn age(self, age: i8) -> Self;
    fn build(self) -> Publisher;

}

impl PublisherBuilder for Publisher{

    fn new() -> Publisher{
        
        Publisher{
            id: 0, name: String::from(""), email: String::from(""), access_level: 0, 
            password: String::from(""), picture: String::from(""), bio: String::from(""), 
            education: String::from(""), age: 0
        }
    }

    fn id(mut self, id: i8) -> Self{
        self.id = id;
        self
    }
    
    fn name(mut self, name: String) -> Self{
        self.name = name;
        self
    }

    fn email(mut self, email: String) -> Self{
        self.email = email;
        self
    }
    
    fn access_level(mut self, access_level: i8) -> Self{
        self.access_level = access_level;
        self
    }
    
    fn password(mut self, password: String) -> Self{
        self.password = password;
        self
    }

    fn picture(mut self, picture: String) -> Self{
        self.picture = picture;
        self
    }

    fn bio(mut self, bio: String) -> Self{
        self.bio = bio;
        self
    }

    fn education(mut self, education: String) -> Self{
        self.education = education;
        self
    }

    fn age(mut self, age: i8) -> Self{
        self.age = age;
        self
    }

    fn build(self) -> Publisher{
        
        Publisher{
            id: self.id, name: self.name, email: self.email, access_level: self.access_level, 
            password: self.password, picture: self.picture, bio: self.bio, education: self.education, age: self.age
        }
    }

}
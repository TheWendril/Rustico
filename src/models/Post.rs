use models::Publisher;

pub struct Post{
    pub title: str,
    pub content: str,
    pub author: Publisher,
    pub likes: i32
}
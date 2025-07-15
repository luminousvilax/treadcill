use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Work {
    pub title: String,
    pub content: String
}

impl Work {
    pub fn new(title: String, content: String) -> Self{
        Self {title, content}
    }

    pub fn show(&self){
        println!("------------------");
        println!("Title: {}\nContent: {}", self.title, self.content);
    }
}

#[derive(Debug)]
pub struct Work {
    title: String,
    content: String
}

impl Work {
    pub fn new(title: String, content: String) -> Self{
        Self {title, content}
    }
}

impl Work {
    pub fn show(&self){
        println!("------------------");
        println!("Title: {}\nContent: {}", self.title, self.content);
    }
}
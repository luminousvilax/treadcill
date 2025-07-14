#[derive(Debug)]
struct Work {
    title: String,
    content: String
}

impl Work {
    pub fn new(title: String, content: String) -> Self{
        Self {title, content}
    }
}

impl Work {
    fn show(&self){
        println!("------------------");
        println!("Title: {}\nContent: {}", self.title, self.content);
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg_len = args.len();
    if arg_len < 2{
        println!("Usage: [title] content");
        return;
    }
    // init
    let mut works: Vec<Work> = vec![];
    if arg_len >= 3{
        works.push(Work::new(args[1].clone(), args[2].clone()));
    } else if args[1].clone() == "create" {
        interactive();
        return;
    } 
    else{
        works.push(Work::new(String::new(), args[1].clone()));
    }
    println!("Todo list:");
    for work in &works {
        work.show();
    }
}

fn interactive() {
    
}

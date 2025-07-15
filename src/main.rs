use std::io::{self, Write};

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
    match args[1].clone().as_str() {
        "create" => {
            works = interactive();
        }
        "list" => {
            println!("Todo list:");
            for work in &works {
                work.show();
            }
        }
        _ => {
            if arg_len >= 3{
                works.push(Work::new(args[1].clone(), args[2].clone()));
            } else{
                works.push(Work::new(String::new(), args[1].clone()));
            }
        }
    }
    println!("Todo list:");
    for work in &works {
        work.show();
    }
}

fn interactive() -> Vec<Work> {
    // init
    let mut works: Vec<Work> = vec![];
    let mut inputs: Vec<String> = vec![];
    loop {
        if inputs.len() == 0{
            println!("Please input todo title and content:");
            
            print("title(Enter to keep empty): ");
            let mut title = String::new();
            std::io::stdin().read_line(&mut title).expect("failed to read title");
            inputs.push(title.trim().to_string());
        }
        if inputs.len() == 1{
            print("content: ");
            let mut content = String::new();
            std::io::stdin().read_line(&mut content).expect("failed to read content");
            content = content.trim().to_string();
            if content == ""{
                continue;
            }
            inputs.push(content);
        } 
        if inputs.len() == 2{
            let new_work = Work::new(inputs[0].clone(), inputs[1].clone());
            // make sure
            new_work.show();
            println!("------------------");
            print("Are you sure to create this work? [y/n] ");
            let mut ok = String::new();
            std::io::stdin().read_line(&mut ok).expect("failed to read sure code");
            ok = ok.trim().to_lowercase();
            if ok == "y"{
                works.push(new_work);
                inputs.clear();
            } else if ok == "n" {
                inputs.clear();
            } else {
                continue;
            }
        }
        // once more?
        print("Add another work? [y/n] ");
        let mut ok = String::new();
        std::io::stdin().read_line(&mut ok).expect("failed to read sure code");
        ok = ok.trim().to_lowercase();
        println!("");
        if ok == "y" {
            continue;
        } else {
            break;
        }
    }

    return works;
}

fn print(msg: &str) {
    let mut lock = io::stdout().lock();
    write!(lock, "{msg}").unwrap();
    io::stdout().flush().expect("failed to flush stdout");
}

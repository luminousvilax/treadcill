mod todo;
use crate::todo::core::Work;
use crate::todo::create::interactive;
use crate::todo::list::todo_list;
use crate::todo::storage::{read_works, save_works};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let arg_len = args.len();
    if arg_len < 2 {
        println!("Usage: [title] content");
        return;
    }
    // init
    let file_path = "data/todo.json";
    let mut works: Vec<Work> = read_works(file_path);
    match args[1].clone().as_str() {
        "create" => {
            let mut new_works = interactive();
            works.append(&mut new_works);
        }
        "list" => {
            todo_list(&works);
        }
        _ => {
            if arg_len >= 3 {
                works.push(Work::new(args[1].clone(), args[2].clone()));
            } else {
                works.push(Work::new(String::new(), args[1].clone()));
            }
        }
    }
    save_works(file_path, &works);
}

mod todo;
use crate::todo::core::{TodoCommand, Work};
use crate::todo::storage::{read_works, save_works};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = "Todo cli")]
struct Program {
    #[command(subcommand)]
    pub command: TodoCommand,
}

fn main() {
    let args = Program::parse();
    // init
    let file_path = "data/todo.json";
    let mut works: Vec<Work> = read_works(file_path);
    match args.command {
        TodoCommand::Create {
            interactive_mode,
            title,
            content,
        } => {
            if interactive_mode {
                let mut new_works = todo::create::interactive();
                works.append(&mut new_works);
            } else {
                works.push(Work::new(title.unwrap_or_default(), content));
            }
        }
        TodoCommand::List { title, content } => {
            todo::list::todo_list(&works, title, content);
            return;
        }
        TodoCommand::Edit { index, title, content } => {
            if let Err(e) = todo::edit::edit_work(&mut works, index-1, title, content) {
                eprintln!("Error editing work: {}", e);
            }
        }
        TodoCommand::Clear { number } => {
            let num = number.unwrap_or_default();
            todo::clear::clear_todo(&mut works, num);
        }
    }
    save_works(file_path, &works).expect("save todos error");
}

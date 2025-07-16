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
        TodoCommand::List => {
            todo::list::todo_list(&works);
        }
        TodoCommand::Clear {
            number
        } => {
            let num = number.unwrap_or_default();
            if num == 0 {
                works.clear();
            } else if num <= works.len() {
                let cleared = works.remove(num - 1);
                println!("Cleared!");
                cleared.show();
            }
        }
    }
    save_works(file_path, &works);
}

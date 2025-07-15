use std::io::{self, Write};
use super::core::Work;

pub fn interactive() -> Vec<Work> {
    // init
    let mut works: Vec<Work> = vec![];
    let mut inputs: Vec<String> = vec![];
    loop {
        if inputs.len() == 0 {
            println!("Please input todo title and content:");

            print("title(Enter to keep empty): ");
            let mut title = String::new();
            std::io::stdin()
                .read_line(&mut title)
                .expect("failed to read title");
            inputs.push(title.trim().to_string());
        }
        if inputs.len() == 1 {
            print("content: ");
            let mut content = String::new();
            std::io::stdin()
                .read_line(&mut content)
                .expect("failed to read content");
            content = content.trim().to_string();
            if content == "" {
                continue;
            }
            inputs.push(content);
        }
        if inputs.len() == 2 {
            let new_work = Work::new(inputs[0].clone(), inputs[1].clone());
            // make sure
            new_work.show();
            println!("------------------");
            print("Are you sure to create this work? [y/n] ");
            let mut ok = String::new();
            std::io::stdin()
                .read_line(&mut ok)
                .expect("failed to read sure code");
            ok = ok.trim().to_lowercase();
            if ok == "y" {
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
        std::io::stdin()
            .read_line(&mut ok)
            .expect("failed to read sure code");
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
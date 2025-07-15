use super::core::Work;
use std::fs;

pub fn read_works(file_path: &str) -> Vec<Work> {
    let mut works: Vec<Work> = vec![];
    match fs::read_to_string(file_path) {
        Ok(content) => match serde_json::from_str(content.as_str()) {
            Ok(mut work_list) => works.append(&mut work_list),
            _ => {
                println!("parse file {file_path} error!")
            }
        },
        _ => {
            println!("read file {file_path} error!")
        }
    }

    return works;
}

pub fn save_works(file_path: &str, works: &Vec<Work>) {
    let data = serde_json::to_string(works).unwrap();
    fs::write(file_path, data).unwrap();
}

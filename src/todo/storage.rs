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

pub fn save_works(file_path: &str, works: &Vec<Work>) -> Result<(), String> {
    let data = serde_json::to_string(works).map_err(|e| e.to_string())?;
    fs::write(file_path, data).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn read_works_nonexistent_file_returns_empty() {
        let path = "data/unlikely_to_exist_001.json";
        let works = read_works(path);
        assert!(works.is_empty());
    }

    #[test]
    fn read_works_with_invalid_json_returns_empty() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(tmp, "{{ invalid json").unwrap();
        let path = tmp.path().to_str().unwrap();
        let works = read_works(path);
        assert!(works.is_empty());
    }

    #[test]
    fn save_and_read_empty_works_roundtrip() {
        let tmp = NamedTempFile::new().unwrap();
        let path = tmp.path().to_str().unwrap();
        let mut works: Vec<Work> = Vec::new();
        works.push(Work::new("Test Title", "Test Content"));
        save_works(path, &works).unwrap();
        let loaded = read_works(path);
        assert_eq!(loaded, works);
    }

    #[test]
    fn save_works_to_bad_path_returns_err() {
        let path = "data/directory/test.json";
        let works: Vec<Work> = vec![];
        let result = save_works(path, &works);
        assert!(result.is_err());
    }
}

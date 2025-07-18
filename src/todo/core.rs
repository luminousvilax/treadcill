use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Work {
    pub title: String,
    pub content: String,
}

impl Work {
    pub fn new<T: Into<String>>(title: T, content: T) -> Self {
        Self { title: title.into(), content: content.into() }
    }

    pub fn show(&self) {
        println!("Title: {}\nContent: {}", self.title, self.content);
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum TodoCommand {
    /// Add todo
    Create {
        /// create in interactive mode
        #[arg(short)]
        interactive_mode: bool,
        /// The title of the todo (optional)
        #[arg(short, long)]
        title: Option<String>,
        /// The content of the todo
        #[arg(short, long)]
        content: String,
    },
    /// List all the todo works
    List {
        /// Text of title contains
        #[arg(short, long)]
        title: Option<String>,
        /// Text of content contains
        #[arg(short, long)]
        content: Option<String>,
    },
    /// Clear works, if do not supply number, clear all of them
    Clear {
        /// select which work to clear
        #[arg(short, long)]
        number: Option<usize>
    }
}


#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::Work;

    #[test]
    fn test_work_creation() {
        let work_str = Work::new("test title", "test content");
        assert_eq!(work_str.title, "test title");
        assert_eq!(work_str.content, "test content");
        let work_string = Work::new(String::from_str("string title").unwrap(), String::from_str("string content").unwrap());
        assert_eq!(work_string.title, "string title");
        assert_eq!(work_string.content, "string content");
    }
}

use clap::Subcommand;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Work {
    pub title: String,
    pub content: String,
}

impl Work {
    pub fn new(title: String, content: String) -> Self {
        Self { title, content }
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

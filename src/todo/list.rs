use super::core::Work;

pub struct WorkFilter {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl WorkFilter {
    pub fn new(title: Option<String>, content: Option<String>) -> Self {
        WorkFilter { title, content }
    }

    pub fn filter(&self, works: &Vec<Work>) -> bool {
        if self.title.is_none() && self.content.is_none() {
            return false;
        } else {
            for (index, work) in works.iter().enumerate() {
                let has_title = self.title.as_ref().is_some_and(|v| work.title.contains(v));
                let has_content = self
                    .content
                    .as_ref()
                    .is_some_and(|v| work.content.contains(v));
                if has_title || has_content {
                    work_detail(work, index);
                }
            }
            return true;
        }
    }
}

pub fn todo_list(works: &Vec<Work>, title: Option<String>, content: Option<String>) {
    if works.len() == 0 {
        println!("Nothing to do, have fun!");
        return;
    }
    // filter works if needed
    let work_filter = WorkFilter::new(title, content);
    if work_filter.filter(works) {
        return;
    }
    println!("Todo list:");
    for (index, work) in works.iter().enumerate() {
        work_detail(work, index);
    }
}

fn work_detail(work: &Work, index: usize) {
    println!("No.{}: ", index + 1);
    println!("------------------");
    work.show();
    println!("------------------");
}

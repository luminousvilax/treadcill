use super::core::Work;

pub fn todo_list(works: &Vec<Work>) {
    println!("Todo list:");
    for work in works {
        work.show();
    }
}

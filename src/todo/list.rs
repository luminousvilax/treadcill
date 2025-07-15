use super::core::Work;

pub fn todo_list(works: &Vec<Work>) {
    if works.len() == 0 {
        println!("Nothing to do, have fun!");
        return;
    }
    println!("Todo list:");
    for work in works {
        work.show();
    }
}

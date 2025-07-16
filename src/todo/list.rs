use super::core::Work;

pub fn todo_list(works: &Vec<Work>) {
    if works.len() == 0 {
        println!("Nothing to do, have fun!");
        return;
    }
    println!("Todo list:");
    for (index, work) in works.iter().enumerate() {
        println!("No.{}: ", index + 1);
        println!("------------------");
        work.show();
        println!("------------------");
    }
}

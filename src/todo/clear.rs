use super::core::Work;

pub fn clear_todo(works: &mut Vec<Work>, num: usize) {
    if num == 0 {
        works.clear();
    } else if num <= works.len() {
        let cleared = works.remove(num - 1);
        println!("Cleared!");
        cleared.show();
    }
}


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

#[cfg(test)]
mod tests {
    use super::clear_todo;
    use super::Work;

    fn make_works() -> Vec<Work> {
        vec![
            Work::new("First", "have breakfast"),
            Work::new("Second", "ride bike"),
            Work::new("Third", "have shower"),
        ]
    }

    #[test]
    fn clear_all_when_zero() {
        let mut works = make_works();
        clear_todo(&mut works, 0);
        assert!(works.is_empty(), "Expected all items to be cleared");
    }

    #[test]
    fn clear_specific_item() {
        let mut works = make_works();
        clear_todo(&mut works, 2);
        // After removing the second item, only "First" and "Third" remain
        let remaining: Vec<String> = works.iter().map(|w| w.title.clone()).collect();
        assert_eq!(remaining, vec!["First".to_string(), "Third".to_string()]);
    }

    #[test]
    fn clear_out_of_bounds_does_nothing() {
        let mut works = make_works();
        let before = works.clone();
        clear_todo(&mut works, 10);
        assert_eq!(works, before, "No item should be removed when index is too large");
    }

    #[test]
    fn clear_only_item() {
        let mut works = vec![Work::new("Solo", "Sleep")];
        clear_todo(&mut works, 1);
        assert!(works.is_empty(), "The single item should be removed");
    }
}


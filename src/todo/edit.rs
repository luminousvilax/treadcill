use super::core::Work;
use super::list::work_detail;

pub fn edit_work(
    works: &mut Vec<Work>,
    index: usize,
    new_title: Option<String>,
    new_content: Option<String>,
) -> Result<(), String> {
    if index >= works.len() {
        return Err(format!(
            "Index {} out of bounds for works of length {}",
            index,
            works.len()
        ));
    }

    let mut changed = false;
    if let Some(title) = new_title {
        works[index].title = title;
        changed = true;
    }
    if let Some(content) = new_content {
        works[index].content = content;
        changed = true;
    }

    if changed {
        work_detail(&works[index], index);
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;

    fn make_work(title: &str, content: &str) -> Work {
        Work {
            title: title.to_string(),
            content: content.to_string(),
        }
    }

    #[test]
    fn test_edit_work_both_fields() {
        let mut works = vec![
            make_work("Old1", "Content1"),
            make_work("Old2", "Content2"),
        ];
        let res = edit_work(
            &mut works,
            1,
            Some("New2".to_string()),
            Some("NewContent2".to_string()),
        );
        assert!(res.is_ok());
        assert_eq!(works[1].title, "New2");
        assert_eq!(works[1].content, "NewContent2");
        // the other element remains unchanged
        assert_eq!(works[0].title, "Old1");
        assert_eq!(works[0].content, "Content1");
    }

    #[test]
    fn test_edit_work_title_only() {
        let mut works = vec![make_work("Old", "Content")];
        let res = edit_work(&mut works, 0, Some("Updated".to_string()), None);
        assert!(res.is_ok());
        assert_eq!(works[0].title, "Updated");
        assert_eq!(works[0].content, "Content");
    }

    #[test]
    fn test_edit_work_content_only() {
        let mut works = vec![make_work("Old", "Content")];
        let res = edit_work(&mut works, 0, None, Some("NewContent".to_string()));
        assert!(res.is_ok());
        assert_eq!(works[0].title, "Old");
        assert_eq!(works[0].content, "NewContent");
    }

    #[test]
    fn test_edit_work_no_changes() {
        let mut works = vec![make_work("Title", "Body")];
        let original = works.clone();
        let res = edit_work(&mut works, 0, None, None);
        assert!(res.is_ok());
        // nothing should change
        assert_eq!(works, original);
    }

    #[test]
    fn test_edit_work_index_out_of_bounds() {
        let mut works = vec![make_work("Only", "One")];
        let err = edit_work(&mut works, 1, Some("X".to_string()), None).unwrap_err();
        assert_eq!(
            err,
            format!("Index 1 out of bounds for works of length {}", works.len())
        );
    }
}

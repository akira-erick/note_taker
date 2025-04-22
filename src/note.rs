pub struct Note {
    title: String,
    content: String,
    date_time: String,
}

impl Note {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_have_title() {
        let note = Note::new(
            "Test Title".to_string(),
            "Test Content".to_string(),
            "2023-10-01 12:00".to_string(),
        );
        assert_eq!(note.get_title(), "Test Title");
    }
}

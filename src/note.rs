pub struct Note {
    title: String,
    content: String,
    date_time: String,
}

impl Note {
    pub fn new(title: String, content: String, date_time: String) -> Self {
        Note {
            title,
            content,
            date_time,
        }
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn get_date_time(&self) -> &String {
        &self.date_time
    }
}

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

    #[test]
    fn test_should_have_content() {
        let note = Note::new(
            "Test Title".to_string(),
            "Test Content".to_string(),
            "2023-10-01 12:00".to_string(),
        );
        assert_eq!(note.get_content(), "Test Content");
    }

    #[test]
    fn test_should_have_date_time() {
        let note = Note::new(
            "Test Title".to_string(),
            "Test Content".to_string(),
            "2023-10-01 12:00".to_string(),
        );
        assert_eq!(note.get_date_time(), "2023-10-01 12:00");
    }
}

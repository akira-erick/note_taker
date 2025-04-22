use chrono::prelude::*;

pub struct Note {
    title: String,
    content: String,
    date_time: NaiveDateTime,
}

impl Note {
    pub fn new(title: String, content: String) -> Self {
        Note::new_private(title, content, Utc::now().naive_utc())
    }

    fn new_private(title: String, content: String, date_time: NaiveDateTime) -> Self {
        Note {
            title,
            content,
            date_time,
        }
    }

    pub fn get_title(&self) -> &str {
        &self.title
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_date_time(&self) -> String {
        self.date_time.format("%Y-%m-%d %H:%M").to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_have_title() {
        let note = Note::new("Test Title".to_string(), "Test Content".to_string());
        assert_eq!(note.get_title(), "Test Title");
    }

    #[test]
    fn test_should_have_content() {
        let note = Note::new("Test Title".to_string(), "Test Content".to_string());
        assert_eq!(note.get_content(), "Test Content");
    }

    #[test]
    fn test_should_have_date_time() {
        let now = Utc::now().naive_utc();

        let note = Note::new_private(
            "Test Title".to_string(),
            "Test Content".to_string(),
            Utc::now().naive_utc(),
        );
        assert_eq!(
            note.get_date_time(),
            now.format("%Y-%m-%d %H:%M").to_string()
        );
    }
}

use chrono::prelude::*;
use std::fmt;

pub struct Note {
    title: String,
    content: String,
    date_time: NaiveDateTime,
}

impl Note {
    pub fn new(mut title: String, mut content: String) -> Self {
        title = title.trim().to_string();

        if title.is_empty() {
            panic!("Title cannot be empty or blank");
        }

        content = content.trim().to_string();

        Note::new_private(title, content, Local::now().naive_local())
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

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title: {}\nContent: {}\nDate Time: {}",
            self.get_title(),
            self.get_content(),
            self.get_date_time()
        )
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
        let now = Local::now().naive_local();

        let note = Note::new_private("Test Title".to_string(), "Test Content".to_string(), now);
        assert_eq!(
            note.get_date_time(),
            now.format("%Y-%m-%d %H:%M").to_string()
        );
    }

    #[test]
    #[should_panic(expected = "Title cannot be empty or blank")]
    fn test_should_panic_if_title_is_empty() {
        let note = Note::new("   ".to_string(), "Test Content".to_string());
        assert_eq!(note.get_title(), "");
    }

    #[test]
    fn test_note_to_string() {
        let now = Utc::now().naive_utc();
        let note = Note::new_private("Test Title".to_string(), "Test Content".to_string(), now);

        let expected_string = format!(
            "Title: {}\nContent: {}\nDate Time: {}",
            note.get_title(),
            note.get_content(),
            note.get_date_time()
        );
        let note_string = note.to_string();

        assert_eq!(note_string, expected_string);
    }

    #[test]
    fn test_notes_with_same_values_are_equal() {
        let now = Utc::now().naive_utc();

        let note1 = Note::new_private("Test Title".to_string(), "Test Content".to_string(), now);
        let note2 = Note::new_private("Test Title".to_string(), "Test Content".to_string(), now);

        assert_eq!(note1, note2);
    }
}

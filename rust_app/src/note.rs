use chrono::prelude::*;
use std::cmp::Ordering;
use std::fmt;

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Note {
    title: String,
    content: String,
    date_time: NaiveDateTime,
}

impl Note {
    pub fn new(mut title: String, mut content: String) -> Result<Note, String> {
        title = title.trim().to_string();

        if title.is_empty() {
            return Err("Title cannot be empty or blank".to_string());
        }

        content = content.trim().to_string();

        Ok(Note::new_private(
            title,
            content,
            Note::get_rounded_date_time(),
        ))
    }

    pub fn new_with_date_time(
        mut title: String,
        mut content: String,
        date_time: String,
    ) -> Result<Note, String> {
        title = title.trim().to_string();

        if title.is_empty() {
            return Err("Title cannot be empty or blank".to_string());
        }

        content = content.trim().to_string();

        let date_time = NaiveDateTime::parse_from_str(&date_time, "%Y-%m-%d %H:%M")
            .map_err(|_| "Invalid date time format")?;

        Ok(Note::new_private(title, content, date_time))
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
        Self::format_date_time(self.date_time)
    }

    pub fn get_as_json(&self) -> String {
        format!(
            r#"{{"title":"{}" ,"content":"{}", "date_time":"{}"}}"#,
            self.get_title(),
            self.get_content(),
            self.get_date_time()
        )
    }

    fn get_rounded_date_time() -> NaiveDateTime {
        let now = Local::now().naive_local();
        NaiveDateTime::new(
            now.date(),
            NaiveTime::from_hms_opt(now.hour(), now.minute(), 0).unwrap(),
        )
    }

    fn format_date_time(date_time: NaiveDateTime) -> String {
        date_time.format("%Y-%m-%d %H:%M").to_string()
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

impl Ord for Note {
    fn cmp(&self, other: &Self) -> Ordering {
        self.date_time
            .cmp(&other.date_time)
            .then_with(|| self.title.cmp(&other.title))
            .then_with(|| self.content.cmp(&other.content))
    }
}

impl PartialOrd for Note {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_have_title() {
        let note = Note::new("Test Title".to_string(), "Test Content".to_string()).unwrap();
        assert_eq!(note.get_title(), "Test Title");
    }

    #[test]
    fn test_should_have_content() {
        let note = Note::new("Test Title".to_string(), "Test Content".to_string()).unwrap();
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
        let _ = Note::new("   ".to_string(), "Test Content".to_string()).unwrap(); // This should panic
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

    #[test]
    fn test_notes_should_be_ordered_by_date_time() {
        let now = Utc::now().naive_utc();
        let note1 = Note::new_private("Note 1".to_string(), "Content 1".to_string(), now);
        let note2 = Note::new_private(
            "Note 2".to_string(),
            "Content 2".to_string(),
            now + chrono::Duration::hours(1),
        );

        assert!(note1 < note2);
    }

    #[test]
    fn test_notes_in_vector_should_be_sorted_after_sort_function() {
        let now = Utc::now().naive_utc();
        let note1 = Note::new_private("Note 1".to_string(), "Content 1".to_string(), now);
        let note2 = Note::new_private(
            "Note 2".to_string(),
            "Content 2".to_string(),
            now + chrono::Duration::hours(1),
        );
        let note3 = Note::new_private(
            "Note 3".to_string(),
            "Content 3".to_string(),
            now + chrono::Duration::hours(2),
        );

        let mut notes = [note3.clone(), note1.clone(), note2.clone()];

        notes.sort();

        assert_eq!(notes[0], note1);
        assert_eq!(notes[1], note2);
        assert_eq!(notes[2], note3);
    }

    #[test]
    fn test_notes_with_same_date_time_should_be_ordered_by_title() {
        let now = Utc::now().naive_utc();
        let note1 = Note::new_private("Note A".to_string(), "Content 1".to_string(), now);
        let note2 = Note::new_private("Note B".to_string(), "Content 1".to_string(), now);

        assert!(note1 < note2);
    }

    #[test]
    fn test_notes_with_same_date_time_and_title_should_be_ordered_by_content() {
        let now = Utc::now().naive_utc();
        let note1 = Note::new_private("Note".to_string(), "Content A".to_string(), now);
        let note2 = Note::new_private("Note".to_string(), "Content B".to_string(), now);

        assert!(note1 < note2);
    }

    #[test]
    fn test_notes_to_json() {
        let now = Utc::now().naive_utc();
        let note = Note::new_private("Test Title".to_string(), "Test Content".to_string(), now);

        let expected_json = format!(
            r#"{{"title":"{}" ,"content":"{}", "date_time":"{}"}}"#,
            note.get_title(),
            note.get_content(),
            note.get_date_time()
        );

        assert_eq!(note.get_as_json(), expected_json);
    }
}

mod note;

struct NoteTaker {
    notes: Vec<Note>,
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_should_store_notes() {
        let mut note_taker = NoteTaker::new();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string());
        let note2 = Note::new("Title 2".to_string(), "Content 2".to_string());

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        assert_eq!(note_taker.get_notes().len(), 2);
        assert_eq!(note_taker.get_notes()[0], note1);
        assert_eq!(note_taker.get_notes()[1], note2);
    }
}
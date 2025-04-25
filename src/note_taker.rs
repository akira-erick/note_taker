use crate::note::Note;

pub struct NoteTaker {
    notes: Vec<Note>,
}

impl NoteTaker {
    pub fn new() -> Self {
        NoteTaker { notes: Vec::new() }
    }

    pub fn add_note(&mut self, note: Note) {
        self.notes.push(note);
    }

    pub fn get_notes(&self) -> Vec<Note> {
        self.notes.clone()
    }
}

#[cfg(test)]
mod tests {
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

    #[test]
    fn test_should_not_allow_duplicate_notes() {
        let mut note_taker = NoteTaker::new();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string());

        note_taker.add_note(note1.clone());
        note_taker.add_note(note1.clone());

        assert_eq!(note_taker.get_notes().len(), 1);
        assert_eq!(note_taker.get_notes()[0], note1);
    }
}

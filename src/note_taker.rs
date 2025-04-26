use crate::note::Note;

#[derive(Debug)]
pub struct NoteTaker {
    notes: Vec<Note>,
    size: usize,
}

impl NoteTaker {
    pub fn new() -> Self {
        NoteTaker {
            notes: Vec::new(),
            size: 0,
        }
    }

    pub fn add_note(&mut self, note: Note) {
        if self.notes.binary_search(&note).is_ok() {
            return;
        }
        let idx = self.notes.partition_point(|n| n < &note);
        self.notes.insert(idx, note.clone());
        self.size += 1;
    }

    pub fn get_notes(&self) -> Vec<Note> {
        self.notes.clone()
    }

    pub fn get_note(&self, index: usize) -> Note {
        if index >= self.get_size() {
            panic!("Index out of bounds");
        }
        self.notes[index].clone()
    }

    pub fn get_by_title(&self, title: &str) -> Vec<usize> {
        let mut result = Vec::new();
        for (i, note) in self.notes.iter().enumerate() {
            if note.get_title() == title {
                result.push(i);
            }
        }
        result
    }

    pub fn delete_note(&mut self, index: usize) {
        if index >= self.get_size() {
            panic!("Index out of bounds");
        }
        self.notes.remove(index);
        self.size -= 1;
    }

    pub fn get_size(&self) -> usize {
        self.size
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
    fn test_should_return_empty_vector_when_no_notes() {
        let note_taker = NoteTaker::new();
        assert_eq!(note_taker.get_notes().len(), 0);
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

    #[test]
    fn test_get_specific_note() {
        let mut note_taker = NoteTaker::new();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string());
        let note2 = Note::new("Title 2".to_string(), "Content 2".to_string());

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        let note = note_taker.get_note(1);
        assert_eq!(note, note2);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_should_panic_if_getting_out_of_bounds_note() {
        let mut note_taker = NoteTaker::new();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string());

        note_taker.add_note(note1.clone());

        let _ = note_taker.get_note(1); // This should panic
    }

    #[test]
    fn test_notes_should_be_sorted() {
        let mut note_taker = NoteTaker::new();
        let note1 = Note::new("Title 2".to_string(), "Content 2".to_string());
        let note2 = Note::new("Title 1".to_string(), "Content 1".to_string());

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        assert_eq!(note_taker.get_note(0), note2);
        assert_eq!(note_taker.get_note(1), note1);
    }

    #[test]
    fn test_should_return_amount_of_notes() {
        let mut note_taker = NoteTaker::new();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string());
        let note2 = Note::new("Title 2".to_string(), "Content 2".to_string());

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        assert_eq!(note_taker.get_size(), 2);
    }

    #[test]
    fn test_should_search_for_note_by_title() {
        let mut note_taker = NoteTaker::new();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string());
        let note2 = Note::new("Title 2".to_string(), "Content 2".to_string());

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        let result = note_taker.get_by_title("Title 1");

        assert_eq!(result.len(), 1);
        assert_eq!(note_taker.get_note(result[0]), note1);
    }

    #[test]
    fn test_should_return_empty_vector_when_no_note_found_by_title() {
        let mut note_taker = NoteTaker::new();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string());
        let note2 = Note::new("Title 2".to_string(), "Content 2".to_string());

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        let result = note_taker.get_by_title("Title 3");

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_should_delete_note() {
        let mut note_taker = NoteTaker::new();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string());
        let note2 = Note::new("Title 2".to_string(), "Content 2".to_string());

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        note_taker.delete_note(0);

        assert_eq!(note_taker.get_notes().len(), 1);
        assert_eq!(note_taker.get_note(0), note2);
    }
}

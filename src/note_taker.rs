use crate::note::Note;
use crate::persistence::persistence_trait::PersistenceTrait;

pub struct NoteTaker {
    notes: Vec<Note>,
    size: usize,
    persistence: Box<dyn PersistenceTrait>,
}

impl NoteTaker {
    pub fn new(persistence: Box<dyn PersistenceTrait>) -> Self {
        NoteTaker {
            notes: Vec::new(),
            size: 0,
            persistence,
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

    #[allow(dead_code)]
    pub fn get_note(&self, index: usize) -> Note {
        if index >= self.get_size() {
            panic!("Index out of bounds");
        }
        self.notes[index].clone()
    }

    #[allow(dead_code)]
    pub fn get_by_title(&self, title: &str) -> Vec<usize> {
        let mut result = Vec::new();
        for (i, note) in self.notes.iter().enumerate() {
            if note.get_title() == title {
                result.push(i);
            }
        }
        result
    }

    pub fn load(&mut self) -> Result<(), String> {
        let loaded_notes = self.persistence.load()?;
        for note in loaded_notes {
            self.add_note(note);
        }
        Ok(())
    }

    pub fn save(&self) -> Result<(), String> {
        self.persistence.save(&self.notes)
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
use crate::persistence::file_persistence::FilePersistence;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_note_taker() -> NoteTaker {
        let persistence = Box::new(FilePersistence::new("test_notes.json".to_string()));
        NoteTaker::new(persistence)
    }

    #[test]
    fn test_should_store_notes() {
        let mut note_taker = setup_note_taker();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string()).unwrap();
        let note2 = Note::new("Title 2".to_string(), "Content 2".to_string()).unwrap();

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        assert_eq!(note_taker.get_notes().len(), 2);
        assert_eq!(note_taker.get_notes()[0], note1);
        assert_eq!(note_taker.get_notes()[1], note2);
    }

    #[test]
    fn test_should_return_empty_vector_when_no_notes() {
        let note_taker = setup_note_taker();
        assert_eq!(note_taker.get_notes().len(), 0);
    }

    #[test]
    fn test_should_not_allow_duplicate_notes() {
        let mut note_taker = setup_note_taker();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string()).unwrap();

        note_taker.add_note(note1.clone());
        note_taker.add_note(note1.clone());

        assert_eq!(note_taker.get_notes().len(), 1);
        assert_eq!(note_taker.get_notes()[0], note1);
    }

    #[test]
    fn test_get_specific_note() {
        let mut note_taker = setup_note_taker();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string()).unwrap();
        let note2 = Note::new("Title 2".to_string(), "Content 2".to_string()).unwrap();

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        let note = note_taker.get_note(1);
        assert_eq!(note, note2);
    }

    #[test]
    #[should_panic(expected = "Index out of bounds")]
    fn test_should_panic_if_getting_out_of_bounds_note() {
        let mut note_taker = setup_note_taker();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string()).unwrap();

        note_taker.add_note(note1.clone());

        let _ = note_taker.get_note(1); // This should panic
    }

    #[test]
    fn test_notes_should_be_sorted() {
        let mut note_taker = setup_note_taker();
        let note1 = Note::new("Title 2".to_string(), "Content 2".to_string()).unwrap();
        let note2 = Note::new("Title 1".to_string(), "Content 1".to_string()).unwrap();

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        assert_eq!(note_taker.get_note(0), note2);
        assert_eq!(note_taker.get_note(1), note1);
    }

    #[test]
    fn test_should_return_amount_of_notes() {
        let mut note_taker = setup_note_taker();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string()).unwrap();
        let note2 = Note::new("Title 2".to_string(), "Content 2".to_string()).unwrap();

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        assert_eq!(note_taker.get_size(), 2);
    }

    #[test]
    fn test_should_search_for_note_by_title() {
        let mut note_taker = setup_note_taker();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string()).unwrap();
        let note2 = Note::new("Title 2".to_string(), "Content 2".to_string()).unwrap();

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        let result = note_taker.get_by_title("Title 1");

        assert_eq!(result.len(), 1);
        assert_eq!(note_taker.get_note(result[0]), note1);
    }

    #[test]
    fn test_should_return_empty_vector_when_no_note_found_by_title() {
        let mut note_taker = setup_note_taker();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string()).unwrap();
        let note2 = Note::new("Title 2".to_string(), "Content 2".to_string()).unwrap();

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        let result = note_taker.get_by_title("Title 3");

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_should_delete_note() {
        let mut note_taker = setup_note_taker();
        let note1 = Note::new("Title 1".to_string(), "Content 1".to_string()).unwrap();
        let note2 = Note::new("Title 2".to_string(), "Content 2".to_string()).unwrap();

        note_taker.add_note(note1.clone());
        note_taker.add_note(note2.clone());

        note_taker.delete_note(0);

        assert_eq!(note_taker.get_notes().len(), 1);
        assert_eq!(note_taker.get_note(0), note2);
    }
}

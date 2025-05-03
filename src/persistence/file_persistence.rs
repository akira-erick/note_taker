use super::persistence_trait::PersistenceTrait;
use crate::note::Note;

use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

pub struct FilePersistence {
    file_path: String,
}

impl FilePersistence {
    #[allow(dead_code)]
    pub fn new(file_path: String) -> Self {
        FilePersistence { file_path }
    }

    fn convert_note_to_string(notes: &[Note]) -> String {
        let mut result = String::new();
        for note in notes {
            result.push_str(&note.get_as_json());
            result.push_str(",\n");
        }
        if !result.is_empty() {
            result.pop(); // Remove the last comma
            result.pop(); // Remove the last newline
        }
        result.push('\n');
        result
    }

    fn convert_string_to_note(json_string: &str) -> Note {
        let json_value: serde_json::Value = serde_json::from_str(json_string).unwrap();
        let title = json_value["title"].as_str().unwrap().to_string();
        let content = json_value["content"].as_str().unwrap().to_string();
        let date_time = json_value["date_time"].as_str().unwrap().to_string();
        Note::new_with_date_time(title, content, date_time)
    }
}

impl PersistenceTrait for FilePersistence {
    fn save(&self, notes: &[Note]) -> Result<(), String> {
        let file = File::create(&self.file_path).map_err(|e| e.to_string())?;
        let mut writer = BufWriter::new(&file);
        let notes_string = FilePersistence::convert_note_to_string(notes);

        writer.write_all(b"[\n").map_err(|e| e.to_string())?;
        writer
            .write_all(notes_string.as_bytes())
            .map_err(|e| e.to_string())?;
        writer.write_all(b"]\n").map_err(|e| e.to_string())?;
        writer.flush().map_err(|e| e.to_string())?;

        Ok(())
    }

    fn load(&self) -> Result<Vec<Note>, String> {
        let file = match File::open(&self.file_path) {
            Ok(file) => file,
            Err(_) => {
                // If the file doesn't exist or any error occurs, return an empty vector
                return Ok(Vec::new());
            }
        };

        let reader = BufReader::new(file);

        let mut notes = Vec::new();

        for line in reader.lines() {
            let mut line = line.map_err(|e| e.to_string())?;
            if !line.trim().is_empty() {
                if line.starts_with('[') || line.starts_with(']') {
                    continue; // Skip the opening and closing brackets
                }
                if line.ends_with(',') {
                    line = line.trim_end_matches(',').to_string();
                }
                let note = FilePersistence::convert_string_to_note(&line);
                notes.push(note);
            }
        }
        Ok(notes)
    }
}

#[cfg(test)]
use std::fs::remove_file;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_file_persistence() {
        let file_path = "test_notes.json".to_string();
        let persistence = FilePersistence { file_path };
        let notes = vec![
            Note::new("Test Title 1".to_string(), "Test Content 1".to_string()),
            Note::new("Test Title 2".to_string(), "Test Content 2".to_string()),
        ];

        persistence.save(&notes).unwrap();

        let loaded_notes = persistence.load().unwrap();

        assert_eq!(notes.len(), loaded_notes.len());
        for i in 0..notes.len() {
            assert_eq!(notes[i].get_title(), loaded_notes[i].get_title());
            assert_eq!(notes[i].get_content(), loaded_notes[i].get_content());
            assert_eq!(notes[i].get_date_time(), loaded_notes[i].get_date_time());
        }

        remove_file("test_notes.json").unwrap(); // Clean up the test file
    }
}

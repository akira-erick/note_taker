use super::persistence_trait::PersistenceTrait;
use crate::note::Note;
use rusqlite::{Connection, Result, params};

pub struct RusqlitePersistence {
    connection: Connection,
}

impl RusqlitePersistence {
    pub fn new(db_name: &str) -> Result<Self> {
        let connection = Connection::open(db_name)?;

        connection.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                date_time TEXT NOT NULL
            )",
            params![],
        )?;

        Ok(RusqlitePersistence { connection })
    }
}

impl PersistenceTrait for RusqlitePersistence {
    fn save(&self, notes: &[Note]) -> Result<(), String> {
        let mut stmt = self
            .connection
            .prepare("INSERT INTO notes (title, content, date_time) VALUES (?,?,?)")
            .unwrap();
        for note in notes {
            stmt.execute(params![
                note.get_title(),
                note.get_content(),
                note.get_date_time()
            ])
            .unwrap();
        }
        Ok(())
    }

    fn load(&self) -> Result<Vec<Note>, String> {
        let mut stmt = self
            .connection
            .prepare("SELECT title, content, date_time FROM notes")
            .unwrap();
        let note_iter = stmt
            .query_map(params![], |row| {
                Ok(Note::new_with_date_time(row.get(0)?, row.get(1)?, row.get(2)?).unwrap())
            })
            .unwrap();

        let mut notes = Vec::new();
        for note in note_iter {
            notes.push(note.unwrap());
        }
        Ok(notes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::note::Note;

    #[test]
    fn test_rusqlite_persistence() {
        let persistence = RusqlitePersistence::new("test.db").unwrap();
        let note1 = Note::new("Test Title 1".to_string(), "Test Content 1".to_string()).unwrap();
        let note2 = Note::new("Test Title 2".to_string(), "Test Content 2".to_string()).unwrap();

        persistence.save(&[note1.clone(), note2.clone()]).unwrap();
        let loaded_notes = persistence.load().unwrap();

        assert_eq!(loaded_notes.len(), 2);
        assert_eq!(loaded_notes[0].get_title(), "Test Title 1");
        assert_eq!(loaded_notes[1].get_title(), "Test Title 2");

        // Clean up the database after the test
        persistence
            .connection
            .execute("DELETE FROM notes", params![])
            .unwrap();
        persistence.connection.close().unwrap();
        std::fs::remove_file("test.db").unwrap();
    }
}

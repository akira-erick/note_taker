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
                Ok(Note::new_with_date_time(
                    row.get(0)?,
                    row.get(1)?,
                    row.get(2)?,
                ))
            })
            .unwrap();

        let mut notes = Vec::new();
        for note in note_iter {
            notes.push(note.unwrap());
        }
        Ok(notes)
    }
}

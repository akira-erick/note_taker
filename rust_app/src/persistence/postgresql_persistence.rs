use super::persistence_trait::PersistenceTrait;
use crate::note::Note;

use dotenv::dotenv;
use std::env;
use tokio::runtime::Runtime;
use tokio_postgres::{Client, Error, NoTls};

pub struct PostgresqlPersistence {
    connection_string: String,
}

impl PostgresqlPersistence {
    pub fn new() -> Result<Self, String> {
        dotenv().ok();

        let connection_string =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

        Ok(PostgresqlPersistence { connection_string })
    }

    async fn connect(&self) -> Result<Client, Error> {
        let (client, connection) = tokio_postgres::connect(&self.connection_string, NoTls).await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("Connection error: {}", e);
            }
        });
        Ok(client)
    }

    async fn save_notes(&self, notes: &[Note]) -> Result<(), Error> {
        let client = self.connect().await?;
        for note in notes {
            client
                .execute(
                    "INSERT INTO notes (title, content, date_time) VALUES ($1, $2, $3)",
                    &[
                        &note.get_title(),
                        &note.get_content(),
                        &note.get_date_time(),
                    ],
                )
                .await?;
        }
        Ok(())
    }

    async fn load_notes(&self) -> Result<Vec<Note>, Error> {
        let client = self.connect().await?;
        let rows = client
            .query("SELECT title, content, date_time FROM notes", &[])
            .await?;

        let mut notes = Vec::new();
        for row in rows {
            let note = Note::new_with_date_time(row.get(0), row.get(1), row.get(2)).unwrap();
            notes.push(note);
        }
        Ok(notes)
    }
}

impl PersistenceTrait for PostgresqlPersistence {
    fn save(&self, notes: &[Note]) -> Result<(), String> {
        let runtime = Runtime::new().unwrap();
        runtime
            .block_on(self.save_notes(notes))
            .map_err(|e| e.to_string())
    }

    fn load(&self) -> Result<Vec<Note>, String> {
        let runtime = Runtime::new().unwrap();
        runtime
            .block_on(self.load_notes())
            .map_err(|e| e.to_string())
    }
}

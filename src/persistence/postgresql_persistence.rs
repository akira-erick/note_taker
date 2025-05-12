use super::persistence_trait::PersistenceTrait;
use crate::note::Note;

use dotenv::dotenv;
use std::env;
use tokio_postgres::{Client, Error, NoTls};

#[allow(dead_code)]
pub struct PostgresqlPersistence {
    connection_string: String,
}

#[allow(dead_code)]
impl PostgresqlPersistence {
    pub fn new() -> Self {
        dotenv().ok();

        let connection_string =
            env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");

        PostgresqlPersistence { connection_string }
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
}

impl PersistenceTrait for PostgresqlPersistence {
    fn save(&self, _notes: &[Note]) -> Result<(), String> {
        // Implement the save method for PostgreSQL
        Ok(())
    }

    fn load(&self) -> Result<Vec<Note>, String> {
        // Implement the load method for PostgreSQL
        Ok(vec![])
    }
}

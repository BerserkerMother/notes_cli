use rusqlite::{self, params, Connection, Result};
use std::{fmt::Display, path::Path};

pub struct Repository {
    db: Connection,
}

impl Repository {
    pub fn new(db_path: impl AsRef<Path>) -> Result<Repository> {
        let db = Connection::open(db_path)?;
        Ok(Repository { db })
    }

    pub fn initialize_db(&self) -> Result<()> {
        self.db.execute(
            "CREATE TABLE IF NOT EXISTS note (
                id   INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL,
                text TEXT NOT NULL
            )",
            (),
        )?;
        Ok(())
    }

    pub fn get_notes(&self) -> Result<Vec<Note>> {
        let mut stmt = self.db.prepare("SELECT id, title, text FROM note")?;
        let notes_iter = stmt.query_map([], |row| {
            Ok(Note::new(row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        let mut notes = Vec::new();
        for note in notes_iter {
            notes.push(note?);
        }
        Ok(notes)
    }

    pub fn add(&mut self, notes: Vec<Note>) -> Result<()> {
        let transaction = self.db.transaction()?;
        {
            let mut stmt = transaction.prepare("INSERT INTO note (title, text) VALUES (?, ?)")?;
            for note in &notes {
                stmt.execute(params![note.title, note.text])?;
            }
        }
        transaction.commit()?;
        Ok(())
    }

    pub fn get_note(&self, note_id: usize) -> Result<Note> {
        let mut stmt = self
            .db
            .prepare("SELECT id, title, text FROM note WHERE id = ?1")?;
        let mut note_iter = stmt.query_map([note_id], |row| {
            Ok(Note::new(row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        note_iter
            .next()
            .ok_or_else(|| rusqlite::Error::QueryReturnedNoRows)?
            .and_then(|r| Ok(r))
    }

    pub fn delete(&self, note_id: usize) -> Result<Note> {
        let note = self.get_note(note_id)?;
        self.db
            .execute("DELETE FROM note WHERE id = ?1", params![note_id])?;
        Ok(note)
    }

    // Placeholder for the update function if needed
    pub fn update(&self, _note_id: usize, _new_note: Note) -> Result<()> {
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Note {
    pub id: Option<usize>,
    pub title: String,
    pub text: String,
}

impl Note {
    pub fn new(id: Option<usize>, title: String, text: String) -> Note {
        Note { id, title, text }
    }
}

impl From<Note> for String {
    fn from(value: Note) -> Self {
        format!("title: {}\n{}", value.title, value.text)
    }
}
impl Display for Note {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "title: {}\n{}", self.title, self.text).unwrap();
        Ok(())
    }
}

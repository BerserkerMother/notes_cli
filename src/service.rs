use rusqlite::Result as SqliteResult;
use std::path::Path;

use crate::repository::{Note, Repository};

pub struct NoteService {
    repository: Repository,
}

impl NoteService {
    // Now takes an implementation of AsRef<Path> for flexibility and proper error handling
    pub fn new<P: AsRef<Path>>(db_path: P) -> SqliteResult<Self> {
        let repository = Repository::new(db_path)?;
        Ok(NoteService { repository })
    }

    pub fn initialize_notes_service(&self) -> SqliteResult<()> {
        self.repository.initialize_db()
    }

    pub fn create_notes(&mut self, notes: Vec<Note>) -> SqliteResult<()> {
        // You might add validation or transformation logic here before saving notes
        self.repository.add(notes)
    }

    pub fn list_all_notes(&self) -> SqliteResult<Vec<Note>> {
        self.repository.get_notes()
    }

    pub fn fetch_note_by_id(&self, note_id: usize) -> SqliteResult<Note> {
        self.repository.get_note(note_id)
    }

    pub fn update_note(&self, note_id: usize, new_note: Note) -> SqliteResult<()> {
        // Additional logic before updating the note could be placed here
        self.repository.update(note_id, new_note)
    }

    pub fn delete_note(&self, note_id: usize) -> SqliteResult<Note> {
        self.repository.delete(note_id)
    }
}

use crate::db::{DbConnection, Note};
use serde::Deserialize;
use tauri::State;
use std::sync::{Arc, Mutex};

use crate::optional::OptionalExtension;

#[derive(Deserialize)]
pub struct NewNote {
    pub title: String,
    pub content: String,
}

#[derive(Deserialize)]
pub struct UpdateNote {
    pub id: i64,
    pub title: String,
    pub content: String,
}

#[tauri::command]
fn create_note(
    db: State<'_, Arc<Mutex<DbConnection>>>,
    note: NewNote,
) -> Result<i64, String> {
    let db = db.lock().unwrap();
    db.add_note(&note.title, &note.content)
        .map_err(|e| e.to_string())
        .and_then(|_| {
            // Get last insert id
            let conn = db.conn.lock().unwrap();
            Ok(conn.last_insert_rowid())
        })
}

#[tauri::command]
fn update_note(
    db: State<'_, Arc<Mutex<DbConnection>>>,
    payload: UpdateNote,
) -> Result<usize, String> {
    let db = db.lock().unwrap();
    let conn = db.conn.lock().unwrap();
    
    conn.execute(
        "UPDATE notes SET title = ?1, content = ?2 WHERE id = ?3",
        [payload.title, payload.content, payload.id.to_string()],
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_note(
    db: State<'_, Arc<Mutex<DbConnection>>>,
    id: i64,
) -> Result<Option<Note>, String> {
    let db = db.lock().unwrap();
    let conn = db.conn.lock().unwrap();
    
    let mut stmt = conn.prepare("SELECT id, title, content, created_at FROM notes WHERE id = ?")
        .map_err(|e| e.to_string())?;
    
    stmt.query_row([id], |row| {
        Ok(Note {
            id: row.get(0)?,
            title: row.get(1)?,
            content: row.get(2)?,
            created_at: row.get(3)?,
        })
    })
    .optional()
    .map_err(|e| e.to_string())
}

#[tauri::command]
fn get_all_notes(
    db: State<'_, Arc<Mutex<DbConnection>>>,
) -> Result<Vec<Note>, String> {
    let db = db.lock().unwrap();
    db.get_all_notes().map_err(|e| e.to_string())
}
use rusqlite::Connection;
use std::sync::Mutex;  // Use std Mutex instead of tokio

pub struct DbConnection {
    conn: Mutex<Connection>,
}

impl DbConnection {
    pub fn new() -> Self {
        let conn = Connection::open("notes.db").expect("Failed to open database");
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS notes (
                id INTEGER PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )",
            [],
        ).expect("Failed to create table");
        
        DbConnection {
            conn: Mutex::new(conn),
        }
    }

    // Remove async from these methods
    pub fn add_note(&self, title: &str, content: &str) -> rusqlite::Result<usize> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO notes (title, content) VALUES (?1, ?2)",
            [title, content],
        )
    }

    pub fn get_all_notes(&self) -> rusqlite::Result<Vec<Note>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare("SELECT id, title, content, created_at FROM notes")?;
        let notes_iter = stmt.query_map([], |row| {
            Ok(Note {
                id: row.get(0)?,
                title: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;
        
        notes_iter.collect()
    }
}

#[derive(serde::Serialize)]
pub struct Note {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub created_at: String,
}
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod db;
mod controllers;

use tauri::Manager;
use std::sync::{Arc, Mutex};
use crate::db::DbConnection;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let db = Arc::new(Mutex::new(DbConnection::new()));
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            controllers::create_note,
            controllers::update_note,
            controllers::get_note,
            controllers::get_all_notes
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
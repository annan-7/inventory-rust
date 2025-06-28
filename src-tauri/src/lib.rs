// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use uuid::Uuid;

// Product struct for inventory
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub quantity: i32,
    pub price: f64,
}

// Shared DB connection
static DB_CONN: once_cell::sync::Lazy<Mutex<Connection>> = once_cell::sync::Lazy::new(|| {
    let conn = Connection::open("inventory.db").expect("Failed to open DB");
    // Create table with indexes for fast lookup
    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS products (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            quantity INTEGER NOT NULL,
            price REAL NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_products_name ON products(name);
        CREATE INDEX IF NOT EXISTS idx_products_quantity ON products(quantity);
        CREATE INDEX IF NOT EXISTS idx_products_price ON products(price);"
    ).expect("Failed to create table or indexes");
    Mutex::new(conn)
});

#[tauri::command]
pub fn create_product(name: String, quantity: i32, price: f64) -> Result<Product, String> {
    let id = Uuid::new_v4().to_string();
    let product = Product { id: id.clone(), name, quantity, price };
    let conn = DB_CONN.lock().unwrap();
    conn.execute(
        "INSERT INTO products (id, name, quantity, price) VALUES (?1, ?2, ?3, ?4)",
        params![product.id, product.name, product.quantity, product.price],
    ).map_err(|e| e.to_string())?;
    Ok(product)
}

#[tauri::command]
pub fn get_products() -> Result<Vec<Product>, String> {
    let conn = DB_CONN.lock().unwrap();
    let mut stmt = conn.prepare("SELECT id, name, quantity, price FROM products").map_err(|e| e.to_string())?;
    let products = stmt
        .query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                quantity: row.get(2)?,
                price: row.get(3)?,
            })
        })
        .map_err(|e| e.to_string())?
        .filter_map(|res| res.ok())
        .collect();
    Ok(products)
}

#[tauri::command]
pub fn update_product(id: String, name: String, quantity: i32, price: f64) -> Result<(), String> {
    let conn = DB_CONN.lock().unwrap();
    let updated = conn.execute(
        "UPDATE products SET name = ?1, quantity = ?2, price = ?3 WHERE id = ?4",
        params![name, quantity, price, id],
    ).map_err(|e| e.to_string())?;
    if updated == 0 {
        Err("Product not found".to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn delete_product(id: String) -> Result<(), String> {
    let conn = DB_CONN.lock().unwrap();
    let deleted = conn.execute("DELETE FROM products WHERE id = ?1", params![id])
        .map_err(|e| e.to_string())?;
    if deleted == 0 {
        Err("Product not found".to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            create_product,
            get_products,
            update_product,
            delete_product
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

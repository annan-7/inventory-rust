// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use tauri::State;
use uuid::Uuid;
use chrono;

// Product struct for inventory
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub category: String,
    pub quantity: i32,
    pub price: f64,
    pub created_at: String,
    pub updated_at: String,
}

// Category struct for grouping products
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Category {
    pub name: String,
    pub count: i32,
}

// Database state wrapper
pub struct DbState {
    conn: Arc<Mutex<Connection>>,
}

impl DbState {
    pub fn new() -> Self {
        let conn = Connection::open("inventory.db").expect("Failed to open DB");
        
        // Enable Write-Ahead Logging for better concurrency
        conn.pragma_update(None, "journal_mode", "WAL").unwrap();
        conn.pragma_update(None, "synchronous", "NORMAL").unwrap();
        
        // Create table with indexes
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS products (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                category TEXT NOT NULL,
                quantity INTEGER NOT NULL,
                price REAL NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_products_name ON products(name);
            CREATE INDEX IF NOT EXISTS idx_products_category ON products(category);
            CREATE INDEX IF NOT EXISTS idx_products_created_at ON products(created_at);"
        ).expect("Failed to create table or indexes");
        
        DbState {
            conn: Arc::new(Mutex::new(conn))
        }
    }
}

// Initialize database state
fn setup_database() -> DbState {
    DbState::new()
}

#[tauri::command]
pub fn create_product(
    name: String,
    category: String,
    quantity: i32,
    price: f64,
    db: State<DbState>
) -> Result<Product, String> {
    let id = Uuid::new_v4().to_string();
    let now = chrono::Utc::now().to_rfc3339();
    let product = Product {
        id: id.clone(),
        name: name.clone(),
        category: category.clone(),
        quantity,
        price,
        created_at: now.clone(),
        updated_at: now.clone(),
    };
    
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    conn.execute(
        "INSERT INTO products (id, name, category, quantity, price, created_at, updated_at) 
        VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        params![
            product.id,
            product.name,
            product.category,
            product.quantity,
            product.price,
            product.created_at,
            product.updated_at
        ],
    )
    .map_err(|e| format!("Failed to create product: {}", e))?;
    
    Ok(product)
}

#[tauri::command]
pub fn get_products(db: State<DbState>) -> Result<Vec<Product>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, category, quantity, price, created_at, updated_at 
                 FROM products ORDER BY created_at DESC")
        .map_err(|e| format!("Failed to prepare query: {}", e))?;
    
    let products = stmt
        .query_map([], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                quantity: row.get(3)?,
                price: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .map_err(|e| format!("Failed to execute query: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to parse results: {}", e))?;
    
    Ok(products)
}

#[tauri::command]
pub fn get_one_product(id: String, db: State<DbState>) -> Result<Product, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, category, quantity, price, created_at, updated_at 
                 FROM products WHERE id = ?1")
        .map_err(|e| format!("Failed to prepare query: {}", e))?;
    
    let product = stmt
        .query_row(params![id], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                quantity: row.get(3)?,
                price: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .map_err(|e| format!("Failed to get product: {}", e))?;
    
    Ok(product)
}

#[tauri::command]
pub fn get_products_by_category(
    category: String, 
    db: State<DbState>
) -> Result<Vec<Product>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, name, category, quantity, price, created_at, updated_at 
                 FROM products WHERE category = ?1 ORDER BY name")
        .map_err(|e| format!("Failed to prepare query: {}", e))?;
    
    let products = stmt
        .query_map(params![category], |row| {
            Ok(Product {
                id: row.get(0)?,
                name: row.get(1)?,
                category: row.get(2)?,
                quantity: row.get(3)?,
                price: row.get(4)?,
                created_at: row.get(5)?,
                updated_at: row.get(6)?,
            })
        })
        .map_err(|e| format!("Failed to execute query: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to parse results: {}", e))?;
    
    Ok(products)
}

#[tauri::command]
pub fn get_categories(db: State<DbState>) -> Result<Vec<Category>, String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT category, COUNT(*) as count FROM products GROUP BY category ORDER BY count DESC")
        .map_err(|e| format!("Failed to prepare query: {}", e))?;
    
    let categories = stmt
        .query_map([], |row| {
            Ok(Category {
                name: row.get(0)?,
                count: row.get(1)?,
            })
        })
        .map_err(|e| format!("Failed to execute query: {}", e))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("Failed to parse results: {}", e))?;
    
    Ok(categories)
}

#[tauri::command]
pub fn update_product(
    id: String,
    name: String,
    category: String,
    quantity: i32,
    price: f64,
    db: State<DbState>
) -> Result<(), String> {
    let now = chrono::Utc::now().to_rfc3339();
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let result = conn.execute(
        "UPDATE products 
        SET name = ?1, category = ?2, quantity = ?3, price = ?4, updated_at = ?5 
        WHERE id = ?6",
        params![name, category, quantity, price, now, id],
    )
    .map_err(|e| format!("Failed to update product: {}", e))?;
    
    if result == 0 {
        Err("Product not found".to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
pub fn delete_product(
    id: String, 
    db: State<DbState>
) -> Result<(), String> {
    let conn = db.conn.lock().map_err(|e| e.to_string())?;
    let result = conn.execute(
        "DELETE FROM products WHERE id = ?1", 
        params![id]
    )
    .map_err(|e| format!("Failed to delete product: {}", e))?;
    
    if result == 0 {
        Err("Product not found".to_string())
    } else {
        Ok(())
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(setup_database())
        .invoke_handler(tauri::generate_handler![
            greet,
            create_product,
            get_products,
            get_one_product,
            get_products_by_category,
            get_categories,
            update_product,
            delete_product
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
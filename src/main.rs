mod args;

use args::Todo;
use clap::Parser;
use colored::*;
use dotenv::dotenv;
use rusqlite::{Connection, Result};
use std::env;

use args::EntityType::Add;
use args::EntityType::Remove;
use args::EntityType::Show;

struct TodoItem {
    id: i32,
    items: String,
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos ( 
        id INTEGER PRIMARY KEY,
        items TEXT
    )",
        [],
    )?;

    Ok(())
}

fn insert(conn: &Connection, item: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO todos (id, items)
    VALUES ((SELECT IFNULL(MAX(id), 0) + 1 FROM todos), ?1);
    ",
        [item],
    )?;

    Ok(())
}

fn show(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, items FROM todos")?;
    let todos_iter = stmt.query_map([], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            items: row.get(1)?,
        })
    })?;

    for todo in todos_iter {
        let todo = todo.unwrap();
        println!("{} {}", todo.id.to_string().bold().white(), todo.items);
    }

    Ok(())
}

fn remove(conn: &Connection, index: i32) -> Result<()> {
    let mut stmt = conn.prepare("SELECT * FROM todos")?;

    let todos_iter = stmt.query_map([], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            items: row.get(1)?,
        })
    })?;

    for todo in todos_iter {
        let todo = todo.unwrap();
        let id_text = todo.id.to_string().bold().white();
        let items_text = if Some(todo.id) == Some(index) {
            todo.items.strikethrough().to_string()
        } else {
            todo.items.clone()
        };

        println!("{} {}", id_text, items_text);
    }

    conn.execute("DELETE FROM todos WHERE id = ?1", [index])?;

    conn.execute("UPDATE todos SET id = id - 1 WHERE id > ?1", [index])?;

    Ok(())
}

fn main() -> Result<()> {
    dotenv().ok();
    let db_file = env::var("DB_FILE").expect("DATABASE_URL must be set");

    let conn = Connection::open(db_file)?;

    create_table(&conn).expect("Failed to create table");

    let todo: Todo = Todo::parse();

    match todo.entity_type {
        Add(item) => {
            let item_text = item.item;
            insert(&conn, &item_text).expect("Failed to insert an item");
            println!("{} {}", "Added item:".white().bold(), item_text);
        }
        Show => {
            show(&conn).expect("Failed to show items");
        }
        Remove(index) => {
            let item_id = index.index;
            remove(&conn, item_id).expect("Failed to remove an item");
        }
    }

    Ok(())
}

mod args;

use args::Todo;
use clap::Parser;
use dotenv::dotenv;
use rusqlite::{Connection, Result};
use std::env;
use std::process::id;

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
        println!("{} {}", todo.id, todo.items);
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

    conn.execute("DELETE FROM todos WHERE id = ?1", [index])?;

    println!("Remaining items:");
    
    for todo in todos_iter {
        let todo = todo.unwrap();
        let mut text = todo.items.clone();

        if todo.id == index {
            text = format!("\x1B[9m{}\x1B[0m", todo.items);
        }

        
        println!("{} {}", todo.id, text);
    }

    conn.execute("UPDATE todos SET id = id - 1 WHERE id > 1", [])?;

    Ok(())
}

fn main() -> Result<()> {
    dotenv().ok();
    let db_file = env::var("DB_FILE").expect("DATABASE_URL must be set");

    let conn = Connection::open(db_file)?;

    create_table(&conn).expect("Error creating table");

    let todo: Todo = Todo::parse();

    match todo.entity_type {
        Add(item) => {
            let str = item.item;
            insert(&conn, &str).expect("Error inserting item");
            println!("Added item: {}", str);
        }
        Show => {
            show(&conn).expect("Error showing items");
        }
        Remove(index) => {
            let int = index.index;
            remove(&conn, int).expect("Error removing item");
        }
    }

    Ok(())
}

use clap::Parser;
use dotenv::dotenv;
use rusqlite::{Connection, Result};
use std::env;

/// Simple program to create a todo list
#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Todo {
    /// Add a todo item
    #[arg(short, long)]
    add: String,
}

struct TodoItem {
    items: String,
}

fn create_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos ( 
        items TEXT)",
        [],
    )?;

    Ok(())
}

fn insert(conn: &Connection, item: &str) -> Result<()> {
    conn.execute("INSERT INTO todos (items) VALUES (?1)", [&item])?;

    Ok(())
}

fn show(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT items FROM todos")?;
    let todos_iter = stmt.query_map([], |row| Ok(TodoItem { items: row.get(0)? }))?;

    for todo in todos_iter {
        println!("{}", todo.unwrap().items);
    }

    Ok(())
}

fn main() -> Result<()> {
    dotenv().ok();
    let db_file = env::var("DB_FILE").expect("DATABASE_URL must be set");

    let conn = Connection::open(db_file)?;

    create_table(&conn).expect("Error creating table");

    let todo: Todo = Todo::parse();

    if !todo.add.is_empty() {
        insert(&conn, &todo.add).expect("Error inserting item");
    }

    println!("{}", todo.add);

    Ok(())
}

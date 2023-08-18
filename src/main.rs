use clap::Parser;

/// Simple program to create a todo list
#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Todo {
    /// Add a todo item
    #[arg(short, long)]
    add: String,
}

fn main() {
    let todo: Todo = Todo::parse();

    println!("{}", todo.add);
}

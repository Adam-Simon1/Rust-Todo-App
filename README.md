# Rust-Todo-App

Simple CLI Todo app made in rust using Sqlite.

## Usage:

In order to use this you have to create a .env file in the root of the project and inset in this variable:

    DB_FILE = ""

Put the path to your sqlite file inside.

## Features:

Add a todo item

    ./todo add "A todo item"

Remove a todo item

    ./todo remove 1 // Index of the todo item. Shown when show command is used.

Show the list of todo items

    ./todo show

## Screenshots:

![Add](https://github.com/Adam-Simon1/Rust-Todo-App/assets/134161057/462e90f6-44aa-49e8-b675-eb1a11f8894e)

![Show](https://github.com/Adam-Simon1/Rust-Todo-App/assets/134161057/fd59f195-eda9-42e2-b614-b243769524e0)

![Remove](https://github.com/Adam-Simon1/Rust-Todo-App/assets/134161057/885beea0-538f-41bd-8db8-b921db749feb)

## Libraries used:

- Clap
- Colored
- Dotenv
- Rusqlite

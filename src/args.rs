use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[clap(author, version, about)]

pub struct Todo {
    #[clap(subcommand)]
    pub entity_type: EntityType,
}

#[derive(Debug, Subcommand)]

pub enum EntityType {
    /// Add a todo item
    Add(Add),

    /// Show the list of todo items
    Show,
}

#[derive(Debug, Args)]
pub struct Add {
    pub item: String,
}

use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};


#[derive(Parser, Debug)]
#[clap(name = "todo-cli", version = "1.0", author = "Ardak", about = "A simple command-line todo app")]
pub struct Cli{
    #[clap(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand, Debug)]
pub enum Commands{
    Add{
        #[clap(value_parser)]
        title: String,
        #[clap(long,default_value = "medium",value_parser= ["high","medium","low"])]
        priority: String,
        #[clap(value_parser)]
        description: String,
        #[clap(long)]
        deadline: Option<String>,
        #[clap(long, default_value = "general")]
        category: String,
    },
    List,
    Complete{
        #[clap(value_parser)]
        id: u32,
    },
    Delete{
        #[clap(value_parser)]
        id: u32,
    },
    Clear,
    FilterPriority{
        #[clap(value_parser = ["high","medium","low"])]
        priority: String,
    },
    FilterStatus{
        #[clap(value_parser = ["completed","incomplete"])]
        status: String,
    },
    Interactive,
}
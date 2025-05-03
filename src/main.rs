mod cli;
mod models;
mod storage;
mod operations;
mod interactive;

use clap::Parser;
use anyhow::Result;
use cli::{Cli, Commands};
use storage::{load_tasks, save_tasks};
use models::task::Task;
use operations::{add_task, list_tasks, complete_task, remove_task, clear_tasks, filter_by_priority, filter_by_status};
use colored::*;
use interactive::run_interactive;


fn main() -> anyhow::Result<()> {

    let file_path= "tasks.json";

    let mut tasks = load_tasks(file_path)?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Add {title, priority, description, deadline, category} => {
            add_task(&mut tasks, title, &priority, description, deadline, category)?;
            println!("{}", format!("Görev eklendi (öncelik: {})!", priority).green());
        }
        Commands::List => {
            list_tasks(&tasks)?;
        }
        Commands::Complete {id} => {
            complete_task(&mut tasks, id)?;
            println!("{}", format!("Görev {} tamamlandı!", id).green());
        }
        Commands::Delete { id } => {
            remove_task(&mut tasks, id)?;
            println!("{}", format!("Görev {} silindi!", id).green());
        }
        Commands::Clear => {
            clear_tasks(&mut tasks)?;
            println!("{}", "Tüm görevler silindi".green());
        }
        Commands::FilterPriority { priority } => {
            filter_by_priority(&tasks, &priority)?;
        }
        Commands::FilterStatus { status } => {
            filter_by_status(&tasks, &status)?;
        }
        Commands::Interactive => {
            run_interactive(&mut tasks, file_path)?;
        }
        _ => {
            println!("{}", "Geçersiz komut".red());
        }
    }
    
    save_tasks(&tasks, file_path)?;
    Ok(())
    
}

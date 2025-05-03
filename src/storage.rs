use crate::models::task::Task;
use anyhow::{Ok, Result};
use serde_json;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

pub fn save_tasks(tasks: &Vec<Task>, file_path: &str) -> Result<()> {

    let json = serde_json::to_string_pretty(tasks)?;

    let mut file = File::create(file_path)?;

    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_tasks(file_path: &str) -> Result<Vec<Task>> {
    
    if Path::new(file_path).exists() {
        let json = fs::read_to_string(file_path)?;

        let tasks: Vec<Task> = serde_json::from_str(&json)?;
        Ok(tasks)
    } else {
        Ok(Vec::new())
    }
}
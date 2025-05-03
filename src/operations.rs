use crate::models::task::{Task, Priority};
use anyhow::{Result, anyhow};
use colored::*;
use chrono::{DateTime, Utc, NaiveDateTime};

pub fn add_task(tasks: &mut Vec<Task>, title: String, priority: &str, description: String, 
    deadline: Option<String>, category: String) -> Result<()> {
    let new_id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
    

    // Görev önceliğini kontrol et
    let priority = match priority.to_lowercase().as_str() {
        "high" => Priority::High,
        "medium" => Priority::Medium,
        "low" => Priority::Low,
        _ => return Err(anyhow!("Geçersiz öncelik: {}. Geçerli değerler: high, medium, low", priority)),
    };

    let deadline = match deadline {
        Some(date_str) => {
            let naive_date = NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%d %H:%M")
                .map_err(|_| anyhow!("Geçersiz tarih formatı. Lütfen YYYY-MM-DD HH:MM formatını kullanın."))?;
            Some(DateTime::<Utc>::from_utc(naive_date, Utc))
        },
        None => None,
    };

    // Yeni görevi ekle
    tasks.push(Task {
        id: new_id,
        title,
        description,
        completed: false,
        priority,
        deadline,
        category,
    });
    
    Ok(())
}

pub fn list_tasks(tasks: &[Task]) -> Result<()> {
    if tasks.is_empty() {
        println!("{}", "Hiç görev yok!".yellow());
    } else {
        println!("{}", "Görevler:".bold());
        for task in tasks {
            let status = if task.completed { "✓".green() } else { " ".white() };
            let priority = match task.priority {
                Priority::High => "Yüksek".red(),
                Priority::Medium => "Orta".yellow(),
                Priority::Low => "Düşük".green(),
            };
            let deadline = task.deadline
                .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or("Yok".to_string());
            println!(
                "{} [{}] {} ({} | {} | {})",
                task.id.to_string().cyan(),
                status,
                task.title,
                priority,
                deadline,
                task.category,
            );
        }
    }
    Ok(())
}

pub fn complete_task(tasks: &mut Vec<Task>, id: u32) -> Result<()> {

    let task = tasks.iter_mut().find(|task| task.id == id)
        .ok_or_else(|| anyhow!("ID {} olan görev bulunamadı", id))?;
    
    // Tamamlandı olarak işaretle
    task.completed = true;
    Ok(())
}

pub fn remove_task(tasks: &mut Vec<Task>, id: u32) -> Result<()> {

    let index = tasks.iter().position(|task| task.id == id)
        .ok_or_else(|| anyhow!("ID {} olan görev bulunamadı", id))?;
    
    // Görevi sil
    tasks.remove(index);
    Ok(())
}

pub fn clear_tasks(tasks: &mut Vec<Task>) -> Result<()> {
    tasks.clear();
    Ok(())
}

pub fn filter_by_priority(tasks: &[Task], priority: &str) -> Result<()> {
    let priority = match priority.to_lowercase().as_str() {
        "high" => Priority::High,
        "medium" => Priority::Medium,
        "low" => Priority::Low,
        _ => return Err(anyhow!("Geçersiz öncelik: {}. Beklenen: high, medium, low", priority)),
    };

    let filtered_tasks: Vec<Task> = tasks.iter()
        .filter(|task| task.priority == priority)
        .cloned()
        .collect();

    list_tasks(&filtered_tasks)?;
    Ok(())
}

pub fn filter_by_status(tasks: &[Task], status: &str) -> Result<()> {
    let completed = match status.to_lowercase().as_str() {
        "completed" => true,
        "incomplete" => false,
        _ => return Err(anyhow!("Geçersiz durum: {}. Beklenen: completed, incomplete", status)),
    };

    let filtered_tasks: Vec<Task> = tasks.iter()
        .filter(|task| task.completed == completed)
        .cloned()
        .collect();

    list_tasks(&filtered_tasks)?;
    Ok(())
}

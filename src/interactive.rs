use crate::models::task::{Task, Priority};
use crate::operations::{add_task, list_tasks, complete_task, remove_task, clear_tasks, filter_by_priority, filter_by_status};
use anyhow::{Result, anyhow};
use crate::storage::save_tasks;
use dialoguer::{Input, Select, theme::ColorfulTheme};
use console::Term;

pub fn run_interactive(tasks: &mut Vec<Task>, file_path: &str) -> Result<()> {
    let term = Term::stdout();
    loop {
        term.clear_screen()?;
        println!("Todo Uygulaması - İnteraktif Mod");
        println!("-----------------------------");

        let options = vec![
            "Görev Ekle",
            "Görevleri Listele",
            "Görev Tamamla",
            "Görev Sil",
            "Tüm Görevleri Sil",
            "Önceliğe Göre Filtrele",
            "Tamamlanma Durumuna Göre Filtrele",
            "Çıkış",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Ne yapmak istiyorsunuz?")
            .items(&options)
            .default(0)
            .interact_on(&term)?;

        match selection {
            0 => {
                add_task_interactive(tasks)?;
                save_tasks(tasks, file_path)?;
            }
            1 => {
                list_tasks(tasks)?;
            }
            2 => {
                complete_task_interactive(tasks)?;
                save_tasks(tasks, file_path)?;
            }
            3 => {
                remove_task_interactive(tasks)?;
                save_tasks(tasks, file_path)?;
            }
            4 => {
                clear_tasks(tasks)?;
                save_tasks(tasks, file_path)?;
                println!("Tüm görevler silindi!");
            }
            5 => {
                filter_by_priority_interactive(tasks)?;
            }
            6 => {
                filter_by_status_interactive(tasks)?;
            }
            7 => break,
            _ => unreachable!(),
        }

        if selection != 7 {
            println!("\nDevam etmek için bir tuşa basın...");
            term.read_key()?;
        }
    }
    Ok(())
}

fn add_task_interactive(tasks: &mut Vec<Task>) -> Result<()> {
    let title: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Görev başlığı")
        .interact_text()?;

    let description: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Görev açıklaması")
        .default("".to_string())
        .interact_text()?;

    let priority_options = vec!["Yüksek", "Orta", "Düşük"];
    let priority_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Öncelik seviyesi")
        .items(&priority_options)
        .default(1)
        .interact()?;
    let priority = match priority_idx {
        0 => "high",
        1 => "medium",
        2 => "low",
        _ => unreachable!(),
    };

    let deadline: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Son tarih (YYYY-MM-DD HH:MM, boş bırakmak için Enter)")
        .allow_empty(true)
        .interact_text()?;
    let deadline = if deadline.is_empty() { None } else { Some(deadline) };

    let category: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Kategori (varsayılan: General)")
        .default("General".to_string())
        .interact_text()?;

    add_task(tasks, title, &priority, description, deadline, category)?;
    println!("Görev eklendi!");
    Ok(())
}

fn complete_task_interactive(tasks: &mut Vec<Task>) -> Result<()> {
    let id: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Tamamlanacak görev ID'si")
        .interact_text()?;
    let id: u32 = id.parse().map_err(|_| anyhow!("Geçersiz ID formatı"))?;
    complete_task(tasks, id)?;
    println!("Görev {} tamamlandı!", id);
    Ok(())
}

fn remove_task_interactive(tasks: &mut Vec<Task>) -> Result<()> {
    let id: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Silinecek görev ID'si")
        .interact_text()?;
    let id: u32 = id.parse().map_err(|_| anyhow!("Geçersiz ID formatı"))?;
    remove_task(tasks, id)?;
    println!("Görev {} silindi!", id);
    Ok(())
}

fn filter_by_priority_interactive(tasks: &Vec<Task>) -> Result<()> {
    let priority_options = vec!["Yüksek", "Orta", "Düşük"];
    let priority_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Filtrelemek istediğiniz öncelik")
        .items(&priority_options)
        .default(1)
        .interact()?;
    let priority = match priority_idx {
        0 => "high",
        1 => "medium",
        2 => "low",
        _ => unreachable!(),
    };
    filter_by_priority(tasks, priority)?;
    Ok(())
}

fn filter_by_status_interactive(tasks: &Vec<Task>) -> Result<()> {
    let status_options = vec!["Tamamlanmış", "Tamamlanmamış"];
    let status_idx = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Filtrelemek istediğiniz durum")
        .items(&status_options)
        .default(0)
        .interact()?;
    let status = match status_idx {
        0 => "completed",
        1 => "incomplete",
        _ => unreachable!(),
    };
    filter_by_status(tasks, status)?;
    Ok(())
}
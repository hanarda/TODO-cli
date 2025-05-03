use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};


#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub priority: Priority,
    pub deadline: Option<DateTime<Utc>>,
    pub category: String,
}
use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, BufReader, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub name: String,
    pub created_at: NaiveDateTime,
    pub completed: bool,
    pub completed_at: Option<NaiveDateTime>,
}

impl Task {
    pub fn new(name: String) -> Self {
        Task {
            name,
            created_at: Local::now().naive_local(),
            completed: false,
            completed_at: None,
        }
    }

    pub fn complete(&mut self) {
        self.completed = true;
        self.completed_at = Some(Local::now().naive_local());
    }
}

pub fn load_tasks(file_path: &str) -> Vec<Task> {
    let file = File::open(file_path).unwrap_or_else(|_| File::create(file_path).unwrap());
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
}

pub fn save_tasks(tasks: &[Task], file_path: &str) -> io::Result<()> {
    let mut file = File::create(file_path)?;
    let data = serde_json::to_string_pretty(tasks)?;
    file.write_all(data.as_bytes())?;
    Ok(())
}

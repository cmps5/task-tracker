use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
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
    
    /*
    pub fn complete(&mut self) {
        self.completed_at = Some(Local::now().naive_local());
    }
    */
}
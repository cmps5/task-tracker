use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub name: String,
}

impl Task {
    pub fn new (name: String) -> Self {
        Task {
            name,
        }
    }
}
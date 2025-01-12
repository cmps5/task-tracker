use clap::{Parser, Subcommand};
use once_cell::sync::OnceCell;
use std::sync::Mutex;

mod task;
use task::Task;

// define the command line interface structure
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// define the subcommands
#[derive(Subcommand)]
enum Commands {
    Add { name: String },
    Complete { name: String },
    Status,
}

static TASKS: OnceCell<Mutex<Vec<Task>>> = OnceCell::new();

fn init_tasks() {
    TASKS.set(Mutex::new(Vec::new())).ok();
}

fn main() {
    let cli = Cli::parse();
    init_tasks();

    match cli.command {
        Commands::Add { name } => {
            println!("Adding task: {}", name);
            add_task(name);
        }
        Commands::Complete { name } => {
            println!("Completing task: {}", name);
        }
        Commands::Status => {
            println!("Showing status for all task");
        }
    }
}

fn add_task(name: String) {
    let array = TASKS.get().unwrap();
    let mut array = array.lock().unwrap();

    let task = Task::new(name);
    println!("Task {} created at {}", task.name, task.created_at);

    array.push(task);
}

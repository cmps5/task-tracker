use clap::{Parser, Subcommand};

mod task;
use std::process;
use task::{load_tasks, save_tasks, Task};

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

static FILE_PATH: &str = "data/tasks.json";

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { name } => {
            println!("Adding task: {}", name);
            add_task(name);
        }
        Commands::Complete { name } => {
            println!("Completing task: {}", name);
            complete_task(name);
        }
        Commands::Status => {
            println!("Showing status for all task");
            show_status();
        }
    }
}

fn add_task(name: String) {
    let mut tasks = load_tasks(FILE_PATH);

    if tasks.iter().any(|t| t.name == name) {
        println!("Task '{}' already exists!", name);
        process::exit(1);
    }

    let task = Task::new(name);
    tasks.push(task);
    save_tasks(&tasks, FILE_PATH).expect("Failed to save tasks");
    println!("Task added successfully!");
}

fn complete_task(name: String) {
    let mut tasks = load_tasks(FILE_PATH);

    if let Some(task) = tasks.iter_mut().find(|t| t.name == name) {
        task.complete();
        save_tasks(&tasks, FILE_PATH).expect("Failed to save tasks");
        println!("Task '{}' marked as complete!", name);
    } else {
        println!("Task '{}' not found!", name);
        process::exit(1);
    }
}

fn show_status() {
    let tasks = load_tasks(FILE_PATH);

    if tasks.is_empty() {
        println!("No tasks found!");
    } else {
        for task in tasks {
            print!("Task '{}' - created on {} - ", task.name, task.created_at.date());
            if task.completed {
                println!("completed on {}", task.completed_at.unwrap().date());
            } else {
                println!("not completed");
            }
        }
    }
}

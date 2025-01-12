use clap :: {Parser, Subcommand};

// define the command line interface structure
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// define the subcommands
#[derive(Subcommand)]
enum Commands {
    Add {
        name: String,
    },
    Complete {
        name: String,
    },
    Status,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
    Commands :: Add {name} => {
        println!("Adding task: {}", name);
    } 
     Commands :: Complete { name } => {
        println!("Completing task: {}", name);
     }
     Commands :: Status => {
        println!("Showing status for all task");
     }     
    }
}
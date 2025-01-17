# Task Tracker
Task Tracker is a simple command-line application written in Rust to help you manage your tasks efficiently.
Also, includes a graphical user interface (GUI) for a more user-friendly experience.

## Features

- Add new tasks
- Mark tasks as completed
- Show status for all tasks
- Delete tasks
- Clear all tasks

## Installation

To install Task Tracker, you need to have Rust installed on your system. You can install Rust from [here](https://www.rust-lang.org/tools/install).

Clone the repository:
```sh
git clone https://github.com/cmps5/task-tracker.git
cd task-tracker
```

Build the project:
```sh
cargo build --release
```

Run the application:
```sh
./target/release/task_tracker
```

## Usage

To add a new task:
```sh
task_tracker add "Task description"
```

To mark a task as completed:
```sh
task_tracker complete "Task description"
```

To show status for all tasks:
```sh
task_tracker status
```

To delete a task:
```sh
task_tracker delete "Task description"
```

To clear all tasks:
```sh
task_tracker clear
```

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## Contact

For any inquiries or issues, please open an issue on the [GitHub repository](https://github.com/cmps5/task-tracker/issues).

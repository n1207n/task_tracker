# Task Tracker CLI

Task Tracker CLI is a command-line application written in Rust that helps you track and manage your tasks. You can add, update, delete, and list tasks, as well as mark them as in-progress or done.

## Features

- Add new tasks
- Update existing tasks
- Delete tasks
- Mark tasks as in-progress or done
- List all tasks or filter by status (todo, in-progress, done)

## Requirements

- Rust and Cargo installed on your system

## Installation

1. **Clone the Repository**

   ```bash
   git clone https://github.com/n1207n/task_tracker.git
   cd task_tracker
   ```
1. **Build the Project**
    ```bash
    cargo build --release
    ```
1. **Run the Application**
    ```bash
    ./target/release/task_tracker
    ```

## Usage
The application accepts commands and arguments from the command line. Below are some examples of how to use the Task Tracker CLI:
- Add a new task
    ```bash
    task_tracker add "Buy groceries"
    ```
- Update a task
    ```bash
    task_tracker update 1 "Buy groceries and cook dinner"
    ```
- Delete a task
    ```bash
    task_tracker delete 1
    ```
- Mark a task as in-progress
    ```bash
    task_tracker mark-in-progress 1
    ```
- Mark a task as done
    ```bash
    task_tracker mark-done 1
    ```
- List all tasks
    ```bash
    task_tracker list
    ```
- List tasks by status
    ```bash
    task_tracker list done
    task_tracker list todo
    task_tracker list in-progress
    ```

## Run Tests
```bash
cargo test
```

Roadmap URL: https://roadmap.sh/projects/task-tracker
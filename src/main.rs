mod test_main;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id: u32,
    description: String,
    status: String, // todo, in-progress, done
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}


fn main() {
    // CLI argument handling
    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("add") => add_task(&args[2]),
        Some("update") => update_task(args[2].parse().unwrap(), &args[3]),
        Some("delete") => delete_task(args[2].parse().unwrap()),
        Some("mark-in-progress") => mark_task(args[2].parse().unwrap(), "in-progress"),
        Some("mark-done") => mark_task(args[2].parse().unwrap(), "done"),
        Some("list") => list_tasks(args.get(2).map(String::as_str)),
        _ => println!("Invalid command"),
    }
}

fn list_tasks(filter: Option<&str>) {
    let tasks = read_tasks();
    let filtered_tasks: Vec<&Task> = match filter {
        Some("done") => tasks.iter().filter(|task| task.status == "done").collect(),
        Some("todo") => tasks.iter().filter(|task| task.status == "todo").collect(),
        Some("in-progress") => tasks.iter().filter(|task| task.status == "in-progress").collect(),
        _ => tasks.iter().collect()
    };

    for task in filtered_tasks {
        println!("ID: {}, Description: {}, Status: {}, Created At: {}, Updated At: {}", task.id, task.description, task.status, task.created_at, task.updated_at);
    }
}

fn mark_task(id: u32, new_status: &str) {
    let mut tasks = read_tasks();
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.status = new_status.to_string();
        task.updated_at = Utc::now();
        write_tasks(&tasks);
        println!("Task marked as {} (ID: {})", new_status, id);
    } else {
        println!("Task not found (ID: {})", id);
    }
}

fn delete_task(id: u32) {
    let mut tasks = read_tasks();
    let initial_len = tasks.len();

    tasks.retain(|task| task.id != id);
    if tasks.len() < initial_len {
        write_tasks(&tasks);
        println!("Task deleted successfully (ID: {})", id);
    } else {
        println!("Task not found (ID: {})", id);
    }
}

fn update_task(id: u32, new_description: &str) {
    let mut tasks = read_tasks();
    if let Some(task) = tasks.iter_mut().find(|task| task.id == id) {
        task.description = new_description.to_string();
        task.updated_at = Utc::now();
        write_tasks(&tasks);
        println!("Task updated successfully (ID: {})", id);
    } else {
        println!("Task not found (ID: {})", id);
    }
}

fn add_task(description: &str) {
    let mut tasks = read_tasks();
    let new_id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;

    let new_task = Task {
        id: new_id,
        description: description.to_string(),
        status: "todo".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    tasks.push(new_task);
    write_tasks(&tasks);
    println!("Task added successfully (ID: {})", new_id);
}

fn read_tasks() -> Vec<Task> {
    let mut file = File::open("tasks.json").unwrap_or_else(|_| File::create("tasks.json").unwrap());
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    serde_json::from_str(&data).unwrap_or_default()
}

fn write_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string(tasks).unwrap();
    let mut file = OpenOptions::new().write(true).truncate(true).open("tasks.json").unwrap();
    file.write_all(data.as_bytes()).unwrap()
}
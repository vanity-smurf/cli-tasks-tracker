use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    fn from_str(status: &str) -> Self {
        match status {
            "todo" => Status::Todo,
            "in-progress" => Status::InProgress,
            "done" => Status::Done,
            _ => {
                panic!("Invalid status. Use: todo, in-progress, done");
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    status: Status,
}

const FILE_NAME: &str = "tasks.json";

impl Task {
    fn new(id: usize, description: String) -> Self {
        Self {
            id,
            description,
            status: Status::Todo,
        }
    }

    fn update_description(&mut self, new_description: String) {
        self.description = new_description;
    }

    fn update_status(&mut self, new_status: Status) {
        self.status = new_status;   
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: add <desc> | update <id> <desc> | delete <id> | status <id> <status> | list [all|done|not-done|in-progress]");
        return;
    }

    let command = args[1].as_str();
    let mut tasks = load_tasks();
    
    match command {
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: add <desc>");
                return;
            }
            let desc = args[2..].join(" ");
            add_task(&mut tasks, desc);
        }

        "update" => {
            if args.len() < 4 {
                eprintln!("Usage: update <id> <desc>");
                return;
            }
            let id = args[2].parse::<usize>().unwrap_or(0);
            let desc = args[3..].join(" ");
            update_task(&mut tasks, id, desc);
        }

        "delete" => {
            if args.len() < 3 {
                eprintln!("Usage: delete <id>");
                return;
            }
            let id = args[2].parse::<usize>().unwrap_or(0);
            delete_task(&mut tasks, id);
        }

        "status" => {
            if args.len() < 4 {
                eprintln!("Usage: status <id> <status>");
                return;
            }
            let id = args[2].parse::<usize>().unwrap_or(0);
            let status = Status::from_str(args[3].as_str());
            update_status(&mut tasks, id, status);
        }

        "list" => {
            if args.len() < 3 {
                list_tasks(&tasks, "all");
            } else {
                list_tasks(&tasks, &args[2]);
            }
        }

        _ => eprintln!("Unknown command"),
    }
    save_tasks(&tasks);
}

fn save_tasks(tasks: &Vec<Task>) {
    let content = serde_json::to_string_pretty(tasks).expect("Failed to serialize tasks");
    let mut file = File::create(FILE_NAME).expect("Failed to create file");
    file.write_all(content.as_bytes()).expect("Failed to write file");
}

fn load_tasks() -> Vec<Task> {
    if !Path::new(FILE_NAME).exists() {
        return Vec::new();
    }

    let mut file = File::open(FILE_NAME).expect("Failed to open file");
    let mut content = String::new();
    file.read_to_string(&mut content).expect("Failed to read file");
    
    serde_json::from_str(&content).unwrap_or_else(|_| Vec::new())
}

fn add_task(tasks: &mut Vec<Task>, desc: String) {
    let id = tasks.len() + 1;
    let task = Task::new(id, desc);
    tasks.push(task);
    println!("Task added with ID: {}", id);
}

fn update_task(tasks: &mut Vec<Task>, id: usize, desc: String) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.update_description(desc);
        println!("Task {} updated", id);
    } else {
        eprintln!("Task {} not found", id);
    } 
}

fn delete_task(tasks: &mut Vec<Task>, id: usize) {
    if let Some(pos) = tasks.iter().position(|t| t.id == id) {
        tasks.remove(pos);
        println!("Task {} deleted", id);
    } else {
        eprintln!("Task {} not found", id);
    }
}

fn update_status(tasks: &mut Vec<Task>, id: usize, new_status: Status) {
    if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
        task.update_status(new_status);
        println!("Task {} status updated", id);
    } else {
        eprintln!("Task {} not found", id);
    }
}

fn list_tasks(tasks: &Vec<Task>, filter: &str) {
    let filtered_tasks: Vec<&Task> = match filter {
        "done" => tasks.iter().filter(|t| matches!(t.status, Status::Done)).collect(),
        "not-done" => tasks.iter().filter(|t| !matches!(t.status, Status::Done)).collect(),
        "in-progress" => tasks.iter().filter(|t| matches!(t.status, Status::InProgress)).collect(),
        _ => tasks.iter().collect(),
    };

    if filtered_tasks.is_empty() {
        println!("No tasks found");
    } else {
        for task in filtered_tasks {
            println!("[{}] {} - {:?}", task.id, task.description, task.status);
        }
    }
}

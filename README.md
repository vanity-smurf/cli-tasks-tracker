# Task Tracker CLI

A minimalistic command-line task manager written in Rust.

## 📦 Installation

```bash
git clone https://github.com/vanity-smurf/cli-tasks-tracker.git
cd cli-tasks-tracker
cargo install --path .
```

## 🚀 Quick Start

```bash
todo add "Complete Rust project"
todo list
todo status 1 done
```

## ✨ Features

- Add/update/delete tasks
- Change task status (`todo`, `in-progress`, `done`)
- Filter tasks by status
- Persistent storage in `tasks.json`
- Simple and fast

## 📝 Usage

### Add Task
```bash
todo add "Task description"
```

### Update Task
```bash
todo update <ID> "New description"
```

### Change Status
```bash
todo status <ID> <status>
# Statuses: todo, in-progress, done
```

### List Tasks
```bash
todo list [filter]
# Filters: all (default), done, not-done, in-progress
```

### Delete Task
```bash
todo delete <ID>
```

## 🗄️ Data Format

Tasks are saved in `tasks.json`:
```json
[
  {
    "id": 1,
    "description": "Learn Rust",
    "status": "InProgress"
  }
]
```

## 🛠️ Development

Build release:
```bash
cargo build --release
```

Run tests:
```bash
cargo test
```

## 📊 Example Session

```bash
$ todo add "Buy groceries"
Task added with ID: 1

$ todo add "Write documentation"
Task added with ID: 2

$ todo list
[1] Buy groceries - Todo
[2] Write documentation - Todo

$ todo status 1 in-progress
Task 1 status updated

$ todo list in-progress
[1] Buy groceries - InProgress
```

## 📌 Notes

- Requires Rust 1.70+
- Tasks persist between sessions
- No internet connection required

# TaskManager

TaskManager is a simple task management system written in Rust.  
It allows you to create, update, and track tasks using a local file database.

⚠️ **Important note**  
The task description **must not contain the `;` character**.  
If a task is created with this character, it will be necessary to manually remove it from the `database.txt` file, as `;` is used internally as a data separator.

## 🚀 Features
- Add and list tasks
- Mark tasks as completed
- Store data in a local file (`database.txt`)

## 🛠 Requirements
- [Rust](https://www.rust-lang.org/) (latest stable version)
- Cargo (comes with Rust)

## ▶️ How to Run
Clone the repository and run:

```bash
cargo run

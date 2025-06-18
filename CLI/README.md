# ✅ Rust CLI To-do Manager

A simple and lightweight command-line tool written in Rust for managing personal to-do tasks stored as Markdown files.

---

## 📌 What It Does

- Add new tasks from the command line
- View all tasks with status (active or completed)
- Edit tasks in your favorite text editor
- Store everything on the local filesystem (no database)

---

## 🧰 Technologies Used

- [Rust](https://www.rust-lang.org/)
- [`clap`](https://docs.rs/clap/latest/clap/) — CLI argument parsing
- [`std::fs`](https://doc.rust-lang.org/std/fs/) — File system operations
- [`std::process::Command`](https://doc.rust-lang.org/std/process/) — Run external editor
- [`colored`](https://docs.rs/colored/) — Optional: color output in terminal

---

## ⚙️ Available Commands

```bash
todo init                     # Initialize the 'journal/' directory
todo add "Task title"         # Add a new Markdown task file
todo list                    # Display all tasks with status
todo toggle "Task title"      # Toggle task status (active <-> completed)
todo edit "Task title"        # Open the task in $EDITOR or nano

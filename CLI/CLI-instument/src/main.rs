use clap::{Parser, Subcommand};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(name = "todo-cli")]
#[command(about = "A Markdown-based to-do list CLI tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initializes the journal directory
    Init,
    /// Adds a new task
    Add { title: String },
    /// Lists all tasks
    List,
    /// Toggles the completion status of a task
    Toggle { title: String },
    /// Edits a task file
    Edit { title: String },
    /// Removes a task
    Remove { title: String },
    /// Clears all tasks from the journal
    Clear,
}

fn main() {
    let cli = Cli::parse();
    let journal_dir: &Path = Path::new("journal");

    match &cli.command {
        Commands::Init => {
            if !journal_dir.exists() {
                fs::create_dir(journal_dir).expect("Failed to create journal directory");
                println!("✅ Folder 'journal' created: {}", journal_dir.display());
            } else {
                println!("📁 Folder already exists: {}", journal_dir.display());
            }
        }

        Commands::Add { title } => {
            if !journal_dir.exists() {
                println!("⚠️ 'journal' directory not found. Please run 'init' first.");
                return;
            }
            if find_task(journal_dir, title).is_some() {
                println!("⚠️ Task '{}' already exists.", title);
                return;
            }
            let file_path: PathBuf = journal_dir.join(format!("{}.md", title));
            fs::File::create(&file_path).expect("Failed to create task file");
            println!("📝 Added task: {}", title);
        }

        Commands::List => {
            if !journal_dir.exists() {
                println!("⚠️ 'journal' directory not found. Please run 'init' first.");
                return;
            }
            println!("📋 List of tasks:\n");
            let entries = fs::read_dir(journal_dir).expect("Could not read journal directory");
            for entry in entries {
                let entry = entry.expect("Error reading directory entry");
                let file_name = entry.file_name();
                let file_name_str: std::borrow::Cow<'_, str> = file_name.to_string_lossy();
                if file_name_str.ends_with(".md") {
                    if file_name_str.ends_with(".completed.md") {
                        println!("✅ {}", file_name_str);
                    } else {
                        println!("🔲 {}", file_name_str);
                    }
                }
            }
        }

        Commands::Toggle { title } => {
            match find_task(journal_dir, title) {
                Some(path) => {
                    let file_name: std::borrow::Cow<'_, str> = path.file_name().unwrap().to_string_lossy();
                    if file_name.ends_with(".completed.md") {
                        let new_path = journal_dir.join(format!("{}.md", title));
                        fs::rename(&path, new_path).expect("Failed to rename task");
                        println!("🔄 Task '{}' marked as pending.", title);
                    } else {
                        let new_path: PathBuf = journal_dir.join(format!("{}.completed.md", title));
                        fs::rename(&path, new_path).expect("Failed to rename task");
                        println!("✅ Task '{}' marked as completed.", title);
                    }
                }
                None => println!("⚠️ Task '{}' not found.", title),
            }
        }

        Commands::Edit { title } => {
            match find_task(journal_dir, title) {
                Some(path) => {
                    edit::edit_file(&path).expect("Failed to open editor");
                    println!("✍️ Finished editing '{}'.", title);
                }
                None => println!("⚠️ Task '{}' not found.", title),
            }
        }

        Commands::Remove { title } => {
            match find_task(journal_dir, title) {
                Some(path) => {
                    fs::remove_file(&path).expect("Failed to remove task file");
                    println!("🗑️ Removed task '{}'.", title);
                }
                None => println!("⚠️ Task '{}' not found.", title),
            }
        }

        Commands::Clear => {
            print!("🔥 Are you sure you want to delete all tasks? [y/N]: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            if input.trim().eq_ignore_ascii_case("y") {
                if journal_dir.exists() {
                    fs::remove_dir_all(journal_dir).expect("Failed to clear journal");
                    fs::create_dir(journal_dir).expect("Failed to recreate journal directory");
                    println!("✨ Journal cleared.");
                }
            } else {
                println!("🚫 Operation cancelled.");
            }
        }
    }
}

/// Finds a task file by its title in the journal directory.
fn find_task(dir: &Path, title: &str) -> Option<PathBuf> {
    let pending_path: PathBuf = dir.join(format!("{}.md", title));
    if pending_path.exists() {
        return Some(pending_path);
    }
    let completed_path = dir.join(format!("{}.completed.md", title));
    if completed_path.exists() {
        return Some(completed_path);
    }
    None
}

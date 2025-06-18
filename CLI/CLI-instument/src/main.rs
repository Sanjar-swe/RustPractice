use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "todo-cli")]
#[command(about = "Markdown To-do CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add { title: String },
    List,
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init => {
            let path = Path::new("journal");
            let msg = "Failed to create journal";

            if !path.exists() {
                fs::create_dir(path).expect(msg);
                println!("✅ Folder 'journal' created: {}", path.display());
            } else {
                println!("📁 Folder already exists: {}", path.display());
            }
        }

        Commands::Add { title } => {
            println!("📝 Adding task: {}", title);
            // only printing for now, implementation will be added later
        }

        // List
        Commands::List => {
            let path = Path::new("journal");

            if !path.exists(){
                println!("⚠️ Folder 'journal' does not found. Start todo cli for starting point");
                return
            }

            println!("📋 List of tasks: \n");

            let entries = fs::read_dir(path).expect("Could not read journal");

            for entry in entries { 
                let entry = entry.expect("Error reading file");
                let file_name = entry.file_name();
                let file_name_str = file_name.to_string_lossy();

                if file_name_str.ends_with(".md") {
                    if file_name_str.ends_with(".completed.md"){
                        println!("✅ {}", file_name_str);
                    } else {
                        println!("🔲 {}", file_name_str)
                    }
                }
            }
        }
    }
}

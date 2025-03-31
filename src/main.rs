use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::process::Command;

// Import the content module
mod content;
use content::templates;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Bruno project
    Init {
        /// Name of the project
        project_name: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Init { project_name } => {
            init_project(project_name)?;
        }
    }

    Ok(())
}

fn init_project(project_name: &str) -> Result<()> {
    println!("\x1b[38;2;255;160;122m"); // Custom RGB color for coral/orange
    println!(r#"
▄▄▄▄· ▄▄▄  ▄• ▄▌ ▐ ▄       
▐█ ▀█▪▀▄ █·█▪██▌•█▌▐█▪     
▐█▀▀█▄▐▀▀▄ █▌▐█▌▐█▐▐▌ ▄█▀▄ 
██▄▪▐█▐█•█▌▐█▄█▌██▐█▌▐█▌.▐▌
·▀▀▀▀ .▀  ▀ ▀▀▀ ▀▀ █▪ ▀█▄▀▪
"#);
    println!("\x1b[0m");

    // Display project initialization message with styling
    println!("\x1b[38;2;255;160;122m🚀 Initializing Bruno Project: {}\x1b[0m", project_name);
    println!("\x1b[90m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");

    // Create the project directory
    let project_dir = Path::new(project_name);
    fs::create_dir_all(project_dir)
        .with_context(|| format!("Failed to create project directory: {}", project_name))?;

    // Create a new Cargo project inside
    let output = Command::new("cargo")
        .arg("init")
        .arg("--name")
        .arg(project_name)
        .current_dir(project_dir)
        .output()
        .with_context(|| "Failed to run 'cargo init'")?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("Failed to initialize Cargo project: {}", error);
    }

    // Create a basic project structure using templates
    create_project_structure(project_dir)?;

    // Generate custom Cargo.toml with dependencies
    update_cargo_toml(project_dir, project_name)?;

    println!("\x1b[90m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
    println!("\x1b[38;2;255;160;122m✅ Project '{}' initialized successfully!\x1b[0m", project_name);
    println!("\n\x1b[38;2;255;160;122m📋 Next steps:\x1b[0m");
    println!("\x1b[38;2;255;160;122m$ cd {}\x1b[0m", project_name);
    println!("\x1b[38;2;255;160;122m$ bruno build\x1b[0m");
    println!("\x1b[90m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\x1b[0m");
    Ok(())
}

fn create_project_structure(project_dir: &Path) -> Result<()> {
    // Create src directory structure
    let src_dir = project_dir.join("src");

    // Create files using templates from content.rs

    // Create lib.rs
    fs::write(src_dir.join("lib.rs"), templates::lib_rs())?;

    // Create main.rs
    fs::write(src_dir.join("main.rs"), templates::main_rs())?;

    // Create .env.example
    fs::write(project_dir.join(".env.example"), templates::env())?;

    // Create configuration files
    fs::write(project_dir.join("README.md"), templates::readme_md())?;
    fs::write(project_dir.join(".gitignore"), templates::gitignore())?;
    fs::write(project_dir.join("bruno.json"), templates::bruno_json())?;

    Ok(())
}

fn update_cargo_toml(project_dir: &Path, project_name: &str) -> Result<()> {
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"
tokio = {{ version = "1.32", features = ["full"] }}
anyhow = "1.0"
thiserror = "1.0"
"#,
        project_name
    );

    fs::write(project_dir.join("Cargo.toml"), cargo_toml)?;

    Ok(())
}

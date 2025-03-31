use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;
use std::process::Command;

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
    println!("Initializing new Bruno project: {}", project_name);

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

    // Create a basic project structure
    create_project_structure(project_dir)?;

    // Generate custom Cargo.toml with dependencies
    update_cargo_toml(project_dir, project_name)?;

    println!(
        "✅ Bruno project '{}' initialized successfully!",
        project_name
    );
    println!("Run the following commands to get started:");
    println!("  cd {}", project_name);
    println!("  cargo build");

    Ok(())
}

fn create_project_structure(project_dir: &Path) -> Result<()> {
    // Create src directory structure
    let src_dir = project_dir.join("src");

    // Create directories
    fs::create_dir_all(src_dir.join("models"))?;
    fs::create_dir_all(src_dir.join("controllers"))?;
    fs::create_dir_all(src_dir.join("views"))?;
    fs::create_dir_all(src_dir.join("utils"))?;

    // Create a README.md file
    let readme_content = "# Bruno Project\n\nA project created with Bruno CLI.\n";
    fs::write(project_dir.join("README.md"), readme_content)?;

    // Create a .gitignore file
    let gitignore_content = "/target\nCargo.lock\n";
    fs::write(project_dir.join(".gitignore"), gitignore_content)?;

    // Create a basic config file
    let config_content = r#"{
    "name": "bruno-project",
    "version": "0.1.0",
    "description": "A project created with Bruno CLI"
}"#;
    fs::write(project_dir.join("bruno.json"), config_content)?;

    // Create a sample main.rs
    let main_content = r#"mod models;
mod controllers;
mod views;
mod utils;

fn main() {
    println!("Welcome to your Bruno project!");
}
"#;
    fs::write(src_dir.join("main.rs"), main_content)?;

    // Create basic module files
    fs::write(
        src_dir.join("models.rs"),
        "// Define your data models here\n",
    )?;
    fs::write(
        src_dir.join("controllers.rs"),
        "// Define your controllers here\n",
    )?;
    fs::write(src_dir.join("views.rs"), "// Define your views here\n")?;
    fs::write(
        src_dir.join("utils.rs"),
        "// Define your utility functions here\n",
    )?;

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

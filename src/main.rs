use clap::Parser;
use std::env;
use std::io;
use toml::Table;
use serde::Deserialize;

/// Create and manage heirarchical project folders.
#[derive(Parser)]
#[command(name="mkpr",version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(long)]
    name: String,
}

#[derive(Deserialize)]
struct Vault {
    name: String,
    project_identifiers: Vec<String>,
    additional_subdirectories: Vec<String>,
    number_of_parts: u32,
}

fn main() {
    let cli = Cli::parse();
    let vault_name = cli.name;

    match env::current_exe() {
    Ok(exe_path) => println!("Vaults will be created in: {}",
                            exe_path.display()),
    Err(e) => println!("failed to get current path: {e}"),
    };

    println!("Create new Project? (y/N): ");
    let input_1 = user_y_n();
    println!("{vault_name}");
    println!("{input_1}");
}

fn user_y_n() -> bool{
    let mut condition =String::new();
    io::stdin()
        .read_line(&mut condition)
        .expect("can not read user input");

    match condition.trim() {
        "y" => true,
        "yes" => true,
        _ => false  // Or whatever appropriate default value or error.
    }
}


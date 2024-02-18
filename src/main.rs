use clap::Parser;
use serde_derive::Serialize;
use std::env;
use std::io;
use serde_derive::Deserialize;

/// Create and manage heirarchical project folders.
#[derive(Parser)]
#[command(name="mkpr",version, about, long_about = None)] // Read from `Cargo.toml`
struct Cli {
    #[arg(long)]
    name: String,
}

#[derive(Deserialize, Serialize)]
struct Vault {
    name: String,
    projects: Vec<Project>,
}

#[derive(Deserialize, Serialize)]
struct Project{
    project_identifiers: Vec<String>,
    additional_subdirectories: Vec<String>,
    number_of_parts: u32,
}

fn main() {
    let cli = Cli::parse();
    let vault_prefix = cli.name;

    match env::current_exe() {
    Ok(exe_path) => println!("Vaults will be created in: {}",
                            exe_path.display()),
    Err(e) => eprintln!("failed to get current path: {e}"),
    };
    println!("{vault_prefix}");

    // Vault object creation
    let mut new_vault: Vault = Vault { name: (String::from(vault_prefix + "-XXXX")), projects: Vec::new() };
    let input_1 = bool_user_prompt("Create new Project? (y/N): ");
    if input_1 {
        let new_project_identifier = &string_user_prompt("New project identifier? (XX): ")[0..1];
        let new_project_identifiers: Vec<String> = vec![String::from(new_project_identifier)];
        let new_sub_directories: Vec<String> = vector_user_prompt("Extra directory name? ('exit' to stop): ");
        let new_number_of_parts: u32 = repeating_numerical_prompt("Enter number of parts to create (non-negative integer): ");
        let new_project: Project = Project { project_identifiers: (new_project_identifiers), additional_subdirectories: (new_sub_directories), number_of_parts: (new_number_of_parts) };
        new_vault.projects.push(new_project); // Push new project into new vault object 
    }

    // Here we need to construct the file system from the object and write it to all the correct files
    // This would complete the process of creating new project files, more or less.

    // Next what would be required is a manner to load an object and potentially refresh its filesystem representation?
    // as well as add new projects to existing vaults
    // this is relatively harder, probably this new project creation could be abstracted to an implicit method of the vault object,
    // automatically pushing the object into the object, so that the toml can be updated

}


/// Promp the user for A yes or no input
fn bool_user_prompt(message: &str) -> bool{
    println!("{}",message);
    let mut condition =String::new();
    io::stdin()
        .read_line(&mut condition)
        .expect("can not read user input");

    match condition.trim() {
        "y" => true,
        "yes" => true,
        _ => false // assume false
    }
}

/// Prompt the user for a string like input
fn string_user_prompt(message: &str) -> String{
    println!("{}",message);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("can not read user input");
    return input;
}

fn vector_user_prompt(message: &str) -> Vec<String>{
    let mut vector_input = Vec::new();
    loop {
        println!("{}",message);
        let mut input = String::new();
        io::stdin()
          .read_line(&mut input)
         .expect("can not read user input");
        if input.as_str() == "exit"{
            break;
        }
        vector_input.push(input);
    }
    return vector_input;
}

fn repeating_numerical_prompt(message: &str) -> u32{
    loop {
        println!("{}",message);
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("can not read user input");
        let input: i32 = match input.trim().parse(){
            Ok(input) => input,
            Err(_) => -1, 
        };
        if input.is_negative(){
            eprintln!("Invalid input.");
        }
        else{
            return input.wrapping_abs() as u32; // Might break???
        }
    }
}
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[derive(Deserialize)]
struct Config {
    directory: String,
    environment: String,
    script: String,
    env_file: String,
    conda_path: String,
}

// Reads the config file and parses it into the Config struct
fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = Path::new("config.toml");

    if !config_path.exists() {
        eprintln!("Config file not found in the same directory as the executable. Please ensure 'config.toml' is present.");

        // Prompt the user to press Enter before exiting
        print!("Press Enter to exit...");
        io::stdout().flush()?; // Ensure the message is printed before reading input
        let _ = io::stdin().read_line(&mut String::new());

        std::process::exit(1); // Exit the program with a non-zero status
    }

    let config_content = fs::read_to_string(config_path)?;
    let config: Config = toml::from_str(&config_content)?;
    Ok(config)
}

fn main() {
    // Read and parse the configuration file
    // Read and parse the configuration file
    let config = read_config().expect("Failed to read configuration");

    // Change directory to the one specified in the config
    env::set_current_dir(&config.directory).expect("Failed to change directory");

    // Full path to the conda executable
    let conda_executable = format!("{}\\condabin\\conda.bat", config.conda_path);

    // Command to create the conda environment from the environment.yaml file
    let create_env_command = format!("{} env create -f {}", conda_executable, config.env_file);

    // Command to update the conda environment from the environment.yaml file
    let update_env_command = format!(
        "{} env update -f {} --prune",
        conda_executable, config.env_file
    );

    // Run the command to create the environment
    Command::new("cmd")
        .args(&["/C", &create_env_command])
        .status()
        .expect("Failed to create the conda environment");

    // Run the command to update the environment
    Command::new("cmd")
        .args(&["/C", &update_env_command])
        .status()
        .expect("Failed to update the conda environment");

    // Command to activate the conda environment and run the Streamlit app
    let run_command = format!(
        "{} activate {} && streamlit run {}",
        conda_executable, config.environment, config.script
    );

    // Execute the command to activate the environment and run the app
    Command::new("cmd")
        .args(&["/C", &run_command])
        .status()
        .expect("Failed to execute process");
}

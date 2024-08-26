use serde::Deserialize;
use std::env;
use std::fs;
use std::process::Command;

#[derive(Deserialize)]
struct Config {
    directory: String,
    environment: String,
    script: String,
    env_file: String,
}

fn main() {
    // Read and parse the configuration file
    let config_data = fs::read_to_string("config.toml").expect("Failed to read configuration file");

    let config: Config = toml::from_str(&config_data).expect("Failed to parse configuration file");

    // Change directory to the one specified in the config
    env::set_current_dir(&config.directory).expect("Failed to change directory");

    // Command to create the conda environment from the environment.yaml file
    let create_env_command = format!("conda env create -f {}", config.env_file);

    // Command to update the conda environment from the environment.yaml file
    let update_env_command = format!("conda env update -f {} --prune", config.env_file);

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
        "conda activate {} && streamlit run {}",
        config.environment, config.script
    );

    // Execute the command to activate the environment and run the app
    Command::new("cmd")
        .args(&["/C", &run_command])
        .status()
        .expect("Failed to execute process");
}

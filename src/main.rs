use log::{error, info};
use serde::Deserialize;
use simplelog::*;
use std::env;
use std::env::consts::OS;
use std::fs::File;
use std::fs::{self};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

// Struct to hold the configuration
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
        error!("Config file not found in the same directory as the executable. Please ensure 'config.toml' is present.");
        print!("Press Enter to exit...");
        io::stdout().flush()?; // Ensure the message is printed before reading input
        let _ = io::stdin().read_line(&mut String::new());
        std::process::exit(1);
    }

    let config_content = fs::read_to_string(config_path).map_err(|e| {
        error!("Failed to read configuration file: {}", e);
        e
    })?;

    info!("Configuration file read successfully.");
    let config: Config = toml::from_str(&config_content).map_err(|e| {
        error!("Failed to parse configuration file: {}", e);
        e
    })?;

    info!("Configuration file parsed successfully.");
    Ok(config)
}

fn run_conda_command(
    command: &str,
) -> Result<std::process::ExitStatus, Box<dyn std::error::Error>> {
    if OS == "windows" {
        // Windows uses cmd
        Command::new("cmd")
            .args(&["/C", command])
            .status()
            .map_err(|e| e.into())
    } else {
        // macOS/Linux can run commands directly
        Command::new("zsh")
            .arg("-c")
            .arg(command)
            .status()
            .map_err(|e| e.into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging to a file
    CombinedLogger::init(vec![WriteLogger::new(
        LevelFilter::Info,
        ConfigBuilder::new().build(),
        File::create("app.log")?,
    )])?;

    info!("Script started.");

    // Read and parse the configuration file
    let config = read_config().expect("Failed to read configuration");

    // Change directory to the one specified in the config
    if let Err(e) = env::set_current_dir(&config.directory) {
        error!("Failed to change directory: {}", e);
        return Ok(());
    }
    info!("Changed directory to: {}", config.directory);

    // Full path to the conda executable
    let conda_executable = if OS == "windows" {
        format!("{}\\condabin\\conda.bat", config.conda_path)
    } else {
        format!("{}/bin/conda", config.conda_path)
    };
    let create_env_command = format!("{} env create -f {}", conda_executable, config.env_file);
    info!(
        "Running command to create conda environment: {}",
        create_env_command
    );

    // Run the command to create the environment
    let create_status = run_conda_command(&create_env_command);

    match create_status {
        Ok(status) if status.success() => {
            info!("Conda environment created successfully.");
        }
        Ok(status) => {
            error!(
                "Conda environment creation failed with exit code: {}",
                status
            );
        }
        Err(e) => {
            error!("Failed to create conda environment: {}", e);
            return Ok(());
        }
    }

    // Command to update the conda environment from the environment.yaml file
    let update_env_command = format!(
            "{} env update -f {} --prune",
            conda_executable, config.env_file
        );
        info!(
            "Running command to update conda environment: {}",
            update_env_command
        );

        // Run the command to update the environment
        let update_status = run_conda_command(&update_env_command);

        match update_status {
            Ok(status) if status.success() => {
                info!("Conda environment updated successfully.");
            }
            Ok(status) => {
                error!("Conda environment update failed with exit code: {}", status);
            }
            Err(e) => {
                error!("Failed to update conda environment: {}", e);
                return Ok(());
            }
        }

        // Command to activate the conda environment and run the Streamlit app
        let run_command = if OS == "windows" {
                format!(
                    "{} activate {} && streamlit run {}",
                    conda_executable, config.environment, config.script
                )
            } else {
                format!(
                    "source {}/etc/profile.d/conda.sh && conda activate {} && streamlit run {}",
                    config.conda_path, config.environment, config.script
                )
            };
            info!("Running command to start Streamlit app: {}", run_command);

        // Execute the command to activate the environment and run the app
        let run_status = run_conda_command(&run_command);

        match run_status {
            Ok(_) => {
                info!("Streamlit app started successfully.");
            }
            Err(e) => {
                error!("Failed to execute process: {}", e);
                return Ok(());
            }
        }

        info!("Script completed.");
        Ok(())
    }

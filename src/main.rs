use serde::Deserialize;
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, BufWriter, Write};
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

fn log_message(message: &str, log_file: &mut BufWriter<std::fs::File>) {
    let log_entry = format!(
        "{}: {}\n",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        message
    );
    log_file
        .write_all(log_entry.as_bytes())
        .expect("Failed to write to log file");
    log_file.flush().expect("Failed to flush log file");
}

// Reads the config file and parses it into the Config struct
fn read_config(
    log_file: &mut BufWriter<std::fs::File>,
) -> Result<Config, Box<dyn std::error::Error>> {
    let config_path = Path::new("config.toml");

    if !config_path.exists() {
        eprintln!("Config file not found in the same directory as the executable. Please ensure 'config.toml' is present.");

        // Prompt the user to press Enter before exiting
        print!("Press Enter to exit...");
        io::stdout().flush()?; // Ensure the message is printed before reading input
        let _ = io::stdin().read_line(&mut String::new());

        std::process::exit(1); // Exit the program with a non-zero status
    }

    let config_content = match fs::read_to_string(config_path) {
        Ok(data) => {
            log_message("Configuration file read successfully.", log_file);
            data
        }
        Err(e) => {
            log_message(
                &format!("Failed to read configuration file: {}", e),
                log_file,
            );
            return Err(Box::new(e));
        }
    };

    let config: Config = match toml::from_str(&config_content) {
        Ok(cfg) => {
            log_message("Configuration file parsed successfully.", log_file);
            cfg
        }
        Err(e) => {
            log_message(
                &format!("Failed to parse configuration file: {}", e),
                log_file,
            );
            return Err(Box::new(e));
        }
    };

    Ok(config)
}

fn main() {
    let log_file_path = "script_log.txt";
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file_path)
        .expect("Failed to open log file");
    let mut log_file = BufWriter::new(log_file);

    log_message("Script started.", &mut log_file);

    // Read and parse the configuration file
    let config = read_config(&mut log_file).expect("Failed to read configuration");

    // Change directory to the one specified in the config
    if let Err(e) = env::set_current_dir(&config.directory) {
        log_message(&format!("Failed to change directory: {}", e), &mut log_file);
        return;
    }
    log_message(
        &format!("Changed directory to: {}", config.directory),
        &mut log_file,
    );

    // Full path to the conda executable
    let conda_executable = format!("{}\\condabin\\conda.bat", config.conda_path);

    // Command to create the conda environment from the environment.yaml file
    let create_env_command = format!("{} env create -f {}", conda_executable, config.env_file);
    log_message(
        &format!(
            "Running command to create conda environment: {}",
            create_env_command
        ),
        &mut log_file,
    );

    // Run the command to create the environment
    let create_status = Command::new("cmd")
        .args(&["/C", &create_env_command])
        .status();

    match create_status {
        Ok(status) => {
            if status.success() {
                log_message("Conda environment created successfully.", &mut log_file);
            } else {
                log_message(
                    &format!(
                        "Conda environment creation failed with exit code: {}",
                        status
                    ),
                    &mut log_file,
                );
            }
        }
        Err(e) => {
            log_message(
                &format!("Failed to create conda environment: {}", e),
                &mut log_file,
            );
            return;
        }
    }

    // Command to update the conda environment from the environment.yaml file
    let update_env_command = format!(
        "{} env update -f {} --prune",
        conda_executable, config.env_file
    );
    log_message(
        &format!(
            "Running command to update conda environment: {}",
            update_env_command
        ),
        &mut log_file,
    );

    // Run the command to update the environment
    let update_status = Command::new("cmd")
        .args(&["/C", &update_env_command])
        .status();

    match update_status {
        Ok(status) => {
            if status.success() {
                log_message("Conda environment updated successfully.", &mut log_file);
            } else {
                log_message(
                    &format!("Conda environment update failed with exit code: {}", status),
                    &mut log_file,
                );
            }
        }
        Err(e) => {
            log_message(
                &format!("Failed to update conda environment: {}", e),
                &mut log_file,
            );
            return;
        }
    }

    // Command to activate the conda environment and run the Streamlit app
    let run_command = format!(
        "{} activate {} && streamlit run {}",
        conda_executable, config.environment, config.script
    );
    log_message(
        &format!("Running command to start Streamlit app: {}", run_command),
        &mut log_file,
    );

    // Execute the command to activate the environment and run the app
    let run_status = Command::new("cmd").args(&["/C", &run_command]).status();

    match run_status {
        Ok(_) => {
            log_message("Streamlit app started successfully.", &mut log_file);
        }
        Err(e) => {
            log_message(&format!("Failed to execute process: {}", e), &mut log_file);
            return;
        }
    }

    log_message("Script completed.", &mut log_file);
}

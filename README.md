# Streamlit Startup Script

> **_(Not supporting any other OS besides Windows currently...)_**

This simple script is designed to be run on startup and will automatically startup a streamlit server that uses conda environments.

The config.toml file controls the setup options for the code. To run run things properly the config.toml should sit with the executible in the same directory.


## Script Steps:

- Reads the config file to get information on what files to read
- Changes directory to the streamlit application folder
- Attempts to create the conda environment from the environment file specified.
- If the environment already exists the above step will skip, however then it will attempt to update the environment if there have been any changes to package versions in the environment file vs the stored environment.
- Next the conda environment that was created or updated is activated
- Then the streamlit server is run. It is possible to add arguments if needed to the ending of the entry python file name (eg. "app.py --port 80")

## Running the Script on Windows Startup

1. Task Scheduler:

   Use Windows Task Scheduler to run your Rust script at startup.

   Steps:

   1. Open Task Scheduler from the Start Menu.
   2. Select “Create Task” from the right panel.
   3. Name the task (e.g., “Streamlit Startup Script”).
   4. Choose “When the computer starts” as the trigger.
   5. Select “Start a Program” as the action.
   6. Browse to the compiled Rust executable (.exe) and select it.
   7. Make sure to modify the "Start In (optional)" field to be the directory that holds the executable and the config. Logs will also generate here after.
   8. Configure the task to run with highest privileges if needed.

2. Startup Folder:

   Place a shortcut to the Rust executable in the Windows Startup folder.

   Steps:

   1. Press Win + R and type shell:startup to open the Startup folder.
   2. Create a shortcut to your Rust script’s executable in this folder.

For both the procedures mentioned above, ensure the config.toml file sits with the actual source executible in the same directory (does not have to be with the shortcut).

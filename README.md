# Chrono Forge

A personal time tracking tool written in Rust.

![Chrono Forge V 0.1.0.png](Chrono%20Forge%20V%200.1.0.png)
## Features

- Projects
- Tasks
- Time entries
- Timer
- CSV import/export
- Terminal UI
- CLI

## Installation

### Artifact
For regular usage, prefer the prebuilt release artifact.
Building from source is mainly intended for development.

1. Download the binary from the latest GitHub Release.
2. Move the downloaded directory to the desired location
3. Run the executable from the terminal
4. Done!🎉

### Build from source

1. Make rust available on your system. Use the rustup tool provided at https://rustup.rs
2. Clone the *Chrono Forge* repository
3. Navigate to the directory you cloned this repository into
4. Run *cargo build* in this directory
5. Use *cargo run* to run the application

## Usage
```bash
chrono-forge --help
chrono-forge project create "My project"                    # Creates a new project
chrono-forge project list                                   # Lists all projects including their IDs
chrono-forge task create "My Task" --project <PROJECT_ID>   # Create a new task, assigned to a project
chrono-forge task list                                      # List all tasks with their ids
chrono-forge timer start --task <TASK_ID>                   # Start a timer for that task
chrono-forge timer stop                                     # Stop the timer currently running

chrono-forge                                                # Start the TUI interface
```

## Data Storage
The SQLite database is currently stored right next to the executable as chrono-forge.db.
**Do not attempt to alter the location of that database file, this is not supported yet.**
Make sure to back up *chrono-forge.db* before updating to a newer version.

## Known limitations
- Early 0.2.0 release
- Paths when writing or reading always need to start at the root directory.
- No Themes or alternative display styles implemented yet
- The major development platform for this project is Linux. It should also work on macOS and Windows, but approach it with extra care.


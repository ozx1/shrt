use colored::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process;
use std::{env, io};

#[derive(Debug, Deserialize, Serialize)]
struct Command {
    command: String,
    args: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Commands {
    #[serde(flatten)]
    commands: HashMap<String, HashMap<String, Command>>,     
}

#[derive(Debug)]
enum CommandType {
    Run,
    Config,
    Help,
}

#[derive(Serialize, Deserialize, Default)]
struct AppConfig {
    path: Option<PathBuf>,
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let is_configured = load_file_path().is_some();

    if args.len() > 3 {
        // to check the number of arguments
        eprintln!("Too many arguments!");
        process::exit(1);
    }

    let is_config_command = args.get(1).map_or(false, |arg| arg == "config");
    if !is_configured && !is_config_command {
        println!("\n{}: Shorter is not configured yet", "Warning".yellow());
        println!(
            "{} : \"{} config <path/to/file.json>",
            "Configuration".green(),
            args[0]
        );
        process::exit(0);
    } else if !is_config_command {
        if let Some(config_path) = load_file_path() {
            if let Err(e) = validate_path(&config_path) {
                eprintln!("{}: The path is not valid anymore", "Error".red());
                eprintln!("{}", e);
                eprintln!("Please re-configure the app");
                println!(
                    "{} : \"{} config <path/to/file.json>",
                    "Configuration".green(),
                    args[0]
                );
                process::exit(1);
            }
        }
    }

    let command = match args.get(1) {
        None => CommandType::Help,
        Some(x) => match x.to_lowercase().as_str() {
            "help" | "-h" | "--help" => CommandType::Help,
            "config" => CommandType::Config,
            _ => CommandType::Run,
        },
    };

    match command {
        CommandType::Config => {
            match args.get(2) {
                Some(path_str) => {
                    // User provided a new path - save it
                    let user_path = PathBuf::from(path_str);
                    if let Err(e) = save_file_path(user_path) {
                        eprintln!("Error: {}", e);
                        eprintln!("Please provide a valid path to a JSON file");
                        process::exit(1)
                    } else {
                        println!("{}", "Configuration saved successfully!".green());
                    }
                }
                None => {
                    // No path provided - open existing config if configured
                    if is_configured {
                        if let Some(config_path) = load_file_path() {
                            if let Err(e) = open::that(&config_path) {
                                eprintln!("Failed to open file: {}", e);
                                process::exit(1);
                            }
                            println!("Opening: {}", config_path.display());
                        }
                    } else {
                        eprintln!("Error: Missing file path");
                        eprintln!("Configure the file before opening it");
                        eprintln!("Usage: {} config <path/to/file.json>", args[0]);
                        process::exit(1);
                    }
                }
            }
        }
        CommandType::Help => {
            print_help(&args[0]);
        }
        CommandType::Run => {
            match read_json() {
                Ok(commands) => {
                    if let Some(command) = commands.commands.get(&args[1]) {
                        for cmd in command {

                            println!("{:#?}",command)
                        //     let scommand = command.get(cmd).unwrap();
                        //     let program = &scommand.command;
                        //     let arguments = &scommand.args;

                        //     println!("command {}",cmd);

                        //     let mut cmd = if cfg!(target_os = "windows") {
                        //         let mut c = process::Command::new("cmd"); // Add process:: here
                        //         c.args(&["/C", program]);
                        //         c
                        //     } else {
                        //         process::Command::new(program)
                        //     };

                        //     cmd.args(arguments)
                        //         .spawn()
                        //         .expect("failed to spawn")
                        //         .wait()
                        //         .expect("failed to wait");
                         }
                    } else {
                        eprintln!("this command is not valid");
                        process::exit(1)
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
            // TODO: implement
        }
    }
}

fn validate_path(path: &PathBuf) -> Result<(), String> {
    if !path.exists() {
        return Err("Path doesn't exist".to_string());
    }
    if !path.is_file() {
        return Err("Path is not a file".to_string());
    }
    if !is_json_file(&path) {
        return Err("File must have .json extension".to_string());
    }
    Ok(())
}

fn save_file_path(path: PathBuf) -> Result<(), String> {
    validate_path(&path)?;

    let config = AppConfig { path: Some(path) };
    confy::store("shrt", "path", &config).map_err(|e| format!("Failed to store config: {}", e))
}

fn is_json_file(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("json"))
        .unwrap_or(false)
}

fn load_file_path() -> Option<PathBuf> {
    let config: AppConfig = confy::load("shrt", "path").ok()?;
    config.path
}

fn print_help(app_name: &str) {
    // TODO: add list for valid commands
    println!("Usage: {} <command>", app_name);
    println!("Commands:");
    println!("  config <path>         Configure the JSON file path");
    println!("  config                Open the config file when it's configured");
    println!("  help / --help / -h    Show this help message");
}

fn read_json() -> Result<Commands, io::Error> {
    if let Some(config_path) = load_file_path() {
        let file = File::open(config_path)?;
        let reader = BufReader::new(file);
        let result: Commands = serde_json::from_reader(reader)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e.to_string()))?;

        Ok(result)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Config file not found",
        ))
    }
}

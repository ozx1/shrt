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
    if let Err(e) = run() {
        eprintln!("{}: {}", "Error".red(), e);
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let is_configured = load_file_path().is_some();

    if args.len() > 3 {
        return Err("Too many arguments".to_string());
    }

    let is_config_command = args.get(1).map_or(false, |arg| arg == "config");
    
    if !is_configured && !is_config_command {
        println!("\n{}: Shorter is not configured yet", "Warning".yellow());
        println!(
            "{}: {} config <path/to/file.json>",
            "Configuration".green(),
            args[0]
        );
        return Ok(());
    } else if !is_config_command {
        if let Some(config_path) = load_file_path() {
            validate_path(&config_path).map_err(|e| {
                format!(
                    "The configured path is no longer valid: {}\nPlease re-configure using: {} config <path/to/file.json>",
                    e, args[0]
                )
            })?;
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
        CommandType::Config => handle_config(&args),
        CommandType::Help => {
            print_help(&args[0]);
            Ok(())
        }
        CommandType::Run => handle_run(&args),
    }
}

fn handle_config(args: &[String]) -> Result<(), String> {
    match args.get(2) {
        Some(path_str) => {
            let user_path = PathBuf::from(path_str);
            save_file_path(user_path)?;
            println!("{}", "Configuration saved successfully!".green());
            Ok(())
        }
        None => {
            let config_path = load_file_path()
                .ok_or_else(|| {
                    format!(
                        "No configuration found\nUsage: {} config <path/to/file.json>",
                        args[0]
                    )
                })?;
            
            open::that(&config_path)
                .map_err(|e| format!("Failed to open file: {}", e))?;
            
            println!("Opening: {}", config_path.display());
            Ok(())
        }
    }
}

fn handle_run(args: &[String]) -> Result<(), String> {
    let commands = read_json()
        .map_err(|e| format!("Failed to read commands file: {}", e))?;
    
    let command_name = &args[1];
    let command_group = commands
        .commands
        .get(command_name)
        .ok_or_else(|| format!("Command '{}' not found", command_name))?;

    let mut sorted_commands: Vec<(&String, &Command)> = command_group.iter().collect();
    sorted_commands.sort_by(|a, b| a.0.cmp(b.0));

    // Keep track of working directory
    let mut current_dir = env::current_dir()
        .map_err(|e| format!("Failed to get current directory: {}", e))?;

    for (key, cmd) in sorted_commands {
        println!("{}: {} {:?}", "Executing".cyan(), cmd.command, cmd.args);

        // Handle 'cd' specially
        if cmd.command == "cd" {
            if let Some(new_dir) = cmd.args.first() {
                current_dir = current_dir.join(new_dir);
                env::set_current_dir(&current_dir)
                    .map_err(|e| format!("Failed to change directory to {}: {}", current_dir.display(), e))?;
                println!("{}: {}", "Changed directory to".green(), current_dir.display());
                continue;
            }
        }

        let mut process_cmd = if cfg!(target_os = "windows") {
            let mut c = process::Command::new("cmd");
            c.args(&["/C", &cmd.command]);
            c
        } else {
            process::Command::new(&cmd.command)
        };

        let status = process_cmd
            .args(&cmd.args)
            .current_dir(&current_dir)  // Set working directory for each command
            .spawn()
            .map_err(|e| format!("Failed to spawn command '{}': {}", key, e))?
            .wait()
            .map_err(|e| format!("Failed to wait for command '{}': {}", key, e))?;

        if !status.success() {
            return Err(format!(
                "Command '{}' failed with exit code: {}",
                key,
                status.code().unwrap_or(-1)
            ));
        }
    }

    Ok(())
}

fn validate_path(path: &PathBuf) -> Result<(), String> {
    if !path.exists() {
        return Err("Path doesn't exist".to_string());
    }
    if !path.is_file() {
        return Err("Path is not a file".to_string());
    }
    if !is_json_file(path) {
        return Err("File must have .json extension".to_string());
    }
    Ok(())
}

fn save_file_path(path: PathBuf) -> Result<(), String> {
    validate_path(&path)?;

    let config = AppConfig { path: Some(path) };
    confy::store("shrt", "path", &config)
        .map_err(|e| format!("Failed to store config: {}", e))
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
    println!("{}", "Shorter - Command Runner".bold());
    println!("\n{}", "Usage:".underline());
    println!("  {} <command>", app_name);
    println!("\n{}", "Commands:".underline());
    println!("  {} <path>         Configure the JSON file path", "config".green());
    println!("  {}                Open the config file when configured", "config".green());
    println!("  {}    Show this help message", "help / --help / -h".green());
}

fn read_json() -> Result<Commands, io::Error> {
    let config_path = load_file_path().ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, "Config file path not configured")
    })?;

    let file = File::open(&config_path).map_err(|e| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!("Cannot open config file at {}: {}", config_path.display(), e),
        )
    })?;

    let reader = BufReader::new(file);
    serde_json::from_reader(reader)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, format!("Invalid JSON: {}", e)))
}
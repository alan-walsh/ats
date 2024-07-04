// Import necessary crates
use std::env; // To work with command line arguments
//use std::fs::{self};
use std::io::{self, Write};
use std::path::PathBuf;
use serde::{Serialize, Deserialize};
use dirs;
use ini::Ini;

#[derive(Serialize, Deserialize)]
struct Config {
	profile: String,
}

fn main() {
	let args: Vec<String> = env::args().collect();

	match args.len() {
		1 => show_help(),
		_ => match args[1].as_str() {
			"help" => show_help(),
			"configure" => {
				if args.len() > 2 && args[2] == "list" {
					list_config();
				} else {
					configure();
				}
			},
            "reset" => reset_config(),
            _ => println!("Unknown command. Use 'ats help' for usage information."),
        },
    }
}

fn show_help() {
    println!("ATS CLI Help");
    println!("Usage:");
    println!("  ats help               Display this help message.");
    println!("  ats configure          Configure the application.");
    println!("  ats configure list     List current configuration.");
    println!("  ats reset              Reset (delete) the configuration.");
}

fn configure() {
    let mut conf: Ini = Ini::new();
    let providers: [&str; 2] = ["AWS", "Google"]; // Extend this list for more providers

    for provider in providers.iter() {
        println!("Configuring {}:", provider);
        let profile: String = prompt(format!("Enter profile name for {} (press enter to skip): ", provider));
        let upload: String = prompt(format!("Enter upload bucket for {} (press enter to skip): ", provider));
        let download: String = prompt(format!("Enter download bucket for {} (press enter to skip): ", provider));

        if !profile.is_empty() {
            conf.with_section(Some(provider.to_owned()))
                .set("profile", &profile);
        }
        if !upload.is_empty() {
            conf.with_section(Some(provider.to_owned()))
                .set("upload", &upload);
        }
        if !download.is_empty() {
            conf.with_section(Some(provider.to_owned()))
                .set("download", &download);
        }
    }

    let config_path: PathBuf = get_config_path().join("config");
    conf.write_to_file(config_path).expect("Failed to write config file");
    println!("Configuration saved.");
}

fn list_config() {
    let mut config_path: PathBuf = get_config_path();
    config_path.push("config");
    if !config_path.exists() {
        println!("Configuration file does not exist. Please run 'ats configure' to set up the application.");
        return;
    }

    let conf: Ini = Ini::load_from_file(config_path).expect("Failed to load config file");
    for (sec, prop) in conf.iter() {
        println!("[{}]", sec.unwrap_or(&"Default".to_string()));
        for (k, v) in prop.iter() {
            println!("{} = {}", k, v);
        }
    }
}

fn prompt(message: String) -> String {
    print!("{}", message);
    io::stdout().flush().unwrap(); // Ensure the message is displayed before reading input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_owned()
}

fn get_config_path() -> PathBuf {
    let mut path = dirs::home_dir().expect("Failed to find home directory");
    path.push(".ats");
    std::fs::create_dir_all(&path).expect("Failed to create config directory");
    path
}

fn reset_config() {
    let config_path: PathBuf = get_config_path().join("config"); // Ensure the file name is correct
    if config_path.exists() {
        let user_input: String = prompt("Delete config file? (y/N)".to_string());
        if ["y", "yes"].contains(&user_input.to_lowercase().as_str()) {
            std::fs::remove_file(&config_path).expect("Failed to delete config file");
            println!("Configuration reset successfully.");
        } else {
            println!("Configuration reset aborted.");
        }
    } else {
        println!("Configuration file does not exist. Please run 'ats configure' to set up the application.");
    }
}

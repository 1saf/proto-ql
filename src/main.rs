use std::env;
use std::fs;

use toml;
use serde::{Deserialize};

const CONFIG_FILE_NAME: &str = "protoql.toml";

#[derive(Deserialize, Debug)]
struct ProtoQLConfig {
    project_settings: ProjectSettings,
}

#[derive(Deserialize, Debug)]
struct ProjectSettings {
    protobuf_location: String,
}


fn read_config_file() -> Result<String, &'static str> {
    println!("Looking for a ProtoQL configuration file. [<working-directory>/protoql.toml]");
    let mut config_path = env::current_dir().unwrap();
    config_path.push(CONFIG_FILE_NAME);
    
    let config_file_contents = fs::read_to_string(config_path);
    match config_file_contents {
        Ok(contents) => {
            println!("Found configuration file. Loading settings.");
            Ok(contents)
        },
        Err(_) => {
            println!("Configuration file not found. Please add a configuration file, configuration options can be found inside the README.");
            Err("Configuration file not found.")
        }
    }
}

fn parse_config_file(config_contents: String) -> Result<ProtoQLConfig, toml::de::Error> {
    let parsed_config: std::result::Result<ProtoQLConfig, toml::de::Error> = toml::from_str(&config_contents);
    match parsed_config {
        Ok(parsed) => {
            Ok(parsed)
        },
        Err(error) => {
            println!("Failed to parse the configuration file. {:?}", error);
            Err(error)
        }
    }
}

fn main() {
    let settings_result = read_config_file();
    match settings_result {
        Ok(contents) => {

            let parsed_config = parse_config_file(contents).unwrap();
            println!("Prased {:?}", parsed_config);
        },
        Err(_) => {
            println!("Besketit");
        }
    }
}
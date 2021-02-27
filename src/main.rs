use colored::*;
use ignore::Walk;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

extern crate clap;
use clap::{App, Arg};

mod ask;
use ask::*;

mod change_file;
use change_file::*;

mod file_exists;
use file_exists::*;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Configuration {
    replacements: Option<HashMap<String, String>>,
    user_replacements: Option<Vec<String>>,
}

fn main() {
    if let Err(error) = crispr() {
        println!("{}", error.red());
        std::process::exit(1);
    }
}

pub enum DefaultConfigurationPath {
    TOML,
    JSON,
}

impl DefaultConfigurationPath {
    pub fn as_str(&self) -> &'static str {
        match *self {
            DefaultConfigurationPath::TOML => ".crispr.toml",
            DefaultConfigurationPath::JSON => ".crispr.json",
        }
    }
}

fn crispr() -> Result<(), &'static str> {
    let matches = App::new("crispr")
        .version("0.3.0")
        .author("Yoav Lavi <yoavlavi122@gmail.com>")
        .about("Scaffolds a project from a template")
        .arg(
            Arg::with_name("dry")
                .short("d")
                .long("dry")
                .help("Dry run - prints output without making changes"),
        )
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .help("The path to the configuration file ('.crispr.{toml,json}' by default)"),
        )
        .arg(
            Arg::with_name("PATH")
                .help("The path to run crispr ('.' by default)")
                .index(1),
        )
        .get_matches();

    let directory_name = matches.value_of("PATH").unwrap_or(".");
    let configuration_file_setting = matches.value_of("config");
    let dry = matches.occurrences_of("dry") != 0;

    let configuration_file = match configuration_file_setting {
        Some(configuration_file_setting) => {
            if !file_exists(configuration_file_setting) {
                return Err("Could find a configuration file");
            }
            configuration_file_setting
        }
        None => {
            let configuration_file = if file_exists(DefaultConfigurationPath::TOML.as_str()) {
                DefaultConfigurationPath::TOML
            } else if file_exists(DefaultConfigurationPath::JSON.as_str()) {
                DefaultConfigurationPath::JSON
            } else {
                return Err("Could find a configuration file");
            };
            configuration_file.as_str()
        }
    };

    if dry {
        println!(
            "{}\n",
            "Running in dry run mode, changes will not be written".bold()
        );
    }

    let configuration = match Path::new(configuration_file)
        .extension()
        .and_then(OsStr::to_str)
    {
        Some("toml") => {
            let raw_configuration =
                match fs::read_to_string(format!("{}/{}", &directory_name, &configuration_file)) {
                    Ok(raw_configuration) => raw_configuration,
                    Err(_) => return Err("Could not read the configuration file"),
                };
            let configuration: Configuration = match toml::from_str(&raw_configuration) {
                Ok(configuration) => configuration,
                Err(_) => return Err("Could not deserialize configuration"),
            };
            configuration
        }
        Some("json") => {
            let raw_configuration =
                match fs::read_to_string(format!("{}/{}", &directory_name, &configuration_file)) {
                    Ok(raw_configuration) => raw_configuration,
                    Err(_) => return Err("Could not read the configuration file"),
                };
            let configuration: Configuration = match serde_json::from_str(&raw_configuration) {
                Ok(configuration) => configuration,
                Err(_) => return Err("Could not deserialize configuration"),
            };
            configuration
        }
        _ => return Err("Unsupported configuration extension"),
    };

    let mut replacement_map: HashMap<String, String> = HashMap::new();

    match configuration.user_replacements {
        Some(user_replacements) => {
            for user_replacement in user_replacements {
                let answer = match ask(&format!("Select a value for {}:", user_replacement.blue()))
                {
                    Ok(answer) => answer,
                    Err(_) => return Err("Ran into an issue while asking for a replacement value"),
                };
                replacement_map.insert(user_replacement, answer.to_string());
                println!();
            }
        }
        None => {}
    }

    match configuration.replacements {
        Some(replacements) => {
            replacement_map.extend(replacements.into_iter());
        }
        None => {}
    }

    if replacement_map.is_empty() {
        return Err("No replacements specified");
    }

    let directory_path = Path::new(&directory_name);

    for entry in Walk::new(directory_path) {
        if let Ok(current_dir_entry) = entry {
            let current_path = current_dir_entry.path();
            if !Path::new(current_path).is_dir()
                && current_path
                    .file_name()
                    .map_or(false, |name| name != configuration_file)
            {
                match change_file(&replacement_map, current_path, dry) {
                    Ok(_) => (),
                    Err(error) => return Err(error),
                }
            }
        }
    }

    Ok(())
}

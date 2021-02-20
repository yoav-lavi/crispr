use colored::*;
use ignore::Walk;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

extern crate clap;
use clap::{App, Arg};

mod ask;
use ask::*;

mod change_file;
use change_file::*;

#[derive(Serialize, Deserialize, Debug)]
struct Configuration {
    replacements: HashMap<String, String>,
    user_replacements: Vec<String>,
}

fn main() {
    if let Err(error) = crispr() {
        println!("{}", error.red());
        std::process::exit(1);
    }
}

fn crispr() -> Result<(), &'static str> {
    let matches = App::new("crispr")
        .version("1.0")
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
                .help("The path to the configuration file ('.crispr.json' by default)"),
        )
        .arg(
            Arg::with_name("PATH")
                .help("The path to run crispr ('.' by default)")
                .index(1),
        )
        .get_matches();

    let directory_name = matches.value_of("PATH").unwrap_or(".");
    let configuration_file = matches.value_of("config").unwrap_or(".crispr.json");
    let dry = matches.occurrences_of("dry") != 0;

    if dry {
        println!(
            "{}\n",
            "Running in dry run mode, changes will not be written".bold()
        );
    }

    let raw_configuration =
        match fs::read_to_string(format!("{}/{}", &directory_name, &configuration_file)) {
            Ok(raw_configuration) => raw_configuration,
            Err(_) => return Err("Could not find or read the configuration file"),
        };

    let configuration: Configuration = match serde_json::from_str(&raw_configuration) {
        Ok(configuration) => configuration,
        Err(_) => return Err("could not deserialize configuration"),
    };

    let mut replacement_map: HashMap<String, String> = HashMap::new();

    for user_replacement in configuration.user_replacements {
        let answer = match ask(&format!("Select a value for {}:", user_replacement.blue())) {
            Ok(answer) => answer,
            Err(_) => return Err("Ran into an issue while asking for a replacement value"),
        };
        replacement_map.insert(user_replacement, answer.to_string());
        println!();
    }

    replacement_map.extend(configuration.replacements.into_iter());

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

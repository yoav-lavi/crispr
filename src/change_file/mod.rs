use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

mod visual_diff;
use visual_diff::get_visual_diff;

use colored::*;

pub fn change_file(
    replacements: &HashMap<String, String>,
    file_path: &Path,
    dry: bool,
) -> Result<(), &'static str> {
    let (changed_file, change_list) =
        match get_changed_contents_and_change_list(&replacements, file_path) {
            Ok((changed_file, change_list)) => (changed_file, change_list),
            Err(_) => return Err("Could not get the changed contents for a line"),
        };
    print_change_list(change_list, file_path);
    if !dry {
        match write_file(changed_file, file_path) {
            Ok(()) => (),
            Err(_) => return Err("Could not write a file"),
        }
    }
    Ok(())
}

fn replace_line(original_line: &str, replacements: &HashMap<String, String>) -> (String, bool) {
    let mut changed = false;
    let mut changed_line = original_line.to_string();
    for key in replacements.keys() {
        if original_line.contains(key) {
            changed_line = changed_line.replace(key, &replacements[key]);
            changed = true
        }
    }
    (changed_line, changed)
}

fn get_changed_contents_and_change_list(
    replacements: &HashMap<String, String>,
    file_path: &Path,
) -> Result<(String, String), io::Error> {
    let original_file = File::open(&file_path)?;
    let reader = BufReader::new(original_file);
    let mut changed_file = String::new();
    let mut change_list = String::new();
    for (index, original_line) in reader.lines().enumerate() {
        if let Ok(original_line) = original_line {
            let (changed_line, changed) = replace_line(&original_line, replacements);
            if changed {
                change_list.push_str(&get_visual_diff(&original_line, &changed_line, index));
            }
            changed_file.push_str(&format!("{}\n", changed_line));
        }
    }
    Ok((changed_file, change_list))
}

fn print_change_list(change_list: String, file_path: &Path) {
    if let Some(file_path) = file_path.to_str() {
        if !change_list.is_empty() {
            println!("{}", &format!("Changes for {}:", &file_path.blue()));
            println!("{}", change_list);
        }
    }
}

fn write_file(contents: String, file_path: &Path) -> Result<(), io::Error> {
    let mut changed_file = File::create(file_path)?;
    changed_file.write_all(contents.as_bytes())?;
    Ok(())
}

use colored::*;

extern crate difference;
use difference::{Changeset, Difference};

pub fn get_visual_diff(original_line: &str, changed_line: &str, index: usize) -> String {
    let mut visual_diff_add = String::new();
    let mut visual_diff_remove = String::new();
    let Changeset { diffs, .. } = Changeset::new(original_line, changed_line, "");
    for diff in diffs {
        match diff {
            Difference::Same(ref original_line) => {
                visual_diff_add.push_str(&original_line);
                visual_diff_remove.push_str(&original_line);
            }
            Difference::Add(ref line_difference) => {
                visual_diff_add.push_str(&line_difference.green().underline().to_string());
            }
            Difference::Rem(ref line_difference) => {
                visual_diff_remove.push_str(&line_difference.red().underline().to_string());
            }
        }
    }
    let mut visual_diff = String::new();
    let line_number = index + 1;
    visual_diff.push_str(&format!(
        "{} {}: {}\n",
        "--".red(),
        line_number,
        visual_diff_remove
    ));
    visual_diff.push_str(&format!(
        "{} {}: {}\n",
        "++".green(),
        line_number,
        visual_diff_add
    ));
    visual_diff
}

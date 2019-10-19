use std::process::exit;

use colored::Colorize;
use promptly::prompt;
use trash_utils::TrashEntry;

use crate::common::{
    filter_out_and_print_errors, filter_trash_entry_by_age, filter_trash_entry_by_dir,
    format_trash_entry, pretty_error,
};
use crate::{CURRENT_DIR, TRASH};

pub fn restore(days: Option<f64>, verbose: bool) {
    let trashed_files: Vec<TrashEntry> = TRASH
        .get_trashed_files()
        .unwrap()
        .into_iter()
        .filter_map(filter_out_and_print_errors)
        .filter(|trash_entry| filter_trash_entry_by_age(trash_entry, days))
        .filter(|trash_entry| filter_trash_entry_by_dir(trash_entry, &*CURRENT_DIR))
        .collect();

    if trashed_files.is_empty() {
        println!("no files to restore");
        return;
    }

    let digit_width = trashed_files.len().to_string().len();
    trashed_files
        .iter()
        .enumerate()
        .for_each(|(i, trash_entry)| {
            println!(
                "{:>digit_width$} {}",
                (i + 1).to_string().purple(),
                format_trash_entry(&trash_entry)
                    .replace(&format!("{}/", (*CURRENT_DIR).display()), ""),
                digit_width = digit_width,
            )
        });

    let input: u32 = prompt("Select file to restore");

    let file_to_restore = &trashed_files
        .get((input - 1) as usize)
        .unwrap_or_else(|| {
            eprintln!("index out of range");
            exit(1)
        })
        .trashed_path;

    if let Err(e) = TRASH.restore_trashed_file(file_to_restore) {
        eprintln!("{}", pretty_error(&e.into()));
    } else if verbose {
        println!("restored '{}'", file_to_restore.display());
    }
}

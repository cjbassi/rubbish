use promptly::prompt;
use xdg_trash::TrashEntry;

use crate::common::{
    filter_out_and_print_errors, filter_trash_entry_by_age, filter_trash_entry_by_dir,
    format_trash_entry, pretty_error,
};
use crate::{CURRENT_DIR_STRING, TRASH};

pub fn run(days: Option<f64>) {
    let trashed_files: Vec<TrashEntry> = TRASH
        .get_trashed_files()
        .unwrap()
        .into_iter()
        .filter_map(filter_out_and_print_errors)
        .filter(|trash_entry| filter_trash_entry_by_age(trash_entry, days))
        .filter(|trash_entry| filter_trash_entry_by_dir(trash_entry, &*CURRENT_DIR_STRING))
        .collect();

    trashed_files
        .iter()
        .enumerate()
        .for_each(|(i, trash_entry)| {
            println!(
                "{} {}",
                i + 1,
                format_trash_entry(&trash_entry).replace(&format!("{}/", *CURRENT_DIR_STRING), "")
            )
        });

    let input: u32 = prompt("Select file to restore");

    if let Err(e) = TRASH.recover_trashed_file(
        &trashed_files
            .get((input - 1) as usize)
            .expect("index out of range")
            .trashed_path,
    ) {
        eprintln!("{}", pretty_error(&e.into()));
    }
}

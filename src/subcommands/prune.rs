use regex::Regex;
use xdg_trash::TrashEntry;

use crate::common::{
    filter_out_and_print_errors, filter_trash_entry_by_age, format_trash_entry, pretty_error,
    prompt_user_for_confirmation,
};
use crate::{HOME_DIR_STRING, TRASH};

pub fn run(pattern: String, no_confirm: bool, days: Option<f64>) {
    let re = Regex::new(&pattern).expect("invalid regex given");

    let trashed_files: Vec<TrashEntry> = TRASH
        .get_trashed_files()
        .unwrap()
        .into_iter()
        .filter_map(filter_out_and_print_errors)
        .filter(|trash_entry| filter_trash_entry_by_age(trash_entry, days))
        .filter(|trash_entry| re.is_match(&trash_entry.trashed_path.to_string_lossy()))
        .collect();

    trashed_files.iter().for_each(|trash_entry| {
        println!(
            "{}",
            format_trash_entry(&trash_entry).replace(&*HOME_DIR_STRING, "~")
        );
    });

    if !no_confirm && !prompt_user_for_confirmation("Permanently erase files?") {
        return;
    }

    trashed_files.iter().for_each(|trash_entry| {
        if let Err(e) = TRASH.erase_file(&trash_entry.trashed_path) {
            eprintln!("{}", pretty_error(&e.into()));
        }
    });
}
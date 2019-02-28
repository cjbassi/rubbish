use crate::common::{
    filter_out_and_print_errors, filter_trash_entry_by_age, filter_trash_entry_by_dir,
    format_trash_entry,
};
use crate::{CURRENT_DIR_STRING, TRASH};

pub fn run(days: Option<f64>) {
    TRASH
        .get_trashed_files()
        .unwrap()
        .into_iter()
        .filter_map(filter_out_and_print_errors)
        .filter(|trash_entry| filter_trash_entry_by_age(trash_entry, days))
        .filter(|trash_entry| filter_trash_entry_by_dir(trash_entry, &*CURRENT_DIR_STRING))
        .for_each(|trash_entry| {
            println!(
                "{}",
                format_trash_entry(&trash_entry).replace(&format!("{}/", *CURRENT_DIR_STRING), "")
            )
        });
}

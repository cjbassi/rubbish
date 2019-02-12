use std::path::Path;

use promptly::prompt;
use xdg_trash::{TrashEntry, TrashError};

use crate::CURRENT_TIME;

pub fn prompt_user_for_confirmation(p: &str) -> bool {
    prompt::<bool, &str>(p)
}

pub fn filter_trash_entry_by_age(trash_entry: &TrashEntry, days: Option<f64>) -> bool {
    match days {
        Some(days) => {
            (*CURRENT_TIME - trash_entry.trash_info.deletion_date).num_days() >= days as i64
        }
        None => true,
    }
}

pub fn filter_trash_entry_by_dir<P>(trash_entry: &TrashEntry, path: P) -> bool
where
    P: AsRef<Path>,
{
    trash_entry.trash_info.original_path.starts_with(path)
}

pub fn format_trash_entry(trash_entry: &TrashEntry) -> String {
    format!(
        "{} {}",
        trash_entry
            .trash_info
            .deletion_date
            .format("%Y-%m-%d %H:%M:%S"),
        trash_entry.trash_info.original_path.display(),
    )
}

pub fn filter_out_and_print_errors(result: Result<TrashEntry, TrashError>) -> Option<TrashEntry> {
    match result {
        Ok(x) => Some(x),
        Err(e) => {
            eprintln!("{}", e);
            None
        }
    }
}

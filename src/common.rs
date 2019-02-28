use std::path::Path;

use colored::Colorize;
use promptly::prompt;
use xdg_trash::{TrashEntry, TrashResult};

use crate::{return_code, CURRENT_TIME};

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
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .blue(),
        trash_entry.trash_info.original_path.display(),
    )
}

pub fn filter_out_and_print_errors(result: TrashResult<TrashEntry>) -> Option<TrashEntry> {
    match result {
        Ok(x) => Some(x),
        Err(e) => {
            eprintln!("{}", pretty_error(&e.into()));
            None
        }
    }
}

// https://github.com/BurntSushi/imdb-rename/blob/master/src/main.rs
/// Return a prettily formatted error, including its entire causal chain.
pub fn pretty_error(err: &failure::Error) -> String {
    *return_code.lock().unwrap() = 1;

    let mut pretty = err.to_string();
    let mut prev = err.as_fail();
    while let Some(next) = prev.cause() {
        pretty.push_str(": ");
        pretty.push_str(&next.to_string());
        prev = next;
    }
    pretty
}

use std::path::Path;

use colored::Colorize;
use lscolors::Style;
use promptly::prompt;
use trash_utils::{Result, TrashEntry};

use crate::{CURRENT_TIME, EXIT_CODE, LSCOLORS};

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
		LSCOLORS
			.style_for_path(&trash_entry.trashed_path)
			.map(Style::to_ansi_term_style)
			.unwrap_or_default()
			.paint(
				trash_entry
					.trash_info
					.original_path
					.to_string_lossy()
					.to_string()
			)
	)
}

pub fn filter_out_and_print_errors(result: Result<TrashEntry>) -> Option<TrashEntry> {
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
	*EXIT_CODE.lock().unwrap() = 1;

	let mut pretty = err.to_string();
	let mut prev = err.as_fail();
	while let Some(next) = prev.cause() {
		pretty.push_str(": ");
		pretty.push_str(&next.to_string());
		prev = next;
	}
	pretty
}

use std::path::Path;

use chrono::prelude::Local;
use colored::Colorize;
use lscolors::{LsColors, Style};
use promptly::prompt;
use trash_utils::{self, TrashEntry};

pub fn prompt_user_for_confirmation(p: &str) -> bool {
	prompt::<bool, &str>(p)
}

pub fn filter_trash_entry_by_age(trash_entry: &TrashEntry, days: Option<f64>) -> bool {
	let current_time = Local::now();

	match days {
		Some(days) => {
			(current_time - trash_entry.trash_info.deletion_date).num_days() >= days as i64
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

pub fn format_trash_entry(lscolors: &LsColors, trash_entry: &TrashEntry) -> String {
	format!(
		"{} {}",
		trash_entry
			.trash_info
			.deletion_date
			.format("%Y-%m-%d %H:%M:%S")
			.to_string()
			.blue(),
		lscolors
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

pub fn filter_out_and_print_errors(result: trash_utils::Result<TrashEntry>) -> Option<TrashEntry> {
	match result {
		Ok(x) => Some(x),
		Err(e) => {
			eprintln!("{}", e);
			None
		}
	}
}

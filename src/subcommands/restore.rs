use std::env;
use std::process::exit;

use anyhow::Result;
use colored::Colorize;
use lscolors::LsColors;
use promptly::prompt;
use trash_utils::{Trash, TrashEntry};

use crate::common::{
	filter_out_and_print_errors, filter_trash_entry_by_age, filter_trash_entry_by_dir,
	format_trash_entry,
};

pub fn restore(days: Option<f64>, verbose: bool) -> Result<()> {
	let trash = Trash::new()?;
	let current_dir = &env::current_dir()?;
	let lscolors = LsColors::from_env().unwrap_or_default();

	let trashed_files: Vec<TrashEntry> = trash
		.get_trashed_files()?
		.into_iter()
		.filter_map(filter_out_and_print_errors)
		.filter(|trash_entry| filter_trash_entry_by_age(trash_entry, days))
		.filter(|trash_entry| filter_trash_entry_by_dir(trash_entry, current_dir))
		.collect();

	if trashed_files.is_empty() {
		println!("no files to restore");
		return Ok(());
	}

	let digit_width = trashed_files.len().to_string().len();
	trashed_files
		.iter()
		.enumerate()
		.for_each(|(i, trash_entry)| {
			println!(
				"{:>digit_width$} {}",
				(i + 1).to_string().purple(),
				format_trash_entry(&lscolors, &trash_entry)
					.replace(&format!("{}/", current_dir.display()), ""),
				digit_width = digit_width,
			)
		});

	let input: u32 = prompt("Select file to restore");

	// TODO fix this error handling
	let file_to_restore = &trashed_files
		.get((input - 1) as usize)
		.unwrap_or_else(|| {
			eprintln!("index out of range");
			exit(1)
		})
		.trashed_path;

	if let Err(e) = trash.restore_trashed_file(file_to_restore) {
		eprintln!("{}", e);
	} else if verbose {
		println!("restored '{}'", file_to_restore.display());
	}

	Ok(())
}

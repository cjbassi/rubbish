use anyhow::Result;
use lscolors::LsColors;
use platform_dirs::home_dir;
use regex::Regex;
use structopt::StructOpt;
use trash_utils::{Trash, TrashEntry};

use crate::common::{
	filter_out_and_print_errors, filter_trash_entry_by_age, format_trash_entry,
	prompt_user_for_confirmation,
};

#[derive(StructOpt, Debug)]
pub struct PruneArgs {
	pattern: String,

	#[structopt(long)]
	no_confirm: bool,

	days: Option<f64>,
}

pub fn prune(args: PruneArgs, verbose: bool) -> Result<()> {
	let re = Regex::new(&args.pattern)?;
	let trash = Trash::new()?;
	let lscolors = LsColors::from_env().unwrap_or_default();

	let trashed_files: Vec<TrashEntry> = trash
		.get_trashed_files()?
		.into_iter()
		.filter_map(filter_out_and_print_errors)
		.filter(|trash_entry| filter_trash_entry_by_age(trash_entry, args.days))
		.filter(|trash_entry| re.is_match(&trash_entry.trashed_path.to_string_lossy()))
		.collect();

	if trashed_files.is_empty() {
		println!("no matching files");
		return Ok(());
	}

	// TODO handle unwrap
	// TODO see if there's a more elegant way of doing this
	let home_dir_string = home_dir().unwrap().to_string_lossy().to_string();

	trashed_files.iter().for_each(|trash_entry| {
		println!(
			"{}",
			format_trash_entry(&lscolors, &trash_entry).replace(&home_dir_string, "~")
		);
	});

	if !args.no_confirm && !prompt_user_for_confirmation("Permanently delete files?") {
		return Ok(());
	}

	trashed_files.iter().for_each(|trash_entry| {
		if let Err(e) = trash.erase_file(&trash_entry.trashed_path) {
			eprintln!("{}", e);
		} else if verbose {
			println!("deleted '{}'", trash_entry.trashed_path.display());
		}
	});

	Ok(())
}

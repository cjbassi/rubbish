use std::env;

use anyhow::Result;
use lscolors::LsColors;
use structopt::StructOpt;
use trash_utils::Trash;

use crate::common::{
	filter_out_and_print_errors, filter_trash_entry_by_age, filter_trash_entry_by_dir,
	format_trash_entry,
};

#[derive(StructOpt, Debug)]
pub struct ListArgs {
	days: Option<f64>,
}

pub fn list(args: ListArgs) -> Result<()> {
	let trash = Trash::new()?;
	let current_dir = &env::current_dir()?;
	let lscolors = LsColors::from_env().unwrap_or_default();

	trash
		.get_trashed_files()?
		.into_iter()
		.filter_map(filter_out_and_print_errors)
		.filter(|trash_entry| filter_trash_entry_by_age(trash_entry, args.days))
		.filter(|trash_entry| filter_trash_entry_by_dir(trash_entry, current_dir))
		.for_each(|trash_entry| {
			println!(
				"{}",
				format_trash_entry(&lscolors, &trash_entry)
					.replace(&format!("{}/", current_dir.display()), "")
			)
		});

	Ok(())
}

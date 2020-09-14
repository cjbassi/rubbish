use anyhow::Result;
use structopt::StructOpt;
use trash_utils::Trash;

use crate::common::{
	filter_out_and_print_errors, filter_trash_entry_by_age, prompt_user_for_confirmation,
};

#[derive(StructOpt, Debug)]
pub struct EmptyArgs {
	/// Only remove files deleted more than this many days ago
	days: Option<f64>,

	#[structopt(long)]
	no_confirm: bool,
}

pub fn empty(args: EmptyArgs, verbose: bool) -> Result<()> {
	if !args.no_confirm && !prompt_user_for_confirmation("Empty trash?") {
		return Ok(());
	}

	let trash = Trash::new()?;

	trash
		.get_trashed_files()?
		.into_iter()
		.filter_map(filter_out_and_print_errors)
		.filter(|trash_entry| filter_trash_entry_by_age(trash_entry, args.days))
		.for_each(|trash_entry| {
			if let Err(e) = trash.erase_file(&trash_entry.trashed_path) {
				eprintln!("{}", e);
			} else if verbose {
				println!("deleted '{}'", trash_entry.trashed_path.display());
			}
		});

	Ok(())
}

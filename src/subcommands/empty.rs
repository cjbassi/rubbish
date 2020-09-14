use anyhow::Result;
use trash_utils::Trash;

use crate::common::{
	filter_out_and_print_errors, filter_trash_entry_by_age, prompt_user_for_confirmation,
};

pub fn empty(days: Option<f64>, no_confirm: bool, verbose: bool) -> Result<()> {
	if !no_confirm && !prompt_user_for_confirmation("Empty trash?") {
		return Ok(());
	}

	let trash = Trash::new()?;

	trash
		.get_trashed_files()?
		.into_iter()
		.filter_map(filter_out_and_print_errors)
		.filter(|trash_entry| filter_trash_entry_by_age(trash_entry, days))
		.for_each(|trash_entry| {
			if let Err(e) = trash.erase_file(&trash_entry.trashed_path) {
				eprintln!("{}", e);
			} else if verbose {
				println!("deleted '{}'", trash_entry.trashed_path.display());
			}
		});

	Ok(())
}

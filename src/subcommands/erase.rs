use std::path::PathBuf;

use anyhow::Result;
use trash_utils::Trash;

use crate::common::prompt_user_for_confirmation;

pub fn erase(files: &[PathBuf], no_confirm: bool, verbose: bool) -> Result<()> {
	let prompt = format!(
		"Permanently erase file{}",
		match files.len() {
			1 => "?",
			_ => "s?",
		}
	);
	if !no_confirm && !prompt_user_for_confirmation(&prompt) {
		return Ok(());
	}

	let trash = Trash::new()?;

	files.iter().for_each(|file| {
		if let Err(e) = trash.erase_file(file) {
			eprintln!("{}", e);
		} else if verbose {
			println!("erased '{}'", file.display());
		}
	});

	Ok(())
}

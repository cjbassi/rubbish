use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;
use trash_utils::Trash;

use crate::common::prompt_user_for_confirmation;

#[derive(StructOpt, Debug)]
pub struct EraseArgs {
	files: Vec<PathBuf>,

	#[structopt(long)]
	no_confirm: bool,
}

pub fn erase(args: EraseArgs, verbose: bool) -> Result<()> {
	let prompt = format!(
		"Permanently erase file{}",
		match args.files.len() {
			1 => "?",
			_ => "s?",
		}
	);
	if !args.no_confirm && !prompt_user_for_confirmation(&prompt) {
		return Ok(());
	}

	let trash = Trash::new()?;

	args.files.iter().for_each(|file| {
		if let Err(e) = trash.erase_file(file) {
			eprintln!("{}", e);
		} else if verbose {
			println!("erased '{}'", file.display());
		}
	});

	Ok(())
}

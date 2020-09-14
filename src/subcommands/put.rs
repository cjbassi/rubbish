use std::path::PathBuf;

use anyhow::Result;
use structopt::StructOpt;
use trash_utils::Trash;

#[derive(StructOpt, Debug)]
pub struct PutArgs {
	files: Vec<PathBuf>,
}

pub fn put(args: PutArgs, verbose: bool) -> Result<()> {
	let trash = Trash::new()?;

	args.files.iter().for_each(|file| {
		if let Err(e) = trash.trash_file(file) {
			eprintln!("{}", e);
		} else if verbose {
			println!("trashed '{}'", file.display());
		}
	});

	Ok(())
}

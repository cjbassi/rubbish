use std::path::PathBuf;

use anyhow::Result;
use trash_utils::Trash;

pub fn put(files: &[PathBuf], verbose: bool) -> Result<()> {
	let trash = Trash::new()?;

	files.iter().for_each(|file| {
		if let Err(e) = trash.trash_file(file) {
			eprintln!("{}", e);
		} else if verbose {
			println!("trashed '{}'", file.display());
		}
	});

	Ok(())
}

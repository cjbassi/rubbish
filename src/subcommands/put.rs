use std::path::PathBuf;

use crate::common::pretty_error;
use crate::TRASH;

pub fn put(files: &[PathBuf], verbose: bool) {
	files.iter().for_each(|file| {
		if let Err(e) = TRASH.trash_file(file) {
			eprintln!("{}", pretty_error(&e.into()));
		} else if verbose {
			println!("trashed '{}'", file.display());
		}
	});
}

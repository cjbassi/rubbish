use std::path::PathBuf;

use crate::common::pretty_error;
use crate::TRASH;

pub fn run(files: &[PathBuf]) {
    files.iter().for_each(|file| {
        if let Err(e) = TRASH.trash_file(file) {
            eprintln!("{}", pretty_error(&e.into()));
        }
    });
    // if args.verbose {
    //     println!(
    //         "{}: {} moved to {}",
    //         NAME,
    //         file_to_str,
    //         new_path.to_str().unwrap()
    //     );
    // }
}

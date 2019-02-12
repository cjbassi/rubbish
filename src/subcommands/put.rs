use std::path::PathBuf;

use crate::TRASH;

pub fn run(files: &[PathBuf]) {
    files.iter().for_each(|file| {
        if let Err(e) = TRASH.trash_file(file) {
            eprintln!("{}", e);
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

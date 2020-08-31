use std::path::PathBuf;

use crate::common::{pretty_error, prompt_user_for_confirmation};
use crate::TRASH;

pub fn erase(files: &[PathBuf], no_confirm: bool, verbose: bool) {
    let prompt = format!(
        "Permanently erase file{}",
        match files.len() {
            1 => "?",
            _ => "s?",
        }
    );
    if !no_confirm && !prompt_user_for_confirmation(&prompt) {
        return;
    }

    files.iter().for_each(|file| {
        if let Err(e) = TRASH.erase_file(file) {
            eprintln!("{}", pretty_error(&e.into()));
        } else if verbose {
            println!("erased '{}'", file.display());
        }
    });
}

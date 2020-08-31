use crate::common::{
    filter_out_and_print_errors, filter_trash_entry_by_age, pretty_error,
    prompt_user_for_confirmation,
};
use crate::TRASH;

pub fn empty(days: Option<f64>, no_confirm: bool, verbose: bool) {
    if !no_confirm && !prompt_user_for_confirmation("Empty trash?") {
        return;
    }

    TRASH
        .get_trashed_files()
        .unwrap()
        .into_iter()
        .filter_map(filter_out_and_print_errors)
        .filter(|trash_entry| filter_trash_entry_by_age(trash_entry, days))
        .for_each(|trash_entry| {
            if let Err(e) = TRASH.erase_file(&trash_entry.trashed_path) {
                eprintln!("{}", pretty_error(&e.into()));
            } else if verbose {
                println!("deleted '{}'", trash_entry.trashed_path.display());
            }
        });
}

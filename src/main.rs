mod args;
mod common;
mod subcommands;

use std::env;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Mutex;

use chrono::prelude::{DateTime, Local};
use lazy_static::lazy_static;
use lscolors::LsColors;
use platform_dirs::home_dir;
use structopt::StructOpt;
use trash_utils::Trash;

use args::{Args, Subcommand};

lazy_static! {
    static ref HOME_DIR_STRING: String = home_dir().unwrap().to_string_lossy().to_string();
    static ref CURRENT_TIME: DateTime<Local> = Local::now();
    static ref CURRENT_DIR: PathBuf = env::current_dir().unwrap();
    static ref TRASH: Trash = Trash::new().unwrap();
    static ref EXIT_CODE: Mutex<i32> = Mutex::new(0);
    static ref LSCOLORS: LsColors = LsColors::from_env().unwrap_or_default();
}

fn main() {
    let args = Args::from_args();

    match args.subcommand {
        Subcommand::Empty { days, no_confirm } => {
            subcommands::empty::empty(days, no_confirm, args.verbose);
        }
        Subcommand::Erase { files, no_confirm } => {
            subcommands::erase::erase(&files, no_confirm, args.verbose);
        }
        Subcommand::List { days } => {
            subcommands::list::list(days);
        }
        Subcommand::Prune {
            pattern,
            no_confirm,
            days,
        } => {
            subcommands::prune::prune(pattern, no_confirm, days, args.verbose);
        }
        Subcommand::Put { files } => {
            subcommands::put::put(&files, args.verbose);
        }
        Subcommand::Restore { days } => {
            subcommands::restore::restore(days, args.verbose);
        }
    }

    exit(*EXIT_CODE.lock().unwrap());
}

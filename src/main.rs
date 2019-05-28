mod args;
mod common;
mod subcommands;

use std::env;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Mutex;

use chrono::prelude::{DateTime, Local};
use dirs::home_dir;
use lazy_static::lazy_static;
use structopt::StructOpt;
use xdg_trash::Trash;

use args::{Args, Subcommand};

lazy_static! {
    static ref HOME_DIR_STRING: String = home_dir().unwrap().to_string_lossy().to_string();
    static ref CURRENT_TIME: DateTime<Local> = Local::now();
    static ref CURRENT_DIR: PathBuf = env::current_dir().unwrap();
    static ref TRASH: Trash = Trash::new().unwrap();
    static ref EXIT_CODE: Mutex<i32> = Mutex::new(0);
}

fn main() {
    let args = Args::from_args();

    match args.subcommand {
        Subcommand::Empty {
            days,
            no_confirm,
            verbose,
        } => {
            subcommands::empty::empty(days, no_confirm, verbose);
        }
        Subcommand::Erase {
            files,
            no_confirm,
            verbose,
        } => {
            subcommands::erase::erase(&files, no_confirm, verbose);
        }
        Subcommand::List { days } => {
            subcommands::list::list(days);
        }
        Subcommand::Prune {
            pattern,
            no_confirm,
            days,
            verbose,
        } => {
            subcommands::prune::prune(pattern, no_confirm, days, verbose);
        }
        Subcommand::Put { files, verbose } => {
            subcommands::put::put(&files, verbose);
        }
        Subcommand::Recover { days, verbose } => {
            subcommands::recover::recover(days, verbose);
        }
    }

    exit(*EXIT_CODE.lock().unwrap());
}

mod args;
mod common;
mod subcommands;

use std::env;

use chrono::prelude::{DateTime, Local};
use dirs::home_dir;
use lazy_static::lazy_static;
use structopt::StructOpt;
use xdg_trash::Trash;

use args::{Args, Subcommand};

lazy_static! {
    static ref HOME_DIR_STRING: String = home_dir().unwrap().to_string_lossy().to_string();
    static ref CURRENT_TIME: DateTime<Local> = Local::now();
    static ref CURRENT_DIR_STRING: String =
        env::current_dir().unwrap().to_string_lossy().to_string();
    static ref TRASH: Trash = Trash::new();
}

fn main() {
    let args = Args::from_args();

    match args.subcommand {
        Subcommand::Empty { days, no_confirm } => {
            subcommands::empty::run(days, no_confirm);
        }
        Subcommand::Erase { files, no_confirm } => {
            subcommands::erase::run(&files, no_confirm);
        }
        Subcommand::List { days } => {
            subcommands::list::run(days);
        }
        Subcommand::Prune {
            pattern,
            no_confirm,
            days,
        } => {
            subcommands::prune::run(pattern, no_confirm, days);
        }
        Subcommand::Put { files } => {
            subcommands::put::run(&files);
        }
        Subcommand::Restore { days } => {
            subcommands::restore::run(days);
        }
    }
}

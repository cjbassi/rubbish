mod args;
mod common;
mod subcommands;

use std::process::exit;

use structopt::StructOpt;

use args::{Args, Subcommand};

fn main() {
	let args = Args::from_args();

	let result = match args.subcommand {
		Subcommand::Empty { days, no_confirm } => {
			subcommands::empty::empty(days, no_confirm, args.verbose)
		}
		Subcommand::Erase { files, no_confirm } => {
			subcommands::erase::erase(&files, no_confirm, args.verbose)
		}
		Subcommand::List { days } => subcommands::list::list(days),
		Subcommand::Prune {
			pattern,
			no_confirm,
			days,
		} => subcommands::prune::prune(pattern, no_confirm, days, args.verbose),
		Subcommand::Put { files } => subcommands::put::put(&files, args.verbose),
		Subcommand::Restore { days } => subcommands::restore::restore(days, args.verbose),
	};

	if let Err(e) = result {
		eprintln!("{}", e);
		exit(1);
	}
}

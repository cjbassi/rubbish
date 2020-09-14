mod args;
mod common;
mod subcommands;

use std::process::exit;

use structopt::StructOpt;

use args::{Args, Subcommand};

fn main() {
	let args = Args::from_args();

	let result = match args.subcommand {
		Subcommand::Empty(empty_args) => subcommands::empty::empty(empty_args, args.verbose),
		Subcommand::Erase(erase_args) => subcommands::erase::erase(erase_args, args.verbose),
		Subcommand::List(list_args) => subcommands::list::list(list_args),
		Subcommand::Prune(prune_args) => subcommands::prune::prune(prune_args, args.verbose),
		Subcommand::Put(put_args) => subcommands::put::put(put_args, args.verbose),
		Subcommand::Restore(restore_args) => {
			subcommands::restore::restore(restore_args, args.verbose)
		}
	};

	if let Err(e) = result {
		eprintln!("{}", e);
		exit(1);
	}
}

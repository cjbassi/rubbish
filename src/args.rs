use structopt::StructOpt;

use crate::subcommands::{
	empty::EmptyArgs, erase::EraseArgs, list::ListArgs, prune::PruneArgs, put::PutArgs,
	restore::RestoreArgs,
};

#[derive(StructOpt, Debug)]
pub struct Args {
	#[structopt(subcommand)]
	pub subcommand: Subcommand,

	#[structopt(short, long)]
	pub verbose: bool,
}

#[derive(StructOpt, Debug)]
#[structopt(rename_all = "lower")]
pub enum Subcommand {
	/// Empty the trash
	Empty(EmptyArgs),

	/// Erase given files (i.e. `rm`)
	Erase(EraseArgs),

	/// Recursively list files trashed from the current directory
	List(ListArgs),

	/// Delete files from the trash that match a given regex
	Prune(PruneArgs),

	/// Trash given files
	Put(PutArgs),

	/// Restore a previously trashed file to its original location
	Restore(RestoreArgs),
}

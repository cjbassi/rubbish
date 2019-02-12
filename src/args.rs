use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
    #[structopt(subcommand)]
    pub subcommand: Subcommand,

    #[structopt(short = "v", long = "verbose")]
    pub verbose: bool,
}

#[derive(StructOpt, Debug)]
pub enum Subcommand {
    /// Empty the trash
    #[structopt(name = "empty")]
    Empty {
        /// Only remove files deleted more than this many days ago
        #[structopt(name = "days")]
        days: Option<f64>,

        #[structopt(long = "no-confirm")]
        no_confirm: bool,
    },

    /// Erase given files (i.e. `rm`)
    #[structopt(name = "erase")]
    Erase {
        files: Vec<PathBuf>,

        #[structopt(long = "no-confirm")]
        no_confirm: bool,
    },

    /// Recursively list files trashed from the current directory
    #[structopt(name = "list")]
    List {
        #[structopt(name = "days")]
        days: Option<f64>,
    },

    /// Erase files from the trash that match a given regex
    #[structopt(name = "prune")]
    Prune {
        pattern: String,

        #[structopt(long = "no-confirm")]
        no_confirm: bool,

        #[structopt(name = "days")]
        days: Option<f64>,
    },

    /// Trash given files
    #[structopt(name = "put")]
    Put { files: Vec<PathBuf> },

    /// Restore a previously trashed file to its original location
    #[structopt(name = "restore")]
    Restore {
        #[structopt(name = "days")]
        days: Option<f64>,
    },
}

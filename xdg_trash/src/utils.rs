use std::fs;
use std::path::{Path, PathBuf};

use failure::ResultExt;

use crate::error::{TrashError, TrashErrorKind};

pub fn rename_file_handle_conflicts<P>(from: P, to: P) -> Result<PathBuf, TrashError>
where
    P: AsRef<Path>,
{
    let from = from.as_ref();
    let mut to = to.as_ref().to_owned();

    let to_dir = to.parent().unwrap().to_owned();
    let to_filename = to.file_name().unwrap().to_string_lossy().to_string();

    let mut count = 1;
    while to.exists() {
        to = to_dir.join(format!("{}_{}", to_filename, count));
        count += 1;
    }

    fs::rename(&from, &to).context(TrashErrorKind::FileMoveError)?;

    Ok(to)
}

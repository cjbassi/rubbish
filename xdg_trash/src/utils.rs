use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use path_clean::PathClean;

// renames a file, creating the destination directories if necessary, adds
// a number to the end of the path if there are any path conflicts, and checks
// if the destination is a directory and moves the file into the dir instead
pub fn move_file_handle_conflicts<P>(from: P, to: P) -> io::Result<PathBuf>
where
    P: AsRef<Path>,
{
    let from = from.as_ref().canonicalize()?;
    let mut to = to.as_ref().to_path_buf().clean();

    let (to_dir, to_filename) = if to.is_dir() {
        (to, file_name(&from))
    } else {
        (
            parent(&to)
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "cannot rename file to '/'")
                })?
                .to_owned(),
            file_name(&to),
        )
    };

    fs::create_dir_all(&to_dir)?;

    to = to_dir.join(&to_filename);

    let mut count = 1;
    while to.exists() {
        to = to.with_file_name(format!("{}_{}", to_filename.display(), count));
        count += 1;
    }

    fs::rename(&from, &to)?;

    Ok(to)
}

// path.parent() does some weird things like telling us the parent of "." is "", so we have to fix that
pub fn parent<P>(path: P) -> Option<PathBuf>
where
    P: AsRef<Path>,
{
    let mut path = path.as_ref().to_path_buf().clean();
    path = match path.to_string_lossy().to_string().as_ref() {
        "." | ".." => path.canonicalize().unwrap(),
        _ => path,
    };
    let parent = match path.to_string_lossy().to_string().as_ref() {
        "/" => return None,
        _ => path.parent().unwrap(),
    };
    Some(parent.to_path_buf())
}

// path.file_name() also does some weird things like telling us the filename of ".." is None, so we fix that too
// it also gives us an OsStr for some reason
pub fn file_name<P>(path: P) -> PathBuf
where
    P: AsRef<Path>,
{
    let mut path = path.as_ref().to_path_buf().clean();
    path = match path.to_string_lossy().to_string().as_ref() {
        "." | ".." => path.canonicalize().unwrap(),
        _ => path,
    };
    PathBuf::from(path.file_name().unwrap().to_string_lossy().to_string())
}

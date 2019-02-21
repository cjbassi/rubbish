use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use failure::Error;
use path_clean::PathClean;
use systemstat::{Platform, System};

// renames a file, creating the destination directories if necessary, adds
// a number to the end of the path if there are any path conflicts, and checks
// if the destination is a directory and moves the file into the dir instead
pub fn move_file_handle_conflicts<P>(from: P, to: P) -> io::Result<PathBuf>
where
    P: AsRef<Path>,
{
    let from = from.as_ref().absolute_path();
    let mut to = to.as_ref().absolute_path();

    if !from.exists() {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("cannot move {}: file does not exist", from.display()),
        ))?
    }

    let (to_dir, to_filename) = if to.is_dir() {
        (to, from.file_name().unwrap().to_string_lossy().to_string())
    } else {
        (
            to.parent()
                .ok_or_else(|| {
                    io::Error::new(io::ErrorKind::InvalidInput, "cannot rename file to '/'")
                })?
                .to_owned(),
            to.file_name().unwrap().to_string_lossy().to_string(),
        )
    };

    fs::create_dir_all(&to_dir)?;

    to = to_dir.join(&to_filename);

    let mut count = 1;
    while to.exists() {
        to = to.with_file_name(format!("{}_{}", to_filename, count));
        count += 1;
    }

    fs::rename(&from, &to)?;

    Ok(to)
}

pub fn get_physical_mountpoints() -> Result<Vec<PathBuf>, Error> {
    Ok(System::new()
        .mounts()?
        .into_iter()
        .filter(|filesystem| Path::new(&filesystem.fs_mounted_from).is_absolute())
        .map(|filesystem| PathBuf::from(filesystem.fs_mounted_on))
        .collect())
}

pub trait AbsolutePath {
    fn absolute_path(&self) -> PathBuf;
}

impl AbsolutePath for PathBuf {
    fn absolute_path(&self) -> PathBuf {
        let file = self.clean();
        if file.is_absolute() {
            file.to_path_buf()
        } else {
            env::current_dir().unwrap().join(file).clean()
        }
    }
}

impl AbsolutePath for Path {
    fn absolute_path(&self) -> PathBuf {
        let file = self.to_path_buf().clean();
        if file.is_absolute() {
            file.to_path_buf()
        } else {
            env::current_dir().unwrap().join(file).clean()
        }
    }
}

pub fn get_physical_mountpoint_of_file<P>(file: P, mountpoints: &[P]) -> PathBuf
where
    P: AsRef<Path>,
{
    let file = file.as_ref().absolute_path();
    let mut mountpoints = mountpoints.into_iter();
    for ancestor in file.ancestors() {
        match mountpoints.find(|mountpoint| mountpoint.as_ref() == ancestor) {
            Some(x) => return x.as_ref().to_path_buf(),
            None => {}
        }
    }
    panic!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rename_file_handle_conflicts() {
        use std::fs::File;

        let folder = PathBuf::from("test1");
        fs::create_dir_all(&folder);
        let file = folder.join("file");
        let file1 = folder.join("foo");
        File::create(&file);
        File::create(&file1);
        assert!(&file.exists());
        assert_eq!(
            rename_file_handle_conflicts(&file, &file1).unwrap(),
            folder.join("foo_1")
        );
        assert!(!&file.exists());
        File::create(&file);
        assert!(rename_file_handle_conflicts(&file, &file.join("asdf")).is_err());
        assert_eq!(
            rename_file_handle_conflicts(&file, &PathBuf::from(format!("{}asdf", file.display(),)))
                .unwrap(),
            folder.join("fileasdf")
        );

        fs::remove_dir_all(folder).unwrap();
    }
}

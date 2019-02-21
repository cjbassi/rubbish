mod error;
mod utils;

pub use error::{TrashError, TrashErrorKind, TrashResult};

use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use chrono::prelude::{DateTime, Local, TimeZone};
use failure::{Error, ResultExt};
use nom::{do_parse, map_res, named, tag, take, take_until_and_consume};
use systemstat::{Platform, System};
use xdg::BaseDirectories;

use utils::{file_name, move_file_handle_conflicts};

type Result<T> = std::result::Result<T, Error>;

pub struct Trash {
    home_trash: PathBuf,
}

impl Trash {
    pub fn new() -> TrashResult<Trash> {
        let home_trash = BaseDirectories::new()
            .context(TrashErrorKind::BaseDirectories)?
            .get_data_home()
            .join("Trash");
        fs::create_dir_all(home_trash.join("files"))?;
        fs::create_dir_all(home_trash.join("info"))?;
        Ok(Trash { home_trash })
    }

    pub fn get_trashed_files(&self) -> TrashResult<Vec<TrashResult<TrashEntry>>> {
        Ok(self
            .home_trash
            .join("info")
            .read_dir()?
            .map(|dir_entry| {
                let trash_info_path = dir_entry?.path();
                let trashed_path =
                    self.home_trash
                        .join("files")
                        .join(trash_info_path.file_stem().ok_or_else(|| {
                            TrashErrorKind::Path(".trashinfo file without file stem".to_owned())
                        })?);
                let trash_info = fs::read_to_string(&trash_info_path)?
                    .parse::<TrashInfo>()
                    .context(TrashErrorKind::ParseTrashInfoError(
                        trash_info_path.to_string_lossy().to_string(),
                    ))?;
                Ok(TrashEntry {
                    trashed_path,
                    trash_info,
                })
            })
            .collect())
    }

    pub fn trash_file<P>(&self, file: P) -> TrashResult<PathBuf>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref();

        if !file.exists() {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("cannot trash {}: file does not exist", file.display()),
            ))?
        }
        // check if given file contains the trash-can
        if self.home_trash.starts_with(&file) {
            Err(TrashErrorKind::TrashingTrashCan(format!(
                "{}",
                file.display()
            )))?
        }

        let mountpoint = System::new().mount_at(&file);
        let mountpoint_home = System::new().mount_at(&self.home_trash);
        dbg!(mountpoint);
        dbg!(mountpoint_home);

        let trashed_path = move_file_handle_conflicts(
            &file,
            &self
                .home_trash
                .join("files")
                .join(&file_name(file))
                .as_ref(),
        )?;

        let trash_info_path = self
            .home_trash
            .join("info")
            .join(format!("{}.trashinfo", file_name(&trashed_path).display()));
        let trash_info = TrashInfo {
            original_path: file.to_path_buf(),
            deletion_date: Local::now(),
        };
        fs::write(trash_info_path, format!("{}\n", trash_info))?;

        Ok(trashed_path)
    }

    pub fn restore_trashed_file<P>(&self, file: P) -> TrashResult<PathBuf>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref();
        let trash_info_path = self
            .home_trash
            .join("info")
            .join(format!("{}.trashinfo", file_name(file).display()));
        let original_path = fs::read_to_string(&trash_info_path)?
            .parse::<TrashInfo>()
            .context(TrashErrorKind::ParseTrashInfoError(
                trash_info_path.to_string_lossy().to_string(),
            ))?
            .original_path;
        let recovered_path = move_file_handle_conflicts(&file, &original_path.as_path())?;
        fs::remove_file(trash_info_path)?;
        Ok(recovered_path)
    }

    pub fn erase_file<P>(&self, file: P) -> TrashResult<()>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref();

        if !file.exists() {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("cannot trash {}: file does not exist", file.display()),
            ))?
        }

        if self.is_file_trashed(&file) {
            fs::remove_file(
                self.home_trash
                    .join("info")
                    .join(format!("{}.trashinfo", file_name(&file).display())),
            )?;
        }
        if file.is_dir() {
            fs::remove_dir_all(file)?;
        } else {
            fs::remove_file(file)?;
        }
        Ok(())
    }

    pub fn is_file_trashed<P>(&self, file: P) -> bool
    where
        P: AsRef<Path>,
    {
        file.as_ref().starts_with(&self.home_trash.join("files"))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TrashEntry {
    pub trashed_path: PathBuf,
    pub trash_info: TrashInfo,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TrashInfo {
    pub original_path: PathBuf,
    pub deletion_date: DateTime<Local>,
}

named!(
    parse_trash_info<&str, TrashInfo>,
    do_parse!(
        tag!("[Trash Info]\n")
            >> tag!("Path=")
            >> original_path: take_until_and_consume!("\n")
            >> tag!("DeletionDate=")
            >> deletion_date: map_res!(take!(19), |input| Local.datetime_from_str(input, "%Y-%m-%dT%H:%M:%S"))
            >> (TrashInfo {
                original_path: PathBuf::from(original_path),
                deletion_date,
            })
    )
);

impl FromStr for TrashInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        parse_trash_info(s).map(|x| x.1).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "failed to parse TrashInfo").into()
        })
    }
}

impl fmt::Display for TrashInfo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[Trash Info]\nPath={}\nDeletionDate={}",
            self.original_path.display(),
            self.deletion_date.format("%Y-%m-%dT%H:%M:%S"),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_trashed_files() {
        use std::fs::File;

        let trash = Trash {
            home_trash: PathBuf::from("test2"),
        };
        fs::create_dir_all(&trash.home_trash.join("files"));
        fs::create_dir_all(&trash.home_trash.join("info"));
        let trash_info = TrashInfo {
            original_path: PathBuf::from("/asdf/123"),
            deletion_date: Local.ymd(2014, 7, 8).and_hms(9, 10, 11),
        };

        fs::remove_dir_all(trash.home_trash).unwrap();
    }

    #[test]
    fn test_trash_file() {}

    #[test]
    fn test_restore_trashed_file() {}

    #[test]
    fn test_erase_file() {
        use std::fs::File;

        let trash = Trash {
            home_trash: PathBuf::from("test"),
        };
        let files_dir = trash.home_trash.join("files");
        let info_dir = trash.home_trash.join("info");
        let in_trash = files_dir.join("in_trash");
        let in_trash_trash_info = info_dir.join("in_trash.trashinfo");

        fs::create_dir_all(&files_dir);
        fs::create_dir_all(&info_dir);

        assert!(&files_dir.exists());
        assert!(&info_dir.exists());

        File::create(&in_trash);
        File::create(&in_trash_trash_info);

        assert!((&in_trash).exists());
        assert!((&in_trash_trash_info).exists());

        trash.erase_file(&in_trash);

        assert!(!(&in_trash).exists());
        assert!(!(&in_trash_trash_info).exists());

        let out_trash = trash.home_trash.join("asdf");
        File::create(&out_trash);
        assert!(&out_trash.exists());
        trash.erase_file(&out_trash);
        assert!(!&out_trash.exists());

        fs::remove_dir_all(trash.home_trash).unwrap();
    }

    #[test]
    fn test_is_file_trashed() {
        let trash = Trash {
            home_trash: PathBuf::from("/test/trash"),
        };
        let file1 = PathBuf::from("/test/trash/files/foo");
        let file2 = PathBuf::from("/test/trash/info/foo");
        assert!(trash.is_file_trashed(file1));
        assert!(!trash.is_file_trashed(file2));
    }

    #[test]
    fn test_trash_info_parsing() {
        let trash_info = TrashInfo {
            original_path: PathBuf::from("/asdf/123"),
            deletion_date: Local.ymd(2014, 7, 8).and_hms(9, 10, 11),
        };
        let trash_info_to_str = "[Trash Info]\nPath=/asdf/123\nDeletionDate=2014-07-08T09:10:11";
        assert_eq!(trash_info, trash_info_to_str.parse::<TrashInfo>().unwrap());
    }

    #[test]
    fn test_trash_info_display() {
        let trash_info = TrashInfo {
            original_path: PathBuf::from("/asdf/123"),
            deletion_date: Local.ymd(2014, 7, 8).and_hms(9, 10, 11),
        };
        let trash_info_to_str = "[Trash Info]\nPath=/asdf/123\nDeletionDate=2014-07-08T09:10:11";
        assert_eq!(trash_info.to_string(), trash_info_to_str);
    }
}

use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use chrono::prelude::{DateTime, Local, TimeZone};
use failure::ResultExt;
use xdg::BaseDirectories;

use crate::error::{TrashError, TrashErrorKind};

fn rename_file_handle_conflicts<P>(from: P, to: P) -> Result<PathBuf, TrashError>
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

pub struct Trash {
    home_trash: PathBuf,
    // home_trash_partition:
}

impl Trash {
    pub fn new() -> Trash {
        let home_trash = BaseDirectories::new()
            .unwrap()
            .get_data_home()
            .join("Trash");
        fs::create_dir_all(home_trash.join("files")).unwrap();
        fs::create_dir_all(home_trash.join("info")).unwrap();
        Trash { home_trash }
    }

    pub fn get_trashed_files(&self) -> Vec<Result<TrashEntry, TrashError>> {
        self.home_trash
            .join("info")
            .read_dir()
            .unwrap()
            .map(|dir_entry| {
                let trash_info_path = dir_entry.unwrap().path();
                let trashed_path = self
                    .home_trash
                    .join("files")
                    .join(trash_info_path.file_stem().unwrap());
                let trash_info = fs::read_to_string(&trash_info_path)
                    .context(TrashErrorKind::TrashInfoFileReadError)?
                    .parse::<TrashInfo>()
                    .context(TrashErrorKind::TrashInfoFileParseError)?;
                Ok(TrashEntry {
                    trashed_path,
                    trash_info,
                })
            })
            .collect()
    }

    pub fn trash_file<P>(&self, file: P) -> Result<PathBuf, TrashError>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref().to_owned().canonicalize().unwrap();

        if !file.exists() {
            Err(TrashErrorKind::FileDoesNotExist)?
        }
        // check if given file contains the trash-can
        if self.home_trash.starts_with(&file) {
            Err(TrashErrorKind::TrashingTrashCan)?
        }

        let trashed_path = rename_file_handle_conflicts(
            &file,
            &self
                .home_trash
                .join("files")
                .join(&(&file).file_name().unwrap().to_string_lossy().into_owned()),
        )
        .unwrap();

        let trash_info_path = self.home_trash.join("info").join(format!(
            "{}.trashinfo",
            trashed_path.file_name().unwrap().to_string_lossy(),
        ));
        let trash_info = TrashInfo {
            original_path: file,
            deletion_date: Local::now(),
        };
        fs::write(trash_info_path, format!("{}\n", trash_info))
            .context(TrashErrorKind::TrashInfoFileWriteError)?;

        Ok(trashed_path)
    }

    pub fn restore_trashed_file<P>(&self, file: P) -> Result<PathBuf, TrashError>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref();
        let filename = file.file_name().unwrap();
        let trash_info_path = self
            .home_trash
            .join("info")
            .join(format!("{}.trashinfo", filename.to_string_lossy()));
        let original_path = fs::read_to_string(&trash_info_path)
            .context(TrashErrorKind::TrashInfoFileReadError)?
            .parse::<TrashInfo>()
            .context(TrashErrorKind::TrashInfoFileParseError)?
            .original_path;
        let recovered_path = rename_file_handle_conflicts(&file, &original_path.as_path()).unwrap();
        fs::remove_file(trash_info_path).unwrap();
        Ok(recovered_path)
    }

    pub fn erase_file<P>(&self, file: P) -> Result<(), TrashError>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref();
        if self.is_file_trashed(&file) {
            fs::remove_file(self.home_trash.join("info").join(format!(
                "{}.trashinfo",
                file.file_name().unwrap().to_string_lossy()
            )))
            .unwrap();
        }
        if file.is_dir() {
            fs::remove_dir_all(file).unwrap();
        } else {
            fs::remove_file(file).unwrap();
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

impl FromStr for TrashInfo {
    type Err = TrashError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = TrashErrorKind::TrashInfoStringParseError;

        let lines = s.lines().skip(1).collect::<Vec<&str>>();
        let original_path = PathBuf::from(&lines.get(0).ok_or(err)?.get(5..).ok_or(err)?);
        let deletion_date = Local
            .datetime_from_str(
                &lines.get(1).ok_or(err)?.get(13..).ok_or(err)?,
                "%Y-%m-%dT%H:%M:%S",
            )
            .context(err)?;

        Ok(TrashInfo {
            original_path,
            deletion_date,
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

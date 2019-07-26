mod error;
mod utils;

pub use error::{TrashError, TrashErrorKind, TrashResult};

use std::cmp::Ordering;
use std::env;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use chrono::prelude::{DateTime, Local, TimeZone};
use failure::{Error, ResultExt};
use itertools::Itertools;
use nom::bytes::complete::{tag, take_until};
use nom::combinator::map_res;
use nom::error::VerboseError;
use nom::IResult;
use platform_dirs::{AppDirs, AppUI};

use utils::{move_file_handle_conflicts, AbsolutePath};

type Result<T> = std::result::Result<T, Error>;

pub struct Trash {
    home_trash: PathBuf,
}

impl Trash {
    pub fn new() -> TrashResult<Trash> {
        let home_trash = AppDirs::new::<PathBuf>(None, AppUI::CommandLine)
            .unwrap()
            .data_dir
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
            .sorted_by(|result1, result2| match result1 {
                Ok(x) => match result2 {
                    Ok(y) => Ord::cmp(&x.trash_info.deletion_date, &y.trash_info.deletion_date),
                    Err(_) => Ordering::Less,
                },
                Err(_) => Ordering::Less,
            })
            .collect())
    }

    pub fn trash_file<P>(&self, file: P) -> TrashResult<PathBuf>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref().absolute_path()?;

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
        // check if file contains the cwd
        if env::current_dir()?.starts_with(&file) {
            Err(TrashErrorKind::TrashingCwd(format!("{}", file.display())))?
        }

        let trashed_path = move_file_handle_conflicts(
            &file,
            &self
                .home_trash
                .join("files")
                .join(file.file_name().unwrap()),
        )?;

        let trash_info_path = get_trash_info_path(&trashed_path, &self.home_trash)?;
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
        let file = file.as_ref().absolute_path()?;
        let trash_info_path = get_trash_info_path(&file, &self.home_trash)?;
        let original_path = fs::read_to_string(&trash_info_path)?
            .parse::<TrashInfo>()
            .context(TrashErrorKind::ParseTrashInfoError(
                trash_info_path.to_string_lossy().to_string(),
            ))?
            .original_path;
        let restored_path = move_file_handle_conflicts(&file, &original_path)?;
        fs::remove_file(trash_info_path)?;
        Ok(restored_path)
    }

    pub fn delete_file<P>(&self, file: P) -> TrashResult<()>
    where
        P: AsRef<Path>,
    {
        let file = file.as_ref().absolute_path()?;

        if !file.exists() {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("cannot delete {}: file does not exist", file.display()),
            ))?
        }
        // check if file contains the cwd
        if env::current_dir()?.starts_with(&file) {
            Err(TrashErrorKind::TrashingCwd(format!("{}", file.display())))?
        }

        if self.is_file_trashed(&file)? {
            fs::remove_file(get_trash_info_path(&file, &self.home_trash)?)?;
        }
        if file.is_dir() {
            fs::remove_dir_all(file)?;
        } else {
            fs::remove_file(file)?;
        }

        Ok(())
    }

    pub fn is_file_trashed<P>(&self, file: P) -> io::Result<bool>
    where
        P: AsRef<Path>,
    {
        Ok(file
            .as_ref()
            .absolute_path()?
            .starts_with(&self.home_trash.join("files")))
    }
}

fn get_trash_info_path<P>(file: P, dir: P) -> io::Result<PathBuf>
where
    P: AsRef<Path>,
{
    let (file, dir) = (
        file.as_ref().absolute_path()?,
        dir.as_ref().absolute_path()?,
    );
    Ok(dir.join("info").join(format!(
        "{}.trashinfo",
        file.file_name().unwrap().to_string_lossy()
    )))
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

fn parse_trash_info<'a>(input: &'a str) -> IResult<&'a str, TrashInfo, VerboseError<&'a str>> {
    let (input, _) = tag("[Trash Info]\n")(input)?;

    let (input, _) = tag("Path=")(input)?;
    let (input, original_path) = take_until("\n")(input)?;
    let (input, _) = tag("\n")(input)?;

    let (input, _) = tag("DeletionDate=")(input)?;
    let (input, deletion_date) = map_res(take_until("\n"), |input| {
        Local.datetime_from_str(input, "%Y-%m-%dT%H:%M:%S")
    })(input)?;

    Ok((
        input,
        TrashInfo {
            original_path: PathBuf::from(original_path),
            deletion_date,
        },
    ))
}

impl FromStr for TrashInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        parse_trash_info(s).map(|x| x.1).map_err(|_| {
            // TODO figure out how to convert nom::error to failure::error while preserving its error message
            // was having issues since failure::error requires a static lifetime and nom::error contains a &str
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
    fn test_delete_file() {
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

        trash.delete_file(&in_trash);

        assert!(!(&in_trash).exists());
        assert!(!(&in_trash_trash_info).exists());

        let out_trash = trash.home_trash.join("asdf");
        File::create(&out_trash);
        assert!(&out_trash.exists());
        trash.delete_file(&out_trash);
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

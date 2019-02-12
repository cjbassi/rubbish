use std::fmt::{self, Display};

use failure::{Backtrace, Context, Fail};

#[derive(Debug)]
pub struct TrashError {
    inner: Context<TrashErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum TrashErrorKind {
    #[fail(display = "failed to move file")]
    FileMoveError,

    #[fail(display = "invalid attempt to modify root folder")]
    ModifyingRoot,

    #[fail(display = "invalid attempt to trash trash-can")]
    TrashingTrashCan,

    #[fail(display = "file does not exist")]
    FileDoesNotExist,

    #[fail(display = "error parsing TrashInfo from String")]
    TrashInfoStringParseError,

    #[fail(display = "error parsing contents of .trashinfo file")]
    TrashInfoFileParseError,

    #[fail(display = "error reading from .trashinfo file")]
    TrashInfoFileReadError,

    #[fail(display = "error writing to .trashinfo file")]
    TrashInfoFileWriteError,
}

impl Fail for TrashError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for TrashError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl TrashError {
    pub fn kind(&self) -> TrashErrorKind {
        *self.inner.get_context()
    }
}

impl From<TrashErrorKind> for TrashError {
    fn from(kind: TrashErrorKind) -> TrashError {
        TrashError {
            inner: Context::new(kind),
        }
    }
}

impl From<Context<TrashErrorKind>> for TrashError {
    fn from(inner: Context<TrashErrorKind>) -> TrashError {
        TrashError { inner: inner }
    }
}

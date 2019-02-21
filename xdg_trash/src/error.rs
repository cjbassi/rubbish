use std::fmt::{self, Display};
use std::io;

use failure::{Backtrace, Context, Fail};

pub type TrashResult<T> = std::result::Result<T, TrashError>;

#[derive(Debug)]
pub struct TrashError {
    inner: Context<TrashErrorKind>,
}

#[derive(Clone, Eq, PartialEq, Debug, Fail)]
pub enum TrashErrorKind {
    #[fail(display = "Path error: {}", _0)]
    Path(String),
    #[fail(display = "I/O error")]
    Io,

    #[fail(display = "BaseDirectories error")]
    BaseDirectories,

    #[fail(display = "failed to run subprocess: {}", _0)]
    SubprocessError(String),
    #[fail(display = "cannot trash trash-can: {}", _0)]
    TrashingTrashCan(String),
    #[fail(display = "failed to parse TrashInfo file: {}", _0)]
    ParseTrashInfoError(String),
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
    pub fn kind(&self) -> &TrashErrorKind {
        self.inner.get_context()
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

impl From<io::Error> for TrashError {
    fn from(err: io::Error) -> TrashError {
        TrashError {
            inner: err.context(TrashErrorKind::Io),
        }
    }
}

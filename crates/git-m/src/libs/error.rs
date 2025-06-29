use std::io::Error as StdIoError;

use libgit::Error as GitError;
use tokio::task::JoinError as TokioTaskJoinError;
use toml::{de::Error as TomlDeError, ser::Error as TomlSeError};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    StdIoError(#[from] StdIoError),

    #[error(transparent)]
    TokioTaskJoinError(#[from] TokioTaskJoinError),

    #[error(transparent)]
    GitError(#[from] GitError),

    #[error(transparent)]
    TomlSeError(#[from] TomlSeError),

    #[error(transparent)]
    TomlDeError(#[from] TomlDeError),

    #[error("{0}")]
    Generic(String),
}

macro_rules! err {
    ($($arg:tt)*) => {
        Err($crate::error::Error::Generic(format!($($arg)*)))
    }
}

pub(crate) use err;

pub type Result<T, E = Error> = std::result::Result<T, E>;

use std::io::Error as StdIoError;

use tokio::task::JoinError as TokioTaskJoinError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    StdIoError(#[from] StdIoError),

    #[error(transparent)]
    TokioTaskJoinError(#[from] TokioTaskJoinError),

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

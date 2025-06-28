pub mod gnu;

use std::path::PathBuf;

use crate::error::Result;

pub trait Git {
    fn is_git_repo<P>(path: P) -> Result<bool>
    where
        P: Into<PathBuf>;
}

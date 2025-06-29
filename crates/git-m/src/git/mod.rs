mod gnu;

use std::path::Path;

use crate::error::Result;
pub trait IGit
where
    Self: Sized,
{
    fn open(path: &Path) -> Result<Option<Self>>;

    fn workdir(&self) -> &Path;
}

pub use gnu::Git;

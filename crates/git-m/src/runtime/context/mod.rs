pub mod opts;

use crate::runtime::context::opts::Opts;

#[derive(Debug)]
pub struct Context {
    pub opts: Opts,
}

impl Context {
    pub fn new(opts: Opts) -> Self {
        Self { opts }
    }
}

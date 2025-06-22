use clap::Args;

use crate::constant::DOT_DIR;

#[derive(Debug, Args)]
#[command(about = "list")]
pub struct List {
    #[arg(long, default_value = DOT_DIR)]
    pub path: String,
}

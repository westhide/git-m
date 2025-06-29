pub mod codehub;
pub mod gdir;

use serde::{Deserialize, Serialize};

use crate::{config::codehub::CodeHub, error::Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub hubs: Vec<CodeHub>,
}

impl Config {
    pub fn push(&mut self, hub: CodeHub) {
        self.hubs.push(hub);
    }

    pub fn encode(&self) -> Result<String> {
        use toml::to_string;
        Ok(to_string(self)?)
    }
}

impl From<CodeHub> for Config {
    fn from(hub: CodeHub) -> Self {
        Self { hubs: vec![hub] }
    }
}

impl FromIterator<CodeHub> for Config {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = CodeHub>,
    {
        Self { hubs: iter.into_iter().collect() }
    }
}

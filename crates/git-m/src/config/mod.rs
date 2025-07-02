pub mod gdir;
pub mod hub;

use serde::{Deserialize, Serialize};

use crate::{config::hub::Hub, error::Result};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub hubs: Vec<Hub>,
}

impl Config {
    pub fn push(&mut self, hub: Hub) {
        self.hubs.push(hub);
    }

    pub fn encode(&self) -> Result<String> {
        use toml::to_string;
        Ok(to_string(self)?)
    }
}

impl From<Hub> for Config {
    fn from(hub: Hub) -> Self {
        Self { hubs: vec![hub] }
    }
}

impl FromIterator<Hub> for Config {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Hub>,
    {
        Self { hubs: iter.into_iter().collect() }
    }
}

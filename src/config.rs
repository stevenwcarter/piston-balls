use anyhow::{Context, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::fs;
use toml;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub max_radius: f64,
    pub max_velocity: f64,
    pub ball_count: usize,
}

lazy_static! {
    pub static ref CONFIG: Config = load_config_file().unwrap();
}

pub fn load_config_file() -> Result<Config> {
    let filename = "Config.toml";
    let contents = fs::read_to_string(filename).context("Could not find config file")?;

    toml::from_str(&contents).context("Could not parse")
}

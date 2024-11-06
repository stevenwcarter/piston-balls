use anyhow::{Context, Result};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{
    fs,
    sync::{Arc, RwLock},
};
use toml;

lazy_static! {
    pub static ref GRAVITY_Y: Arc<RwLock<f64>> = Arc::new(RwLock::new(0.0));
    pub static ref GRAVITY_X: Arc<RwLock<f64>> = Arc::new(RwLock::new(0.0));
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub max_radius: f64,
    pub max_velocity: f64,
    pub ball_count: usize,
    #[serde(default)]
    pub damping_wall: f64,
    #[serde(default)]
    pub damping_ball: f64,
    #[serde(default)]
    pub air_resistance: f64,
    #[serde(default)]
    pub collision_tolerance: f64,
}

lazy_static! {
    pub static ref CONFIG: Config = load_config_file().unwrap();
}

pub fn load_config_file() -> Result<Config> {
    let filename = "Config.toml";
    let contents = fs::read_to_string(filename).context("Could not find config file")?;

    toml::from_str(&contents).context("Could not parse")
}

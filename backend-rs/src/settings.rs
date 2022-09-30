use serde::{Deserialize, Serialize};
use std::{path::PathBuf, fmt::Display};

use crate::helper;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub enable : bool,
    pub tun_mode : bool,
    pub current_sub: String,
    pub subscriptions : Vec<Subscription>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Subscription {
    pub path : String,
    pub url : String
}

#[derive(Debug)]
pub enum JsonError {
    Serde(serde_json::Error),
    Io(std::io::Error),
}

impl Display for JsonError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Serde(e) => (e as &dyn Display).fmt(f),
            Self::Io(e) => (e as &dyn Display).fmt(f),
        }
    }
}

impl Subscription {
    pub fn new(path: String, url: String) -> Self
    {
        Self { path: path, url: url }
    }
}


#[derive(Debug)]
pub struct State {
    pub home: PathBuf,
    pub dirty: bool,
}

impl State {
    pub fn new() -> Self {
        let def = Self::default();
        if cfg!(debug_assertions) {
            return Self {
            home: "./tmp".into(),
            dirty: true,
        }
        }
        Self {
            home: usdpl_back::api::dirs::home().unwrap_or(def.home),
            dirty: true,
        }
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            home: "/home/deck".into(),
            dirty: true,
        }
    }
}


impl Settings {
    pub fn save<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), JsonError> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(JsonError::Io)?;
        }
        let mut file = std::fs::File::create(path).map_err(JsonError::Io)?;
        serde_json::to_writer_pretty(&mut file, &self).map_err(JsonError::Serde)
    }

    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Self, JsonError> {
        let mut file = std::fs::File::open(path).map_err(JsonError::Io)?;
        serde_json::from_reader(&mut file).map_err(JsonError::Serde)
    }
}

impl Default for Settings {
    fn default() -> Self {
        let default_profile = helper::get_current_working_dir().unwrap().join("bin/core/config.yaml");
        Self {
            enable: false,
            tun_mode: true,
            current_sub: default_profile.to_string_lossy().to_string(),
            subscriptions: Vec::new()
        }
    }
}
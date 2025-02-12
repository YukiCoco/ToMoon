use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

use crate::helper;

use crate::control::EnhancedMode;

#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    #[serde(default = "default_enable")]
    pub enable: bool,
    #[serde(default = "default_skip_proxy")]
    pub skip_proxy: bool,
    #[serde(default = "default_override_dns")]
    pub override_dns: bool,
    #[serde(default = "default_enhanced_mode")]
    pub enhanced_mode: EnhancedMode,
    #[serde(default = "default_current_sub")]
    pub current_sub: String,
    #[serde(default = "default_subscriptions")]
    pub subscriptions: Vec<Subscription>,
    #[serde(default = "default_allow_remote_access")]
    pub allow_remote_access: bool,
}

fn default_skip_proxy() -> bool {
    true
}

fn default_enable() -> bool {
    false
}

fn default_override_dns() -> bool {
    true
}

fn default_allow_remote_access() -> bool {
    false
}

fn default_enhanced_mode() -> EnhancedMode {
    EnhancedMode::FakeIp
}

fn default_current_sub() -> String {
    let default_profile = helper::get_current_working_dir()
        .unwrap()
        .join("bin/core/config.yaml");
    default_profile.to_string_lossy().to_string()
}

fn default_subscriptions() -> Vec<Subscription> {
    Vec::new()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Subscription {
    pub path: String,
    pub url: String,
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
    pub fn new(path: String, url: String) -> Self {
        Self {
            path: path,
            url: url,
        }
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
            };
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
            home: "/root".into(),
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

    pub fn open<P: AsRef<std::path::Path>>(path: P) -> Result<Settings, JsonError> {
        let mut file = std::fs::File::open(path).map_err(JsonError::Io)?;
        serde_json::from_reader(&mut file).map_err(JsonError::Serde)
    }
}

impl Default for Settings {
    fn default() -> Self {
        let default_profile = helper::get_current_working_dir()
            .unwrap()
            .join("bin/core/config.yaml");
        Self {
            enable: false,
            skip_proxy: true,
            override_dns: true,
            enhanced_mode: EnhancedMode::FakeIp,
            current_sub: default_profile.to_string_lossy().to_string(),
            subscriptions: Vec::new(),
            allow_remote_access: default_allow_remote_access()
        }
    }
}

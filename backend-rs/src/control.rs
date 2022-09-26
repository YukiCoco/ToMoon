use std::fmt::Display;
use std::io::ErrorKind;
use std::process::{Command, Child};
use std::sync::{RwLock, Arc};

use std::thread;
use std::time::{Duration, Instant};

use super::settings::{Settings, State};

pub struct ControlRuntime {
    settings: Arc<RwLock<Settings>>,
    state: Arc<RwLock<State>>
}

impl ControlRuntime {
    pub fn new() -> Self {
        let new_state = State::new();
        let settings_p = settings_path(&new_state.home);
        Self {
            settings: Arc::new(RwLock::new(super::settings::Settings::open(settings_p).unwrap_or_default().into())),
            state: Arc::new(RwLock::new(new_state)),
        }
    }
    

    pub(crate) fn settings_clone(&self) -> Arc<RwLock<Settings>> {
        self.settings.clone()
    }

    pub(crate) fn state_clone(&self) -> Arc<RwLock<State>> {
        self.state.clone()
    }

    pub fn run(&self) -> thread::JoinHandle<()>{
        let runtime_settings = self.settings_clone();
        let runtime_state = self.state_clone();

        //save config
        thread::spawn(move || {
            let sleep_duration = Duration::from_millis(1000);
            loop {
                //let start_time = Instant::now();
                { // save to file
                    let state = match runtime_state.read() {
                        Ok(x) => x,
                        Err(e) => {
                            log::error!("runtime failed to acquire state read lock: {}", e);
                            continue;
                        }
                    };
                    if state.dirty {
                        // save settings to file
                        let settings = match runtime_settings.read() {
                            Ok(x) => x,
                            Err(e) => {
                                log::error!("runtime failed to acquire settings read lock: {}", e);
                                continue;
                            }
                        };
                        let settings_json: Settings = settings.clone().into();
                        if let Err(e) = settings_json.save(settings_path(&state.home)) {
                            log::error!("SettingsJson.save({}) error: {}", settings_path(&state.home).display(), e);
                        }
                        //Self::on_set_enable(&settings, &state);
                        drop(state);
                        let mut state = match runtime_state.write() {
                            Ok(x) => x,
                            Err(e) => {
                                log::error!("runtime failed to acquire state write lock: {}", e);
                                continue;
                            }
                        };
                        state.dirty = false;
                    }
                }
                thread::sleep(sleep_duration);
            }
        })
    }
}

fn settings_path<P: AsRef<std::path::Path>>(home: P) -> std::path::PathBuf {
    home.as_ref().join(".config/clashdeck/clashdeck.json")
}

fn get_current_working_dir() -> std::io::Result<std::path::PathBuf> {
    std::env::current_dir()
}

pub struct clash {
    path : std::path::PathBuf,
    config : std::path::PathBuf,
    instence: Child
}

#[derive(Debug)]
pub enum ClashErrorKind {
    CoreNotFound,
    ConfigFormatError,
    ConfigNotFound,
    Default
}


#[derive(Debug)]
pub struct ClashError {
    Message : String,
    ErrorKind : ClashErrorKind
}

impl Display for ClashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error Kind: {:?}, Error Message: {})", self.ErrorKind, self.Message)
    }
}

impl std::error::Error for ClashError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl ClashError {
    fn new() -> Self {
        Self { Message: "".to_string(), ErrorKind: ClashErrorKind::Default }
    }
}

impl Default for clash {
    fn default() -> Self {
        Self { path: get_current_working_dir().unwrap(), config: get_current_working_dir().unwrap().join("config.yaml")}
    }
}

impl clash {
    pub fn run(&self) -> Result<Child, ClashError> {
        let clash = Command::new(self.path.clone())
        .arg("-f")
        .arg(self.config.clone())
        .spawn();
        let clash = match clash {
            Ok(x) => Ok(x),
            Err(e) => {
                log::error!("run flash failed: {}", e);
                //TODO: 开启 Clash 的错误处理
                return Err(ClashError::new());
            }
        };
        clash
    }

    pub fn stop() {

    }
}
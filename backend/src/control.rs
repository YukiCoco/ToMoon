use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::sync::{Arc, RwLock};

use std::time::Duration;
use std::{error, fs, thread};

use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};

use super::helper;
use super::settings::{Settings, State};

pub struct ControlRuntime {
    settings: Arc<RwLock<Settings>>,
    state: Arc<RwLock<State>>,
    clash_state: Arc<RwLock<Clash>>,
    downlaod_status: Arc<RwLock<DownloadStatus>>,
    update_status: Arc<RwLock<DownloadStatus>>,
    running_status: Arc<RwLock<RunningStatus>>,
}

#[derive(Debug)]
pub enum RunningStatus {
    Loading,
    Failed,
    Success,
    None,
}

#[derive(Debug)]
pub enum DownloadStatus {
    Downloading,
    Failed,
    Success,
    Error,
    None,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum EnhancedMode {
    RedirHost,
    FakeIp,
}

impl std::fmt::Display for DownloadStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

impl std::fmt::Display for RunningStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

// pub struct DownloadStatus {

// }

impl ControlRuntime {
    pub fn new() -> Self {
        let new_state = State::new();
        let settings_p = settings_path(&new_state.home);
        //TODO: Clash 路径
        let clash = Clash::default();
        let download_status = DownloadStatus::None;
        let update_status = DownloadStatus::None;
        let running_status = RunningStatus::None;
        Self {
            settings: Arc::new(RwLock::new(
                super::settings::Settings::open(settings_p)
                    .unwrap_or_default()
                    .into(),
            )),
            state: Arc::new(RwLock::new(new_state)),
            clash_state: Arc::new(RwLock::new(clash)),
            downlaod_status: Arc::new(RwLock::new(download_status)),
            update_status: Arc::new(RwLock::new(update_status)),
            running_status: Arc::new(RwLock::new(running_status)),
        }
    }

    pub(crate) fn settings_clone(&self) -> Arc<RwLock<Settings>> {
        self.settings.clone()
    }

    pub(crate) fn state_clone(&self) -> Arc<RwLock<State>> {
        self.state.clone()
    }

    pub fn clash_state_clone(&self) -> Arc<RwLock<Clash>> {
        self.clash_state.clone()
    }

    pub fn downlaod_status_clone(&self) -> Arc<RwLock<DownloadStatus>> {
        self.downlaod_status.clone()
    }

    pub fn update_status_clone(&self) -> Arc<RwLock<DownloadStatus>> {
        self.update_status.clone()
    }

    pub fn running_status_clone(&self) -> Arc<RwLock<RunningStatus>> {
        self.running_status.clone()
    }

    pub fn run(&self) -> thread::JoinHandle<()> {
        let runtime_settings = self.settings_clone();
        let runtime_state = self.state_clone();

        //health check
        //当程序上次异常退出时的处理
        if let Ok(mut v) = runtime_settings.write() {
            if !helper::is_clash_running() && v.enable {
                v.enable = false;
                drop(v);
                //刷新网卡
                match helper::reset_system_network() {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("runtime failed to acquire settings write lock: {}", e);
                    }
                }
            }
        }

        //保存复原脚本
        if !Path::new("/home/deck/tomoon_recover.sh").exists() {
            let recover_script = r#"sudo chattr -i /etc/resolv.conf
sudo systemctl stop systemd-resolved
sudo chmod a+w /etc/NetworkManager/conf.d/dns.conf
sudo echo -e "[main]\ndns=auto"  > /etc/NetworkManager/conf.d/dns.conf
sudo nmcli general reload"#;
            match fs::write("/home/deck/tomoon_recover.sh", recover_script) {
                Ok(_) => {
                    log::info!("Write recover script to /home/deck/tomoon_recover.sh");
                }
                Err(e) => {
                    log::error!(
                        "Error occurred while writing recover script, Error msg: {}",
                        e.to_string()
                    );
                }
            }
        }

        //save config
        thread::spawn(move || {
            let sleep_duration = Duration::from_millis(1000);
            loop {
                //let start_time = Instant::now();
                {
                    // save to file
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
                            log::error!(
                                "SettingsJson.save({}) error: {}",
                                settings_path(&state.home).display(),
                                e
                            );
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
    home.as_ref().join(".config/tomoon/tomoon.json")
}

fn get_current_working_dir() -> std::io::Result<std::path::PathBuf> {
    std::env::current_dir()
}

fn get_decky_data_dir() -> std::io::Result<std::path::PathBuf> {
    let data_dir = get_current_working_dir().unwrap().join("../../data/tomoon");
    Ok(data_dir)
}

pub struct Clash {
    pub path: std::path::PathBuf,
    pub config: std::path::PathBuf,
    pub instence: Option<Child>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ClashErrorKind {
    ConfigFormatError,
    ConfigNotFound,
    NetworkError,
    InnerError,
    Default,
    CpDbError,
}

#[derive(Debug)]
pub struct ClashError {
    pub Message: String,
    pub ErrorKind: ClashErrorKind,
}

impl error::Error for ClashError {}

impl Display for ClashError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error Kind: {:?}, Error Message: {})",
            self.ErrorKind, self.Message
        )
    }
}

impl ClashError {
    pub fn new() -> Self {
        Self {
            Message: "".to_string(),
            ErrorKind: ClashErrorKind::Default,
        }
    }
}

impl Default for Clash {
    fn default() -> Self {
        Self {
            path: get_current_working_dir().unwrap().join("bin/core/clash"),
            config: get_current_working_dir()
                .unwrap()
                .join("bin/core/config.yaml"),
            instence: None,
        }
    }
}

impl Clash {
    pub fn run(&mut self, config_path: &String, skip_proxy: bool, override_dns: bool, enhanced_mode: EnhancedMode) -> Result<(), ClashError> {
        // decky 插件数据目录 
        let decky_data_dir = get_decky_data_dir().unwrap();
        let new_country_db_path = get_current_working_dir()
            .unwrap()
            .join("bin/core/country.mmdb");
        let new_asn_db_path = get_current_working_dir()
            .unwrap()
            .join("bin/core/asn.mmdb");
        let new_geosite_path = get_current_working_dir()
            .unwrap()
            .join("bin/core/geosite.dat");
        let country_db_path = decky_data_dir.join("country.mmdb");
        let asn_db_path = decky_data_dir.join("asn.mmdb");
        let geosite_path = decky_data_dir.join("geosite.dat");

        // 检查 decky_data_dir 是否存在，不存在则创建
        if !decky_data_dir.exists() {
            fs::create_dir_all(&decky_data_dir).unwrap();
        }

        // 检查数据库文件是否存在，不存在则复制
        if !PathBuf::from(country_db_path.clone()).is_file() {
            match fs::copy(
                new_country_db_path.clone(),
                country_db_path.clone(),
            ) {
                Ok(_) => {
                    log::info!("Copy country.mmdb to decky data dir")
                },
                Err(e) => {
                    return Err(ClashError {
                        Message: e.to_string(),
                        ErrorKind: ClashErrorKind::CpDbError,
                    });
                }
            }
        }

        if !PathBuf::from(asn_db_path.clone()).is_file() {
            match fs::copy(
                new_asn_db_path.clone(),
                asn_db_path.clone(),
            ) {
                Ok(_) => {
                    log::info!("Copy asn.mmdb to decky data dir")
                },
                Err(e) => {
                    return Err(ClashError {
                        Message: e.to_string(),
                        ErrorKind: ClashErrorKind::CpDbError,
                    });
                }
            }
        }
        
        if !PathBuf::from(geosite_path.clone()).is_file() {
            match fs::copy(
                new_geosite_path.clone(),
                geosite_path.clone(),
            ) {
                Ok(_) => {
                    log::info!("Copy geosite.dat to decky data dir")
                },
                Err(e) => {
                    return Err(ClashError {
                        Message: e.to_string(),
                        ErrorKind: ClashErrorKind::CpDbError,
                    });
                }
            }
        }

        self.update_config_path(config_path);
        // 修改配置文件为推荐配置
        match self.change_config(skip_proxy, override_dns, enhanced_mode) {
            Ok(_) => (),
            Err(e) => {
                return Err(ClashError {
                    Message: e.to_string(),
                    ErrorKind: ClashErrorKind::ConfigFormatError,
                });
            }
        }

        //log::info!("Pre-setting network");
        //TODO: 未修改的 unwarp
        let run_config = decky_data_dir.join("running_config.yaml");
        let outputs = fs::File::create("/tmp/tomoon.clash.log").unwrap();
        let errors = outputs.try_clone().unwrap();

        log::info!("Starting Clash...");

        let clash = Command::new(self.path.clone())
            .arg("-d")
            .arg(decky_data_dir)
            .arg("-f")
            .arg(run_config)
            .stdout(outputs)
            .stderr(errors)
            .spawn();
        let clash: Result<Child, ClashError> = match clash {
            Ok(x) => Ok(x),
            Err(e) => {
                log::error!("run Clash failed: {}", e);
                //TODO: 开启 Clash 的错误处理
                return Err(ClashError::new());
            }
        };
        self.instence = Some(clash.unwrap());
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn error::Error>> {
        let instance = self.instence.as_mut();
        match instance {
            Some(x) => {
                x.kill()?;
                x.wait()?;

                //直接重置网络
                helper::reset_system_network()?;
            }
            None => {
                //Not launch Clash yet...
                log::error!("Error occurred while disabling Clash: Not launch Clash yet");
            }
        };
        Ok(())
    }

    pub fn update_config_path(&mut self, path: &String) {
        self.config = std::path::PathBuf::from((*path).clone());
    }

    pub fn change_config(&self, skip_proxy: bool, override_dns: bool, enhanced_mode: EnhancedMode) -> Result<(), Box<dyn error::Error>> {
        let path = self.config.clone();
        let config = fs::read_to_string(path)?;
        let mut yaml: serde_yaml::Value = serde_yaml::from_str(config.as_str())?;
        let yaml = yaml.as_mapping_mut().unwrap();

        log::info!("Changing Clash config...");

        //修改 WebUI

        match yaml.get_mut("external-controller") {
            Some(x) => {
                *x = Value::String(String::from("127.0.0.1:9090"));
            }
            None => {
                yaml.insert(
                    Value::String(String::from("external-controller")),
                    Value::String(String::from("127.0.0.1:9090")),
                );
            }
        }

        //修改 test.steampowered.com
        //这个域名用于 Steam Deck 网络连接验证，可以直连
        if let Some(x) = yaml.get_mut("rules") {
            let rules = x.as_sequence_mut().unwrap();
            rules.insert(
                0,
                Value::String(String::from("DOMAIN,test.steampowered.com,DIRECT")),
            );

            if skip_proxy {
                rules.insert(
                0,
                Value::String(String::from("DOMAIN-SUFFIX,cm.steampowered.com,DIRECT")),
            );
            rules.insert(
            0,
            Value::String(String::from("DOMAIN-SUFFIX,steamserver.net,DIRECT")),
        );
            }
        }

        let webui_dir = get_current_working_dir()?.join("bin/core/web");

        match yaml.get_mut("external-ui") {
            Some(x) => {
                //TODO: 修改 Web UI 的路径
                *x = Value::String(String::from(webui_dir.to_str().unwrap()));
            }
            None => {
                yaml.insert(
                    Value::String(String::from("external-ui")),
                    Value::String(String::from(webui_dir.to_str().unwrap())),
                );
            }
        }

        //修改 TUN 和 DNS 配置

        let tun_config = "
        enable: true
        stack: system
        auto-route: true
        auto-detect-interface: true
        dns-hijack:
            - any:53
        ";

        //部分配置来自 https://www.xkww3n.cyou/2022/02/08/use-clash-dns-anti-dns-hijacking/

        let dns_config_fakeip = "
        enable: true
        listen: 127.0.0.1:8853
        default-nameserver:
            - 223.5.5.5
            - 8.8.4.4
        ipv6: false
        enhanced-mode: fake-ip
        nameserver:
            - 119.29.29.29
            - 223.5.5.5
            - tls://223.5.5.5:853
            - tls://223.6.6.6:853
        fallback:
            - https://1.0.0.1/dns-query
            - https://public.dns.iij.jp/dns-query
            - tls://8.8.4.4:853
        fallback-filter:
            geoip: false
            ipcidr:
            - 240.0.0.0/4
            - 0.0.0.0/32
            - 127.0.0.1/32
        fake-ip-filter:
            - \"*.lan\"
            - \"*.localdomain\"
            - \"*.localhost\"
            - \"*.local\"
            - \"*.home.arpa\"
            - stun.*.*
            - stun.*.*.*
            - +.stun.*.*
            - +.stun.*.*.*
            - +.stun.*.*.*.*
        ";

        let dns_config_redir_host = "
        enable: true
        ipv6: false
        listen: 127.0.0.1:8853
        default-nameserver:
            - 223.5.5.5
            - 8.8.4.4
        enhanced-mode: redir-host
        nameserver:
            - 119.29.29.29
            - 223.5.5.5
            - tls://223.5.5.5:853
            - tls://223.6.6.6:853
        fallback:
            - https://1.0.0.1/dns-query
            - https://public.dns.iij.jp/dns-query
            - tls://8.8.4.4:853
        fallback-filter:
            geoip: false
            ipcidr:
            - 240.0.0.0/4
            - 0.0.0.0/32
            - 127.0.0.1/32
        ";

        let profile_config = "
        store-selected: true
        store-fake-ip: false
        ";

        let insert_config = |yaml: &mut Mapping, config: &str, key: &str| {
            let inner_config: Value = serde_yaml::from_str(config).unwrap();
            yaml.insert(Value::String(String::from(key)), inner_config);
        };

        //开启 tun 模式
        match yaml.get("tun") {
            Some(_) => {
                yaml.remove("tun").unwrap();
                insert_config(yaml, tun_config, "tun");
            }
            None => {
                insert_config(yaml, tun_config, "tun");
            }
        }

        match yaml.get("dns") {
            Some(_) => {
                //删除 DNS 配置
                if override_dns {
                    log::info!("EnhancedMode: {:?}", enhanced_mode);
                    yaml.remove("dns").unwrap();
                    match enhanced_mode {
                        EnhancedMode::FakeIp => {
                            insert_config(yaml, dns_config_fakeip, "dns");
                        }
                        EnhancedMode::RedirHost => {
                            insert_config(yaml, dns_config_redir_host, "dns");
                        }
                    }
                }
            }
            None => {
                insert_config(yaml, dns_config_redir_host, "dns");
            }
        }

        // 保存上次的配置
        match yaml.get("profile") {
            Some(_) => {
                yaml.remove("profile").unwrap();
                insert_config(yaml, profile_config, "profile");
            }
            None => {
                insert_config(yaml, profile_config, "profile");
            }
        }

        let run_config = get_decky_data_dir()?.join("running_config.yaml");

        let yaml_str = serde_yaml::to_string(&yaml)?;
        fs::write(run_config, yaml_str)?;

        log::info!("Clash config changed successfully");
        Ok(())
    }

}

use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::process::{Child, Command};
use std::sync::{Arc, RwLock};

use std::time::Duration;
use std::{error, fs, thread};

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

pub struct Clash {
    pub path: std::path::PathBuf,
    pub config: std::path::PathBuf,
    pub instence: Option<Child>,
    pub smartdns_instence: Option<Child>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ClashErrorKind {
    ConfigFormatError,
    ConfigNotFound,
    RuleProviderDownloadError,
    NetworkError,
    CpDbError,
    InnerError,
    Default,
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
            smartdns_instence: None,
        }
    }
}

impl Clash {
    pub fn run(&mut self, config_path: &String) -> Result<(), ClashError> {
        //没有 Country.mmdb
        let country_db_path = "/root/.config/clash/Country.mmdb";
        if let Some(parent) = PathBuf::from(country_db_path).parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                log::error!("Failed while creating /root/.config/clash dir.");
                log::error!("Error Message:{}", e);
                return Err(ClashError {
                    ErrorKind: ClashErrorKind::CpDbError,
                    Message: "Error occurred while creating /root/.config/clash dir.".to_string(),
                });
            }
        }
        let new_country_db_path = get_current_working_dir()
            .unwrap()
            .join("bin/core/Country.mmdb");
        if !PathBuf::from(country_db_path).is_file() {
            match fs::copy(new_country_db_path, country_db_path) {
                Ok(_) => {
                    log::info!("cp Country.mmdb to .clash dir");
                }
                Err(e) => {
                    log::info!("Error occurred while coping Country.mmdb");
                    return Err(ClashError {
                        Message: e.to_string(),
                        ErrorKind: ClashErrorKind::CpDbError,
                    });
                }
            }
        }
        self.update_config_path(config_path);
        // 修改配置文件为推荐配置
        match self.change_config() {
            Ok(_) => (),
            Err(e) => {
                return Err(ClashError {
                    Message: e.to_string(),
                    ErrorKind: ClashErrorKind::ConfigFormatError,
                });
            }
        }
        //在 clash 启动前修改 DNS
        //先结束 systemd-resolve ，否则会因为端口占用启动失败
        match helper::set_system_network() {
            Ok(_) => {
                log::info!("Successfully set network status");
            }
            Err(e) => {
                log::error!("Error occurred while setting system network: {}", e);
                return Err(ClashError {
                    Message: e.to_string(),
                    ErrorKind: ClashErrorKind::NetworkError,
                });
            }
        }

        //log::info!("Pre-setting network");
        //TODO: 未修改的 unwarp
        let run_config = get_current_working_dir()
            .unwrap()
            .join("bin/core/running_config.yaml");
        let outputs = fs::File::create("/tmp/tomoon.clash.log").unwrap();
        let errors = outputs.try_clone().unwrap();

        let smartdns_path = get_current_working_dir()
            .unwrap()
            .join("bin/smartdns/smartdns");

        let smartdns_config_path = get_current_working_dir()
            .unwrap()
            .join("bin/smartdns/config.conf");

        // let smartdns_outputs = fs::File::create("/tmp/tomoon.smartdns.log").unwrap();
        // let smartdns_errors = outputs.try_clone().unwrap();

        // 启动 SmartDNS 作为 DNS 上游
        let smart_dns = Command::new(smartdns_path)
            .arg("-c")
            .arg(smartdns_config_path)
            .arg("-f")
            // .stdout(smartdns_outputs)
            // .stderr(smartdns_errors)
            .spawn();

        let clash = Command::new(self.path.clone())
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
        self.smartdns_instence = Some(smart_dns.unwrap());
        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), Box<dyn error::Error>> {
        let instance = self.instence.as_mut();
        match instance {
            Some(x) => {
                x.kill()?;
                x.wait()?;

                // 复原 DNS
                //弃用，因为可能是在中途换过 WiFi 导致原有 DNS 失效
                // Command::new("chattr")
                //     .arg("-i")
                //     .arg("/etc/resolv.conf")
                //     .spawn()
                //     .unwrap()
                //     .wait()
                //     .unwrap();
                // fs::copy("./resolv.conf.bk", "/etc/resolv.conf")?;

                //直接重置网络
                helper::reset_system_network()?;
            }
            None => {
                //Not launch Clash yet...
                log::error!("Error occurred while disabling Clash: Not launch Clash yet");
            }
        };
        let smartdns_instance = self.smartdns_instence.as_mut();
        match smartdns_instance {
            Some(x) => {
                x.kill()?;
                x.wait()?;
            }
            None => {
                log::error!("Error occurred while disabling SmartDNS : Not launch SmartDNS yet");
            }
        };
        Ok(())
    }

    pub fn update_config_path(&mut self, path: &String) {
        self.config = std::path::PathBuf::from((*path).clone());
    }

    pub fn change_config(&self) -> Result<(), Box<dyn error::Error>> {
        let path = self.config.clone();
        let config = fs::read_to_string(path)?;
        let mut yaml: serde_yaml::Value = serde_yaml::from_str(config.as_str())?;
        let yaml = yaml.as_mapping_mut().unwrap();

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
        }

        //下载 rules-provider
        if let Some(x) = yaml.get_mut("rule-providers") {
            let provider = x.as_mapping().unwrap();
            match self.downlaod_proxy_providers(provider) {
                Ok(_) => {
                    log::info!("All rules provider downloaded");
                }
                Err(e) => return Err(Box::new(e)),
            }
        } else {
            log::info!("no rule-providers found.");
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
        ";

        //部分配置来自 https://www.xkww3n.cyou/2022/02/08/use-clash-dns-anti-dns-hijacking/

        let dns_config = match helper::is_resolve_running() {
            true => {
                "
        enable: true
        listen: 0.0.0.0:5354
        enhanced-mode: fake-ip
        fake-ip-range: 198.18.0.1/16
        nameserver:
            - tcp://127.0.0.1:5353
        "
            }
            false => {
                "
        enable: true
        listen: 0.0.0.0:53
        enhanced-mode: fake-ip
        fake-ip-range: 198.18.0.1/16
        nameserver:
            - tcp://127.0.0.1:5353
        "
            }
        };

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
                yaml.remove("dns").unwrap();
                insert_config(yaml, dns_config, "dns");
            }
            None => {
                insert_config(yaml, dns_config, "dns");
            }
        }

        // 保存上次的配置
        match yaml.get("profile") {
            Some(_) => {
                //删除 DNS 配置
                yaml.remove("profile").unwrap();
                insert_config(yaml, profile_config, "profile");
            }
            None => {
                insert_config(yaml, profile_config, "profile");
            }
        }

        let run_config = get_current_working_dir()?.join("bin/core/running_config.yaml");

        let yaml_str = serde_yaml::to_string(&yaml)?;
        fs::write(run_config, yaml_str)?;
        Ok(())
    }

    pub fn downlaod_proxy_providers(&self, yaml: &serde_yaml::Mapping) -> Result<(), ClashError> {
        for (_, value) in yaml {
            if let Some(url) = value.get("url") {
                if let Some(path) = value.get("path") {
                    //替换有些规则前的 ./
                    let r = regex::Regex::new(r"^\./").unwrap();
                    let result = r.replace(path.as_str().unwrap(), "");
                    let save_path = PathBuf::from("/root/.config/clash/").join(result.to_string());
                    if !save_path.exists() {
                        match minreq::get(url.as_str().unwrap())
                            .with_timeout(30)
                            .with_header("User-Agent", format!("ToMoonClash/{}",env!("CARGO_PKG_VERSION")))
                            .send()
                        {
                            Ok(response) => {
                                let response = match response.as_str() {
                                    Ok(x) => x,
                                    Err(_) => {
                                        log::error!("Error occurred while parase Rule Provder.");
                                        return Err(ClashError {
                                            ErrorKind: ClashErrorKind::RuleProviderDownloadError,
                                            Message: String::from(
                                                "Error occurred while parase Rule Provder.",
                                            ),
                                        });
                                    }
                                };

                                //保存订阅
                                if let Some(parent) = save_path.parent() {
                                    if let Err(e) = std::fs::create_dir_all(parent) {
                                        log::error!("Failed while creating sub dir.");
                                        log::error!("Error Message:{}", e);
                                        return Err(ClashError {
                                            ErrorKind: ClashErrorKind::RuleProviderDownloadError,
                                            Message:
                                                "Error occurred while creating Rule Provder dir."
                                                    .to_string(),
                                        });
                                    }
                                }

                                match fs::write(save_path.clone(), response) {
                                    Ok(_) => {
                                        log::info!(
                                            "Rule-Provider {} downloaded.",
                                            save_path.display()
                                        );
                                    }
                                    Err(_) => {
                                        log::error!(
                                            "Error occurred while saving Rule Provder. path: {}",
                                            save_path.clone().to_str().unwrap()
                                        );
                                        return Err(ClashError {
                                            ErrorKind: ClashErrorKind::RuleProviderDownloadError,
                                            Message:
                                                "Error occurred while downloading Rule Provder."
                                                    .to_string(),
                                        });
                                    }
                                }
                            }
                            Err(e) => {
                                let in_msg = e.to_string();
                                let mut err_msg = String::from("Error occurred while downloading Rule Provder with error message : ");
                                err_msg.push_str(in_msg.as_str());
                                return Err(ClashError {
                                    ErrorKind: ClashErrorKind::RuleProviderDownloadError,
                                    Message: err_msg,
                                });
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

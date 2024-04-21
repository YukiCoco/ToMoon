use std::{path::Path, process::Command};

use regex::Regex;

use std::fs;

use sysinfo::{ProcessExt, System, SystemExt};

pub fn reset_system_network() -> Result<(), Box<dyn std::error::Error>> {
    //读入程序的 DNS
    let default_config = "[main]\ndns=auto";
    fs::write("/etc/NetworkManager/conf.d/dns.conf", default_config)?;
    // 修改 DNS 为可写
    Command::new("chattr")
        .arg("-i")
        .arg("/etc/resolv.conf")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    //fs::copy("./resolv.conf.bk", "/etc/resolv.conf")?;

    // 更新 NetworkManager
    Command::new("nmcli")
        .arg("general")
        .arg("reload")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
    // match fs::copy("./resolv.conf.bk", "/etc/resolv.conf") {
    //     Ok(_) => (),
    //     Err(e) => {
    //         log::error!("reset_network() error: {}", e);
    //         return vec![];
    //     }
    // }
    log::info!("Successfully reset network");
    Ok(())
}

pub fn get_current_working_dir() -> std::io::Result<std::path::PathBuf> {
    std::env::current_dir()
}

pub fn check_yaml(str: &String) -> bool {
    if let Ok(x) = serde_yaml::from_str::<serde_yaml::Value>(str) {
        if let Some(v) = x.as_mapping() {
            if v.contains_key("rules") {
                return true;
            } else {
                return false;
            }
        } else {
            return false;
        }
    } else {
        return false;
    }
}

pub fn is_clash_running() -> bool {
    //关闭 systemd-resolved
    let mut sys = System::new_all();
    sys.refresh_all();
    for (_, process) in sys.processes() {
        if process.name() == "clash" {
            return true;
        }
    }
    return false;
}

pub fn get_file_path(url: String) -> Option<String> {
    let r = Regex::new(r"^file://").unwrap();
    if let Some(x) = r.find(url.clone().as_str()) {
        let file_path = url[x.end()..url.len()].to_string();
        return Some(file_path);
    };
    return None;
}

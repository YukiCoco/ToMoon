mod tests {

    use crate::{control, helper};
    use regex::Regex;
    use serde_yaml::{Mapping, Number, Value};
    use std::{
        fs,
        path::PathBuf,
        process::{Command, Stdio},
        thread,
        time::Duration,
    };

    use sysinfo::{Pid, ProcessExt, System, SystemExt};

    #[test]
    fn check_systemd_resolved() {}

    #[test]
    fn it_works() {
        assert_eq!(2 + 3, 4);
    }

    #[test]
    fn read_dns() {
        assert_eq!(helper::is_clash_running(), true);
    }

    #[test]
    fn get_version() {
        let version = env!("CARGO_PKG_VERSION");
        println!("{}",version);
    }

    #[test]
    fn run_clash() {
        //TODO: no such files
        println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
        // clash
        //     .run(&String::from("/home/deck/.config/tomoon/subs/Ob3jZ.yaml"))
        //     .unwrap();
    }

    #[test]
    fn test_network() {
        let is_resolve_running = || {
            let mut sys = System::new_all();
            // First we update all information of our `System` struct.
            sys.refresh_all();
            for (_, process) in sys.processes() {
                if process.name() == "systemd-resolve" {
                    return true;
                }
            }
            return false;
        };
        assert_eq!(false, is_resolve_running());
    }

    #[test]
    fn find_process() {
        let mut sys = System::new_all();
        sys.refresh_all();
        for (pid, process) in sys.processes() {
            if process.name() == "systemd-resolve" {
                println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
            }
        }
    }

    #[test]
    fn test_yaml() {
        println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
        let mut clash = control::Clash::default();
        clash.change_config(true, true, true, true);
    }

    #[test]
    fn regex_test() {
        let url = String::from("file:///home/dek/b.yaml");
        if let Some(path) = helper::get_file_path(url) {
            println!("{}", path);
        }
    }

    fn fun_name(url: String) {
        let r = Regex::new(r"^file://").unwrap();
        if let Some(x) = r.find(url.clone().as_str()) {
            let file_path = url[x.end()..url.len()].to_string();
            println!("{}", file_path);
        };
    }

    #[test]
    fn test_privider_path() {
        let test_yaml = "./Rules/IPfake.yaml";
        let r = Regex::new(r"^\./").unwrap();
        let result = r.replace(test_yaml, "");
        let save_path = PathBuf::from("/root/.config/clash/").join(result.to_string());
        println!("Rule-Provider {} updated.", save_path.display());
    }

    #[test]
    fn test_rules_provider() {
        let path = "./bin/config.yaml";
        let config = fs::read_to_string(path).unwrap();
        let mut yaml: serde_yaml::Value = serde_yaml::from_str(config.as_str()).unwrap();
        let yaml = yaml.as_mapping_mut().unwrap();
        if let Some(x) = yaml.get_mut("rule-providers") {
            let provider = x.as_mapping().unwrap();
            for (key, value) in provider {
                if let Some(url) = value.get("url") {
                    if let Some(path) = value.get("path") {
                        println!("{} {}", path.as_str().unwrap(), url.as_str().unwrap());
                    }
                }
            }
        } else {
            log::info!("no rule-providers found.");
        }
    }

    #[test]
    fn run_yaml() {
        let path = "./bin/config.yaml";
        let config = fs::read_to_string(path).unwrap();
        let mut yaml: serde_yaml::Value = serde_yaml::from_str(config.as_str()).unwrap();
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

        match yaml.get_mut("external-ui") {
            Some(x) => {
                //TODO: 修改 Web UI 的路径
                *x = Value::String(String::from(
                    "/home/deck/homebrew/plugins/tomoon/bin/core/web",
                ));
            }
            None => {
                yaml.insert(
                    Value::String(String::from("external-controller")),
                    Value::String(String::from(
                        "/home/deck/homebrew/plugins/tomoon/bin/core/web",
                    )),
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

        let dns_config = match helper::is_resolve_running() {
            true => {
                "
        enable: true
        listen: 0.0.0.0:5354
        enhanced-mode: fake-ip
        fake-ip-range: 198.18.0.1/16
        nameserver:
            - https://doh.pub/dns-query
            - https://dns.alidns.com/dns-query
            - '114.114.114.114'
            - '223.5.5.5'
        default-nameserver:
            - 119.29.29.29
            - 223.5.5.5
        fallback:
            - https://1.1.1.1/dns-query
            - https://dns.google/dns-query
            - https://doh.opendns.com/dns-query
            - https://doh.pub/dns-query
        fallback-filter:
            geoip: true
            geoip-code: CN
            ipcidr:
                - 240.0.0.0/4
        "
            }
            false => {
                "
        enable: true
        listen: 0.0.0.0:53
        enhanced-mode: fake-ip
        fake-ip-range: 198.18.0.1/16
        nameserver:
            - https://doh.pub/dns-query
            - https://dns.alidns.com/dns-query
            - '114.114.114.114'
            - '223.5.5.5'
        default-nameserver:
            - 119.29.29.29
            - 223.5.5.5
        fallback:
            - https://1.1.1.1/dns-query
            - https://dns.google/dns-query
            - https://doh.opendns.com/dns-query
            - https://doh.pub/dns-query
        fallback-filter:
            geoip: true
            geoip-code: CN
            ipcidr:
                - 240.0.0.0/4
        "
            }
        };

        //部分配置来自 https://www.xkww3n.cyou/2022/02/08/use-clash-dns-anti-dns-hijacking/

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

        let yaml_str = serde_yaml::to_string(&yaml).unwrap();
        fs::write("./bin/config.new.yaml", yaml_str).unwrap();
    }

    #[test]
    fn debug_log() {
        let running_status = format!(
            "Clash status : {} \n",
            helper::is_clash_running()
        );
        let tomoon_log = match fs::read_to_string("/tmp/tomoon.log") {
            Ok(x) => x,
            Err(e) => {
                format!("can not find Tomoon log, error message: {} \n", e)
            }
        };
        let clash_log = match fs::read_to_string("/tmp/tomoon.clash.log") {
            Ok(x) => x,
            Err(e) => {
                format!("can not find Clash log, error message: {} \n", e)
            }
        };
        let dns_resolve_config = match fs::read_to_string("/etc/resolv.conf") {
            Ok(x) => x,
            Err(e) => {
                format!("can not find /etc/resolv.conf, error message: {} \n", e)
            }
        };

        let network_config = match fs::read_to_string("/etc/NetworkManager/conf.d/dns.conf") {
            Ok(x) => x,
            Err(e) => {
                format!(
                    "can not find /etc/NetworkManager/conf.d/dns.conf, error message: {} \n",
                    e
                )
            }
        };

        let log = format!(
            "
        {}\n
        ToMoon log:\n
        {}\n
        Clash log:\n
        {}\n
        resolv log:\n
        {}\n
        network log:\n
        {}\n
        ",
            running_status, tomoon_log, clash_log, dns_resolve_config, network_config
        );
        fs::write("/tmp/tomoon.debug.log", log).unwrap();
    }
}

mod tests {

    use crate::{control, helper};
    use regex::Regex;
    use serde_yaml::{Mapping, Number, Value};
    use std::{fs, process::Command, thread, time::Duration};

    use sysinfo::{Pid, ProcessExt, System, SystemExt};

    #[test]
    fn it_works() {
        assert_eq!(2 + 3, 4);
    }

    #[test]
    fn read_dns() {
        assert_eq!(helper::is_clash_running(),true);
    }

    #[test]
    fn run_clash() {
        let mut clash = control::Clash::default();
        println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
        //clash.run().unwrap();
        thread::sleep(Duration::from_secs(5));
        println!("disable clash");
        clash.stop();
        thread::sleep(Duration::from_secs(10));
    }

    #[test]
    fn test_network() {
        helper::set_system_network().unwrap();
    }

    #[test]
    fn find_process() {
        let mut sys = System::new_all();

        // First we update all information of our `System` struct.
        sys.refresh_all();
        // if let Some(process) = sys.process(Pid::from(28656)) {
        //     println!("process : {}", process.name());
        // } else {
        //     println!("Not found");
        // }

        for (pid, process) in sys.processes() {
            if process.name() == "systemd-resolve"
            {
                println!("[{}] {} {:?}", pid, process.name(), process.disk_usage());
            }
        }
    }

    #[test]
    fn test_yaml() {
        println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
        let mut clash = control::Clash::default();
        clash.change_config();
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

        //部分配置来自 https://www.xkww3n.cyou/2022/02/08/use-clash-dns-anti-dns-hijacking/
        let dns_config = "
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

        let yaml_str = serde_yaml::to_string(&yaml).unwrap();
        fs::write("./bin/config.new.yaml", yaml_str).unwrap();
    }
}

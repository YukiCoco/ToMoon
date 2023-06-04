# ToMoon 
## 功能  
**使用 ToMoon，让您在恶劣的网络环境下可以打开任何界面，体验到完整的 Steam Deck**  
1. 提供开箱即用的 Clash SteamOS 客户端，由 Rust 驱动
2. 自动配置 DNS，使用 [SmartDNS](https://pymumu.github.io/smartdns/) 作为 DNS 上游，基于分流规则绕过 DNS 污染
3. 自动配置 FAKE-IP 模式，让游戏流量通过 TCP / UDP 加速
4. 基于 [yacd](https://github.com/haishanh/yacd) 的策略管理仪表盘  

## 安装
如果已经安装过 Plugin Loader 2.0 以上版本，直接从第 8 点开始即可。  

1. 打开到 Steam Deck 设置界面
2. 系统 -> 系统设置 -> 打开开发者模式
3. 回到设置向下翻，找到开发者 -> 打开 CEF 远程调试
4. 等待 Steam Deck 重启
5. 按电源键切换到 Desktop 桌面模式
6. 打开 Konsole，如果事先没有创建过终端密码，使用 `passwd` 设置你的密码  
7. 输入 `curl -L http://dl.ohmydeck.net | sh` 安装 Plugin Loader  
8. 输入 `curl -L http://i.ohmydeck.net | sh` 安装 Tomoon  
9. 切换回到 Gamming 游戏模式，按下右侧摇杆下的快捷按钮（三个点的按钮），可以看到多了一个 Decky 插件面板  

## 使用
1. 打开 Manage Subscriptions，添加你服务商提供的 Clash 订阅链接并下载  
> **若使用扫描二维码添加订阅功能，请确保手机和 Steam Deck 在同一局域网下**

> 如果需要添加本地文件，使用  `file://` 加绝对路径作为下载链接填入即可，如 `file:///home/deck/config.yaml`
2. 下载完成后，切换回主界面选择订阅并点击启动  
3. 在桌面模式可通过浏览器 http://127.0.0.1:9090/ui 打开仪表盘  

## 演示  
![Gamming](https://github.com/YukiCoco/StaticFilesCDN/blob/main/deck_gaming.jpg?raw=true)
![Dashboard](https://github.com/YukiCoco/StaticFilesCDN/blob/main/deck_dashboard2.jpg?raw=true)
![Subs](https://github.com/YukiCoco/StaticFilesCDN/blob/main/deck_subs.jpg?raw=true)

## 支持
加入我们的讨论社群，提交 Bug & Feature Request  
[Telegram Group](https://t.me/steamdecktalk)  
## 已知 BUG
当 SteamOS 系统更新等某些外部原因导致 Decky Loader 失效，ToMoon 没有正确关闭 Clash，会出现**无法上网**的情况。此时请进入桌面模式，使用 Konsole 复原 DNS.  
````shell
sudo chattr -i /etc/resolv.conf
sudo systemctl stop systemd-resolved
sudo chmod a+w /etc/NetworkManager/conf.d/dns.conf
sudo echo -e "[main]\ndns=auto"  > /etc/NetworkManager/conf.d/dns.conf
sudo nmcli general reload
````
如果安装的是 `v0.0.5` *(2022/11/18)* 以上版本，可以使用脚本直接恢复。
````shell
bash ~/tomoon_recover.sh
````

## Reference
[decky-loader](https://github.com/SteamDeckHomebrew/decky-loader)  
[PowerTools](https://github.com/NGnius/PowerTools/)  
[usdpl-rs](https://github.com/NGnius/usdpl-rs)  

## Sponsor
[CloudFlare](https://www.cloudflare.com/)

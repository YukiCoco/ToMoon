# ToMoon 
## 功能  
**使用 ToMoon，让您在恶劣的网络环境下可以打开任何界面，体验到完整的 Steam Deck**  
1. 提供开箱即用的 Clash SteamOS 客户端，由 Rust 驱动
2. 自动配置 DNS，基于分流规则及 DoH 模式绕过 DNS 污染
3. 自动配置 FAKE-IP 模式，让游戏流量通过 TCP / UDP 加速
4. 基于 [yacd](https://github.com/haishanh/yacd) 的策略管理仪表盘  

## 安装
1. 打开到 Steam Deck 设置界面
2. 系统 -> 系统设置 -> 打开开发者模式
3. 回到设置向下翻，找到开发者 -> 打开 CEF 远程调试
4. 等待 Steam Deck 重启
5. 按电源键切换到 Desktop 桌面模式
6. 打开 Konsole，输入 `curl -L http://dl.ohmydeck.net | sh` 安装 Plugin Loader
7. 切换回到 Gamming 游戏模式，按下右侧摇杆下的按钮，可以看到多了一个 Decky 插件面板
8. 打开左上角设置，在 Mannul plugin install 中输入 `http://moon.ohmydeck.net`，点击 install
9. 重启你的 Steam Deck

## 使用
1. 打开 Manage Subscriptions，添加你服务商提供的 Clash 订阅链接并下载
2. 下载完成后，切换回主界面选择订阅并点击启动

>注意：若是订阅链接过长可以使用短域名缩短服务，如 [t.ly](https://t.ly/) [n9.cl](https://n9.cl/zh).  
> 别忘了在缩短后的链接前面加 `http(s)://`，形如 `https://n9.cl/abcdef` 才是有效的订阅链接

## 演示  
![Gamming](https://github.com/YukiCoco/StaticFilesCDN/blob/main/deck_gaming.jpg?raw=true)
![Dashboard](https://github.com/YukiCoco/StaticFilesCDN/blob/main/deck_dashboard2.jpg?raw=true)
![Subs](https://github.com/YukiCoco/StaticFilesCDN/blob/main/deck_subs.jpg?raw=true)

## 支持
加入我们的讨论社群，提交 Bug & Feature Request  
[Telegram Group](https://t.me/steamdecktalk)  

## Reference
[decky-loader](https://github.com/SteamDeckHomebrew/decky-loader)  
[PowerTools](https://github.com/NGnius/PowerTools/)  
[usdpl-rs](https://github.com/NGnius/usdpl-rs)  
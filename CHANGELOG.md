# Changelog

## [1.1.0] - 2024-12-25

### Added

- 账号管理
- 一键切换校园网登录账号
- 解绑 MAC 界面：本机匹配到 MAC 高亮显示
- 解绑 MAC 界面：当前设备是否与"查询的账号"绑定：
- 解绑 MAC 界面：一键解绑(注销)当前设备匹配的 MAC 地址

### Changed

- 样式修改

## [1.0.0] - 2024-12-12

### Added

- 添加图表展示
- 获取到的公网 ip 点击后可以直接复制到剪切板
- 添加获取 ip 归属地功能
- 制作了图标

### Changed

- 更改了大部分 UI

## [0.9.1] - 2024-12-08

### Added

- 通过 Onedrive 对配置文件进行云同步

## [0.9.0] - 2024-11-18

### Changed

- 样式修改

### Fixed

- 登录校园网的 error 返回信息

## [0.8.9] - 2024-11-15

### Added

- 点击年度扣费账单/每日使用详情/月度使用概览，默认显示当天/月/年的内容。
- 月度使用概览的日历可以切换 MB 或者 GB 作为单位。
- 月度使用概览的日历可以弹出当日的详情。

## [0.8.8] - 2024-11-12

### Added

- 自定义 MAC 地址对应的设备名

### Changed

- 登录校园网可以自动获取 ipv6，我忘了原来能不能自动获取了，好像是能的，改的好像没有意义

## [0.8.7] - 2024-11-06

### Added

- 下载新版本时候的进度条
- 小工具添加了 Web VPN 转换

## [0.8.6] - 2024-11-03

### Added

- 更换背景，调整背景透明度和模糊程度（透明度只有在 Linux，Windows 10 及以下是可以透过去的，否则后面会还有一层 macOS/Windows 11 mica 半透明效果）

### Fixed

- ipv4 超出 120GB，会显示负数
- 直接点提交 start_date 是现在导致获取不到该月信息的问题
- win10 及以下模糊背景看不清问题（其实直接删了这个效果）
- inline css 问题

## [0.8.2] - 2024-10-29

### Added

- 小工具：查询他人校园网流量
- 小工具：根据电表号查询电费（并记录该电表号）
- 校园网登录功能

## [0.8.1] - 2024-10-26

### Fixed

- 修复了 Windows 11 24H2 报错问题

## [0.8.0] - 2024-10-25

### Changed

- 更改了登录方式，弃用了原来的 Headless Browser 登录方式

## [0.7.4] - 2024-10-24

### Added

- 月度概览添加了 ipv4 和 ipv6 的上下行合计

### Changed

- 调整样式

### Fixed

- 更新功能可用了

## [0.7.3] - 2024-10-24

### Fixed

- 更新功能可用了（其实不可用）

## [0.7.2] - 2024-10-23

### Added

- 在月度使用概览上的东西的选择

### Changed

- 月度使用情况的 UI 更改，暗色模式正常了。大概好看了一些

## [0.7.1] - 2024-10-21

### Added

- 支持应用自更新
- 添加了月度使用概览（支持 ipv4 下行）

## [0.6.0] - 2024-08-16

### Added

- Windows 支持了背景效果
- 添加了校外使用的功能

## [0.5.1] - 2024-07-18

### Removed

- 删除了 Windows 的背景效果，由于 Tauri v1 对 Windows 的窗口 decorate 有 bug

## [0.5.0] - 2024-07-18

### Added

- 记住密码登记信息
- 查询周日详细流量信息
- 支持 macOS
- 添加磨砂玻璃效果

### Changed

- 更改登录方式为 Headless Browser 模拟登录

## [0.2.0] - 2024-06-22

初始发布版本

[1.1.0]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v1.1.0
[1.0.0]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v1.0.0
[0.9.1]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.9.1
[0.9.0]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.9.0
[0.8.9]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.8.9
[0.8.8]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.8.8
[0.8.7]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.8.7
[0.8.6]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.8.6
[0.8.2]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.8.2
[0.8.1]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.8.1
[0.8.0]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.8.0
[0.7.4]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.7.4
[0.7.3]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.7.3
[0.7.2]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.7.2
[0.7.1]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.7.1
[0.6.0]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.6.0
[0.5.1]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.5.1
[0.5.0]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.5.0
[0.2.0]: https://github.com/CakeAL/ustb-wifi-tools/releases/tag/v0.2.0

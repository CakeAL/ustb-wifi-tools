# USTB Wifi Tools 贝壳校园网实用工具

## 简介

使用 Tauri 构建的跨平台 APP，前端使用 Vue + NaiveUI，后端使用 Rust。 \
通过
旨在可以让大家更方便的获取 USTB 校园网每日使用情况，查询流量，解绑 MAC 地址等。 \
通过调用校园网的 API，并且数据都在本地进行存储。

## 开发

目前只有Windows 7以上支持，需要电脑安装WebView2。 \
首先确保你已经安装了[`Node.js`](https://nodejs.cn/download/)，[`pnpm`](https://www.pnpm.cn/)以及[`Rust环境`](https://www.rust-lang.org/zh-CN/tools/install)

```bash
# 安装 create-tauri-app
cargo install create-tauri-app --locked
# 安装tauri 命令行 https://tauri.app/zh-cn/blog/2022/09/15/tauri-1-1/#cargo-binstall-support-for-tauri-cli
# 运行
cargo tauri dev
# 或者
pnpm i
pnpm tauri dev
```

## 构建

```bash
cargo tauri build
# 或者
pnpm tauri build
```

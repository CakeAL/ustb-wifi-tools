# USTB Wifi Tools 贝壳校园网实用工具

## 简介

使用Tauri构建的跨平台APP，前端使用Vue + NaiveUI，后端使用Rust。 \
通过
旨在可以让大家更方便的获取USTB校园网每日使用情况，查询流量，解绑MAC地址等。 \
通过调用校园网的API，并且数据都在本地进行存储。

## 开发

大概是 macOS，Windows 和 Linux 都能运行的。 \
首先确保你已经安装了[`Node.js`](https://nodejs.cn/download/)，[`pnpm`](https://www.pnpm.cn/)以及[`Rust环境`](https://www.rust-lang.org/zh-CN/tools/install)

```bash
# 安装 tauri
cargo install create-tauri-app --locked
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

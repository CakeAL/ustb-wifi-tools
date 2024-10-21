default: 
    @just --list

alias b := build
alias d := dev
alias ba := build-all

build: 
    cargo tauri build

dev: 
    cargo tauri dev

build-all: 
    cargo tauri build --target aarch64-apple-darwin
    cargo tauri build --target x86_64-apple-darwin
    # cargo tauri build --target x86_64-pc-windows-gnu

clippy: 
    cd src-tauri && cargo clippy

change_version:
    python3 change_version.py
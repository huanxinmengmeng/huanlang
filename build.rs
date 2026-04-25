// Copyright (c) 2026 幻心梦梦（huanxinmengmeng）
// build.rs - Windows 资源文件配置
//
// 此脚本用于在 Windows 平台上为可执行文件嵌入自定义图标
// 使用方法:
//   1. 运行 prebuild.ps1 将 logo.jpg 转换为 icon.ico
//   2. 运行 cargo build --release 构建项目

use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();

    println!("cargo:rerun-if-changed=build.rs");

    if target.contains("windows") {
        let resources_dir = get_resources_dir();
        let icon_path = resources_dir.join("icon.ico");
        let rc_path = resources_dir.join("app.rc");

        if icon_path.exists() {
            println!("cargo:rerun-if-changed={}", icon_path.display());

            // 设置 Windows 资源文件
            println!("cargo:rustc-link-arg=/MANIFEST:embed");

            // 告诉链接器使用资源文件
            if rc_path.exists() {
                println!("cargo:rerun-if-changed={}", rc_path.display());
            }
        } else {
            let jpg_path = PathBuf::from("images").join("logo.jpg");
            if jpg_path.exists() {
                eprintln!();
                eprintln!("===========================================");
                eprintln!("Warning: Icon file not found!",);
                eprintln!("===========================================");
                eprintln!("Required: {}", icon_path.display());
                eprintln!("Source:   {}", jpg_path.display());
                eprintln!();
                eprintln!("Please run the prebuild script to convert the image:");
                eprintln!("  powershell -ExecutionPolicy Bypass -File prebuild.ps1");
                eprintln!();
            }
        }
    }

    println!("cargo:rustc-env=RESOURCES_DIR={}", get_resources_dir().display());
}

fn get_resources_dir() -> PathBuf {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(manifest_dir).join("resources")
}

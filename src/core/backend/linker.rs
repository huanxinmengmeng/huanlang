// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::path::PathBuf;
use std::process::Command;
use super::error::LinkError;
use super::target::TargetTriple;

/// 链接目标文件
pub fn link_objects(objects: &[PathBuf], output: &PathBuf, triple: &TargetTriple) -> Result<(), LinkError> {
    // 根据目标选择链接器
    let (linker, _args): (&str, Vec<&str>) = if triple.as_str().contains("windows") {
        ("link.exe", vec![])
    } else if triple.as_str().contains("wasm") {
        ("wasm-ld", vec![])
    } else {
        ("ld", vec![])
    };

    let mut cmd = Command::new(linker);
    
    // 添加目标文件
    for obj in objects {
        cmd.arg(obj);
    }
    
    // 添加输出选项
    cmd.arg("-o").arg(output);
    
    // 添加运行时库（如果不是裸机）
    if !triple.as_str().contains("none") {
        cmd.arg("-lhuanrt");
    }
    
    // 执行链接命令
    let status = cmd.status().map_err(|e| LinkError::LinkFailed(e.to_string()))?;
    
    if status.success() {
        Ok(())
    } else {
        Err(LinkError::LinkFailed(format!(
            "链接器退出码: {}",
            status.code().unwrap_or(-1)
        )))
    }
}

/// 链接Wasm文件（Wasm特定）
pub fn link_wasm(objects: &[PathBuf], output: &PathBuf) -> Result<(), LinkError> {
    let mut cmd = Command::new("wasm-ld");
    
    for obj in objects {
        cmd.arg(obj);
    }
    cmd.arg("-o").arg(output);
    
    // Wasm特定选项
    cmd.arg("--no-entry");
    cmd.arg("--export-all");
    
    let status = cmd.status().map_err(|e| LinkError::LinkFailed(e.to_string()))?;
    
    if status.success() {
        Ok(())
    } else {
        Err(LinkError::LinkFailed(format!(
            "链接器退出码: {}",
            status.code().unwrap_or(-1)
        )))
    }
}

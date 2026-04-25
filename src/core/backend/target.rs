// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::fmt;

/// 目标三元组
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetTriple(String);

impl TargetTriple {
    pub fn host() -> Self {
        let arch = std::env::consts::ARCH;
        let os = std::env::consts::OS;
        let _env = if cfg!(windows) { "msvc" } else { "gnu" };
        Self(format!("{}-unknown-{}", arch, os))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn arch(&self) -> Arch {
        if self.0.starts_with("x86_64") {
            Arch::X86_64
        } else if self.0.starts_with("aarch64") || self.0.starts_with("arm64") {
            Arch::AArch64
        } else if self.0.starts_with("arm") || self.0.starts_with("thumb") {
            Arch::ARM
        } else if self.0.starts_with("riscv32") {
            Arch::RiscV32
        } else if self.0.starts_with("riscv64") {
            Arch::RiscV64
        } else if self.0.starts_with("wasm32") {
            Arch::Wasm32
        } else if self.0.starts_with("x86") {
            Arch::X86
        } else {
            Arch::Unknown
        }
    }

    /// 创建特定架构的目标
    pub fn x86_64_linux() -> Self {
        Self("x86_64-unknown-linux-gnu".into())
    }

    pub fn x86_64_windows() -> Self {
        Self("x86_64-pc-windows-msvc".into())
    }

    pub fn aarch64_apple() -> Self {
        Self("aarch64-apple-darwin".into())
    }

    pub fn wasm32_unknown_unknown() -> Self {
        Self("wasm32-unknown-unknown".into())
    }
}

impl fmt::Display for TargetTriple {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
    X86,
    X86_64,
    ARM,
    AArch64,
    RiscV32,
    RiscV64,
    Wasm32,
    Unknown,
}

impl Arch {
    /// 获取指针宽度（字节）
    pub fn pointer_width(&self) -> u8 {
        match self {
            Arch::X86 | Arch::ARM | Arch::RiscV32 | Arch::Wasm32 => 4,
            Arch::X86_64 | Arch::AArch64 | Arch::RiscV64 => 8,
            Arch::Unknown => 8,
        }
    }

    /// 获取架构名称
    pub fn as_str(&self) -> &'static str {
        match self {
            Arch::X86 => "x86",
            Arch::X86_64 => "x86_64",
            Arch::ARM => "arm",
            Arch::AArch64 => "aarch64",
            Arch::RiscV32 => "riscv32",
            Arch::RiscV64 => "riscv64",
            Arch::Wasm32 => "wasm32",
            Arch::Unknown => "unknown",
        }
    }
}

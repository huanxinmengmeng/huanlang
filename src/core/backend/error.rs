// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::fmt;

/// 代码生成错误
#[derive(Debug, Clone, PartialEq)]
pub enum CodeGenError {
    /// 不支持的架构或特性
    Unsupported(String),
    /// MLIR 降级失败
    LoweringError(String),
    /// LLVM 错误
    LlvmError(String),
    /// 文件 I/O 错误
    IoError(String),
    /// 链接错误
    LinkError(String),
}

impl fmt::Display for CodeGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodeGenError::Unsupported(msg) => write!(f, "Unsupported feature: {}", msg),
            CodeGenError::LoweringError(msg) => write!(f, "Lowering failed: {}", msg),
            CodeGenError::LlvmError(msg) => write!(f, "LLVM error: {}", msg),
            CodeGenError::IoError(msg) => write!(f, "I/O error: {}", msg),
            CodeGenError::LinkError(msg) => write!(f, "Link error: {}", msg),
        }
    }
}

/// 链接错误
#[derive(Debug, Clone, PartialEq)]
pub enum LinkError {
    /// 链接器未找到
    LinkerNotFound,
    /// 链接失败
    LinkFailed(String),
    /// 未定义符号
    UndefinedSymbol(String),
    /// 重复定义符号
    DuplicateSymbol(String),
}

impl fmt::Display for LinkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LinkError::LinkerNotFound => write!(f, "Linker not found"),
            LinkError::LinkFailed(msg) => write!(f, "Link failed: {}", msg),
            LinkError::UndefinedSymbol(s) => write!(f, "Undefined symbol: {}", s),
            LinkError::DuplicateSymbol(s) => write!(f, "Duplicate symbol: {}", s),
        }
    }
}

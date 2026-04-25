
// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq)]
pub enum FFIError {
    LanguageNotSupported(String),
    InvalidExternSyntax(String),
    SymbolNotFound(String),
    TypeMismatch(String),
    HeaderParseFailed(String),
    ModuleImportFailed(String),
}

impl fmt::Display for FFIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FFIError::LanguageNotSupported(lang) => write!(f, "不支持的语言: {}", lang),
            FFIError::InvalidExternSyntax(msg) => write!(f, "无效的外部语法: {}", msg),
            FFIError::SymbolNotFound(sym) => write!(f, "未找到符号: {}", sym),
            FFIError::TypeMismatch(msg) => write!(f, "类型不匹配: {}", msg),
            FFIError::HeaderParseFailed(msg) => write!(f, "头文件解析失败: {}", msg),
            FFIError::ModuleImportFailed(msg) => write!(f, "模块导入失败: {}", msg),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TranspileError {
    SyntaxError(String),
    TypeError(String),
    UnsupportedFeature(String),
    LanguageNotSupported(String),
    IOError(PathBuf, String),
}

impl fmt::Display for TranspileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TranspileError::SyntaxError(msg) => write!(f, "语法错误: {}", msg),
            TranspileError::TypeError(msg) => write!(f, "类型错误: {}", msg),
            TranspileError::UnsupportedFeature(msg) => write!(f, "不支持的功能: {}", msg),
            TranspileError::LanguageNotSupported(lang) => write!(f, "不支持的语言: {}", lang),
            TranspileError::IOError(path, msg) => write!(f, "IO错误 {:?}: {}", path, msg),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BindGenError {
    NoExportedItems,
    InvalidExportAttribute(String),
    LanguageNotSupported(String),
    TypeNotExportable(String),
    FileWriteError(PathBuf, String),
}

impl fmt::Display for BindGenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BindGenError::NoExportedItems => write!(f, "没有找到导出的项目"),
            BindGenError::InvalidExportAttribute(msg) => write!(f, "无效的导出属性: {}", msg),
            BindGenError::LanguageNotSupported(lang) => write!(f, "不支持的语言: {}", lang),
            BindGenError::TypeNotExportable(ty) => write!(f, "无法导出的类型: {}", ty),
            BindGenError::FileWriteError(path, msg) => write!(f, "写入文件失败 {:?}: {}", path, msg),
        }
    }
}

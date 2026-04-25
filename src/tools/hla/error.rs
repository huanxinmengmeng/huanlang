
// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use thiserror::Error;

/// HLA 解析错误
#[derive(Error, Debug, Clone, PartialEq)]
pub enum HlaParseError {
    #[error("未知操作码 '{opcode}' (行 {line})")]
    UnknownOpcode { opcode: String, line: usize },

    #[error("无效的操作数: {expected}, 实际为 {found} (行 {line})")]
    InvalidOperand { expected: String, found: String, line: usize },

    #[error("未定义的标签 '{label}' (行 {line})")]
    UndefinedLabel { label: String, line: usize },

    #[error("重复的标签 '{label}' (行 {line})")]
    DuplicateLabel { label: String, line: usize },

    #[error("类型不匹配: 期望 {expected:?}, 实际为 {found:?} (行 {line})")]
    TypeMismatch { expected: String, found: String, line: usize },

    #[error("无效的元数据: {message} (行 {line})")]
    InvalidMetadata { message: String, line: usize },

    #[error("I/O 错误: {0}")]
    IoError(String),

    #[error("其他错误: {0}")]
    Other(String),
}

/// HLA 序列化错误
#[derive(Error, Debug, Clone, PartialEq)]
pub enum HlaSerializeError {
    #[error("无效的 AST 结构: {0}")]
    InvalidAst(String),

    #[error("类型转换失败: {0}")]
    TypeConversion(String),

    #[error("I/O 错误: {0}")]
    IoError(String),
}

/// 结果类型别名
pub type ParseResult<T> = Result<T, Vec<HlaParseError>>;
pub type SerializeResult<T> = Result<T, HlaSerializeError>;

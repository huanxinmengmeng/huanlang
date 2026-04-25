// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::fmt;

#[derive(Debug)]
pub struct HuanError {
    pub message: String,
    pub kind: ErrorKind,
}

#[derive(Debug)]
pub enum ErrorKind {
    Lexer,
    Parser,
    Semantic,
    TypeCheck,
    CodeGen,
    Io,
    Other,
}

impl fmt::Display for HuanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{:?}] {}", self.kind, self.message)
    }
}

impl std::error::Error for HuanError {}

pub type Result<T> = std::result::Result<T, HuanError>;

impl HuanError {
    pub fn new(message: String, kind: ErrorKind) -> Self {
        Self { message, kind }
    }

    pub fn lexer(message: String) -> Self {
        Self { message, kind: ErrorKind::Lexer }
    }

    pub fn parser(message: String) -> Self {
        Self { message, kind: ErrorKind::Parser }
    }

    pub fn semantic(message: String) -> Self {
        Self { message, kind: ErrorKind::Semantic }
    }

    pub fn type_check(message: String) -> Self {
        Self { message, kind: ErrorKind::TypeCheck }
    }

    pub fn code_gen(message: String) -> Self {
        Self { message, kind: ErrorKind::CodeGen }
    }

    pub fn io(message: String) -> Self {
        Self { message, kind: ErrorKind::Io }
    }
}

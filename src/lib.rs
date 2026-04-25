// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

pub mod core;
pub mod backend;
pub mod stdlib;
pub mod tools;
pub mod utils;
pub mod interpreter;
pub mod lang_identity;
pub mod file_format;
pub mod test;

pub use core::lexer::Lexer;
pub use core::parser::Parser;
pub use core::sema::SemanticAnalyzer;
pub use utils::error::{HuanError, Result};
pub use interpreter::Interpreter;

pub use lang_identity::*;
pub use file_format::*;

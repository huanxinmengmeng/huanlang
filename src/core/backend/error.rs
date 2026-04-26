// Copyright © 2026 幻心梦梦 (huanxinmengmeng)
// Licensed under the Apache License, Version 2.0 (the "License");
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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

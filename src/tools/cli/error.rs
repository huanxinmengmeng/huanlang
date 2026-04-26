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

//! CLI 错误类型定义

/// CLI 错误类型
#[derive(Debug, Clone)]
pub enum CliError {
    /// 一般错误
    General(String),
    /// 命令行参数错误
    Argument(String),
    /// 配置文件错误
    Config(String),
    /// 编译错误
    Compile(String),
    /// 运行时错误
    Runtime(String),
    /// 文件操作错误
    Io(String),
    /// 未找到命令
    CommandNotFound(String),
    /// 不支持的特性
    Unsupported(String),
    /// 内部错误
    Internal(String),
}

impl CliError {
    /// 获取错误消息
    pub fn message(&self) -> &str {
        match self {
            CliError::General(msg) => msg,
            CliError::Argument(msg) => msg,
            CliError::Config(msg) => msg,
            CliError::Compile(msg) => msg,
            CliError::Runtime(msg) => msg,
            CliError::Io(msg) => msg,
            CliError::CommandNotFound(msg) => msg,
            CliError::Unsupported(msg) => msg,
            CliError::Internal(msg) => msg,
        }
    }

    /// 获取退出码
    pub fn exit_code(&self) -> i32 {
        match self {
            CliError::General(_) => 1,
            CliError::Argument(_) => 2,
            CliError::Config(_) => 3,
            CliError::Compile(_) => 1,
            CliError::Runtime(_) => 1,
            CliError::Io(_) => 1,
            CliError::CommandNotFound(_) => 127,
            CliError::Unsupported(_) => 1,
            CliError::Internal(_) => 101,
        }
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CliError::General(msg) => write!(f, "错误：{}", msg),
            CliError::Argument(msg) => write!(f, "命令行参数错误：{}", msg),
            CliError::Config(msg) => write!(f, "配置文件错误：{}", msg),
            CliError::Compile(msg) => write!(f, "编译错误：{}", msg),
            CliError::Runtime(msg) => write!(f, "运行时错误：{}", msg),
            CliError::Io(msg) => write!(f, "文件错误：{}", msg),
            CliError::CommandNotFound(cmd) => write!(f, "未找到命令：{}", cmd),
            CliError::Unsupported(msg) => write!(f, "不支持的特性：{}", msg),
            CliError::Internal(msg) => write!(f, "内部错误：{}", msg),
        }
    }
}

impl std::error::Error for CliError {}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::Io(err.to_string())
    }
}

impl From<std::path::PathBuf> for CliError {
    fn from(path: std::path::PathBuf) -> Self {
        CliError::Io(format!("路径错误：{:?}", path))
    }
}

/// CLI 结果类型
pub type CliResult<T> = Result<T, CliError>;

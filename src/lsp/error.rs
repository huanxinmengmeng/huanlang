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

//! LSP 错误类型定义

/// LSP 错误类型
#[derive(Debug, Clone)]
pub enum LspError {
    /// JSON-RPC 错误
    JsonRpc(String),
    /// 解析错误
    Parse(String),
    /// 文档未找到
    DocumentNotFound(String),
    /// 符号未找到
    SymbolNotFound(String),
    /// 内部错误
    Internal(String),
    /// 不支持的操作
    Unsupported(String),
    /// 请求被取消
    Cancelled,
}

impl std::fmt::Display for LspError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LspError::JsonRpc(msg) => write!(f, "JSON-RPC 错误：{}", msg),
            LspError::Parse(msg) => write!(f, "解析错误：{}", msg),
            LspError::DocumentNotFound(uri) => write!(f, "文档未找到：{}", uri),
            LspError::SymbolNotFound(name) => write!(f, "符号未找到：{}", name),
            LspError::Internal(msg) => write!(f, "内部错误：{}", msg),
            LspError::Unsupported(msg) => write!(f, "不支持的操作：{}", msg),
            LspError::Cancelled => write!(f, "请求被取消"),
        }
    }
}

impl std::error::Error for LspError {}

impl serde::Serialize for LspError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

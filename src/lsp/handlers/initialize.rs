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

//! 初始化请求处理器

use crate::lsp::{InitializeParams, ServerCapabilities};

/// 初始化请求处理器
pub struct InitializeHandler;

impl InitializeHandler {
    /// 处理初始化请求
    pub fn handle(_params: InitializeParams) -> InitializeResult {
        // 创建服务器能力集
        let capabilities = ServerCapabilities::default();
        
        InitializeResult {
            server_info: ServerInfo {
                name: "幻语 LSP".to_string(),
                version: "0.0.1".to_string(),
            },
            capabilities,
        }
    }
}

/// 初始化结果
#[derive(Debug, Clone)]
pub struct InitializeResult {
    pub server_info: ServerInfo,
    pub capabilities: ServerCapabilities,
}

/// 服务器信息
#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
}

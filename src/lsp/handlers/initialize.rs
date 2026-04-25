// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 初始化请求处理器

use crate::lsp::{InitializeParams, ServerCapabilities};

/// 初始化请求处理器
pub struct InitializeHandler;

impl InitializeHandler {
    /// 处理初始化请求
    pub fn handle(params: InitializeParams) -> InitializeResult {
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

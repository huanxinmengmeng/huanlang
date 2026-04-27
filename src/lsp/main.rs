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

//! LSP 服务器主入口

use std::io::{stdin, stdout, Write, Read};
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::{self, Value};

use crate::lsp::{LspServer, ServerCapabilities};

/// LSP 消息类型
#[derive(Debug, Deserialize, Serialize)]
pub struct LspMessage {
    pub jsonrpc: String,
    pub id: Option<serde_json::Value>,
    pub method: Option<String>,
    pub params: Option<Value>,
    pub result: Option<Value>,
    pub error: Option<Value>,
}

/// 初始化响应
#[derive(Debug, Serialize)]
pub struct InitializeResult {
    pub capabilities: ServerCapabilities,
}

/// 启动 LSP 服务器
pub fn start_server() {
    let server = LspServer::new();
    let server = Arc::new(std::sync::Mutex::new(server));
    
    println!("幻语 LSP 服务器已启动，等待客户端连接...");
    
    loop {
        // 读取消息
        let message = match read_message() {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("读取消息失败: {}", e);
                continue;
            }
        };
        
        // 处理消息
        handle_message(&server, message);
    }
}

/// 读取 LSP 消息
fn read_message() -> Result<LspMessage, std::io::Error> {
    let mut header = String::new();
    let mut stdin = stdin();
    
    // 读取头部
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        header.push_str(&line);
        
        if line.trim().is_empty() {
            break;
        }
    }
    
    // 解析 Content-Length
    let content_length = header
        .lines()
        .find(|line| line.starts_with("Content-Length:"))
        .and_then(|line| line.split(":").nth(1))
        .and_then(|len| len.trim().parse::<usize>().ok())
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid Content-Length"))?;
    
    // 读取消息体
    let mut body = vec![0; content_length];
    stdin.read_exact(&mut body)?;
    
    // 解析 JSON
    let message: LspMessage = serde_json::from_slice(&body)?;
    Ok(message)
}

/// 发送 LSP 消息
fn send_message(message: &LspMessage) -> Result<(), std::io::Error> {
    let json = serde_json::to_string(message)?;
    let content_length = json.len();
    
    let mut stdout = stdout();
    writeln!(stdout, "Content-Length: {}", content_length)?;
    writeln!(stdout)?;
    writeln!(stdout, "{}", json)?;
    stdout.flush()?;
    
    Ok(())
}

/// 处理 LSP 消息
fn handle_message(server: &Arc<std::sync::Mutex<LspServer>>, message: LspMessage) {
    match message.method.as_deref() {
        Some("initialize") => handle_initialize(server, message),
        Some("textDocument/didOpen") => handle_did_open(server, message),
        Some("textDocument/didChange") => handle_did_change(server, message),
        Some("textDocument/didClose") => handle_did_close(server, message),
        Some("textDocument/completion") => handle_completion(server, message),
        Some("textDocument/hover") => handle_hover(server, message),
        Some("shutdown") => handle_shutdown(server, message),
        Some("exit") => handle_exit(server, message),
        _ => eprintln!("未知方法: {:?}", message.method),
    }
}

/// 处理初始化请求
fn handle_initialize(server: &Arc<std::sync::Mutex<LspServer>>, message: LspMessage) {
    let mut server = server.lock().unwrap();
    server.initialize();
    
    let result = InitializeResult {
        capabilities: ServerCapabilities::default(),
    };
    
    let response = LspMessage {
        jsonrpc: "2.0".to_string(),
        id: message.id,
        method: None,
        params: None,
        result: Some(serde_json::to_value(result).unwrap()),
        error: None,
    };
    
    if let Err(e) = send_message(&response) {
        eprintln!("发送初始化响应失败: {}", e);
    }
}

/// 处理文档打开
fn handle_did_open(server: &Arc<std::sync::Mutex<LspServer>>, message: LspMessage) {
    if let Some(params) = message.params {
        if let Ok(did_open) = serde_json::from_value::<DidOpenParams>(params) {
            let mut server = server.lock().unwrap();
            server.text_document_did_open(
                did_open.text_document.uri,
                did_open.text_document.text,
            );
        }
    }
}

/// 处理文档变更
fn handle_did_change(server: &Arc<std::sync::Mutex<LspServer>>, message: LspMessage) {
    if let Some(params) = message.params {
        if let Ok(did_change) = serde_json::from_value::<DidChangeParams>(params) {
            let mut server = server.lock().unwrap();
            server.text_document_did_change(
                &did_change.text_document.uri,
                did_change.content_changes[0].text.clone(),
            );
        }
    }
}

/// 处理文档关闭
fn handle_did_close(server: &Arc<std::sync::Mutex<LspServer>>, message: LspMessage) {
    if let Some(params) = message.params {
        if let Ok(did_close) = serde_json::from_value::<DidCloseParams>(params) {
            let mut server = server.lock().unwrap();
            server.text_document_did_close(&did_close.text_document.uri);
        }
    }
}

/// 处理补全请求
fn handle_completion(server: &Arc<std::sync::Mutex<LspServer>>, message: LspMessage) {
    if let Some(params) = message.params {
        if let Ok(completion_params) = serde_json::from_value::<CompletionParams>(params) {
            let server = server.lock().unwrap();
            let items = server.completion(
                &completion_params.text_document.uri,
                completion_params.position,
            );
            
            let result = CompletionResult {
                is_incomplete: false,
                items,
            };
            
            let response = LspMessage {
                jsonrpc: "2.0".to_string(),
                id: message.id,
                method: None,
                params: None,
                result: Some(serde_json::to_value(result).unwrap()),
                error: None,
            };
            
            if let Err(e) = send_message(&response) {
                eprintln!("发送补全响应失败: {}", e);
            }
        }
    }
}

/// 处理悬停请求
fn handle_hover(server: &Arc<std::sync::Mutex<LspServer>>, message: LspMessage) {
    if let Some(params) = message.params {
        if let Ok(hover_params) = serde_json::from_value::<HoverParams>(params) {
            let server = server.lock().unwrap();
            let hover = server.hover(
                &hover_params.text_document.uri,
                hover_params.position,
            );
            
            let response = LspMessage {
                jsonrpc: "2.0".to_string(),
                id: message.id,
                method: None,
                params: None,
                result: hover.map(|h| serde_json::to_value(h).unwrap()),
                error: None,
            };
            
            if let Err(e) = send_message(&response) {
                eprintln!("发送悬停响应失败: {}", e);
            }
        }
    }
}

/// 处理关闭请求
fn handle_shutdown(server: &Arc<std::sync::Mutex<LspServer>>, message: LspMessage) {
    let mut server = server.lock().unwrap();
    server.shutdown();
    
    let response = LspMessage {
        jsonrpc: "2.0".to_string(),
        id: message.id,
        method: None,
        params: None,
        result: Some(Value::Null),
        error: None,
    };
    
    if let Err(e) = send_message(&response) {
        eprintln!("发送关闭响应失败: {}", e);
    }
}

/// 处理退出请求
fn handle_exit(_server: &Arc<std::sync::Mutex<LspServer>>, _message: LspMessage) {
    std::process::exit(0);
}

/// 文档打开参数
#[derive(Debug, Deserialize)]
pub struct DidOpenParams {
    pub text_document: TextDocumentItem,
}

/// 文档项
#[derive(Debug, Deserialize)]
pub struct TextDocumentItem {
    pub uri: String,
    pub language_id: String,
    pub version: i32,
    pub text: String,
}

/// 文档变更参数
#[derive(Debug, Deserialize)]
pub struct DidChangeParams {
    pub text_document: VersionedTextDocumentIdentifier,
    pub content_changes: Vec<TextDocumentContentChangeEvent>,
}

/// 版本化文档标识符
#[derive(Debug, Deserialize)]
pub struct VersionedTextDocumentIdentifier {
    pub uri: String,
    pub version: i32,
}

/// 文档内容变更事件
#[derive(Debug, Deserialize)]
pub struct TextDocumentContentChangeEvent {
    pub range: Option<Value>,
    pub range_length: Option<u32>,
    pub text: String,
}

/// 文档关闭参数
#[derive(Debug, Deserialize)]
pub struct DidCloseParams {
    pub text_document: TextDocumentIdentifier,
}

/// 文档标识符
#[derive(Debug, Deserialize)]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

/// 补全参数
#[derive(Debug, Deserialize)]
pub struct CompletionParams {
    pub text_document: TextDocumentIdentifier,
    pub position: crate::lsp::Position,
}

/// 补全结果
#[derive(Debug, Serialize)]
pub struct CompletionResult {
    pub is_incomplete: bool,
    pub items: Vec<crate::lsp::handlers::completion::CompletionItem>,
}

/// 悬停参数
#[derive(Debug, Deserialize)]
pub struct HoverParams {
    pub text_document: TextDocumentIdentifier,
    pub position: crate::lsp::Position,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_lsp_message_serialization() {
        let message = LspMessage {
            jsonrpc: "2.0".to_string(),
            id: Some(serde_json::Value::Number(1.into())),
            method: Some("initialize".to_string()),
            params: Some(serde_json::Value::Object(serde_json::Map::new())),
            result: None,
            error: None,
        };
        
        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("initialize"));
    }
    
    #[test]
    fn test_initialize_result() {
        let result = InitializeResult {
            capabilities: ServerCapabilities::default(),
        };
        
        let json = serde_json::to_string(&result).unwrap();
        assert!(json.contains("capabilities"));
    }
}
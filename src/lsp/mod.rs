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

//! 幻语语言服务器（LSP）实现模块
//!
//! 本模块提供了完整的语言服务器协议（LSP）实现，包括：
//! - 标准 LSP 功能（补全、跳转、悬停、诊断等）
//! - 语义标记（Semantic Tokens）
//! - 自定义扩展（AI 代码生成、AEF 转换、关键词风格转换等）
//!
//! ## 模块结构
//!
//! ```text
//! lsp_server/
//! ├── main.rs                 # 入口
//! ├── server.rs               # LSP 协议处理主循环
//! ├── handlers/               # 请求处理器
//! │   ├── initialize.rs
//! │   ├── completion.rs
//! │   ├── hover.rs
//! │   ├── definition.rs
//! │   └── ...
//! ├── analysis/               # 语义分析
//! │   ├── document.rs         # 文档状态管理
//! │   ├── symbol_table.rs     # 符号表索引
//! │   ├── type_checker.rs     # 增量类型检查
//! │   └── diagnostics.rs      # 诊断生成
//! ├── ai/                     # AI 功能
//! │   ├── generator.rs        # 代码生成
//! │   └── explainer.rs        # 代码解释
//! └── utils/                  # 工具函数
//! ```

pub mod server;
pub mod document;
pub mod symbol_table;
pub mod handlers;
pub mod diagnostics;
pub mod semantic_tokens;
pub mod extensions;
pub mod error;
pub mod main;

pub use server::LspServer;
pub use document::Document;
pub use symbol_table::{SymbolTable, Symbol, SymbolKind, Location};
pub use diagnostics::Diagnostic;
pub use error::LspError;
pub use main::{start_server, LspMessage};

use serde::{Serialize, Deserialize};

/// LSP 位置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub line: u32,
    pub character: u32,
}

impl Position {
    pub fn new(line: u32, character: u32) -> Self {
        Position { line, character }
    }
}

/// LSP 范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

impl Range {
    pub fn new(start: Position, end: Position) -> Self {
        Range { start, end }
    }
}

/// LSP 位置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Location {
    pub uri: String,
    pub range: Range,
}

impl Location {
    pub fn new(uri: String, range: Range) -> Self {
        Location { uri, range }
    }
}

/// 文本文档标识符
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

impl TextDocumentIdentifier {
    pub fn new(uri: String) -> Self {
        TextDocumentIdentifier { uri }
    }
}

/// 初始化参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeParams {
    pub process_id: Option<u32>,
    pub root_uri: Option<String>,
    pub capabilities: ClientCapabilities,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ClientCapabilities {
    pub text_document: Option<TextDocumentClientCapabilities>,
    pub workspace: Option<WorkspaceClientCapabilities>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextDocumentClientCapabilities {
    pub synchronization: Option<SynchronizationCapabilities>,
    pub completion: Option<CompletionCapabilities>,
    pub hover: Option<HoverCapabilities>,
    pub definition: Option<DefinitionCapabilities>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceClientCapabilities {
    pub workspace_folders: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SynchronizationCapabilities {
    pub will_save: Option<bool>,
    pub did_save: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompletionCapabilities {
    pub completion_item: Option<CompletionItemCapabilities>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CompletionItemCapabilities {
    pub snippet_support: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct HoverCapabilities {
    pub dynamic_registration: Option<bool>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DefinitionCapabilities {
    pub dynamic_registration: Option<bool>,
}

/// 服务器能力
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub text_document_sync: TextDocumentSyncOptions,
    pub completion_provider: Option<CompletionOptions>,
    pub hover_provider: Option<bool>,
    pub definition_provider: Option<bool>,
    pub references_provider: Option<bool>,
    pub document_highlight_provider: Option<bool>,
    pub document_symbol_provider: Option<bool>,
    pub workspace_symbol_provider: Option<bool>,
    pub code_action_provider: Option<CodeActionOptions>,
    pub code_lens_provider: Option<CodeLensOptions>,
    pub document_formatting_provider: Option<bool>,
    pub document_range_formatting_provider: Option<bool>,
    pub rename_provider: Option<RenameOptions>,
    pub signature_help_provider: Option<SignatureHelpOptions>,
    pub semantic_tokens_provider: Option<SemanticTokensOptions>,
    pub workspace: WorkspaceCapabilities,
}

impl Default for ServerCapabilities {
    fn default() -> Self {
        ServerCapabilities {
            text_document_sync: TextDocumentSyncOptions::default(),
            completion_provider: Some(CompletionOptions::default()),
            hover_provider: Some(true),
            definition_provider: Some(true),
            references_provider: Some(true),
            document_highlight_provider: Some(true),
            document_symbol_provider: Some(true),
            workspace_symbol_provider: Some(true),
            code_action_provider: Some(CodeActionOptions::default()),
            code_lens_provider: Some(CodeLensOptions::default()),
            document_formatting_provider: Some(true),
            document_range_formatting_provider: Some(true),
            rename_provider: Some(RenameOptions::default()),
            signature_help_provider: Some(SignatureHelpOptions::default()),
            semantic_tokens_provider: Some(SemanticTokensOptions::default()),
            workspace: WorkspaceCapabilities::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextDocumentSyncOptions {
    pub open_close: bool,
    pub change: i32,
    pub save: Option<SaveOptions>,
}

impl Default for TextDocumentSyncOptions {
    fn default() -> Self {
        TextDocumentSyncOptions {
            open_close: true,
            change: 2,
            save: Some(SaveOptions { include_text: true }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveOptions {
    pub include_text: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletionOptions {
    pub trigger_characters: Option<Vec<String>>,
    pub resolve_provider: Option<bool>,
}

impl Default for CompletionOptions {
    fn default() -> Self {
        CompletionOptions {
            trigger_characters: Some(vec![
                "令".to_string(), "let".to_string(), " ".to_string(),
                ".".to_string(), ":".to_string(), "(".to_string(), "{".to_string()
            ]),
            resolve_provider: Some(true),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeActionOptions {
    pub code_action_kinds: Option<Vec<String>>,
}

impl Default for CodeActionOptions {
    fn default() -> Self {
        CodeActionOptions {
            code_action_kinds: Some(vec![
                "quickfix".to_string(),
                "refactor".to_string(),
                "source".to_string()
            ]),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLensOptions {
    pub resolve_provider: Option<bool>,
}

impl Default for CodeLensOptions {
    fn default() -> Self {
        CodeLensOptions { resolve_provider: Some(true) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenameOptions {
    pub prepare_provider: Option<bool>,
}

impl Default for RenameOptions {
    fn default() -> Self {
        RenameOptions { prepare_provider: Some(true) }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureHelpOptions {
    pub trigger_characters: Option<Vec<String>>,
}

impl Default for SignatureHelpOptions {
    fn default() -> Self {
        SignatureHelpOptions {
            trigger_characters: Some(vec!["(".to_string(), ",".to_string()]),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticTokensOptions {
    pub legend: SemanticTokensLegend,
    pub full: Option<SemanticTokensFullOptions>,
    pub range: Option<bool>,
}

impl Default for SemanticTokensOptions {
    fn default() -> Self {
        SemanticTokensOptions {
            legend: SemanticTokensLegend::default(),
            full: Some(SemanticTokensFullOptions { delta: Some(true) }),
            range: Some(true),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticTokensLegend {
    pub token_types: Vec<String>,
    pub token_modifiers: Vec<String>,
}

impl Default for SemanticTokensLegend {
    fn default() -> Self {
        SemanticTokensLegend {
            token_types: vec![
                "keyword".to_string(),
                "type".to_string(),
                "function".to_string(),
                "variable".to_string(),
                "parameter".to_string(),
                "string".to_string(),
                "number".to_string(),
                "comment".to_string(),
                "operator".to_string(),
                "property".to_string(),
            ],
            token_modifiers: vec![
                "declaration".to_string(),
                "definition".to_string(),
                "readonly".to_string(),
                "static".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticTokensFullOptions {
    pub delta: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceCapabilities {
    pub workspace_folders: WorkspaceFoldersCapabilities,
}

impl Default for WorkspaceCapabilities {
    fn default() -> Self {
        WorkspaceCapabilities {
            workspace_folders: WorkspaceFoldersCapabilities {
                supported: Some(true),
                change_notifications: Some(true),
            },
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceFoldersCapabilities {
    pub supported: Option<bool>,
    pub change_notifications: Option<bool>,
}

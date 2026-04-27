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

//! LSP 服务器主模块



/// LSP 服务器
pub struct LspServer {
    /// 文档管理器
    documents: crate::lsp::document::DocumentManager,
    /// 工作区索引
    workspace_index: crate::lsp::symbol_table::WorkspaceIndex,
    /// 是否已初始化
    initialized: bool,
}

impl LspServer {
    /// 创建新的 LSP 服务器
    pub fn new() -> Self {
        LspServer {
            documents: crate::lsp::document::DocumentManager::new(),
            workspace_index: crate::lsp::symbol_table::WorkspaceIndex::new(),
            initialized: false,
        }
    }

    /// 初始化服务器
    pub fn initialize(&mut self) {
        self.initialized = true;
    }

    /// 关闭服务器
    pub fn shutdown(&mut self) {
        self.initialized = false;
        self.documents.clear();
        self.workspace_index.clear();
    }

    /// 检查是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// 获取文档管理器
    pub fn documents(&self) -> &crate::lsp::document::DocumentManager {
        &self.documents
    }

    /// 获取可变文档管理器
    pub fn documents_mut(&mut self) -> &mut crate::lsp::document::DocumentManager {
        &mut self.documents
    }

    /// 获取工作区索引
    pub fn workspace_index(&self) -> &crate::lsp::symbol_table::WorkspaceIndex {
        &self.workspace_index
    }

    /// 获取可变工作区索引
    pub fn workspace_index_mut(&mut self) -> &mut crate::lsp::symbol_table::WorkspaceIndex {
        &mut self.workspace_index
    }

    /// 处理文本文档打开
    pub fn text_document_did_open(&mut self, uri: String, content: String) {
        self.documents.open_document(uri, content);
    }

    /// 处理文本文档变更
    pub fn text_document_did_change(&mut self, uri: &str, content: String) {
        self.documents.update_document(uri, content);
    }

    /// 处理文本文档关闭
    pub fn text_document_did_close(&mut self, uri: &str) {
        self.documents.close_document(uri);
    }

    /// 处理补全请求
    pub fn completion(&self, uri: &str, position: crate::lsp::Position) -> Vec<crate::lsp::handlers::completion::CompletionItem> {
        // 获取当前文档
        if let Some(doc) = self.documents.get_document(uri) {
            // 获取光标处的词语
            if let Some(_word) = doc.get_word_at(&position) {
                let result = crate::lsp::handlers::completion::CompletionHandler::handle(
                    crate::lsp::handlers::completion::CompletionParams {
                        text_document: crate::lsp::handlers::completion::TextDocumentIdentifier::new(uri.to_string()),
                        position,
                        trigger_character: None,
                        context: None,
                    }
                );
                return result.items;
            }
        }
        
        // 返回默认补全
        let result = crate::lsp::handlers::completion::CompletionHandler::handle(
            crate::lsp::handlers::completion::CompletionParams {
                text_document: crate::lsp::handlers::completion::TextDocumentIdentifier::new(uri.to_string()),
                position,
                trigger_character: None,
                context: None,
            }
        );
        result.items
    }

    /// 处理悬停请求
    pub fn hover(&self, uri: &str, position: crate::lsp::Position) -> Option<crate::lsp::handlers::hover::Hover> {
        // 获取当前文档
        if let Some(doc) = self.documents.get_document(uri) {
            // 获取光标处的词语
            if let Some(word) = doc.get_word_at(&position) {
                return crate::lsp::handlers::hover::HoverHandler::handle(uri, position, &word);
            }
        }
        
        None
    }
}

impl Default for LspServer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_creation() {
        let server = LspServer::new();
        assert!(!server.is_initialized());
        assert_eq!(server.documents().num_documents(), 0);
    }

    #[test]
    fn test_initialize() {
        let mut server = LspServer::new();
        server.initialize();
        assert!(server.is_initialized());
    }

    #[test]
    fn test_text_document_operations() {
        let mut server = LspServer::new();
        
        server.text_document_did_open(
            "file:///test.hl".to_string(),
            "令 x 为 42".to_string(),
        );
        
        assert_eq!(server.documents().num_documents(), 1);
        assert!(server.documents().get_document("file:///test.hl").is_some());
        
        server.text_document_did_close("file:///test.hl");
        assert_eq!(server.documents().num_documents(), 0);
    }
}

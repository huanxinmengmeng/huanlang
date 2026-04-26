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

//! 文档状态管理模块
//!
//! 本模块负责管理每个打开文档的状态，包括：
//! - 文档内容（Rope 文本缓冲区）
//! - 版本号
//! - 语法树（AST）
//! - 符号表
//! - 诊断信息
//! - 最后修改时间

use std::collections::HashMap;
use std::time::SystemTime;
use ropey::Rope;
use crate::lsp::{Position, Range, Location};

/// 文档结构
#[derive(Debug, Clone)]
pub struct Document {
    /// 文档 URI
    pub uri: String,
    /// 文档版本号
    pub version: i32,
    /// 文档内容（Rope 文本缓冲区，支持高效的增量更新）
    pub content: Rope,
    /// 语法树（AST）
    pub ast: Option<crate::core::ast::Program>,
    /// 文档符号列表
    pub symbols: Vec<DocumentSymbol>,
    /// 诊断信息
    pub diagnostics: Vec<crate::lsp::diagnostics::Diagnostic>,
    /// 最后修改时间
    pub last_modified: SystemTime,
    /// 依赖的其他文档
    pub dependencies: Vec<String>,
}

impl Document {
    /// 创建新文档
    pub fn new(uri: String, content: String) -> Self {
        Document {
            uri,
            version: 1,
            content: Rope::from_str(&content),
            ast: None,
            symbols: Vec::new(),
            diagnostics: Vec::new(),
            last_modified: SystemTime::now(),
            dependencies: Vec::new(),
        }
    }

    /// 更新文档内容
    pub fn update_content(&mut self, content: String) {
        self.content = Rope::from_str(&content);
        self.version += 1;
        self.last_modified = SystemTime::now();
    }

    /// 获取行数
    pub fn num_lines(&self) -> usize {
        self.content.len_lines()
    }

    /// 获取文档文本
    pub fn text(&self) -> String {
        self.content.to_string()
    }

    /// 获取指定范围的文本
    pub fn get_text(&self, range: &Range) -> Option<String> {
        let start_pos = self.position_to_offset(&range.start)?;
        let end_pos = self.position_to_offset(&range.end)?;
        self.content.get_slice(start_pos..end_pos).map(|s| s.to_string())
    }

    /// 将 Position 转换为字节偏移
    pub fn position_to_offset(&self, pos: &Position) -> Option<usize> {
        let line_idx = pos.line as usize;
        let char_idx = pos.character as usize;
        
        if line_idx >= self.num_lines() {
            return None;
        }
        
        let line_start = self.content.line_to_char(line_idx);
        let line = self.content.line(line_idx);
        
        if char_idx > line.len_chars() {
            return None;
        }
        
        Some(line_start + char_idx)
    }

    /// 将字节偏移转换为 Position
    pub fn offset_to_position(&self, offset: usize) -> Position {
        let offset = offset.min(self.content.len_chars());
        let line_idx = self.content.char_to_line(offset);
        let line_start = self.content.line_to_char(line_idx);
        let char_idx = offset - line_start;
        
        Position::new(line_idx as u32, char_idx as u32)
    }

    /// 获取光标处的词语
    pub fn get_word_at(&self, pos: &Position) -> Option<String> {
        let offset = self.position_to_offset(pos)?;
        let line_idx = pos.line as usize;
        let line = self.content.line(line_idx);
        
        let line_start = self.content.line_to_char(line_idx);
        let char_idx = offset - line_start;
        
        let chars: Vec<char> = line.chars().collect();
        if char_idx >= chars.len() {
            return None;
        }
        
        // 找到词语边界
        let mut start = char_idx;
        let mut end = char_idx;
        
        // 向左查找词语开始
        while start > 0 && Self::is_identifier_char(chars[start - 1]) {
            start -= 1;
        }
        
        // 向右查找词语结束
        while end < chars.len() && Self::is_identifier_char(chars[end]) {
            end += 1;
        }
        
        if start == end {
            return None;
        }
        
        Some(chars[start..end].iter().collect())
    }

    /// 判断字符是否为标识符字符
    fn is_identifier_char(c: char) -> bool {
        c.is_alphanumeric() || c == '_' || Self::is_cjk(c)
    }

    /// 判断是否为中日韩字符
    fn is_cjk(c: char) -> bool {
        let code = c as u32;
        (0x4E00..=0x9FFF).contains(&code) ||  // CJK Unified Ideographs
        (0x3400..=0x4DBF).contains(&code) ||  // CJK Unified Ideographs Extension A
        (0x20000..=0x2A6DF).contains(&code)    // CJK Unified Ideographs Extension B
    }

    /// 设置语法树
    pub fn set_ast(&mut self, ast: crate::core::ast::Program) {
        self.ast = Some(ast);
    }

    /// 设置诊断信息
    pub fn set_diagnostics(&mut self, diagnostics: Vec<crate::lsp::diagnostics::Diagnostic>) {
        self.diagnostics = diagnostics;
    }

    /// 添加诊断信息
    pub fn add_diagnostic(&mut self, diagnostic: crate::lsp::diagnostics::Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// 清空诊断信息
    pub fn clear_diagnostics(&mut self) {
        self.diagnostics.clear();
    }

    /// 检查文档是否有错误
    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| d.severity == crate::lsp::diagnostics::DiagnosticSeverity::Error)
    }
}

/// 文档符号
#[derive(Debug, Clone)]
pub struct DocumentSymbol {
    /// 符号名称
    pub name: String,
    /// 符号详情
    pub detail: Option<String>,
    /// 符号类型
    pub kind: SymbolKind,
    /// 符号位置
    pub location: Location,
    /// 符号范围（包含所有声明）
    pub range: Range,
    /// 子符号
    pub children: Vec<DocumentSymbol>,
    /// 标签
    pub tags: Vec<SymbolTag>,
}

/// 符号类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    File,
    Module,
    Function,
    Variable,
    Parameter,
    Type,
    Struct,
    Enum,
    Interface,
    Method,
    Property,
    Field,
    Constructor,
}

impl Default for SymbolKind {
    fn default() -> Self {
        SymbolKind::Variable
    }
}

/// 符号标签
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolTag {
    Deprecated,
    Readonly,
    Optional,
}

impl Default for SymbolTag {
    fn default() -> Self {
        SymbolTag::Deprecated
    }
}

/// 文档管理器
#[derive(Debug, Clone)]
pub struct DocumentManager {
    /// 所有打开的文档
    documents: HashMap<String, Document>,
    /// 当前活动的文档
    active_document: Option<String>,
}

impl DocumentManager {
    /// 创建新的文档管理器
    pub fn new() -> Self {
        DocumentManager {
            documents: HashMap::new(),
            active_document: None,
        }
    }

    /// 打开文档
    pub fn open_document(&mut self, uri: String, content: String) -> &mut Document {
        let doc = Document::new(uri.clone(), content);
        self.documents.insert(uri.clone(), doc);
        self.active_document = Some(uri);
        self.documents.get_mut(&uri.clone()).unwrap()
    }

    /// 关闭文档
    pub fn close_document(&mut self, uri: &str) -> Option<Document> {
        self.documents.remove(uri)
    }

    /// 获取文档
    pub fn get_document(&self, uri: &str) -> Option<&Document> {
        self.documents.get(uri)
    }

    /// 获取可变文档
    pub fn get_document_mut(&mut self, uri: &str) -> Option<&mut Document> {
        self.documents.get_mut(uri)
    }

    /// 获取所有文档
    pub fn all_documents(&self) -> &HashMap<String, Document> {
        &self.documents
    }

    /// 获取活动文档
    pub fn get_active_document(&self) -> Option<&Document> {
        self.active_document.as_ref().and_then(|uri| self.documents.get(uri))
    }

    /// 获取可变活动文档
    pub fn get_active_document_mut(&mut self) -> Option<&mut Document> {
        self.active_document.as_ref().and_then(|uri| self.documents.get_mut(uri))
    }

    /// 设置活动文档
    pub fn set_active_document(&mut self, uri: String) {
        if self.documents.contains_key(&uri) {
            self.active_document = Some(uri);
        }
    }

    /// 更新文档内容
    pub fn update_document(&mut self, uri: &str, content: String) -> Option<&mut Document> {
        if let Some(doc) = self.documents.get_mut(uri) {
            doc.update_content(content);
            Some(doc)
        } else {
            None
        }
    }

    /// 获取文档数量
    pub fn num_documents(&self) -> usize {
        self.documents.len()
    }

    /// 清空所有文档
    pub fn clear(&mut self) {
        self.documents.clear();
        self.active_document = None;
    }
}

impl Default for DocumentManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let doc = Document::new(
            "file:///test.hl".to_string(),
            "令 x 为 42".to_string()
        );
        assert_eq!(doc.uri, "file:///test.hl");
        assert_eq!(doc.version, 1);
        assert_eq!(doc.num_lines(), 1);
    }

    #[test]
    fn test_position_offset_conversion() {
        let doc = Document::new(
            "file:///test.hl".to_string(),
            "令 x 为 42\n令 y 为 10".to_string()
        );
        
        let pos = Position::new(1, 4);
        let offset = doc.position_to_offset(&pos);
        assert!(offset.is_some());
        
        let converted_pos = doc.offset_to_position(offset.unwrap());
        assert_eq!(converted_pos.line, pos.line);
        assert_eq!(converted_pos.character, pos.character);
    }

    #[test]
    fn test_get_word_at() {
        let doc = Document::new(
            "file:///test.hl".to_string(),
            "令 年龄 为 25".to_string()
        );
        
        let pos = Position::new(0, 3);
        let word = doc.get_word_at(&pos);
        assert!(word.is_some());
        assert_eq!(word.unwrap(), "年龄");
    }

    #[test]
    fn test_document_manager() {
        let mut manager = DocumentManager::new();
        
        manager.open_document("file:///test1.hl".to_string(), "令 x 为 1".to_string());
        manager.open_document("file:///test2.hl".to_string(), "令 y 为 2".to_string());
        
        assert_eq!(manager.num_documents(), 2);
        
        let doc = manager.get_document("file:///test1.hl");
        assert!(doc.is_some());
        
        manager.close_document("file:///test1.hl");
        assert_eq!(manager.num_documents(), 1);
    }
}

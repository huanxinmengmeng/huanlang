// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 符号表和工作区索引模块
//!
//! 本模块负责：
//! - 维护每个文档的符号表
//! - 维护工作区级别的符号索引
//! - 支持符号查找、引用、定义定位

use std::collections::{HashMap, HashSet};
use crate::lsp::{Location, Range, Position};

/// 符号定义
#[derive(Debug, Clone)]
pub struct Symbol {
    /// 符号名称
    pub name: String,
    /// 符号类型
    pub kind: SymbolKind,
    /// 定义位置
    pub location: Location,
    /// 符号范围
    pub range: Range,
    /// 可见性
    pub visibility: Visibility,
    /// 符号类型（用于变量、函数等）
    pub type_info: Option<String>,
    /// 文档字符串
    pub documentation: Option<String>,
    /// 依赖的模块
    pub dependencies: Vec<String>,
}

impl Symbol {
    /// 创建新的符号
    pub fn new(
        name: String,
        kind: SymbolKind,
        uri: String,
        range: Range,
    ) -> Self {
        Symbol {
            name,
            kind,
            location: Location::new(uri, range.clone()),
            range,
            visibility: Visibility::default(),
            type_info: None,
            documentation: None,
            dependencies: Vec::new(),
        }
    }

    /// 设置符号类型
    pub fn with_type(mut self, type_info: String) -> Self {
        self.type_info = Some(type_info);
        self
    }

    /// 设置文档
    pub fn with_docs(mut self, docs: String) -> Self {
        self.documentation = Some(docs);
        self
    }
}

/// 符号类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    Import,
}

impl Default for SymbolKind {
    fn default() -> Self {
        SymbolKind::Variable
    }
}

/// 符号可见性
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Visibility {
    /// 公共符号
    Public,
    /// 私有符号
    Private,
    /// 模块级私有
    #[default]
    Module,
}

/// 符号位置
#[derive(Debug, Clone)]
pub struct SymbolLocation {
    /// 符号名称
    pub name: String,
    /// 位置
    pub location: Location,
    /// 符号定义位置
    pub definition: Option<Location>,
}

impl SymbolLocation {
    /// 创建新的符号位置
    pub fn new(name: String, location: Location) -> Self {
        SymbolLocation {
            name,
            location,
            definition: None,
        }
    }
}

/// 符号表
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// 符号名 → 符号列表
    symbols: HashMap<String, Vec<Symbol>>,
    /// 当前作用域的符号
    current_scope: Vec<HashSet<String>>,
}

impl SymbolTable {
    /// 创建新的符号表
    pub fn new() -> Self {
        SymbolTable {
            symbols: HashMap::new(),
            current_scope: vec![HashSet::new()],
        }
    }

    /// 进入新作用域
    pub fn enter_scope(&mut self) {
        self.current_scope.push(HashSet::new());
    }

    /// 退出当前作用域
    pub fn exit_scope(&mut self) {
        if self.current_scope.len() > 1 {
            self.current_scope.pop();
        }
    }

    /// 添加符号
    pub fn add_symbol(&mut self, symbol: Symbol) {
        let name = symbol.name.clone();
        
        // 添加到当前作用域
        if let Some(scope) = self.current_scope.last_mut() {
            scope.insert(name.clone());
        }
        
        // 添加到符号映射
        self.symbols
            .entry(name)
            .or_insert_with(Vec::new)
            .push(symbol);
    }

    /// 查找符号
    pub fn find_symbol(&self, name: &str) -> Option<&Symbol> {
        self.symbols.get(name).and_then(|symbols| symbols.first())
    }

    /// 查找所有同名符号
    pub fn find_all_symbols(&self, name: &str) -> Option<&Vec<Symbol>> {
        self.symbols.get(name)
    }

    /// 查找定义
    pub fn find_definition(&self, name: &str) -> Option<&Symbol> {
        self.find_symbol(name)
    }

    /// 查找所有引用
    pub fn find_references(&self, name: &str) -> Vec<&Symbol> {
        self.symbols.get(name).map(|s| s.iter().collect()).unwrap_or_default()
    }

    /// 检查符号是否在当前作用域定义
    pub fn is_in_current_scope(&self, name: &str) -> bool {
        self.current_scope
            .last()
            .map(|scope| scope.contains(name))
            .unwrap_or(false)
    }

    /// 获取所有符号
    pub fn all_symbols(&self) -> Vec<&Symbol> {
        self.symbols.values().flatten().collect()
    }

    /// 清空符号表
    pub fn clear(&mut self) {
        self.symbols.clear();
        self.current_scope = vec![HashSet::new()];
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

/// 工作区索引
#[derive(Debug, Clone)]
pub struct WorkspaceIndex {
    /// 文件 URI → 符号表
    file_symbols: HashMap<String, SymbolTable>,
    /// 符号名 → 定义位置列表
    symbol_definitions: HashMap<String, Vec<SymbolLocation>>,
    /// 文件依赖图
    dependencies: HashMap<String, Vec<String>>,
    /// 所有文件
    all_files: HashSet<String>,
}

impl WorkspaceIndex {
    /// 创建新的工作区索引
    pub fn new() -> Self {
        WorkspaceIndex {
            file_symbols: HashMap::new(),
            symbol_definitions: HashMap::new(),
            dependencies: HashMap::new(),
            all_files: HashSet::new(),
        }
    }

    /// 更新文件
    pub fn update_file(&mut self, uri: &str, symbols: Vec<Symbol>) {
        // 创建新的符号表
        let mut table = SymbolTable::new();
        for symbol in symbols {
            // 记录定义位置
            let def_loc = SymbolLocation::new(
                symbol.name.clone(),
                symbol.location.clone(),
            );
            self.symbol_definitions
                .entry(symbol.name.clone())
                .or_insert_with(Vec::new)
                .push(def_loc);
            
            // 添加到符号表
            table.add_symbol(symbol);
        }
        
        self.file_symbols.insert(uri.to_string(), table);
        self.all_files.insert(uri.to_string());
    }

    /// 移除文件
    pub fn remove_file(&mut self, uri: &str) {
        self.file_symbols.remove(uri);
        self.all_files.remove(uri);
    }

    /// 查找定义
    pub fn find_definition(&self, name: &str, _context: &str) -> Option<Location> {
        self.symbol_definitions
            .get(name)
            .and_then(|locs| locs.first())
            .map(|loc| loc.location.clone())
    }

    /// 查找所有引用
    pub fn find_references(&self, name: &str, _context: &str) -> Vec<Location> {
        self.symbol_definitions
            .get(name)
            .map(|locs| {
                locs.iter()
                    .map(|loc| loc.location.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 添加依赖关系
    pub fn add_dependency(&mut self, from: &str, to: &str) {
        self.dependencies
            .entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());
    }

    /// 获取文件依赖
    pub fn get_dependencies(&self, uri: &str) -> Vec<String> {
        self.dependencies.get(uri).cloned().unwrap_or_default()
    }

    /// 获取反向依赖（依赖该文件的所有文件）
    pub fn get_reverse_dependencies(&self, uri: &str) -> Vec<String> {
        self.dependencies
            .iter()
            .filter(|(_, deps)| deps.contains(&uri.to_string()))
            .map(|(file, _)| file.clone())
            .collect()
    }

    /// 检查文件是否存在
    pub fn has_file(&self, uri: &str) -> bool {
        self.all_files.contains(uri)
    }

    /// 获取所有文件
    pub fn all_files(&self) -> &HashSet<String> {
        &self.all_files
    }

    /// 清空索引
    pub fn clear(&mut self) {
        self.file_symbols.clear();
        self.symbol_definitions.clear();
        self.dependencies.clear();
        self.all_files.clear();
    }

    /// 获取文件的符号表
    pub fn get_symbol_table(&self, uri: &str) -> Option<&SymbolTable> {
        self.file_symbols.get(uri)
    }

    /// 获取所有符号定义
    pub fn all_definitions(&self) -> &HashMap<String, Vec<SymbolLocation>> {
        &self.symbol_definitions
    }
}

impl Default for WorkspaceIndex {
    fn default() -> Self {
        Self::new()
    }
}

/// 符号查询上下文
#[derive(Debug, Clone)]
pub struct SymbolQueryContext {
    /// 当前文件 URI
    pub current_file: String,
    /// 光标位置
    pub position: Position,
    /// 符号名称
    pub symbol_name: String,
}

impl SymbolQueryContext {
    /// 创建新的查询上下文
    pub fn new(current_file: String, position: Position, symbol_name: String) -> Self {
        SymbolQueryContext {
            current_file,
            position,
            symbol_name,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table() {
        let mut table = SymbolTable::new();
        
        let symbol = Symbol::new(
            "变量".to_string(),
            SymbolKind::Variable,
            "file:///test.hl".to_string(),
            Range::new(
                Position::new(0, 0),
                Position::new(0, 6),
            ),
        ).with_type("整数".to_string());
        
        table.add_symbol(symbol);
        
        assert!(table.find_symbol("变量").is_some());
        assert_eq!(table.find_symbol("变量").unwrap().type_info, Some("整数".to_string()));
    }

    #[test]
    fn test_workspace_index() {
        let mut index = WorkspaceIndex::new();
        
        let symbols = vec![
            Symbol::new(
                "函数".to_string(),
                SymbolKind::Function,
                "file:///test.hl".to_string(),
                Range::new(
                    Position::new(0, 0),
                    Position::new(0, 4),
                ),
            )
        ];
        
        index.update_file("file:///test.hl", symbols);
        
        assert!(index.has_file("file:///test.hl"));
        assert!(index.find_definition("函数", "file:///test.hl").is_some());
    }

    #[test]
    fn test_scope_management() {
        let mut table = SymbolTable::new();
        
        let global_var = Symbol::new(
            "全局变量".to_string(),
            SymbolKind::Variable,
            "file:///test.hl".to_string(),
            Range::new(
                Position::new(0, 0),
                Position::new(0, 4),
            ),
        );
        table.add_symbol(global_var);
        
        table.enter_scope();
        
        let local_var = Symbol::new(
            "局部变量".to_string(),
            SymbolKind::Variable,
            "file:///test.hl".to_string(),
            Range::new(
                Position::new(1, 0),
                Position::new(1, 4),
            ),
        );
        table.add_symbol(local_var);
        
        assert!(table.is_in_current_scope("局部变量"));
        assert!(table.is_in_current_scope("全局变量"));
        
        table.exit_scope();
        
        assert!(!table.is_in_current_scope("局部变量"));
        assert!(table.is_in_current_scope("全局变量"));
    }
}

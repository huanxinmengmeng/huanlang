// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 补全请求处理器

use crate::lsp::{Position, Range, Location};
use std::collections::HashMap;

/// 补全项类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionItemKind {
    Text,
    Method,
    Function,
    Constructor,
    Field,
    Variable,
    Constant,
    Class,
    Interface,
    TypeParameter,
    Keyword,
    Snippet,
    Color,
    File,
    Reference,
    Folder,
    EnumMember,
    Word,
    Automation,
}

impl Default for CompletionItemKind {
    fn default() -> Self {
        CompletionItemKind::Text
    }
}

/// 补全项
#[derive(Debug, Clone)]
pub struct CompletionItem {
    /// 显示标签
    pub label: String,
    /// 补全类型
    pub kind: CompletionItemKind,
    /// 详情
    pub detail: Option<String>,
    /// 文档
    pub documentation: Option<String>,
    /// 插入文本
    pub insert_text: Option<String>,
    /// 插入文本格式
    pub insert_text_format: InsertTextFormat,
    /// 排序文本
    pub sort_text: Option<String>,
    /// 过滤文本
    pub filter_text: Option<String>,
    /// 幻语同义词
    pub huan_synonyms: Option<HuanSynonyms>,
}

impl CompletionItem {
    /// 创建新的补全项
    pub fn new(label: String, kind: CompletionItemKind) -> Self {
        CompletionItem {
            label,
            kind,
            detail: None,
            documentation: None,
            insert_text: None,
            insert_text_format: InsertTextFormat::PlainText,
            sort_text: None,
            filter_text: None,
            huan_synonyms: None,
        }
    }

    /// 设置详情
    pub fn with_detail(mut self, detail: String) -> Self {
        self.detail = Some(detail);
        self
    }

    /// 设置文档
    pub fn with_documentation(mut self, docs: String) -> Self {
        self.documentation = Some(docs);
        self
    }

    /// 设置插入文本
    pub fn with_insert_text(mut self, text: String) -> Self {
        self.insert_text = Some(text);
        self
    }

    /// 设置幻语同义词
    pub fn with_huan_synonyms(mut self, synonyms: HuanSynonyms) -> Self {
        self.huan_synonyms = Some(synonyms);
        self
    }
}

/// 插入文本格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InsertTextFormat {
    PlainText,
    Snippet,
}

impl Default for InsertTextFormat {
    fn default() -> Self {
        InsertTextFormat::PlainText
    }
}

/// 幻语同义词
#[derive(Debug, Clone)]
pub struct HuanSynonyms {
    /// 中文关键词
    pub chinese: String,
    /// 拼音关键词
    pub pinyin: String,
    /// 英文关键词
    pub english: String,
}

impl HuanSynonyms {
    /// 创建新的同义词
    pub fn new(chinese: &str, pinyin: &str, english: &str) -> Self {
        HuanSynonyms {
            chinese: chinese.to_string(),
            pinyin: pinyin.to_string(),
            english: english.to_string(),
        }
    }
}

/// 补全请求参数
#[derive(Debug, Clone)]
pub struct CompletionParams {
    /// 文本文档标识符
    pub text_document: TextDocumentIdentifier,
    /// 光标位置
    pub position: Position,
    /// 触发字符
    pub trigger_character: Option<String>,
    /// 触发补全上下文
    pub context: Option<CompletionContext>,
}

/// 文本文档标识符
#[derive(Debug, Clone)]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

impl TextDocumentIdentifier {
    pub fn new(uri: String) -> Self {
        TextDocumentIdentifier { uri }
    }
}

/// 补全上下文
#[derive(Debug, Clone)]
pub struct CompletionContext {
    /// 触发方式
    pub trigger_kind: CompletionTriggerKind,
    /// 触发字符
    pub trigger_character: Option<String>,
}

/// 补全触发方式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompletionTriggerKind {
    Invoked,
    TriggerCharacter,
    TriggerForIncompleteCompletions,
}

impl Default for CompletionTriggerKind {
    fn default() -> Self {
        CompletionTriggerKind::Invoked
    }
}

/// 补全请求处理器
pub struct CompletionHandler;

impl CompletionHandler {
    /// 处理补全请求
    pub fn handle(params: CompletionParams) -> CompletionResult {
        let mut items = Vec::new();
        
        // 获取上下文信息
        let trigger = params.trigger_character.as_deref().unwrap_or("");
        
        // 根据触发字符提供不同的补全
        match trigger {
            "令" | "let" => {
                items.push(Self::create_variable_completion());
            }
            "函数" | "fn" => {
                items.push(Self::create_function_completion());
            }
            ":" => {
                items.extend(Self::create_type_completions());
            }
            "." => {
                items.extend(Self::create_member_completions());
            }
            _ => {
                // 提供完整补全列表
                items.extend(Self::create_keyword_completions());
                items.extend(Self::create_type_completions());
                items.extend(Self::create_snippet_completions());
            }
        }
        
        CompletionResult {
            is_incomplete: false,
            items,
        }
    }

    /// 创建变量声明补全
    fn create_variable_completion() -> CompletionItem {
        CompletionItem::new("变量名".to_string(), CompletionItemKind::Snippet)
            .with_detail("变量声明".to_string())
            .with_documentation("声明一个新的可变变量\n\n语法: 令 变量名 类型 为 表达式".to_string())
            .with_insert_text("令 ${1:变量名} 为 ${2:值}".to_string())
            .with_huan_synonyms(HuanSynonyms::new("令", "ling", "let"))
    }

    /// 创建函数补全
    fn create_function_completion() -> CompletionItem {
        CompletionItem::new("函数名".to_string(), CompletionItemKind::Snippet)
            .with_detail("函数定义".to_string())
            .with_documentation("定义一个新的函数\n\n语法: 函数 函数名(参数: 类型) 返回 类型".to_string())
            .with_insert_text("函数 ${1:函数名}(${2:参数}) 返回 ${3:类型}\n    ${4:// 函数体}\n结束".to_string())
            .with_huan_synonyms(HuanSynonyms::new("函数", "hanshu", "function"))
    }

    /// 创建类型补全列表
    fn create_type_completions() -> Vec<CompletionItem> {
        vec![
            CompletionItem::new("整数".to_string(), CompletionItemKind::Keyword)
                .with_detail("整数类型".to_string())
                .with_huan_synonyms(HuanSynonyms::new("整数", "zhengshu", "int")),
            CompletionItem::new("浮点64".to_string(), CompletionItemKind::Keyword)
                .with_detail("64位浮点数类型".to_string())
                .with_huan_synonyms(HuanSynonyms::new("浮点64", "fudain64", "f64")),
            CompletionItem::new("字符串".to_string(), CompletionItemKind::Keyword)
                .with_detail("字符串类型".to_string())
                .with_huan_synonyms(HuanSynonyms::new("字符串", "zifuchuan", "string")),
            CompletionItem::new("布尔".to_string(), CompletionItemKind::Keyword)
                .with_detail("布尔类型".to_string())
                .with_huan_synonyms(HuanSynonyms::new("布尔", "buer", "bool")),
            CompletionItem::new("字符".to_string(), CompletionItemKind::Keyword)
                .with_detail("字符类型".to_string())
                .with_huan_synonyms(HuanSynonyms::new("字符", "zifu", "char")),
            CompletionItem::new("列表".to_string(), CompletionItemKind::Keyword)
                .with_detail("列表类型".to_string())
                .with_huan_synonyms(HuanSynonyms::new("列表", "liebiao", "list")),
            CompletionItem::new("字典".to_string(), CompletionItemKind::Keyword)
                .with_detail("字典类型".to_string())
                .with_huan_synonyms(HuanSynonyms::new("字典", "zidian", "dict")),
        ]
    }

    /// 创建成员补全列表
    fn create_member_completions() -> Vec<CompletionItem> {
        vec![
            CompletionItem::new("长度".to_string(), CompletionItemKind::Method)
                .with_detail("获取长度".to_string())
                .with_documentation("返回列表或字符串的长度".to_string())
                .with_insert_text("长度()".to_string()),
            CompletionItem::new("为空".to_string(), CompletionItemKind::Method)
                .with_detail("检查是否为空".to_string())
                .with_documentation("返回布尔值，表示是否为空".to_string())
                .with_insert_text("为空()".to_string()),
            CompletionItem::new("获取".to_string(), CompletionItemKind::Method)
                .with_detail("获取元素".to_string())
                .with_documentation("根据索引获取元素".to_string())
                .with_insert_text("获取(${1:索引})".to_string()),
        ]
    }

    /// 创建关键词补全列表
    fn create_keyword_completions() -> Vec<CompletionItem> {
        vec![
            CompletionItem::new("令".to_string(), CompletionItemKind::Keyword)
                .with_detail("变量声明".to_string())
                .with_documentation("声明一个新的可变变量".to_string())
                .with_insert_text("令 ".to_string())
                .with_huan_synonyms(HuanSynonyms::new("令", "ling", "let")),
            CompletionItem::new("定".to_string(), CompletionItemKind::Keyword)
                .with_detail("常量声明".to_string())
                .with_documentation("声明一个新的不可变常量".to_string())
                .with_insert_text("定 ".to_string())
                .with_huan_synonyms(HuanSynonyms::new("定", "ding", "const")),
            CompletionItem::new("函数".to_string(), CompletionItemKind::Keyword)
                .with_detail("函数定义".to_string())
                .with_documentation("定义一个新的函数".to_string())
                .with_insert_text("函数 ".to_string())
                .with_huan_synonyms(HuanSynonyms::new("函数", "hanshu", "fn")),
            CompletionItem::new("若".to_string(), CompletionItemKind::Keyword)
                .with_detail("条件语句".to_string())
                .with_documentation("条件分支语句".to_string())
                .with_insert_text("若 ${1:条件} 那么\n    ${2:// 代码}\n结束".to_string())
                .with_huan_synonyms(HuanSynonyms::new("若", "ruo", "if")),
            CompletionItem::new("循环".to_string(), CompletionItemKind::Keyword)
                .with_detail("循环语句".to_string())
                .with_documentation("循环执行代码块".to_string())
                .with_insert_text("循环 ${1:次数}\n    ${2:// 代码}\n结束".to_string())
                .with_huan_synonyms(HuanSynonyms::new("循环", "xunhuan", "loop")),
            CompletionItem::new("返回".to_string(), CompletionItemKind::Keyword)
                .with_detail("返回语句".to_string())
                .with_documentation("从函数返回值".to_string())
                .with_insert_text("返回 ".to_string())
                .with_huan_synonyms(HuanSynonyms::new("返回", "fanhui", "return")),
        ]
    }

    /// 创建代码片段补全
    fn create_snippet_completions() -> Vec<CompletionItem> {
        vec![
            CompletionItem::new("函数模板".to_string(), CompletionItemKind::Snippet)
                .with_detail("完整的函数定义".to_string())
                .with_documentation("插入一个完整的函数模板")
                .with_insert_text("函数 ${1:函数名}(${2:参数}) 返回 ${3:类型}\n    ${4:// 函数体}\n结束".to_string()),
            CompletionItem::new("条件语句".to_string(), CompletionItemKind::Snippet)
                .with_detail("完整的条件语句".to_string())
                .with_documentation("插入一个完整的条件语句模板")
                .with_insert_text("若 ${1:条件} 那么\n    ${2:// 代码}\n结束".to_string()),
            CompletionItem::new("循环语句".to_string(), CompletionItemKind::Snippet)
                .with_detail("完整的循环语句".to_string())
                .with_documentation("插入一个完整的循环语句模板")
                .with_insert_text("循环 ${1:次数}\n    ${2:// 代码}\n结束".to_string()),
        ]
    }
}

/// 补全结果
#[derive(Debug, Clone)]
pub struct CompletionResult {
    /// 是否不完整
    pub is_incomplete: bool,
    /// 补全项列表
    pub items: Vec<CompletionItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_item_creation() {
        let item = CompletionItem::new("测试".to_string(), CompletionItemKind::Text)
            .with_detail("测试详情".to_string())
            .with_insert_text("测试插入".to_string());
        
        assert_eq!(item.label, "测试");
        assert_eq!(item.kind, CompletionItemKind::Text);
        assert_eq!(item.detail, Some("测试详情".to_string()));
        assert_eq!(item.insert_text, Some("测试插入".to_string()));
    }

    #[test]
    fn test_huan_synonyms() {
        let synonyms = HuanSynonyms::new("令", "ling", "let");
        assert_eq!(synonyms.chinese, "令");
        assert_eq!(synonyms.pinyin, "ling");
        assert_eq!(synonyms.english, "let");
    }

    #[test]
    fn test_completion_handler() {
        let params = CompletionParams {
            text_document: TextDocumentIdentifier::new("file:///test.hl".to_string()),
            position: Position::new(0, 0),
            trigger_character: None,
            context: None,
        };
        
        let result = CompletionHandler::handle(params);
        assert!(!result.items.is_empty());
    }

    #[test]
    fn test_keyword_completions() {
        let completions = CompletionHandler::create_keyword_completions();
        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.label == "令"));
        assert!(completions.iter().any(|c| c.label == "函数"));
        assert!(completions.iter().any(|c| c.label == "若"));
    }
}

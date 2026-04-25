// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 语义标记模块
//!
//! 本模块提供比语法高亮更精确的语义着色信息，支持：
//! - 关键词
//! - 类型
//! - 函数
//! - 变量
//! - 参数
//! - 字符串
//! - 数字
//! - 注释
//! - 运算符

use crate::lsp::{Position, Range};

/// 语义标记类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticTokenType {
    Namespace,
    Type,
    Struct,
    Class,
    Enum,
    Interface,
    TypeParameter,
    Function,
    Method,
    Variable,
    Constant,
    String,
    Number,
    Boolean,
    Array,
    Object,
    Key,
    Null,
    EnumMember,
    Event,
    Operator,
    Decorator,
}

impl SemanticTokenType {
    /// 获取标记类型的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            SemanticTokenType::Namespace => "namespace",
            SemanticTokenType::Type => "type",
            SemanticTokenType::Struct => "struct",
            SemanticTokenType::Class => "class",
            SemanticTokenType::Enum => "enum",
            SemanticTokenType::Interface => "interface",
            SemanticTokenType::TypeParameter => "typeParameter",
            SemanticTokenType::Function => "function",
            SemanticTokenType::Method => "method",
            SemanticTokenType::Variable => "variable",
            SemanticTokenType::Constant => "constant",
            SemanticTokenType::String => "string",
            SemanticTokenType::Number => "number",
            SemanticTokenType::Boolean => "boolean",
            SemanticTokenType::Array => "array",
            SemanticTokenType::Object => "object",
            SemanticTokenType::Key => "key",
            SemanticTokenType::Null => "null",
            SemanticTokenType::EnumMember => "enumMember",
            SemanticTokenType::Event => "event",
            SemanticTokenType::Operator => "operator",
            SemanticTokenType::Decorator => "decorator",
        }
    }

    /// 从字符串获取标记类型
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "namespace" => Some(SemanticTokenType::Namespace),
            "type" => Some(SemanticTokenType::Type),
            "struct" => Some(SemanticTokenType::Struct),
            "class" => Some(SemanticTokenType::Class),
            "enum" => Some(SemanticTokenType::Enum),
            "interface" => Some(SemanticTokenType::Interface),
            "typeParameter" => Some(SemanticTokenType::TypeParameter),
            "function" => Some(SemanticTokenType::Function),
            "method" => Some(SemanticTokenType::Method),
            "variable" => Some(SemanticTokenType::Variable),
            "constant" => Some(SemanticTokenType::Constant),
            "string" => Some(SemanticTokenType::String),
            "number" => Some(SemanticTokenType::Number),
            "boolean" => Some(SemanticTokenType::Boolean),
            "array" => Some(SemanticTokenType::Array),
            "object" => Some(SemanticTokenType::Object),
            "key" => Some(SemanticTokenType::Key),
            "null" => Some(SemanticTokenType::Null),
            "enumMember" => Some(SemanticTokenType::EnumMember),
            "event" => Some(SemanticTokenType::Event),
            "operator" => Some(SemanticTokenType::Operator),
            "decorator" => Some(SemanticTokenType::Decorator),
            _ => None,
        }
    }
}

/// 语义标记修饰符
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SemanticTokenModifier {
    Declaration,
    Definition,
    Readonly,
    Static,
    Deprecated,
    Abstract,
    Async,
    Modification,
    Documentation,
    DefaultLibrary,
}

impl SemanticTokenModifier {
    /// 获取修饰符的字符串表示
    pub fn as_str(&self) -> &'static str {
        match self {
            SemanticTokenModifier::Declaration => "declaration",
            SemanticTokenModifier::Definition => "definition",
            SemanticTokenModifier::Readonly => "readonly",
            SemanticTokenModifier::Static => "static",
            SemanticTokenModifier::Deprecated => "deprecated",
            SemanticTokenModifier::Abstract => "abstract",
            SemanticTokenModifier::Async => "async",
            SemanticTokenModifier::Modification => "modification",
            SemanticTokenModifier::Documentation => "documentation",
            SemanticTokenModifier::DefaultLibrary => "defaultLibrary",
        }
    }

    /// 从字符串获取修饰符
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "declaration" => Some(SemanticTokenModifier::Declaration),
            "definition" => Some(SemanticTokenModifier::Definition),
            "readonly" => Some(SemanticTokenModifier::Readonly),
            "static" => Some(SemanticTokenModifier::Static),
            "deprecated" => Some(SemanticTokenModifier::Deprecated),
            "abstract" => Some(SemanticTokenModifier::Abstract),
            "async" => Some(SemanticTokenModifier::Async),
            "modification" => Some(SemanticTokenModifier::Modification),
            "documentation" => Some(SemanticTokenModifier::Documentation),
            "defaultLibrary" => Some(SemanticTokenModifier::DefaultLibrary),
            _ => None,
        }
    }
}

/// 语义标记
#[derive(Debug, Clone)]
pub struct SemanticToken {
    /// 标记范围
    pub range: Range,
    /// 标记类型
    pub token_type: SemanticTokenType,
    /// 标记修饰符
    pub modifiers: Vec<SemanticTokenModifier>,
}

impl SemanticToken {
    /// 创建新的语义标记
    pub fn new(
        range: Range,
        token_type: SemanticTokenType,
    ) -> Self {
        SemanticToken {
            range,
            token_type,
            modifiers: Vec::new(),
        }
    }

    /// 添加修饰符
    pub fn with_modifier(mut self, modifier: SemanticTokenModifier) -> Self {
        if !self.modifiers.contains(&modifier) {
            self.modifiers.push(modifier);
        }
        self
    }

    /// 转换为 LSP 格式
    pub fn to_lsp_token(&self, previous_line: u32, previous_char: u32) -> Vec<u32> {
        let delta_line = self.range.start.line - previous_line;
        let delta_start;
        
        if delta_line == 0 {
            delta_start = self.range.start.character - previous_char;
        } else {
            delta_start = self.range.start.character;
        }
        
        let length = self.range.end.character - self.range.start.character;
        let token_type = Self::token_type_to_index(self.token_type);
        let token_modifiers = self.compute_modifier_bits();
        
        vec![
            delta_line,
            delta_start as u32,
            length as u32,
            token_type as u32,
            token_modifiers,
        ]
    }

    /// 标记类型转索引
    fn token_type_to_index(token_type: SemanticTokenType) -> usize {
        match token_type {
            SemanticTokenType::Namespace => 0,
            SemanticTokenType::Type => 1,
            SemanticTokenType::Struct => 2,
            SemanticTokenType::Class => 3,
            SemanticTokenType::Enum => 4,
            SemanticTokenType::Interface => 5,
            SemanticTokenType::TypeParameter => 6,
            SemanticTokenType::Function => 7,
            SemanticTokenType::Method => 8,
            SemanticTokenType::Variable => 9,
            SemanticTokenType::Constant => 10,
            SemanticTokenType::String => 11,
            SemanticTokenType::Number => 12,
            SemanticTokenType::Boolean => 13,
            SemanticTokenType::Array => 14,
            SemanticTokenType::Object => 15,
            SemanticTokenType::Key => 16,
            SemanticTokenType::Null => 17,
            SemanticTokenType::EnumMember => 18,
            SemanticTokenType::Event => 19,
            SemanticTokenType::Operator => 20,
            SemanticTokenType::Decorator => 21,
        }
    }

    /// 计算修饰符位掩码
    fn compute_modifier_bits(&self) -> u32 {
        let mut bits = 0u32;
        for modifier in &self.modifiers {
            let index = match modifier {
                SemanticTokenModifier::Declaration => 0,
                SemanticTokenModifier::Definition => 1,
                SemanticTokenModifier::Readonly => 2,
                SemanticTokenModifier::Static => 3,
                SemanticTokenModifier::Deprecated => 4,
                SemanticTokenModifier::Abstract => 5,
                SemanticTokenModifier::Async => 6,
                SemanticTokenModifier::Modification => 7,
                SemanticTokenModifier::Documentation => 8,
                SemanticTokenModifier::DefaultLibrary => 9,
            };
            bits |= 1 << index;
        }
        bits
    }
}

/// 语义标记生成器
#[derive(Debug, Clone)]
pub struct SemanticTokenGenerator {
    /// 标记列表
    tokens: Vec<SemanticToken>,
    /// 最后处理的行号
    last_line: u32,
    /// 最后处理的字符位置
    last_char: u32,
}

impl SemanticTokenGenerator {
    /// 创建新的标记生成器
    pub fn new() -> Self {
        SemanticTokenGenerator {
            tokens: Vec::new(),
            last_line: 0,
            last_char: 0,
        }
    }

    /// 添加标记
    pub fn add_token(&mut self, token: SemanticToken) {
        self.tokens.push(token);
    }

    /// 添加关键词标记
    pub fn add_keyword(&mut self, range: Range) {
        self.add_token(SemanticToken::new(range, SemanticTokenType::Keyword));
    }

    /// 添加类型标记
    pub fn add_type(&mut self, range: Range) {
        self.add_token(SemanticToken::new(range, SemanticTokenType::Type));
    }

    /// 添加函数标记
    pub fn add_function(&mut self, range: Range) {
        self.add_token(SemanticToken::new(range, SemanticTokenType::Function));
    }

    /// 添加变量标记
    pub fn add_variable(&mut self, range: Range) {
        self.add_token(SemanticToken::new(range, SemanticTokenType::Variable));
    }

    /// 添加字符串标记
    pub fn add_string(&mut self, range: Range) {
        self.add_token(SemanticToken::new(range, SemanticTokenType::String));
    }

    /// 添加数字标记
    pub fn add_number(&mut self, range: Range) {
        self.add_token(SemanticToken::new(range, SemanticTokenType::Number));
    }

    /// 添加注释标记
    pub fn add_comment(&mut self, range: Range) {
        self.add_token(SemanticToken::new(range, SemanticTokenType::Decorator));
    }

    /// 添加运算符标记
    pub fn add_operator(&mut self, range: Range) {
        self.add_token(SemanticToken::new(range, SemanticTokenType::Operator));
    }

    /// 生成 LSP 格式的标记数据
    pub fn generate(&mut self) -> Vec<u32> {
        let mut result = Vec::new();
        
        // 按位置排序
        self.tokens.sort_by(|a, b| {
            let start_a = (a.range.start.line, a.range.start.character);
            let start_b = (b.range.start.line, b.range.start.character);
            start_a.cmp(&start_b)
        });
        
        let mut last_line = 0u32;
        let mut last_char = 0u32;
        
        for token in &self.tokens {
            let token_data = token.to_lsp_token(last_line, last_char);
            result.extend(token_data);
            
            last_line = token.range.start.line;
            last_char = token.range.start.character;
        }
        
        result
    }

    /// 获取标记列表
    pub fn into_tokens(self) -> Vec<SemanticToken> {
        self.tokens
    }

    /// 清空生成器
    pub fn clear(&mut self) {
        self.tokens.clear();
        self.last_line = 0;
        self.last_char = 0;
    }

    /// 获取标记数量
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }
}

impl Default for SemanticTokenGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// 幻语关键词映射
pub struct HuanKeywords;

impl HuanKeywords {
    /// 获取所有关键词
    pub fn all() -> Vec<(&'static str, &'static str)> {
        vec![
            // 声明关键词
            ("令", "声明变量"),
            ("定", "声明常量"),
            ("函数", "函数定义"),
            ("结构", "结构体定义"),
            ("枚举", "枚举定义"),
            ("接口", "接口定义"),
            ("特征", "特征定义"),
            ("导入", "导入模块"),
            ("返回", "返回语句"),
            ("导出", "导出符号"),
            
            // 控制流
            ("若", "条件语句"),
            ("否则", "条件分支"),
            ("选择", "匹配语句"),
            ("当", "循环语句"),
            ("循环", "循环语句"),
            ("跳出", "跳出循环"),
            ("继续", "继续循环"),
            
            // 类型关键词
            ("整数", "整数类型"),
            ("浮点", "浮点类型"),
            ("字符串", "字符串类型"),
            ("布尔", "布尔类型"),
            ("字符", "字符类型"),
            ("真", "真值"),
            ("假", "假值"),
            ("空", "空值"),
            ("列表", "列表类型"),
            ("字典", "字典类型"),
            ("可选", "可选类型"),
            
            // 访问修饰符
            ("公共", "公共访问"),
            ("私有", "私有访问"),
            ("受保护", "受保护访问"),
            
            // 生命周期
            ("借用", "借用引用"),
            ("拥有", "所有权转移"),
            ("引用", "引用类型"),
            
            // 异步和并发
            ("异步", "异步函数"),
            ("等待", "等待异步操作"),
            ("线程", "线程操作"),
            ("通道", "通道通信"),
            
            // 错误处理
            ("尝试", "错误处理"),
            ("捕获", "捕获异常"),
            ("抛出", "抛出异常"),
            
            // 特殊关键词
            ("自引用", "self"),
            ("类型", "类型注解"),
            ("作为", "类型转换"),
            ("是", "类型检查"),
            ("在", "迭代器"),
            ("或者", "或操作"),
            ("并且", "与操作"),
            ("不", "非操作"),
        ]
    }

    /// 获取关键词的拼音形式
    pub fn get_pinyin(chinese: &str) -> Option<&'static str> {
        match chinese {
            "令" => Some("ling"),
            "定" => Some("ding"),
            "函数" => Some("hanshu"),
            "结构" => Some("jiegou"),
            "枚举" => Some("meiju"),
            "接口" => Some("jiekou"),
            "特征" => Some("tezheng"),
            "导入" => Some("daoru"),
            "返回" => Some("fanhui"),
            "导出" => Some("daochu"),
            "若" => Some("ruo"),
            "否则" => Some("fouze"),
            "选择" => Some("xuanze"),
            "当" => Some("dang"),
            "循环" => Some("xunhuan"),
            "跳出" => Some("tuochu"),
            "继续" => Some("jixu"),
            "整数" => Some("zhengshu"),
            "浮点" => Some("fudain"),
            "字符串" => Some("zifuchuan"),
            "布尔" => Some("buer"),
            "字符" => Some("zifu"),
            "真" => Some("zhen"),
            "假" => Some("jia"),
            "空" => Some("kong"),
            "列表" => Some("liebiao"),
            "字典" => Some("zidian"),
            "可选" => Some("kexuan"),
            "公共" => Some("gonggong"),
            "私有" => Some("siyou"),
            "受保护" => Some("shoubaohu"),
            "借用" => Some("jieyong"),
            "拥有" => Some("yongyou"),
            "引用" => Some("yinyong"),
            "异步" => Some("yibu"),
            "等待" => Some("dengdai"),
            "线程" => Some("xiancheng"),
            "通道" => Some("tongdao"),
            "尝试" => Some("changshi"),
            "捕获" => Some("buhuo"),
            "抛出" => Some("paochu"),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semantic_token_creation() {
        let range = Range::new(
            Position::new(0, 0),
            Position::new(0, 5),
        );
        let token = SemanticToken::new(range, SemanticTokenType::Function);
        
        assert_eq!(token.token_type, SemanticTokenType::Function);
        assert!(token.modifiers.is_empty());
    }

    #[test]
    fn test_semantic_token_with_modifier() {
        let range = Range::new(
            Position::new(0, 0),
            Position::new(0, 5),
        );
        let token = SemanticToken::new(range, SemanticTokenType::Function)
            .with_modifier(SemanticTokenModifier::Declaration)
            .with_modifier(SemanticTokenModifier::Static);
        
        assert_eq!(token.modifiers.len(), 2);
    }

    #[test]
    fn test_semantic_token_generator() {
        let mut generator = SemanticTokenGenerator::new();
        
        generator.add_keyword(Range::new(
            Position::new(0, 0),
            Position::new(0, 2),
        ));
        generator.add_function(Range::new(
            Position::new(0, 3),
            Position::new(0, 8),
        ));
        
        assert_eq!(generator.len(), 2);
        
        let data = generator.generate();
        assert!(!data.is_empty());
    }

    #[test]
    fn test_huan_keywords() {
        let keywords = HuanKeywords::all();
        assert!(!keywords.is_empty());
        
        // 测试关键词存在
        assert!(keywords.iter().any(|(k, _)| *k == "令"));
        assert!(keywords.iter().any(|(k, _)| *k == "函数"));
        assert!(keywords.iter().any(|(k, _)| *k == "若"));
    }

    #[test]
    fn test_pinyin_lookup() {
        assert_eq!(HuanKeywords::get_pinyin("令"), Some("ling"));
        assert_eq!(HuanKeywords::get_pinyin("函数"), Some("hanshu"));
        assert_eq!(HuanKeywords::get_pinyin("整数"), Some("zhengshu"));
    }
}

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

//! 三语关键词系统 - 支持中文、拼音和英文关键词的完整实现
//!
//! 根据《幻语编程语言 - 完整详细的开发规范文档.md》第2章实现

use super::token::TokenKind;
use std::collections::HashMap;

/// 关键词风格枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeywordStyle {
    /// 中文关键词
    Chinese,
    /// 拼音关键词
    Pinyin,
    /// 英文关键词
    English,
}

/// 三语关键词映射表
///
/// 包含：
/// - 正向映射：任意语言字符串 -> TokenKind
/// - 反向映射：TokenKind -> 各语言字符串 (通过 match 语句实现)
pub struct KeywordTable {
    /// 正向映射：字符串 -> TokenKind
    map: HashMap<&'static str, TokenKind>,
}

/// 关键词三语元组定义
const KEYWORD_TRIPLES: &[(&str, &str, &str, TokenKind)] = &[
    // 控制流与声明关键词
    ("令", "ling", "let", TokenKind::Let),
    ("变量", "bianliang", "let", TokenKind::Let),
    ("定", "ding", "const", TokenKind::Const),
    ("为", "wei", "be", TokenKind::Be),
    ("类型", "leixing", "type", TokenKind::TypeAnno),
    ("若", "ruo", "if", TokenKind::If),
    ("如果", "ruguo", "if", TokenKind::If),
    ("则", "ze", "then", TokenKind::Then),
    ("否则", "fouze", "else", TokenKind::Else),
    ("结束", "jieshu", "end", TokenKind::End),
    ("当", "dang", "while", TokenKind::While),
    ("循环", "xunhuan", "while", TokenKind::While),
    ("重复", "chongfu", "repeat", TokenKind::Repeat),
    ("次", "ci", "times", TokenKind::Times),
    ("对于", "duiyu", "for", TokenKind::For),
    ("每个", "meige", "each", TokenKind::Each),
    ("在", "zai", "in", TokenKind::In),
    ("中", "zhong", "do", TokenKind::Do),
    ("开始", "kaishi", "begin", TokenKind::Begin),
    ("函数", "hanshu", "func", TokenKind::Func),
    ("函数", "hanshu", "function", TokenKind::Func),
    ("返回", "fanhui", "return", TokenKind::Return),
    ("结构", "jiegou", "struct", TokenKind::Struct),
    ("字段", "ziduan", "field", TokenKind::Field),
    ("特征", "tezheng", "trait", TokenKind::Trait),
    ("需要", "xuyao", "requires", TokenKind::Requires),
    ("实现", "shixian", "impl", TokenKind::Impl),
    ("对于类型", "duiyuleixing", "for", TokenKind::ForTy),
    ("模块", "mokuai", "module", TokenKind::Module),
    ("公开", "gongkai", "pub", TokenKind::Pub),
    ("导入", "daoru", "import", TokenKind::Import),
    ("从", "cong", "from", TokenKind::From),
    ("匹配", "pipei", "match", TokenKind::Match),
    ("默认", "moren", "default", TokenKind::Default),
    ("跳出", "tiaochu", "break", TokenKind::Break),
    ("继续", "jixu", "continue", TokenKind::Continue),
    
    // 运算符关键词
    ("且", "qie", "and", TokenKind::And),
    ("或", "huo", "or", TokenKind::Or),
    ("非", "fei", "not", TokenKind::Not),
    ("加", "jia", "add", TokenKind::Add),
    ("减", "jian", "sub", TokenKind::Sub),
    ("乘", "cheng", "mul", TokenKind::Mul),
    ("除", "chu", "div", TokenKind::Div),
    ("取余", "quyu", "mod", TokenKind::Mod),
    ("大于", "dayu", "gt", TokenKind::Gt),
    ("小于", "xiaoyu", "lt", TokenKind::Lt),
    ("等于", "dengyu", "eq", TokenKind::Eq),
    ("不小于", "buxiaoyu", "ge", TokenKind::Ge),
    ("不大于", "budayu", "le", TokenKind::Le),
    ("不等于", "budengyu", "ne", TokenKind::Ne),
    ("设为", "shewei", "assign", TokenKind::Assign),
    ("左移", "zuoyi", "shl", TokenKind::Shl),
    ("右移", "youyi", "shr", TokenKind::Shr),
    ("按位与", "anweiyu", "bitand", TokenKind::BitAnd),
    ("按位或", "anweihuo", "bitor", TokenKind::BitOr),
    ("按位异或", "anweiyihuo", "bitxor", TokenKind::BitXor),
    
    // 字面量关键词
    ("真", "zhen", "true", TokenKind::True),
    ("假", "jia", "false", TokenKind::False),
    ("空", "kong", "null", TokenKind::Null),
    
    // 汇编关键词
    ("汇编", "huibian", "asm", TokenKind::Asm),
    ("易失", "yishi", "volatile", TokenKind::Volatile),
    ("全局", "quanju", "global", TokenKind::Global),
    ("段", "duan", "section", TokenKind::Section),
    ("对齐", "duiqi", "align", TokenKind::Align),
    ("外部", "waibu", "extern", TokenKind::Extern),
    ("类型定义", "leixingdingyi", "type", TokenKind::Type),
    ("可变", "kebian", "mut", TokenKind::Mut),
    
    // 裸机编程关键词
    ("外设", "waishu", "peripheral", TokenKind::Peripheral),
    ("寄存器", "jicunqi", "register", TokenKind::Register),
    ("内存", "neicun", "memory", TokenKind::Memory),
    ("布局", "buju", "layout", TokenKind::Layout),
    ("段定义", "duandingyi", "segment", TokenKind::Segment),
    
    // 类型关键词
    ("整数", "zhengshu", "int", TokenKind::TypeInt),
    ("整数8", "zhengshu8", "i8", TokenKind::TypeI8),
    ("整数16", "zhengshu16", "i16", TokenKind::TypeI16),
    ("整数32", "zhengshu32", "i32", TokenKind::TypeI32),
    ("整数64", "zhengshu64", "i64", TokenKind::TypeI64),
    ("无符号8", "wufuhao8", "u8", TokenKind::TypeU8),
    ("无符号16", "wufuhao16", "u16", TokenKind::TypeU16),
    ("无符号32", "wufuhao32", "u32", TokenKind::TypeU32),
    ("无符号64", "wufuhao64", "u64", TokenKind::TypeU64),
    ("浮点32", "fudian32", "f32", TokenKind::TypeF32),
    ("浮点64", "fudian64", "f64", TokenKind::TypeF64),
    ("布尔", "buer", "bool", TokenKind::TypeBool),
    ("字符", "zifu", "char", TokenKind::TypeChar),
    ("字符串", "zifuchuan", "string", TokenKind::TypeString),
    ("单元", "danyuan", "unit", TokenKind::TypeUnit),
    ("列表", "liebiao", "list", TokenKind::TypeList),
    ("数组", "shuzu", "array", TokenKind::TypeArray),
    ("字典", "zidian", "map", TokenKind::TypeMap),
    ("指针", "zhizhen", "ptr", TokenKind::TypePtr),
    ("可选", "kexuan", "option", TokenKind::TypeOption),
];

impl KeywordTable {
    /// 创建新的关键词表
    pub fn new() -> Self {
        let mut map: HashMap<&'static str, TokenKind> = HashMap::new();
        
        // 插入所有三语关键词映射
        for &(ch, py, en, ref kind) in KEYWORD_TRIPLES {
            map.insert(ch, kind.clone());
            map.insert(py, kind.clone());
            map.insert(en, kind.clone());
        }
        
        Self {
            map,
        }
    }

    /// 查询关键词，返回对应的 TokenKind
    pub fn get(&self, s: &str) -> Option<TokenKind> {
        self.map.get(s).cloned()
    }

    /// 判断字符串是否是关键词
    pub fn is_keyword(&self, s: &str) -> bool {
        self.map.contains_key(s)
    }

    /// 将 TokenKind 转换为指定风格的关键词字符串
    pub fn to_style(&self, kind: &TokenKind, style: KeywordStyle) -> Option<&'static str> {
        match style {
            KeywordStyle::Chinese => self.to_chinese(kind),
            KeywordStyle::Pinyin => self.to_pinyin(kind),
            KeywordStyle::English => self.to_english(kind),
        }
    }

    /// 转换为中文关键词
    pub fn to_chinese(&self, kind: &TokenKind) -> Option<&'static str> {
        for &(ch, _, _, ref k) in KEYWORD_TRIPLES {
            // 直接匹配输入的 kind 和我们的 k
            if std::mem::discriminant(kind) == std::mem::discriminant(k) {
                return Some(ch);
            }
        }
        None
    }

    /// 转换为拼音关键词
    pub fn to_pinyin(&self, kind: &TokenKind) -> Option<&'static str> {
        for &(_, py, _, ref k) in KEYWORD_TRIPLES {
            if std::mem::discriminant(kind) == std::mem::discriminant(k) {
                return Some(py);
            }
        }
        None
    }

    /// 转换为英文关键词
    pub fn to_english(&self, kind: &TokenKind) -> Option<&'static str> {
        // 特殊处理 Func 类型，始终返回 "func"
        if std::mem::discriminant(kind) == std::mem::discriminant(&TokenKind::Func) {
            return Some("func");
        }
        
        for &(_, _, en, ref k) in KEYWORD_TRIPLES {
            if std::mem::discriminant(kind) == std::mem::discriminant(k) {
                return Some(en);
            }
        }
        None
    }
}

impl Default for KeywordTable {
    fn default() -> Self {
        Self::new()
    }
}

/// 关键词风格转换器
///
/// 用于将源代码转换为指定的关键词风格
pub struct KeywordStyleConverter {
    table: KeywordTable,
}

impl KeywordStyleConverter {
    /// 创建新的转换器
    pub fn new() -> Self {
        Self {
            table: KeywordTable::new(),
        }
    }
    
    /// 判断一个 TokenKind 是否是可转换的关键词
    fn is_convertible_keyword(kind: &TokenKind) -> bool {
        matches!(
            kind,
            TokenKind::Let
                | TokenKind::Const
                | TokenKind::Be
                | TokenKind::TypeAnno
                | TokenKind::If
                | TokenKind::Then
                | TokenKind::Else
                | TokenKind::End
                | TokenKind::While
                | TokenKind::Repeat
                | TokenKind::Times
                | TokenKind::For
                | TokenKind::Each
                | TokenKind::In
                | TokenKind::Do
                | TokenKind::Begin
                | TokenKind::Func
                | TokenKind::Return
                | TokenKind::Struct
                | TokenKind::Field
                | TokenKind::Trait
                | TokenKind::Requires
                | TokenKind::Impl
                | TokenKind::ForTy
                | TokenKind::Module
                | TokenKind::Pub
                | TokenKind::Import
                | TokenKind::From
                | TokenKind::And
                | TokenKind::Or
                | TokenKind::Not
                | TokenKind::Add
                | TokenKind::Sub
                | TokenKind::Mul
                | TokenKind::Div
                | TokenKind::Mod
                | TokenKind::Gt
                | TokenKind::Lt
                | TokenKind::Eq
                | TokenKind::Ge
                | TokenKind::Le
                | TokenKind::Ne
                | TokenKind::Assign
                | TokenKind::True
                | TokenKind::False
                | TokenKind::Null
                | TokenKind::Match
                | TokenKind::Default
                | TokenKind::Break
                | TokenKind::Continue
                | TokenKind::Shl
                | TokenKind::Shr
                | TokenKind::BitAnd
                | TokenKind::BitOr
                | TokenKind::BitXor
                | TokenKind::Asm
                | TokenKind::Volatile
                | TokenKind::Global
                | TokenKind::Section
                | TokenKind::Align
                | TokenKind::Peripheral
                | TokenKind::Register
                | TokenKind::Memory
                | TokenKind::Layout
                | TokenKind::Segment
                | TokenKind::Extern
                | TokenKind::Type
                | TokenKind::Mut
                | TokenKind::TypeInt
                | TokenKind::TypeI8
                | TokenKind::TypeI16
                | TokenKind::TypeI32
                | TokenKind::TypeI64
                | TokenKind::TypeU8
                | TokenKind::TypeU16
                | TokenKind::TypeU32
                | TokenKind::TypeU64
                | TokenKind::TypeF32
                | TokenKind::TypeF64
                | TokenKind::TypeBool
                | TokenKind::TypeChar
                | TokenKind::TypeString
                | TokenKind::TypeUnit
                | TokenKind::TypeList
                | TokenKind::TypeArray
                | TokenKind::TypeMap
                | TokenKind::TypePtr
                | TokenKind::TypeOption
        )
    }
    
    /// 将单个 Token 转换为目标风格的字符串
    pub fn convert_token(&self, kind: &TokenKind, lexeme: &str, style: KeywordStyle) -> String {
        if Self::is_convertible_keyword(kind) {
            if let Some(converted) = self.table.to_style(kind, style) {
                return converted.to_string();
            }
        }
        // 不可转换或转换失败，保持原样
        lexeme.to_string()
    }
}

impl Default for KeywordStyleConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyword_lookup_chinese() {
        let table = KeywordTable::new();
        assert_eq!(table.get("令"), Some(TokenKind::Let));
        assert_eq!(table.get("若"), Some(TokenKind::If));
        assert_eq!(table.get("函数"), Some(TokenKind::Func));
        assert_eq!(table.get("真"), Some(TokenKind::True));
        assert_eq!(table.get("整数"), Some(TokenKind::TypeInt));
    }

    #[test]
    fn test_keyword_lookup_pinyin() {
        let table = KeywordTable::new();
        assert_eq!(table.get("ling"), Some(TokenKind::Let));
        assert_eq!(table.get("ruo"), Some(TokenKind::If));
        assert_eq!(table.get("hanshu"), Some(TokenKind::Func));
        assert_eq!(table.get("zhen"), Some(TokenKind::True));
        assert_eq!(table.get("zhengshu"), Some(TokenKind::TypeInt));
    }

    #[test]
    fn test_keyword_lookup_english() {
        let table = KeywordTable::new();
        assert_eq!(table.get("let"), Some(TokenKind::Let));
        assert_eq!(table.get("if"), Some(TokenKind::If));
        assert_eq!(table.get("func"), Some(TokenKind::Func));
        assert_eq!(table.get("true"), Some(TokenKind::True));
        assert_eq!(table.get("int"), Some(TokenKind::TypeInt));
    }

    #[test]
    fn test_non_keyword() {
        let table = KeywordTable::new();
        assert_eq!(table.get("xyz"), None);
        assert_eq!(table.get("123"), None);
        assert_eq!(table.get(""), None);
    }

    #[test]
    fn test_to_chinese() {
        let table = KeywordTable::new();
        assert_eq!(table.to_chinese(&TokenKind::Let), Some("令"));
        assert_eq!(table.to_chinese(&TokenKind::If), Some("若"));
        assert_eq!(table.to_chinese(&TokenKind::True), Some("真"));
    }

    #[test]
    fn test_to_pinyin() {
        let table = KeywordTable::new();
        assert_eq!(table.to_pinyin(&TokenKind::Let), Some("ling"));
        assert_eq!(table.to_pinyin(&TokenKind::If), Some("ruo"));
        assert_eq!(table.to_pinyin(&TokenKind::True), Some("zhen"));
    }

    #[test]
    fn test_to_english() {
        let table = KeywordTable::new();
        assert_eq!(table.to_english(&TokenKind::Let), Some("let"));
        assert_eq!(table.to_english(&TokenKind::If), Some("if"));
        assert_eq!(table.to_english(&TokenKind::True), Some("true"));
    }

    #[test]
    fn test_convert_token() {
        let converter = KeywordStyleConverter::new();
        
        // 中文 -> 英文
        let result = converter.convert_token(&TokenKind::Let, "令", KeywordStyle::English);
        assert_eq!(result, "let");
        
        // 英文 -> 中文
        let result = converter.convert_token(&TokenKind::If, "if", KeywordStyle::Chinese);
        assert_eq!(result, "若");
        
        // 拼音 -> 英文
        let result = converter.convert_token(&TokenKind::Func, "hanshu", KeywordStyle::English);
        assert_eq!(result, "func");
    }

    #[test]
    fn test_is_keyword() {
        let table = KeywordTable::new();
        assert!(table.is_keyword("令"));
        assert!(table.is_keyword("ling"));
        assert!(table.is_keyword("let"));
        assert!(!table.is_keyword("not_a_keyword"));
    }

    #[test]
    fn test_all_keywords_covered() {
        let table = KeywordTable::new();
        // 测试一些特殊关键词
        assert_eq!(table.get("开始"), Some(TokenKind::Begin));
        assert_eq!(table.get("跳出"), Some(TokenKind::Break));
        assert_eq!(table.get("继续"), Some(TokenKind::Continue));
        assert_eq!(table.get("指针"), Some(TokenKind::TypePtr));
        assert_eq!(table.get("可选"), Some(TokenKind::TypeOption));
    }
}

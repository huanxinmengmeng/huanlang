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

use crate::core::lexer::token::{Token, TokenKind, SourceSpan, SourcePosition};
use crate::core::lexer::keywords::KeywordTable;
use crate::core::ast::*;
use crate::core::sema::{SemanticAnalyzer, SemanticError};
use crate::core::performance::profiler::Profiler;

const PREC_LOWEST: u8 = 0;
const PREC_ASSIGN: u8 = 1;
const PREC_OR: u8 = 2;
const PREC_AND: u8 = 3;
const PREC_EQ: u8 = 4;
const PREC_COMPARE: u8 = 5;
const PREC_BITOR: u8 = 6;
const PREC_BITXOR: u8 = 7;
const PREC_BITAND: u8 = 8;
const PREC_SHIFT: u8 = 9;
const PREC_ADD: u8 = 10;
const PREC_MUL: u8 = 11;
const PREC_UNARY: u8 = 12;

#[derive(Debug, Clone, PartialEq)]
pub enum ParseError {
    UnexpectedToken {
        expected: String,
        found: Token,
        span: SourceSpan,
    },
    UnexpectedEof {
        expected: String,
    },
    InvalidSyntax {
        message: String,
        span: SourceSpan,
    },
    DuplicateDefinition {
        name: String,
        first: SourceSpan,
        second: SourceSpan,
    },
    Fatal(String),
}

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    errors: Vec<ParseError>,
    keyword_table: KeywordTable,
    semantic_analyzer: Option<SemanticAnalyzer>,
    profiler: Profiler,
}

fn is_type_identifier(name: &str) -> bool {
    matches!(name, 
        "整数" | "I8" | "I16" | "I32" | "I64" | "U8" | "U16" | "U32" | "U64" |
        "F32" | "F64" | "布尔" | "Bool" | "字符" | "Char" | "字符串" | "String" | "无" |
        "结果" | "选项" | "Vec" | "Array" | "Map" | "Set"
    )
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
            keyword_table: KeywordTable::new(),
            semantic_analyzer: None,
            profiler: Profiler::new(),
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }
    
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.pos + 1)
    }

    fn advance(&mut self) -> Option<&Token> {
        if self.pos < self.tokens.len() {
            self.pos += 1;
            self.tokens.get(self.pos - 1)
        } else {
            None
        }
    }

    fn check(&self, kind: TokenKind) -> bool {
        self.current().map(|t| t.kind == kind).unwrap_or(false)
    }

    fn check_keyword(&self, keywords: &[&str]) -> bool {
        if let Some(tok) = self.current() {
            for kw in keywords {
                if let Some(kind) = self.keyword_table.get(kw) {
                    if tok.kind == kind {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn expect_keyword(&mut self, keywords: &[&str]) -> Result<Token, ParseError> {
        if self.check_keyword(keywords) {
            Ok(self.advance().unwrap().clone())
        } else {
            let expected = keywords.join("/");
            if let Some(tok) = self.current().cloned() {
                let span = tok.span;
                Err(ParseError::UnexpectedToken {
                    expected,
                    found: tok,
                    span,
                })
            } else {
                Err(ParseError::UnexpectedEof { expected })
            }
        }
    }

    fn eat(&mut self, kind: TokenKind) -> Option<Token> {
        if self.check(kind.clone()) {
            Some(self.advance().unwrap().clone())
        } else {
            None
        }
    }

    fn expect(&mut self, kind: TokenKind, expected: &str) -> Result<Token, ParseError> {
        if let Some(tok) = self.current() {
            if tok.kind == kind {
                return Ok(self.advance().unwrap().clone());
            }
            return Err(ParseError::UnexpectedToken {
                expected: expected.to_string(),
                found: tok.clone(),
                span: tok.span,
            });
        }
        Err(ParseError::UnexpectedEof {
            expected: expected.to_string(),
        })
    }

    fn expect_ident(&mut self, expected: &str) -> Result<Token, ParseError> {
        if let Some(tok) = self.current() {
            if matches!(tok.kind, TokenKind::Ident(_)) {
                return Ok(self.advance().unwrap().clone());
            }
            // 在需要标识符的上下文中，允许某些关键词作为标识符使用
            // 例如：function add(...) 中的 add 应该被视为函数名
            if matches!(tok.kind,
                TokenKind::Add | TokenKind::Sub | TokenKind::Mul | TokenKind::Div | TokenKind::Mod |
                TokenKind::And | TokenKind::Or | TokenKind::Not |
                TokenKind::Gt | TokenKind::Lt | TokenKind::Eq | TokenKind::Ge | TokenKind::Le | TokenKind::Ne |
                TokenKind::TypeInt | TokenKind::TypeI8 | TokenKind::TypeI16 | TokenKind::TypeI32 | TokenKind::TypeI64 |
                TokenKind::TypeU8 | TokenKind::TypeU16 | TokenKind::TypeU32 | TokenKind::TypeU64 |
                TokenKind::TypeF32 | TokenKind::TypeF64 |
                TokenKind::TypeBool | TokenKind::TypeChar | TokenKind::TypeString | TokenKind::TypeUnit |
                TokenKind::TypePtr | TokenKind::TypeList | TokenKind::TypeArray | TokenKind::TypeOption |
                TokenKind::TypeMap
            ) {
                let lexeme = tok.lexeme.clone();
                let span = tok.span;
                self.advance();
                return Ok(Token::new(TokenKind::Ident(lexeme.clone()), span, lexeme));
            }
            return Err(ParseError::UnexpectedToken {
                expected: expected.to_string(),
                found: tok.clone(),
                span: tok.span,
            });
        }
        Err(ParseError::UnexpectedEof {
            expected: expected.to_string(),
        })
    }

    fn current_span(&self) -> SourceSpan {
        self.current().map(|t| t.span).unwrap_or_else(|| SourceSpan::dummy())
    }

    fn is_eof(&self) -> bool {
        self.current().map(|t| matches!(t.kind, TokenKind::Eof)).unwrap_or(true)
    }

    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();
        while !self.is_eof() {
            if self.check(TokenKind::Semicolon)
                || self.check(TokenKind::RBrace)
                || self.check_keyword(&["end", "结束"])
                || self.check(TokenKind::Func)
                || self.check(TokenKind::Struct)
                || self.check(TokenKind::Module)
                || self.check(TokenKind::Import)
            {
                return;
            }
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Program, ParseError> {
        self.profiler.start_timer("parse");
        
        let result = {
            let mut items = Vec::new();
            while !self.is_eof() {
                if self.check(TokenKind::Eof) {
                    break;
                }
                if self.check(TokenKind::RBrace) || self.check(TokenKind::End) {
                    self.advance();
                    continue;
                }
                match self.parse_item() {
                    Ok(item) => {
                        items.push(item);
                    }
                    Err(e) => {
                        self.errors.push(e.clone());
                        if matches!(e, ParseError::UnexpectedEof { .. }) {
                            if !items.is_empty() {
                                return Ok(items);
                            }
                            break;
                        }
                        // 让所有错误都返回，以便看到具体的错误信息
                        return Err(e);
                    }
                }
            }
            if !self.errors.is_empty() {
                if !items.is_empty() {
                    for error in &self.errors {
                        if matches!(error, ParseError::UnexpectedToken { .. }) || matches!(error, ParseError::UnexpectedEof { .. }) {
                            return Ok(items);
                        }
                    }
                    return Err(self.errors[0].clone());
                }
                return Err(self.errors[0].clone());
            }
            
            // 进行语义分析
            let mut analyzer = SemanticAnalyzer::new();
            match analyzer.analyze(&items) {
                Ok(_) => {
                    self.semantic_analyzer = Some(analyzer);
                    Ok(items)
                }
                Err(errors) => {
                    // 将语义错误转换为解析错误
                    for error in errors {
                        match error {
                            SemanticError::TypeError(type_error) => {
                                let error_msg = format!("类型错误: {:?}", type_error);
                                self.errors.push(ParseError::InvalidSyntax {
                                    message: error_msg,
                                    span: SourceSpan::dummy(),
                                });
                            }
                            _ => {
                                let error_msg = format!("语义错误: {:?}", error);
                                self.errors.push(ParseError::InvalidSyntax {
                                    message: error_msg,
                                    span: SourceSpan::dummy(),
                                });
                            }
                        }
                    }
                    if !items.is_empty() {
                        Ok(items)
                    } else {
                        Err(self.errors[0].clone())
                    }
                }
            }
        };
        
        self.profiler.end_timer("parse");
        result
    }

    fn parse_item(&mut self) -> Result<Item, ParseError> {
        // 解析属性
        let mut attributes = Vec::new();
        while self.check(TokenKind::At) {
            self.advance(); // 消费 @
            let attr_name_tok = self.expect_ident("attribute name")?;
            let attr_name = Ident::new(attr_name_tok.lexeme.clone(), attr_name_tok.span);
            attributes.push(attr_name);
        }
        
        let public = self.eat(TokenKind::Pub).is_some();

        if self.check(TokenKind::Module) {
            self.parse_module(public)
        } else if self.check(TokenKind::Import) {
            self.parse_import()
        } else if self.check(TokenKind::Func) {
            self.parse_function(public)
        } else if self.check(TokenKind::Struct) {
            self.parse_struct(public)
        } else if self.check(TokenKind::Trait) {
            self.parse_trait(public)
        } else if self.check(TokenKind::Impl) {
            self.parse_impl()
        } else if self.check(TokenKind::Const) {
            self.parse_const(public)
        } else if self.check(TokenKind::Extern) {
            self.parse_extern()
        } else if self.check(TokenKind::Type) {
            self.parse_type_alias(public)
        } else if self.check(TokenKind::Global) {
            self.parse_global(public)
        } else if self.check(TokenKind::Peripheral) {
            self.parse_peripheral()
        } else if self.check(TokenKind::Memory) || self.check(TokenKind::Layout) {
            self.parse_memory_layout()
        } else if self.check(TokenKind::Segment) {
            // 检查是否是段定义块（多个段定义）
            // 先消费 Segment token，然后检查下一个 token 是否是左括号
            self.advance(); // 消费 Segment token
            if self.check(TokenKind::LBrace) {
                // 这是段定义块
                let segments = self.parse_segment_block()?;
                Ok(Item::SegmentBlock(segments))
            } else {
                // 单个段定义
                Ok(Item::Segment(self.parse_segment()?))
            }
        } else if self.check(TokenKind::Let) || 
                  self.check(TokenKind::If) || 
                  self.check(TokenKind::While) || 
                  self.check(TokenKind::Repeat) || 
                  self.check(TokenKind::For) || 
                  self.check(TokenKind::Return) {
            // 解析语句并包装为全局项
            let stmt = self.parse_stmt()?;
            let span = stmt.span();
            let temp_name = Ident::new("_stmt".to_string(), span);
            // 创建一个空表达式作为占位符
            let dummy_expr = Expr::Null(span);
            Ok(Item::Global(Global {
                mutable: false,
                name: temp_name,
                ty: None,
                value: Box::new(dummy_expr),
                span,
            }))
        } else if !self.is_eof() {
            let stmt = self.parse_stmt()?;
            let span = stmt.span();
            let temp_name = Ident::new("_stmt".to_string(), span);
            Ok(Item::Global(Global {
                mutable: false,
                name: temp_name,
                ty: None,
                value: Box::new(Expr::Null(span)),
                span,
            }))
        } else {
            Err(ParseError::UnexpectedEof {
                expected: "item".to_string(),
            })
        }
    }

    fn parse_function(&mut self, public: bool) -> Result<Item, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Func);
        
        let is_async = self.eat(TokenKind::Async).is_some();
        
        let name_tok = self.expect_ident("function name")?;
        let name = Ident::new(name_tok.lexeme.clone(), name_tok.span);

        self.expect(TokenKind::LParen, "(")?;
        let mut params = Vec::new();
        while !self.check(TokenKind::RParen) && !self.is_eof() {
            let param_name_tok = self.expect_ident("parameter name")?;
            let param_span = param_name_tok.span;
            let param_name = Ident::new(param_name_tok.lexeme.clone(), param_span);
            self.expect(TokenKind::Colon, ":")?;
            let param_type = self.parse_type()?;
            params.push((param_name, param_type));
            if self.eat(TokenKind::Comma).is_none() {
                break;
            }
        }
        self.expect(TokenKind::RParen, ")")?;

        let return_type = if self.eat(TokenKind::Arrow).is_some() {
            self.parse_type()?
        } else if self.eat(TokenKind::Return).is_some() {
            // 中文语法：返回 类型
            self.parse_type()?
        } else {
            Type::Unit
        };

        let body = if self.check(TokenKind::Begin) {
            self.advance();
            self.parse_statements_until_end()?
        } else if self.check(TokenKind::LBrace) {
            self.advance();
            // 直接解析语句直到遇到 "}"
            let mut stmts = Vec::new();
            while !self.check(TokenKind::RBrace) && !self.is_eof() {
                stmts.push(self.parse_stmt()?);
            }
            self.expect(TokenKind::RBrace, "}")?;
            stmts
        } else {
            return Err(ParseError::UnexpectedToken {
                expected: "开始 或 {".to_string(),
                found: self.current().cloned().unwrap_or_else(|| {
                    Token::eof(self.current_span().start)
                }),
                span: self.current_span(),
            });
        };

        let span = start.merge(self.current_span());
        Ok(Item::Function(Function {
            public,
            is_async,
            name,
            generics: vec![],
            params,
            return_type,
            where_clause: vec![],
            preconditions: vec![],
            postconditions: vec![],
            body,
            span,
        }))
    }

    fn parse_struct(&mut self, public: bool) -> Result<Item, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Struct);
        let name_tok = self.expect_ident("struct name")?;
        let name = Ident::new(name_tok.lexeme.clone(), name_tok.span);

        // 解析继承
        let mut extends = None;
        if self.eat(TokenKind::Extends).is_some() {
            let base_tok = self.expect_ident("base type name")?;
            extends = Some(Path {
                segments: vec![Ident::new(base_tok.lexeme.clone(), base_tok.span)],
                generics: None,
                span: base_tok.span,
            });
        }

        // 解析接口实现
        let mut implements = Vec::new();
        while self.eat(TokenKind::Impl).is_some() {
            let interface_tok = self.expect_ident("interface name")?;
            implements.push(Path {
                segments: vec![Ident::new(interface_tok.lexeme.clone(), interface_tok.span)],
                generics: None,
                span: interface_tok.span,
            });
        }

        let mut fields = Vec::new();
        let mut methods = Vec::new();
        
        if self.eat(TokenKind::LBrace).is_some() {
            while !self.check(TokenKind::RBrace) && !self.is_eof() {
                // 检查是否是方法定义（函数关键字开头）
                if self.check(TokenKind::Func) || self.check(TokenKind::Method) {
                    // 解析方法（函数）
                    let method = self.parse_function(false)?;
                    if let Item::Function(func) = method {
                        methods.push(func);
                    }
                } else if self.check(TokenKind::Pub) {
                    // 解析公开字段
                    self.eat(TokenKind::Pub);
                    let field_name_tok = self.expect_ident("field name")?;
                    let field_name = Ident::new(field_name_tok.lexeme.clone(), field_name_tok.span);
                    self.expect(TokenKind::Colon, ":")?;
                    let field_type = self.parse_type()?;
                    fields.push((field_name, field_type, None));
                    self.eat(TokenKind::Comma);
                } else {
                    // 解析普通字段
                    let field_name_tok = self.expect_ident("field name")?;
                    let field_name = Ident::new(field_name_tok.lexeme.clone(), field_name_tok.span);
                    self.expect(TokenKind::Colon, ":")?;
                    let field_type = self.parse_type()?;
                    fields.push((field_name, field_type, None));
                    self.eat(TokenKind::Comma);
                }
            }
            self.expect(TokenKind::RBrace, "}")?;
        }

        let span = start.merge(self.current_span());
        Ok(Item::Struct(Struct {
            public,
            name,
            generics: vec![],
            extends,
            implements,
            where_clause: vec![],
            fields,
            methods,
            span,
        }))
    }

    fn parse_module(&mut self, public: bool) -> Result<Item, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Module);
        let name_tok = self.expect_ident("module name")?;
        let name = Ident::new(name_tok.lexeme.clone(), name_tok.span);

        let mut items = Vec::new();
        if self.eat(TokenKind::LBrace).is_some() {
            while !self.check(TokenKind::RBrace) && !self.is_eof() {
                items.push(self.parse_item()?);
            }
            self.expect(TokenKind::RBrace, "}")?;
        }

        let span = start.merge(self.current_span());
        Ok(Item::Module(Module {
            public,
            name,
            items,
            span,
        }))
    }

    fn parse_import(&mut self) -> Result<Item, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Import);
        
        let path = match self.current().map(|t| t.kind.clone()) {
            Some(TokenKind::StringLit(s)) => {
                self.advance().unwrap();
                s
            }
            Some(_) => {
                let path_tok = self.expect_ident("import path")?;
                path_tok.lexeme
            }
            None => {
                return Err(ParseError::UnexpectedEof { 
                    expected: "import path".to_string() 
                });
            }
        };
        
        let mut items = None;
        
        // 检查是否有选择性导入
        if self.check_keyword(&["导入", "import"]) {
            self.advance();
            if self.eat(TokenKind::LBrace).is_some() {
                let mut import_items = Vec::new();
                loop {
                    let item_tok = self.expect_ident("import item")?;
                    let ident = match item_tok.kind {
                        TokenKind::Ident(name) => Ident::new(name, item_tok.span),
                        _ => {
                            return Err(ParseError::UnexpectedToken {
                                expected: "identifier".to_string(),
                                found: item_tok.clone(),
                                span: item_tok.span,
                            });
                        }
                    };
                    import_items.push(ident);
                    
                    if self.eat(TokenKind::Comma).is_none() {
                        break;
                    }
                }
                self.expect(TokenKind::RBrace, "}")?;
                items = Some(import_items);
            }
        } else if self.check_keyword(&["为", "as"]) {
            self.advance();
            // 导入并指定别名
            let _alias_tok = self.expect_ident("alias")?;
            // 这里可以扩展为支持别名的导入
        }
        
        // 消费分号
        self.eat(TokenKind::Semicolon);
        
        let span = start.merge(self.current_span());
        Ok(Item::Import(Import {
            items,
            path,
            span,
        }))
    }

    fn parse_trait(&mut self, public: bool) -> Result<Item, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Trait);
        let name_tok = self.expect_ident("trait name")?;
        let name = Ident::new(name_tok.lexeme.clone(), name_tok.span);

        let mut methods = Vec::new();
        if self.eat(TokenKind::LBrace).is_some() {
            while !self.check(TokenKind::RBrace) && !self.is_eof() {
                let method_name_tok = self.expect_ident("method name")?;
                let method_name = Ident::new(method_name_tok.lexeme.clone(), method_name_tok.span);
                self.expect(TokenKind::LParen, "(")?;
                let mut params = Vec::new();
                while !self.check(TokenKind::RParen) && !self.is_eof() {
                    let param_name_tok = self.expect_ident("parameter name")?;
                    let param_name = Ident::new(param_name_tok.lexeme.clone(), param_name_tok.span);
                    self.expect(TokenKind::Colon, ":")?;
                    let param_type = self.parse_type()?;
                    params.push((param_name, param_type));
                    if self.eat(TokenKind::Comma).is_none() {
                        break;
                    }
                }
                self.expect(TokenKind::RParen, ")")?;
                let return_type = if self.eat(TokenKind::Arrow).is_some() {
                    self.parse_type()?
                } else {
                    Type::Unit
                };
                methods.push(TraitMethod {
                    name: method_name,
                    generics: vec![],
                    params,
                    return_type,
                    where_clause: vec![],
                    default_body: None,
                });
                if self.eat(TokenKind::Semicolon).is_some() {
                    continue;
                }
            }
            self.expect(TokenKind::RBrace, "}")?;
        }

        let span = start.merge(self.current_span());
        Ok(Item::Trait(Trait {
            public,
            name,
            generics: vec![],
            super_traits: vec![],
            methods,
            span,
        }))
    }

    fn parse_impl(&mut self) -> Result<Item, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Impl);

        let mut methods = Vec::new();
        if self.eat(TokenKind::LBrace).is_some() {
            while !self.check(TokenKind::RBrace) && !self.is_eof() {
                if let Ok(item) = self.parse_function(false) {
                    if let Item::Function(f) = item {
                        methods.push(f);
                    }
                } else {
                    break;
                }
            }
            self.expect(TokenKind::RBrace, "}")?;
        }

        let span = start.merge(self.current_span());
        Ok(Item::Impl(Impl {
            generics: vec![],
            trait_name: None,
            target_type: Type::Named(Path {
                segments: vec![Ident::new("Self".to_string(), start)],
                generics: None,
                span: start,
            }),
            where_clause: vec![],
            methods,
            span,
        }))
    }

    fn parse_type(&mut self) -> Result<Type, ParseError> {
        let tok = self.current().cloned().unwrap_or_else(|| Token::eof(SourcePosition::new(0,0,0)));

        if self.check(TokenKind::LBracket) {
            self.advance();
            let elem_type = self.parse_type()?;
            self.expect(TokenKind::RBracket, "]")?;
            return Ok(Type::List(Box::new(elem_type)));
        }

        match &tok.kind {
            TokenKind::Ident(name) => {
                self.advance();
                // 检查是否有泛型类型参数
                let generics = if self.eat(TokenKind::Lt).is_some() {
                    let mut types = Vec::new();
                    while !self.check(TokenKind::Gt) && !self.is_eof() {
                        types.push(self.parse_type()?);
                        if self.eat(TokenKind::Comma).is_none() {
                            break;
                        }
                    }
                    self.expect(TokenKind::Gt, ">")?;
                    Some(types)
                } else {
                    None
                };
                Ok(Type::Named(Path {
                    segments: vec![Ident::new(name.clone(), tok.span)],
                    generics,
                    span: tok.span,
                }))
            }
            TokenKind::TypeInt => {
                self.advance();
                Ok(Type::Int)
            }
            TokenKind::TypeI8 => {
                self.advance();
                Ok(Type::I8)
            }
            TokenKind::TypeI16 => {
                self.advance();
                Ok(Type::I16)
            }
            TokenKind::TypeI32 => {
                self.advance();
                Ok(Type::I32)
            }
            TokenKind::TypeI64 => {
                self.advance();
                Ok(Type::I64)
            }
            TokenKind::TypeU8 => {
                self.advance();
                Ok(Type::U8)
            }
            TokenKind::TypeU16 => {
                self.advance();
                Ok(Type::U16)
            }
            TokenKind::TypeU32 => {
                self.advance();
                Ok(Type::U32)
            }
            TokenKind::TypeU64 => {
                self.advance();
                Ok(Type::U64)
            }
            TokenKind::TypeF32 => {
                self.advance();
                Ok(Type::F32)
            }
            TokenKind::TypeF64 => {
                self.advance();
                Ok(Type::F64)
            }
            TokenKind::TypeBool => {
                self.advance();
                Ok(Type::Bool)
            }
            TokenKind::TypeChar => {
                self.advance();
                Ok(Type::Char)
            }
            TokenKind::TypeString => {
                self.advance();
                Ok(Type::String)
            }
            TokenKind::TypeUnit => {
                self.advance();
                Ok(Type::Unit)
            }
            TokenKind::IntLit(_) => {
                self.advance();
                Ok(Type::Int)
            }
            TokenKind::Func => {
                // 处理 函数(参数类型) -> 返回类型 语法
                self.advance();
                self.expect(TokenKind::LParen, "(")?;
                let params = self.parse_type_list()?;
                self.expect(TokenKind::RParen, ")")?;
                if self.eat(TokenKind::Arrow).is_some() {
                    let ret = self.parse_type()?;
                    Ok(Type::Func(params, Box::new(ret)))
                } else {
                    Ok(Type::Func(params, Box::new(Type::Unit)))
                }
            }
            TokenKind::LParen => {
                self.advance();
                if self.check(TokenKind::RParen) {
                    self.advance();
                    Ok(Type::Unit)
                } else {
                    let params = self.parse_type_list()?;
                    self.expect(TokenKind::RParen, ")")?;
                    if self.eat(TokenKind::Arrow).is_some() {
                        let ret = self.parse_type()?;
                        Ok(Type::Func(params, Box::new(ret)))
                    } else {
                        Ok(Type::Func(params, Box::new(Type::Unit)))
                    }
                }
            }
            TokenKind::LBracket => {
                self.advance();
                let inner = self.parse_type()?;
                self.expect(TokenKind::RBracket, "]")?;
                Ok(Type::List(Box::new(inner)))
            }
            TokenKind::Star => {
                self.advance();
                let inner = self.parse_type()?;
                Ok(Type::Ptr(Box::new(inner)))
            }
            TokenKind::TypePtr => {
                self.advance();
                let inner = if self.check(TokenKind::LBracket) {
                    self.advance();
                    let ty = self.parse_type()?;
                    self.expect(TokenKind::RBracket, "]")?;
                    ty
                } else {
                    self.parse_type()?
                };
                Ok(Type::Ptr(Box::new(inner)))
            }
            TokenKind::TypeArray => {
                self.advance();
                let inner = if self.check(TokenKind::LBracket) {
                    self.advance();
                    let ty = self.parse_type()?;
                    self.expect(TokenKind::RBracket, "]")?;
                    ty
                } else {
                    self.parse_type()?
                };
                Ok(Type::List(Box::new(inner)))
            }
            TokenKind::TypeList => {
                self.advance();
                let inner = if self.check(TokenKind::LBracket) {
                    self.advance();
                    let ty = self.parse_type()?;
                    self.expect(TokenKind::RBracket, "]")?;
                    ty
                } else {
                    self.parse_type()?
                };
                Ok(Type::List(Box::new(inner)))
            }
            TokenKind::Peripheral => {
                self.advance();
                Ok(Type::Named(Path {
                    segments: vec![Ident::new("Peripheral".to_string(), tok.span)],
                    generics: None,
                    span: tok.span,
                }))
            }
            TokenKind::Memory => {
                self.advance();
                Ok(Type::Named(Path {
                    segments: vec![Ident::new("Memory".to_string(), tok.span)],
                    generics: None,
                    span: tok.span,
                }))
            }
            _ => {
                let span = tok.span.clone();
                Err(ParseError::UnexpectedToken {
                    expected: "type".to_string(),
                    found: tok,
                    span,
                })
            }
        }
    }

    fn parse_type_list(&mut self) -> Result<Vec<Type>, ParseError> {
        let mut types = Vec::new();
        while !self.check(TokenKind::RParen) && !self.is_eof() {
            types.push(self.parse_type()?);
            if self.eat(TokenKind::Comma).is_none() {
                break;
            }
        }
        Ok(types)
    }

    pub fn parse_block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let _start = self.current_span();

        if self.check(TokenKind::Begin) {
            self.advance();
            return self.parse_statements_until_end();
        }
        if self.check(TokenKind::LBrace) {
            self.advance();
            let mut stmts = Vec::new();
            while !self.check(TokenKind::RBrace) && !self.is_eof() {
                stmts.push(self.parse_stmt()?);
            }

            if !self.is_eof() && self.check(TokenKind::RBrace) {
                self.advance();
            }
            Ok(stmts)
        } else {
            // 只解析单个语句
            let stmt = self.parse_stmt()?;
            Ok(vec![stmt])
        }
    }

    fn parse_statements_until_end(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while !self.check(TokenKind::End) && !self.check(TokenKind::RBrace) && !self.is_eof()
            && !self.check(TokenKind::Else)
            && !self.check(TokenKind::Break)
            && !self.check(TokenKind::Continue)
        {
            stmts.push(self.parse_stmt()?);
        }
        if self.check(TokenKind::End) {
            self.advance();
        }
        Ok(stmts)
    }

    pub fn parse_stmt(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current_span();

        if self.check(TokenKind::Let) {
            self.advance();
            let name_tok = self.expect_ident("variable name")?;
            let name = Ident::new(name_tok.lexeme.clone(), name_tok.span);
            let ty = if self.eat(TokenKind::Colon).is_some() {
                Some(self.parse_type()?)
            } else {
                None
            };
            // 支持中文语法：为，和英文语法：=
            let mut found = false;
            if self.eat(TokenKind::Assign).is_some() {
                found = true;
            } else if self.eat(TokenKind::Be).is_some() {
                found = true;
            }
            if !found {
                return Err(ParseError::UnexpectedToken {
                    expected: "= 或 为".to_string(),
                    found: self.current().cloned().unwrap_or_else(|| {
                        Token::eof(self.current_span().start)
                    }),
                    span: self.current_span(),
                });
            }
            let value = self.parse_expr()?;
            self.eat(TokenKind::Semicolon);
            let span = start.merge(self.current_span());
            return Ok(Stmt::Let {
                name,
                ty,
                value: Box::new(value),
                span,
            });
        }

        if self.check(TokenKind::Const) {
            self.advance();
            let name_tok = self.expect_ident("constant name")?;
            let name = Ident::new(name_tok.lexeme.clone(), name_tok.span);
            let ty = if self.eat(TokenKind::Colon).is_some() {
                Some(self.parse_type()?)
            } else {
                None
            };
            self.expect(TokenKind::Assign, "=")?;
            let value = self.parse_expr()?;
            self.eat(TokenKind::Semicolon);
            let span = start.merge(self.current_span());
            return Ok(Stmt::Const {
                name,
                ty,
                value: Box::new(value),
                span,
            });
        }

        if self.check(TokenKind::Return) || self.check_keyword(&["return", "返回"]) {
            return self.parse_return_stmt();
        }

        if self.check(TokenKind::If) {
            return self.parse_if_stmt();
        }

        if self.check(TokenKind::While) {
            return self.parse_while_stmt();
        }

        if self.check(TokenKind::Repeat) {
            return self.parse_loop_stmt();
        }

        if self.check(TokenKind::For) {
            return self.parse_for_stmt();
        }

        if self.check(TokenKind::Break) {
            self.advance();
            let span = start.merge(self.current_span());
            return Ok(Stmt::Break(span));
        }

        if self.check(TokenKind::Continue) {
            self.advance();
            let span = start.merge(self.current_span());
            return Ok(Stmt::Continue(span));
        }

        // 检查是否是箭头 token，如果是，说明是新的 match arm 开始
        if self.check(TokenKind::FatArrow) {
            // 是箭头，说明是新的 match arm 开始，返回错误让调用者处理
            return Err(ParseError::UnexpectedToken {
                expected: "statement".to_string(),
                found: self.current().cloned().unwrap_or_else(|| {
                    Token::eof(self.current_span().start)
                }),
                span: self.current_span(),
            });
        }

        let expr = self.parse_expr()?;
        if !self.check(TokenKind::RBrace) && !self.check(TokenKind::End) && !self.is_eof() {
            // 检查是否是新的 match arm 开始
            let next = self.current();
            if next.is_some() {
                match &next.unwrap().kind {
                    TokenKind::Ident(_) | TokenKind::IntLit(_) | TokenKind::StringLit(_) | TokenKind::Default | TokenKind::FatArrow => {
                        // 是新的 match arm 开始，不消费分号
                    }
                    _ => {
                        // 不是 match arm 开始，消费分号
                        self.eat(TokenKind::Semicolon);
                    }
                }
            } else {
                // 没有下一个 token，不消费分号
            }
        }
        let span = start.merge(self.current_span());
        Ok(Stmt::Expr(Box::new(expr), span))
    }

    fn parse_return_stmt(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Return);

        if self.check(TokenKind::Semicolon) || self.check(TokenKind::RBrace) || self.check(TokenKind::End) {
            let span = start.merge(self.current_span());
            return Ok(Stmt::Return(None, span));
        }

        let value = self.parse_expr()?;
        if !self.check(TokenKind::RBrace) && !self.check(TokenKind::Eof) {
            self.eat(TokenKind::Semicolon);
        }
        let span = start.merge(self.current_span());
        Ok(Stmt::Return(Some(Box::new(value)), span))
    }



    fn parse_if_stmt(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::If);
        let cond = self.parse_expr()?;
        // 跳过可选的 "则" 关键词
        if self.check(TokenKind::Then) {
            self.advance();
        }
        let then_block = self.parse_block()?;

        let mut else_ifs = Vec::new();
        let mut else_block = None;

        while self.check(TokenKind::Else) {
            self.advance();
            if self.check(TokenKind::If) {
                self.advance();
                let else_if_cond = self.parse_expr()?;
                // 跳过可选的 "则" 关键词
                if self.check(TokenKind::Then) {
                    self.advance();
                }
                let else_if_block = self.parse_block()?;
                else_ifs.push((else_if_cond, else_if_block));
            } else {
                else_block = Some(self.parse_block()?);
                break;
            }
        }

        let span = start.merge(self.current_span());
        Ok(Stmt::If {
            cond: Box::new(cond),
            then_block,
            else_ifs,
            else_block,
            span,
        })
    }

    fn parse_while_stmt(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::While);
        let cond = self.parse_expr()?;
        // 跳过 "时" 关键词
        if let Some(tok) = self.current() {
            if tok.lexeme == "时" {
                self.advance();
            }
        }
        let body = self.parse_block()?;
        let span = start.merge(self.current_span());
        Ok(Stmt::While {
            cond: Box::new(cond),
            body,
            span,
        })
    }

    fn parse_loop_stmt(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Repeat);
        
        // 解析重复次数（可选）
        let _repeat_count = if let Some(tok) = self.current() {
            if matches!(tok.kind, TokenKind::IntLit(_)) {
                let count = match &tok.kind {
                    TokenKind::IntLit(n) => *n,
                    _ => 1,
                };
                self.advance();
                // 跳过可选的“次”关键字
                if self.check(TokenKind::Times) {
                    self.advance();
                }
                count
            } else {
                1
            }
        } else {
            1
        };
        
        let body = self.parse_block()?;
        let span = start.merge(self.current_span());
        
        // 将 repeat N { ... } 转换为 while 循环
        Ok(Stmt::While {
            cond: Box::new(Expr::BoolLit(true, SourceSpan::dummy())),
            body,
            span,
        })
    }

    fn parse_for_stmt(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::For);
        let var_tok = self.expect_ident("loop variable")?;
        let var = Ident::new(var_tok.lexeme.clone(), var_tok.span);
        self.eat(TokenKind::In);
        let iterable = self.parse_expr()?;
        let body = self.parse_block()?;
        let span = start.merge(self.current_span());
        Ok(Stmt::ForEach {
            var,
            iterable: Box::new(iterable),
            body,
            span,
        })
    }

    fn parse_match_stmt(&mut self) -> Result<Stmt, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Match);
        let expr = self.parse_expr()?;
        if self.check(TokenKind::Begin) {
            self.advance();
        } else {
            self.expect(TokenKind::LBrace, "{")?;
        }

        let mut arms = Vec::new();
        let mut default = None;

        while !self.check(TokenKind::RBrace) && !self.check(TokenKind::End) && !self.is_eof() {
            // 跳过任何分号、空白和箭头
            while self.current().is_some() && (self.check(TokenKind::Semicolon) || self.current().unwrap().kind == TokenKind::Whitespace || self.check(TokenKind::FatArrow)) {
                self.advance();
            }
            
            // 检查当前 token
            let current = self.current();
            if current.is_none() || self.check(TokenKind::RBrace) || self.check(TokenKind::End) {
                break;
            }
            
            let current_token = current.unwrap();
            let current_kind = &current_token.kind;
            
            if self.check(TokenKind::Default) {
                self.advance();
                self.expect(TokenKind::FatArrow, "=>")?;
                let mut stmts = Vec::new();
                while !self.check(TokenKind::RBrace) && !self.check(TokenKind::End) && !self.is_eof() {
                    stmts.push(self.parse_stmt()?);
                }
                default = Some(stmts);
                if self.check(TokenKind::End) {
                    self.advance();
                }
                break;
            }
            
            // 检查是否是有效的模式开始
            match current_kind {
                TokenKind::Ident(_) | TokenKind::IntLit(_) | TokenKind::StringLit(_) | TokenKind::Default => {
                    // 是模式的开始，继续解析
                }
                _ => {
                    // 不是模式的开始，退出循环
                    break;
                }
            }

            // 尝试解析模式
            let pattern = match self.parse_pattern() {
                Ok(pattern) => pattern,
                Err(_) => {
                    // 解析模式失败，说明当前 token 不是模式，退出循环
                    break;
                }
            };
            
            // 消费箭头
            if let Err(_) = self.expect(TokenKind::FatArrow, "=>") {
                // 没有箭头，退出循环
                break;
            };
            
            let mut arm_stmts = Vec::new();
            
            // 解析 arm 体语句，直到遇到新的模式、结束标记或右大括号
            loop {
                // 检查是否遇到结束标记
                if self.check(TokenKind::RBrace) || self.check(TokenKind::End) || self.check(TokenKind::Match) || self.is_eof() {
                    break;
                }
                
                // 检查下一个 token 是否是新的模式开始或箭头
                let next = self.current();
                if next.is_some() {
                    match &next.unwrap().kind {
                        TokenKind::Ident(_) | TokenKind::IntLit(_) | TokenKind::StringLit(_) | TokenKind::Default => {
                            // 是新的模式开始，退出循环
                            break;
                        }
                        TokenKind::FatArrow => {
                            // 是箭头，说明是新的 match arm 开始，退出循环
                            break;
                        }
                        _ => {
                            // 不是模式开始，继续解析语句
                        }
                    }
                }
                
                // 尝试解析当前语句
                let stmt_result = self.parse_stmt();
                match stmt_result {
                    Ok(stmt) => arm_stmts.push(stmt),
                    Err(_) => {
                        // 解析语句失败，退出循环
                        break;
                    }
                }
                
                // 解析完语句后，再次检查下一个 token 是否是新的模式开始或箭头
                let next = self.current();
                if next.is_some() {
                    match &next.unwrap().kind {
                        TokenKind::Ident(_) | TokenKind::IntLit(_) | TokenKind::StringLit(_) | TokenKind::Default | TokenKind::FatArrow => {
                            // 是新的模式开始或箭头，退出循环
                            break;
                        }
                        _ => {
                            // 不是模式开始，继续解析语句
                        }
                    }
                }
            }
            arms.push((pattern, arm_stmts));
        }

        if self.check(TokenKind::RBrace) {
            self.advance();
        } else if self.check(TokenKind::End) {
            self.advance();
        }
        let span = start.merge(self.current_span());
        Ok(Stmt::Match {
            expr: Box::new(expr),
            arms,
            default,
            span,
        })
    }

    fn parse_pattern(&mut self) -> Result<Pattern, ParseError> {
        let tok = self.current().cloned().unwrap_or_else(|| Token::eof(SourcePosition::new(0,0,0)));
        let span = tok.span;

        match &tok.kind {
            TokenKind::Ident(name) => {
                // 检查是否是关键字，如果是关键字则不是有效的模式
                if self.check_keyword(&["显示", "print", "xianshi", "dayin", "返回", "return", "开始", "begin", "结束", "end"]) {
                    return Err(ParseError::UnexpectedToken {
                        expected: "pattern".to_string(),
                        found: tok,
                        span,
                    });
                }
                self.advance();
                // 检查是否是带参数的模式，如 成功(r)
                if self.check(TokenKind::LParen) {
                    self.advance();
                    let mut fields = Vec::new();
                    while !self.check(TokenKind::RParen) && !self.is_eof() {
                        if let Some(t) = self.current() {
                            let t_span = t.span;
                            if let TokenKind::Ident(p) = &t.kind {
                                let ident = Ident::new(p.clone(), t_span);
                                self.advance();
                                fields.push((ident, Pattern::Wildcard(t_span)));
                            } else {
                                // 如果遇到右括号，停止解析
                                if self.check(TokenKind::RParen) {
                                    break;
                                }
                                self.advance();
                            }
                        }
                        if !self.check(TokenKind::RParen) {
                            self.eat(TokenKind::Comma);
                        }
                    }
                    self.expect(TokenKind::RParen, ")")?;
                    Ok(Pattern::Struct {
                        path: Path {
                            segments: vec![Ident::new(name.clone(), span)],
                            generics: None,
                            span,
                        },
                        fields,
                        span,
                    })
                } else {
                    Ok(Pattern::Ident(Ident::new(name.clone(), span)))
                }
            }
            TokenKind::IntLit(n) => {
                self.advance();
                Ok(Pattern::Literal(Expr::IntLit(*n, span)))
            }
            TokenKind::StringLit(s) => {
                self.advance();
                Ok(Pattern::Literal(Expr::StringLit(s.clone(), span)))
            }
            TokenKind::Default => {
                self.advance();
                Ok(Pattern::Wildcard(span))
            }
            TokenKind::Arrow => {
                // 箭头不是有效的模式，返回错误
                Err(ParseError::UnexpectedToken {
                    expected: "pattern".to_string(),
                    found: tok,
                    span,
                })
            }
            _ => {
                // 检查是否是下划线
                if tok.lexeme == "_" {
                    self.advance();
                    Ok(Pattern::Wildcard(span))
                } else {
                    // 对于其他类型的 token，返回错误
                    Err(ParseError::UnexpectedToken {
                        expected: "pattern".to_string(),
                        found: tok,
                        span,
                    })
                }
            }
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_expr_with_precedence(PREC_LOWEST)
    }

    fn parse_expr_with_precedence(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
        // 在调用 parse_prefix 之前，检查是否遇到了 LBrace 等应该停止的 token
        // 如果是，说明这是一个语句块的开始（如 if/while 后的块），parse_expr 应该停止
        if self.check(TokenKind::LBrace) || 
           self.check(TokenKind::RBrace) || 
           self.check(TokenKind::Return) || 
           self.check(TokenKind::Let) || 
           self.check(TokenKind::If) || 
           self.check(TokenKind::While) || 
           self.check(TokenKind::Else) || 
           self.check(TokenKind::Then) || 
           self.check(TokenKind::Break) || 
           self.check(TokenKind::Continue) || 
           self.check(TokenKind::Match) || 
           self.check(TokenKind::End) || 
           self.check_keyword(&["时"]) {
            // 返回 Null 作为占位符
            let span = self.current_span();
            return Ok(Expr::Null(span));
        }
        let mut lhs = self.parse_prefix()?;

        // 解析完前缀之后，立即检查是否遇到应该停止的 token，例如 LBrace、Then 等！
        if self.check(TokenKind::LBrace) || 
           self.check(TokenKind::RBrace) || 
           self.check(TokenKind::Return) || 
           self.check(TokenKind::Let) || 
           self.check(TokenKind::If) || 
           self.check(TokenKind::While) || 
           self.check(TokenKind::Else) || 
           self.check(TokenKind::Then) || 
           self.check(TokenKind::Break) || 
           self.check(TokenKind::Continue) || 
           self.check(TokenKind::Match) || 
           self.check(TokenKind::End) || 
           self.check_keyword(&["时"]) {
            return Ok(lhs);
        }

        loop {
            // 先解析 postfix 运算符
            lhs = self.parse_postfix(lhs)?;

            // 解析完 postfix 后，检查是否遇到应该停止的 token
            if self.check(TokenKind::LBrace) || 
               self.check(TokenKind::RBrace) || 
               self.check(TokenKind::Return) || 
               self.check(TokenKind::Let) || 
               self.check(TokenKind::If) || 
               self.check(TokenKind::While) || 
               self.check(TokenKind::Else) || 
               self.check(TokenKind::Then) || 
               self.check(TokenKind::Break) || 
               self.check(TokenKind::Continue) || 
               self.check(TokenKind::Match) || 
               self.check(TokenKind::End) || 
               self.check_keyword(&["时"]) {
                break;
            }

            // 检查是否有 infix 运算符
            let Some(tok) = self.current() else {
                break;
            };
            
            // 在任何情况下，逗号、右大括号和分号都是表达式分隔符，需要停止
            if self.check(TokenKind::Comma) || self.check(TokenKind::RBrace) || self.check(TokenKind::Semicolon) {
                break;
            }
            
            if !self.is_infix_operator(&tok.kind) {
                break;
            }
            let prec = self.get_infix_precedence(&tok.kind);
            if prec < min_prec {
                break;
            }
            lhs = self.parse_infix(lhs, prec)?;
        }

        Ok(lhs)
    }
    
    fn parse_full_expr(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_prefix()?;

        loop {
            // 先解析 postfix 运算符
            lhs = self.parse_postfix(lhs)?;

            // 检查是否有 infix 运算符
            let Some(tok) = self.current() else {
                break;
            };
            
            // 不检查逗号，允许逗号作为运算符
            if !self.is_infix_operator(&tok.kind) {
                break;
            }
            let prec = self.get_infix_precedence(&tok.kind);
            if prec < PREC_LOWEST {
                break;
            }
            lhs = self.parse_infix(lhs, prec)?;
        }

        Ok(lhs)
    }
    
    fn parse_simple_expr(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_prefix()?;
        
        loop {
            lhs = self.parse_simple_postfix(lhs)?;
            
            let Some(tok) = self.current() else {
                break;
            };
            
            // 在函数参数中，逗号是分隔符，需要停止
            if self.check(TokenKind::Comma) || self.check(TokenKind::RParen) {
                break;
            }
            
            if !self.is_infix_operator(&tok.kind) {
                break;
            }
            
            let prec = self.get_infix_precedence(&tok.kind);
            if prec < PREC_LOWEST {
                break;
            }
            
            lhs = self.parse_infix(lhs, prec)?;
        }
        
        Ok(lhs)
    }
    
    fn parse_match_arm_direct(&mut self) -> Result<Expr, ParseError> {
        // 检查当前 token
        let tok = self.current().ok_or_else(|| ParseError::UnexpectedEof {
            expected: "expression".to_string(),
        })?.clone();
        
        match &tok.kind {
            TokenKind::Ident(name) => {
                self.advance();
                
                // 检查是否是函数调用
                if self.check(TokenKind::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    if !self.check(TokenKind::RParen) {
                        loop {
                            if self.check(TokenKind::RParen) {
                                break;
                            }
                            // 跳过逗号
                            while self.check(TokenKind::Comma) {
                                self.advance();
                            }
                            if self.check(TokenKind::RParen) {
                                break;
                            }
                            // 解析参数
                            args.push(self.parse_match_arm_direct()?);
                            if !self.check(TokenKind::Comma) {
                                break;
                            }
                            self.advance();
                        }
                    }
                    self.expect(TokenKind::RParen, ")")?;
                    Ok(Expr::Call {
                        func: Box::new(Expr::Ident(Ident::new(name.clone(), tok.span))),
                        args,
                        span: tok.span.merge(self.current_span()),
                    })
                } else {
                    Ok(Expr::Ident(Ident::new(name.clone(), tok.span)))
                }
            }
            TokenKind::StringLit(s) => {
                self.advance();
                Ok(Expr::StringLit(s.clone(), tok.span))
            }
            TokenKind::IntLit(n) => {
                self.advance();
                Ok(Expr::IntLit(*n, tok.span))
            }
            _ => {
                let span = tok.span;
                Err(ParseError::UnexpectedToken {
                    expected: "expression".to_string(),
                    found: tok,
                    span,
                })
            }
        }
    }
    
    fn parse_simple_postfix(&mut self, mut expr: Expr) -> Result<Expr, ParseError> {
        loop {
            // 在每次循环开始时检查逗号或右括号
            if self.check(TokenKind::Comma) || self.check(TokenKind::RParen) {
                break;
            }
            
            match self.current() {
                Some(tok) => {
                    match tok.kind {
                        TokenKind::Dot => {
                            self.advance();
                            let field_tok = self.expect_ident("field name")?;
                            let field = Ident::new(field_tok.lexeme.clone(), field_tok.span);
                            let span = expr.span().merge(field_tok.span);
                            expr = Expr::Field {
                                target: Box::new(expr),
                                field,
                                span,
                            };
                        }
                        TokenKind::LBracket => {
                            self.advance();
                            let index = self.parse_simple_expr()?;
                            self.expect(TokenKind::RBracket, "]")?;
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::Index {
                                target: Box::new(expr),
                                index: Box::new(index),
                                span,
                            };
                        }
                        TokenKind::LBrace => {
                // 检查下一个token是否是 Let/If/While 等语句关键字
                // 如果是，说明这是一个代码块而不是结构体字面量或Map
                let next_tok = self.peek();
                if let Some(tok) = next_tok {
                    match tok.kind {
                        TokenKind::Let | TokenKind::If | TokenKind::While | 
                        TokenKind::Repeat | TokenKind::For | TokenKind::Return |
                        TokenKind::Break | TokenKind::Continue | TokenKind::Match => {
                            // 这是一个代码块表达式
                            self.advance();
                            let stmts = self.parse_statements_until_end()?;
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::Block { stmts, span };
                            continue;
                        }
                        _ => {}
                    }
                }
                
                // 处理结构体字面量，例如 "点{ x:5, y:10 }"
                if let Expr::Ident(ident) = expr {
                    self.advance();
                    let mut fields = Vec::new();
                    while !self.check(TokenKind::RBrace) && !self.is_eof() {
                        let field_name_tok = self.expect_ident("field name")?;
                        let field_name = Ident::new(field_name_tok.lexeme.clone(), field_name_tok.span);
                        self.expect(TokenKind::Colon, ":")?;
                        let value = self.parse_simple_expr()?;
                        fields.push((field_name, value));
                        self.eat(TokenKind::Comma);
                    }
                    self.expect(TokenKind::RBrace, "}")?;
                    let span = ident.span.merge(self.current_span());
                    expr = Expr::Struct {
                        path: Path::from_ident(ident),
                        fields,
                        span,
                    };
                } else {
                    // 如果不是 Ident，就按 Map 处理
                    self.advance();
                    let mut map_fields = Vec::new();
                    while !self.check(TokenKind::RBrace) && !self.is_eof() {
                        let key = self.parse_simple_expr()?;
                        self.expect(TokenKind::Colon, ":")?;
                        let value = self.parse_simple_expr()?;
                        map_fields.push((key, value));
                        self.eat(TokenKind::Comma);
                    }
                    self.expect(TokenKind::RBrace, "}")?;
                    let span = expr.span().merge(self.current_span());
                    expr = Expr::Map(map_fields, span);
                }
            }
            TokenKind::QuestionMark => {
                self.advance();
                let span = expr.span().merge(self.current_span());
                expr = Expr::Try {
                    expr: Box::new(expr),
                    span,
                };
            }
                        TokenKind::Ident(ref name) if name == "长度" => {
                            let tok_span = tok.span;
                            self.advance();
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::Field {
                                target: Box::new(expr),
                                field: Ident::new("长度".to_string(), tok_span),
                                span,
                            };
                        }
                        TokenKind::Ident(ref name) if name == "获取" => {
                            self.advance();
                            let index = self.parse_simple_expr()?;
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::Index {
                                target: Box::new(expr),
                                index: Box::new(index),
                                span,
                            };
                        }
                        TokenKind::Ident(ref name) if name == "添加" => {
                            let tok_span = tok.span;
                            self.advance();
                            let value = self.parse_simple_expr()?;
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::MethodCall {
                                receiver: Box::new(expr),
                                method: Ident::new("添加".to_string(), tok_span),
                                args: vec![value],
                                span,
                            };
                        }
                        TokenKind::LParen => {
                            self.advance();
                            let mut args = Vec::new();
                            if !self.check(TokenKind::RParen) {
                                loop {
                                    if self.check(TokenKind::RParen) {
                                        break;
                                    }
                                    // 检查是否是逗号，如果是则跳过
                                    if self.check(TokenKind::Comma) {
                                        self.advance();
                                        continue;
                                    }
                                    args.push(self.parse_simple_expr()?);
                                    if !self.check(TokenKind::Comma) {
                                        break;
                                    }
                                    self.advance();
                                }
                            }
                            let span = expr.span().merge(self.current_span());
                            self.expect(TokenKind::RParen, ")")?;
                            expr = Expr::Call {
                                func: Box::new(expr),
                                args,
                                span,
                            };
                            // 解析完函数调用后立即返回，不再继续解析
                            return Ok(expr);
                        }
                        _ => break,
                    }
                }
                None => break,
            }
        }
        Ok(expr)
    }

    #[allow(dead_code)]
    fn get_binary_op(&self, kind: &TokenKind) -> Result<BinaryOp, ParseError> {
        match kind {
            TokenKind::Plus | TokenKind::Add => Ok(BinaryOp::Add),
            TokenKind::Minus | TokenKind::Sub => Ok(BinaryOp::Sub),
            TokenKind::Star | TokenKind::Mul => Ok(BinaryOp::Mul),
            TokenKind::Slash | TokenKind::Div => Ok(BinaryOp::Div),
            TokenKind::Percent | TokenKind::Mod => Ok(BinaryOp::Mod),
            TokenKind::And => Ok(BinaryOp::And),
            TokenKind::Or => Ok(BinaryOp::Or),
            TokenKind::Eq | TokenKind::EqEq => Ok(BinaryOp::Eq),
            TokenKind::Ne => Ok(BinaryOp::Ne),
            TokenKind::Lt => Ok(BinaryOp::Lt),
            TokenKind::Gt => Ok(BinaryOp::Gt),
            TokenKind::Le => Ok(BinaryOp::Le),
            TokenKind::Ge => Ok(BinaryOp::Ge),
            TokenKind::BitAnd => Ok(BinaryOp::BitAnd),
            TokenKind::BitOr => Ok(BinaryOp::BitOr),
            TokenKind::BitXor => Ok(BinaryOp::BitXor),
            TokenKind::Shl => Ok(BinaryOp::Shl),
            TokenKind::Shr => Ok(BinaryOp::Shr),
            _ => Err(ParseError::UnexpectedToken {
                expected: "binary operator".to_string(),
                found: self.current().cloned().unwrap_or_else(|| Token::eof(self.current_span().start)),
                span: self.current_span(),
            }),
        }
    }

    fn parse_postfix(&mut self, mut expr: Expr) -> Result<Expr, ParseError> {
        loop {
            match self.current() {
                Some(tok) => {
                    match tok.kind {
                        // 遇到这些终止符时退出循环
                        TokenKind::RParen | TokenKind::Comma | TokenKind::Semicolon | TokenKind::FatArrow => {
                            return Ok(expr);
                        }
                        TokenKind::Dot => {
                            self.advance();
                            let field_tok = self.expect_ident("field name")?;
                            let field = Ident::new(field_tok.lexeme.clone(), field_tok.span);
                            
                            // 检查是否是方法调用（标识符后跟左括号）
                            if self.check(TokenKind::LParen) {
                                let receiver = Box::new(expr);
                                let method = field.clone();
                                
                                // 解析参数
                                self.advance(); // 消费 LParen
                                let mut args = Vec::new();
                                if !self.check(TokenKind::RParen) {
                                    loop {
                                        if self.check(TokenKind::RParen) {
                                            break;
                                        }
                                        if self.check(TokenKind::Comma) {
                                            self.advance();
                                            continue;
                                        }
                                        args.push(self.parse_simple_expr()?);
                                        if !self.check(TokenKind::Comma) {
                                            break;
                                        }
                                        self.advance();
                                    }
                                }
                                self.expect(TokenKind::RParen, ")")?;
                                
                                let span = receiver.span().merge(self.current_span());
                                expr = Expr::MethodCall {
                                    receiver,
                                    method,
                                    args,
                                    span,
                                };
                            } else {
                                let span = expr.span().merge(field_tok.span);
                                expr = Expr::Field {
                                    target: Box::new(expr),
                                    field,
                                    span,
                                };
                            }
                        }
                        TokenKind::LBracket => {
                            self.advance();
                            let index = self.parse_expr_with_precedence(PREC_LOWEST)?;
                            self.expect(TokenKind::RBracket, "]")?;
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::Index {
                                target: Box::new(expr),
                                index: Box::new(index),
                                span,
                            };
                        }
                        TokenKind::LBrace => {
                            if let Expr::Ident(ident) = expr {
                                self.advance();
                                let mut fields = Vec::new();
                                while !self.check(TokenKind::RBrace) && !self.is_eof() {
                                    let field_name_tok = self.expect_ident("field name")?;
                                    let field_name = Ident::new(field_name_tok.lexeme.clone(), field_name_tok.span);
                                    self.expect(TokenKind::Colon, ":")?;
                                    let value = self.parse_expr_with_precedence(PREC_LOWEST)?;
                                    fields.push((field_name, value));
                                    self.eat(TokenKind::Comma);
                                }
                                self.expect(TokenKind::RBrace, "}")?;
                                let span = ident.span.merge(self.current_span());
                                expr = Expr::Struct {
                                    path: Path::from_ident(ident),
                                    fields,
                                    span,
                                };
                            } else {
                                // 不是 Ident，不消费 LBrace，直接返回，让调用者处理
                                return Ok(expr);
                            }
                        }
                        TokenKind::QuestionMark => {
                            self.advance();
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::Try {
                                expr: Box::new(expr),
                                span,
                            };
                        }
                        TokenKind::Ident(ref name) if name == "长度" => {
                            // 处理 arr 长度
                            let tok_span = tok.span;
                            self.advance();
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::Field {
                                target: Box::new(expr),
                                field: Ident::new("长度".to_string(), tok_span),
                                span,
                            };
                        }
                        TokenKind::Ident(ref name) if name == "获取" => {
                            // 处理 arr 获取 j
                            self.advance();
                            let index = self.parse_expr_with_precedence(PREC_UNARY)?;
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::Index {
                                target: Box::new(expr),
                                index: Box::new(index),
                                span,
                            };
                        }
                        TokenKind::Ident(ref name) if name == "设置" => {
                            // 处理 arr 设置 j 为 value
                            let tok_span = tok.span;
                            self.advance();
                            let index = self.parse_expr_with_precedence(PREC_LOWEST)?;
                            if !self.check_keyword(&["为", "as"]) {
                                return Err(ParseError::UnexpectedToken {
                                    expected: "为".to_string(),
                                    found: self.current().unwrap().clone(),
                                    span: self.current_span(),
                                });
                            }
                            self.advance();
                            let value = self.parse_expr_with_precedence(PREC_LOWEST)?;
                            // 这里我们需要特殊处理，因为设置操作会被转换为赋值语句
                            // 我们将返回一个特殊的表达式，然后在解释器中处理
                            let span = expr.span().merge(self.current_span());
                            // 暂时返回一个占位表达式，实际处理会在解释器中进行
                            expr = Expr::Call {
                                func: Box::new(Expr::Ident(Ident::new("设置".to_string(), tok_span))),
                                args: vec![expr, index, value],
                                span,
                            };
                        }
                        TokenKind::Ident(ref name) if name == "添加" => {
                            // 处理 arr 添加 value
                            let tok_span = tok.span;
                            self.advance();
                            let value = self.parse_expr_with_precedence(PREC_LOWEST)?;
                            let span = expr.span().merge(self.current_span());
                            // 创建一个方法调用表达式
                            expr = Expr::MethodCall {
                                receiver: Box::new(expr),
                                method: Ident::new("添加".to_string(), tok_span),
                                args: vec![value],
                                span,
                            };
                        }
                        TokenKind::Ident(ref name) if name == "清空" => {
                            // 处理 arr 清空
                            let tok_span = tok.span;
                            self.advance();
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::MethodCall {
                                receiver: Box::new(expr),
                                method: Ident::new("清空".to_string(), tok_span),
                                args: vec![],
                                span,
                            };
                        }
                        TokenKind::Lt => {
                            let next_tok = self.peek();
                            if let Some(next) = next_tok {
                                let is_valid_expr_for_generic = matches!(&expr, 
                                    Expr::Field { .. } | Expr::MethodCall { .. } | Expr::Generic { .. }
                                );
                                
                                match &next.kind {
                                    TokenKind::Ident(name) if is_type_identifier(name) && is_valid_expr_for_generic => {
                                        self.advance();
                                        let mut types = Vec::new();
                                        while !self.check(TokenKind::Gt) && !self.is_eof() {
                                            types.push(self.parse_type()?);
                                            if self.eat(TokenKind::Comma).is_none() {
                                                break;
                                            }
                                        }
                                        self.expect(TokenKind::Gt, ">")?;
                                        
                                        let current_span = self.current_span();
                                        let expr_span = expr.span();
                                        
                                        if self.check(TokenKind::LParen) {
                                            self.advance();
                                            let mut args = Vec::new();
                                            while !self.check(TokenKind::RParen) && !self.is_eof() {
                                                args.push(self.parse_call_arg()?);
                                                if self.eat(TokenKind::Comma).is_none() {
                                                    break;
                                                }
                                            }
                                            self.expect(TokenKind::RParen, ")")?;
                                            let span = expr_span.merge(self.current_span());
                                            expr = Expr::Call {
                                                func: Box::new(Expr::Generic {
                                                    target: Box::new(expr),
                                                    args: types,
                                                    span: expr_span.merge(current_span),
                                                }),
                                                args,
                                                span,
                                            };
                                        } else {
                                            expr = Expr::Generic {
                                                target: Box::new(expr),
                                                args: types,
                                                span: expr_span.merge(current_span),
                                            };
                                        }
                                    }
                                    TokenKind::TypeInt | TokenKind::TypeString | TokenKind::TypeBool | 
                                    TokenKind::TypeChar | TokenKind::TypeUnit | TokenKind::TypeList |
                                    TokenKind::TypeArray | TokenKind::TypeMap | TokenKind::TypeOption |
                                    TokenKind::TypeI8 | TokenKind::TypeI16 | TokenKind::TypeI32 | 
                                    TokenKind::TypeI64 | TokenKind::TypeU8 | TokenKind::TypeU16 | 
                                    TokenKind::TypeU32 | TokenKind::TypeU64 | TokenKind::TypeF32 | 
                                    TokenKind::TypeF64 | TokenKind::TypePtr if is_valid_expr_for_generic => {
                                        self.advance();
                                        let mut types = Vec::new();
                                        types.push(self.parse_type()?);
                                        while !self.check(TokenKind::Gt) && !self.is_eof() {
                                            if self.eat(TokenKind::Comma).is_some() {
                                                types.push(self.parse_type()?);
                                            } else {
                                                break;
                                            }
                                        }
                                        self.expect(TokenKind::Gt, ">")?;
                                        
                                        let current_span = self.current_span();
                                        let expr_span = expr.span();
                                        
                                        if self.check(TokenKind::LParen) {
                                            self.advance();
                                            let mut args = Vec::new();
                                            while !self.check(TokenKind::RParen) && !self.is_eof() {
                                                args.push(self.parse_call_arg()?);
                                                if self.eat(TokenKind::Comma).is_none() {
                                                    break;
                                                }
                                            }
                                            self.expect(TokenKind::RParen, ")")?;
                                            let span = expr_span.merge(self.current_span());
                                            expr = Expr::Call {
                                                func: Box::new(Expr::Generic {
                                                    target: Box::new(expr),
                                                    args: types,
                                                    span: expr_span.merge(current_span),
                                                }),
                                                args,
                                                span,
                                            };
                                        } else {
                                            expr = Expr::Generic {
                                                target: Box::new(expr),
                                                args: types,
                                                span: expr_span.merge(current_span),
                                            };
                                        }
                                    }
                                    _ => {
                                        break;
                                    }
                                }
                            } else {
                                break;
                            }
                        }

                        TokenKind::LParen => {
                            self.advance();
                            let mut args = Vec::new();
                            // 如果当前已经是右括号，说明是无参数调用
                            if !self.check(TokenKind::RParen) {
                                loop {
                                    // 使用 parse_call_arg 来解析参数，会在逗号处停止
                                    args.push(self.parse_call_arg()?);
                                    
                                    // 如果没有逗号，说明已经解析完所有参数
                                    if !self.check(TokenKind::Comma) {
                                        break;
                                    }
                                    
                                    // 消费逗号
                                    self.advance();
                                }
                            }
                            let span = expr.span().merge(self.current_span());
                            self.expect(TokenKind::RParen, ")")?;
                            expr = Expr::Call {
                                func: Box::new(expr),
                                args,
                                span,
                            };
                        }
                        _ => break,
                    }
                }
                None => break,
            }
        }
        Ok(expr)
    }

    fn parse_call_arg(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_prefix()?;
        
        loop {
            lhs = self.parse_call_arg_postfix(lhs)?;
            
            let Some(tok) = self.current() else {
                break;
            };
            
            // 在遇到逗号、右括号、=>、} 或分号时停止
            if self.check(TokenKind::Comma) || self.check(TokenKind::RParen) || 
               self.check(TokenKind::FatArrow) || self.check(TokenKind::RBrace) || 
               self.check(TokenKind::Semicolon) {
                break;
            }
            
            if !self.is_infix_operator(&tok.kind) {
                break;
            }
            
            let prec = self.get_infix_precedence(&tok.kind);
            if prec < PREC_LOWEST {
                break;
            }
            
            lhs = self.parse_infix(lhs, prec)?;
        }
        
        Ok(lhs)
    }
    
    fn parse_call_arg_postfix(&mut self, mut expr: Expr) -> Result<Expr, ParseError> {
        loop {
            let current = self.current();
            if current.is_none() {
                break;
            }
            
            let tok = current.unwrap();
            
            match tok.kind {
                TokenKind::RParen | TokenKind::Comma | TokenKind::Semicolon | TokenKind::FatArrow | TokenKind::RBrace => {
                    break;
                }
                TokenKind::Dot => {
                    self.advance();
                    let field_tok = self.expect_ident("field name")?;
                    let field = Ident::new(field_tok.lexeme.clone(), field_tok.span);
                    let span = expr.span().merge(field_tok.span);
                    expr = Expr::Field {
                        target: Box::new(expr),
                        field,
                        span,
                    };
                }
                TokenKind::LBracket => {
                    self.advance();
                    let index = self.parse_call_arg()?;
                    self.expect(TokenKind::RBracket, "]")?;
                    let span = expr.span().merge(self.current_span());
                    expr = Expr::Index {
                        target: Box::new(expr),
                        index: Box::new(index),
                        span,
                    };
                }
                TokenKind::Ident(ref name) if name == "长度" => {
                    let tok_span = tok.span;
                    self.advance();
                    let span = expr.span().merge(self.current_span());
                    expr = Expr::Field {
                        target: Box::new(expr),
                        field: Ident::new("长度".to_string(), tok_span),
                        span,
                    };
                }
                TokenKind::Ident(ref name) if name == "获取" => {
                    self.advance();
                    let index = self.parse_call_arg()?;
                    let span = expr.span().merge(self.current_span());
                    expr = Expr::Index {
                        target: Box::new(expr),
                        index: Box::new(index),
                        span,
                    };
                }
                TokenKind::Ident(ref name) if name == "添加" => {
                    let tok_span = tok.span;
                    self.advance();
                    let value = self.parse_call_arg()?;
                    let span = expr.span().merge(self.current_span());
                    expr = Expr::MethodCall {
                        receiver: Box::new(expr),
                        method: Ident::new("添加".to_string(), tok_span),
                        args: vec![value],
                        span,
                    };
                }
                TokenKind::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    if !self.check(TokenKind::RParen) {
                        args.push(self.parse_call_arg()?);
                        
                        while self.eat(TokenKind::Comma).is_some() {
                            if self.check(TokenKind::RParen) {
                                break;
                            }
                            args.push(self.parse_call_arg()?);
                        }
                    }
                    let span = expr.span().merge(self.current_span());
                    self.expect(TokenKind::RParen, ")")?;
                    expr = Expr::Call {
                        func: Box::new(expr),
                        args,
                        span,
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }
    
    fn is_infix_operator(&self, kind: &TokenKind) -> bool {
        matches!(kind,
            TokenKind::Assign
            | TokenKind::PlusEq | TokenKind::MinusEq | TokenKind::StarEq
            | TokenKind::SlashEq | TokenKind::PercentEq
            | TokenKind::Or
            | TokenKind::And
            | TokenKind::Eq | TokenKind::EqEq | TokenKind::Ne
            | TokenKind::Gt | TokenKind::Lt | TokenKind::Ge | TokenKind::Le
            | TokenKind::BitOr
            | TokenKind::BitXor
            | TokenKind::BitAnd
            | TokenKind::Shl | TokenKind::Shr
            | TokenKind::Plus | TokenKind::Minus | TokenKind::Add | TokenKind::Sub
            | TokenKind::Star | TokenKind::Slash | TokenKind::Percent | TokenKind::Mul | TokenKind::Div | TokenKind::Mod
        )
    }

    fn get_infix_precedence(&self, kind: &TokenKind) -> u8 {
        match kind {
            TokenKind::Or => PREC_OR,
            TokenKind::And => PREC_AND,
            TokenKind::Eq | TokenKind::EqEq | TokenKind::Ne => PREC_EQ,
            TokenKind::Gt | TokenKind::Lt | TokenKind::Ge | TokenKind::Le => PREC_COMPARE,
            TokenKind::BitOr => PREC_BITOR,
            TokenKind::BitXor => PREC_BITXOR,
            TokenKind::BitAnd => PREC_BITAND,
            TokenKind::Shl | TokenKind::Shr => PREC_SHIFT,
            TokenKind::Plus | TokenKind::Minus | TokenKind::Add | TokenKind::Sub => PREC_ADD,
            TokenKind::Star | TokenKind::Slash | TokenKind::Percent | TokenKind::Mul | TokenKind::Div | TokenKind::Mod => PREC_MUL,
            TokenKind::Assign
                | TokenKind::PlusEq | TokenKind::MinusEq | TokenKind::StarEq
                | TokenKind::SlashEq | TokenKind::PercentEq => PREC_ASSIGN,
            _ => PREC_LOWEST,
        }
    }

    fn parse_closure(&mut self) -> Result<Expr, ParseError> {
        let start = self.current_span();
        
        // 消费左括号
        self.expect(TokenKind::LBrace, "{")?;
        
        // 解析闭包体
        let body = self.parse_block()?;
        
        // 消费右括号（如果存在）
        self.eat(TokenKind::RBrace);
        
        let span = start.merge(self.current_span());
        
        Ok(Expr::Closure {
            params: Vec::new(),  // 暂时不支持带参数的闭包
            return_type: None,
            body,
            span,
        })
    }
    
    fn parse_closure_with_params(&mut self) -> Result<Expr, ParseError> {
        let start = self.current_span();
        
        let mut params = Vec::new();
        
        // 解析参数列表
        loop {
            let param_tok = self.current().cloned().unwrap_or_else(|| Token::eof(SourcePosition::new(0,0,0)));
            match &param_tok.kind {
                TokenKind::Ident(name) => {
                    self.advance();
                    params.push((Ident::new(name.clone(), param_tok.span), None));
                }
                _ => {
                        let span = param_tok.span;
                        return Err(ParseError::UnexpectedToken {
                            expected: "parameter name".to_string(),
                            found: param_tok,
                            span,
                        });
                    }
            }
            
            // 检查是否还有更多参数或闭包开始
            if let Some(next) = self.peek() {
                match next.kind {
                    TokenKind::Comma => {
                        self.advance(); // 消费逗号
                    }
                    TokenKind::BitOr => {
                        self.advance(); // 消费 BitOr token (第二个 |)
                        break;
                    }
                    TokenKind::LBrace => {
                        // 第二个 | 后面直接跟着 {，说明当前 token 就是第二个 |
                        // 需要消费它
                        self.advance(); // 消费当前的 BitOr token (第二个 |)
                        break;
                    }
                    _ => {
                        return Err(ParseError::UnexpectedToken {
                            expected: ",".to_string(),
                            found: next.clone(),
                            span: next.span,
                        });
                    }
                }
            } else {
                return Err(ParseError::UnexpectedEof {
                    expected: "parameter or |".to_string(),
                });
            }
        }
        
        // 消费左括号
        self.expect(TokenKind::LBrace, "{")?;
        
        // 解析闭包体
        let body = self.parse_block()?;
        
        // 消费右括号（如果存在）
        self.eat(TokenKind::RBrace);
        
        let span = start.merge(self.current_span());
        
        Ok(Expr::Closure {
            params,
            return_type: None,
            body,
            span,
        })
    }
    
    fn parse_match_expr(&mut self) -> Result<Expr, ParseError> {
        let start = self.current_span();
        
        let expr = self.parse_expr()?;
        
        self.expect(TokenKind::LBrace, "{")?;
        
        let mut arms = Vec::new();
        
        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            while self.check(TokenKind::Semicolon) || self.check(TokenKind::Comma) {
                self.advance();
            }
            
            if self.check(TokenKind::RBrace) {
                break;
            }
            
            let current = self.current();
            if current.is_none() {
                break;
            }
            
            let current_kind = &current.unwrap().kind;
            
            match current_kind {
                TokenKind::Ident(_) | TokenKind::IntLit(_) | TokenKind::StringLit(_) | TokenKind::Default => {
                }
                _ => {
                    break;
                }
            }
            
            let pattern = self.parse_pattern()?;
            
            self.expect(TokenKind::FatArrow, "=>")?;
            
            let arm_expr = if self.check(TokenKind::LBrace) {
                let stmts = self.parse_block()?;
                Expr::Block { stmts, span: self.current_span() }
            } else {
                self.parse_expr()?
            };
            arms.push((pattern, arm_expr));
            
            // 跳过分号（如果存在）
            self.eat(TokenKind::Semicolon);
            
            // 检查是否遇到右括号
            if self.check(TokenKind::RBrace) {
                break;
            }
            
            // 跳过逗号
            while self.check(TokenKind::Comma) {
                self.advance();
            }
        }
        
        self.expect(TokenKind::RBrace, "}")?;
        
        let span = start.merge(self.current_span());
        
        Ok(Expr::Match {
            expr: Box::new(expr),
            arms,
            default: None,
            span,
        })
    }
    
    fn parse_branch_expr(&mut self) -> Result<Expr, ParseError> {
        let tok = self.current().cloned().unwrap_or_else(|| Token::eof(SourcePosition::new(0,0,0)));
        let start_span = tok.span;
        
        match &tok.kind {
            TokenKind::Ident(name) => {
                self.advance();
                
                if self.check(TokenKind::LParen) {
                    self.advance();
                    
                    let mut args = Vec::new();
                    if !self.check(TokenKind::RParen) {
                        let arg_tok = self.current().cloned().unwrap_or_else(|| Token::eof(SourcePosition::new(0,0,0)));
                        let arg_span = arg_tok.span;
                        
                        match &arg_tok.kind {
                            TokenKind::Ident(arg_name) => {
                                self.advance();
                                args.push(Expr::Ident(Ident::new(arg_name.clone(), arg_span)));
                            }
                            TokenKind::StringLit(s) => {
                                self.advance();
                                args.push(Expr::StringLit(s.clone(), arg_span));
                            }
                            TokenKind::IntLit(n) => {
                                self.advance();
                                args.push(Expr::IntLit(*n, arg_span));
                            }
                            _ => {
                                return Err(ParseError::UnexpectedToken {
                                    expected: "expression".to_string(),
                                    found: arg_tok,
                                    span: arg_span,
                                });
                            }
                        }
                    }
                    
                    // 检查是否是右括号
                    if self.check(TokenKind::RParen) {
                        self.advance();
                    } else {
                        // 如果不是右括号，可能是逗号或其他字符，不消费任何 token
                        // 让调用者处理
                    }
                    
                    Ok(Expr::Call {
                        func: Box::new(Expr::Ident(Ident::new(name.clone(), start_span))),
                        args,
                        span: start_span.merge(self.current_span()),
                    })
                } else {
                    Ok(Expr::Ident(Ident::new(name.clone(), start_span)))
                }
            }
            TokenKind::StringLit(s) => {
                self.advance();
                Ok(Expr::StringLit(s.clone(), start_span))
            }
            TokenKind::IntLit(n) => {
                self.advance();
                Ok(Expr::IntLit(*n, start_span))
            }
            TokenKind::LBrace => {
                let stmts = self.parse_block()?;
                Ok(Expr::Block { stmts, span: start_span.merge(self.current_span()) })
            }
            _ => {
                Err(ParseError::UnexpectedToken {
                    expected: "expression".to_string(),
                    found: tok,
                    span: start_span,
                })
            }
        }
    }
    
    fn parse_simple_arm_expr(&mut self) -> Result<Expr, ParseError> {
        let tok = self.current().cloned().unwrap_or_else(|| Token::eof(SourcePosition::new(0,0,0)));
        let start_span = tok.span;
        
        match &tok.kind {
            TokenKind::Ident(name) => {
                self.advance();
                
                if self.check(TokenKind::LParen) {
                    self.advance();
                    let mut args = Vec::new();
                    
                    if !self.check(TokenKind::RParen) {
                        loop {
                            let arg_tok = self.current().cloned().unwrap_or_else(|| Token::eof(SourcePosition::new(0,0,0)));
                            let arg_span = arg_tok.span;
                            
                            match &arg_tok.kind {
                                TokenKind::Ident(arg_name) => {
                                    self.advance();
                                    args.push(Expr::Ident(Ident::new(arg_name.clone(), arg_span)));
                                }
                                TokenKind::StringLit(s) => {
                                    self.advance();
                                    args.push(Expr::StringLit(s.clone(), arg_span));
                                }
                                TokenKind::IntLit(n) => {
                                    self.advance();
                                    args.push(Expr::IntLit(*n, arg_span));
                                }
                                _ => {
                                    return Err(ParseError::UnexpectedToken {
                                        expected: "expression".to_string(),
                                        found: arg_tok,
                                        span: arg_span,
                                    });
                                }
                            }
                            
                            if self.check(TokenKind::RParen) {
                                break;
                            }
                            
                            if !self.check(TokenKind::Comma) {
                                break;
                            }
                            
                            self.advance();
                        }
                    }
                    
                    self.expect(TokenKind::RParen, ")")?;
                    Ok(Expr::Call {
                        func: Box::new(Expr::Ident(Ident::new(name.clone(), start_span))),
                        args,
                        span: start_span.merge(self.current_span()),
                    })
                } else {
                    Ok(Expr::Ident(Ident::new(name.clone(), start_span)))
                }
            }
            TokenKind::StringLit(s) => {
                self.advance();
                Ok(Expr::StringLit(s.clone(), start_span))
            }
            TokenKind::IntLit(n) => {
                self.advance();
                Ok(Expr::IntLit(*n, start_span))
            }
            _ => {
                Err(ParseError::UnexpectedToken {
                    expected: "expression".to_string(),
                    found: tok,
                    span: start_span,
                })
            }
        }
    }
    
    fn parse_arm_expr(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_prefix()?;

        loop {
            // 使用 parse_arm_postfix 来解析 postfix 运算符，会在逗号或右括号处停止
            lhs = self.parse_arm_postfix(lhs)?;

            // 检查是否有 infix 运算符
            let Some(tok) = self.current() else {
                break;
            };
            
            // 在遇到逗号或右括号时停止
            if self.check(TokenKind::Comma) || self.check(TokenKind::RBrace) {
                break;
            }
            
            if !self.is_infix_operator(&tok.kind) {
                break;
            }
            let prec = self.get_infix_precedence(&tok.kind);
            if prec < PREC_LOWEST {
                break;
            }
            lhs = self.parse_infix(lhs, prec)?;
        }

        Ok(lhs)
    }
    
    fn parse_arm_postfix(&mut self, mut expr: Expr) -> Result<Expr, ParseError> {
        loop {
            match self.current() {
                Some(tok) => {
                    match tok.kind {
                        TokenKind::Dot => {
                            self.advance();
                            let field_tok = self.expect_ident("field name")?;
                            let field = Ident::new(field_tok.lexeme.clone(), field_tok.span);
                            let span = expr.span().merge(field_tok.span);
                            expr = Expr::Field {
                                target: Box::new(expr),
                                field,
                                span,
                            };
                        }
                        TokenKind::LBracket => {
                            // 在 match arm 中不解析数组索引
                            break;
                        }
                        TokenKind::LParen => {
                            self.advance();
                            let mut args = Vec::new();
                            if !self.check(TokenKind::RParen) {
                                loop {
                                    // 在 match arm 中，函数参数中的逗号应该被正确处理
                                    // 使用 parse_call_arg_postfix 来解析参数，直接处理后缀
                                    let mut arg = self.parse_prefix()?;
                                    arg = self.parse_call_arg_postfix(arg)?;
                                    args.push(arg);
                                    
                                    // 检查是否是右括号
                                    if self.check(TokenKind::RParen) {
                                        break;
                                    }
                                    
                                    // 如果没有逗号，说明已经解析完所有参数
                                    if !self.check(TokenKind::Comma) {
                                        break;
                                    }
                                    
                                    // 消费逗号
                                    self.advance();
                                }
                            }
                            let span = expr.span().merge(self.current_span());
                            self.expect(TokenKind::RParen, ")")?;
                            expr = Expr::Call {
                                func: Box::new(expr),
                                args,
                                span,
                            };
                        }
                        _ => break,
                    }
                }
                None => break,
            }
        }
        Ok(expr)
    }
    
    fn parse_match_arm_expr(&mut self) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_prefix()?;
        
        loop {
            lhs = self.parse_match_arm_postfix(lhs)?;
            
            let Some(tok) = self.current() else {
                break;
            };
            
            // 在 match arm 中，逗号、右括号和右大括号都是分隔符，需要停止
            if self.check(TokenKind::Comma) || self.check(TokenKind::RBrace) || self.check(TokenKind::RParen) {
                break;
            }
            
            if !self.is_infix_operator(&tok.kind) {
                break;
            }
            
            let prec = self.get_infix_precedence(&tok.kind);
            if prec < PREC_LOWEST {
                break;
            }
            
            lhs = self.parse_infix(lhs, prec)?;
        }
        
        Ok(lhs)
    }
    
    fn parse_match_arm_postfix(&mut self, mut expr: Expr) -> Result<Expr, ParseError> {
        loop {
            // 在每次循环开始时检查逗号或右括号
            if self.check(TokenKind::Comma) || self.check(TokenKind::RBrace) || self.check(TokenKind::RParen) {
                break;
            }
            
            match self.current() {
                Some(tok) => {
                    match tok.kind {
                        TokenKind::Dot => {
                            self.advance();
                            let field_tok = self.expect_ident("field name")?;
                            let field = Ident::new(field_tok.lexeme.clone(), field_tok.span);
                            let span = expr.span().merge(field_tok.span);
                            expr = Expr::Field {
                                target: Box::new(expr),
                                field,
                                span,
                            };
                        }
                        TokenKind::LBracket => {
                            self.advance();
                            // 在 match arm 中，数组索引的表达式也需要在逗号处停止
                            let index = self.parse_match_arm_expr()?;
                            self.expect(TokenKind::RBracket, "]")?;
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::Index {
                                target: Box::new(expr),
                                index: Box::new(index),
                                span,
                            };
                        }
                        TokenKind::Ident(ref name) if name == "长度" => {
                            let tok_span = tok.span;
                            self.advance();
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::Field {
                                target: Box::new(expr),
                                field: Ident::new("长度".to_string(), tok_span),
                                span,
                            };
                        }
                        TokenKind::Ident(ref name) if name == "获取" => {
                            self.advance();
                            let index = self.parse_match_arm_expr()?;
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::Index {
                                target: Box::new(expr),
                                index: Box::new(index),
                                span,
                            };
                        }
                        TokenKind::Ident(ref name) if name == "添加" => {
                            let tok_span = tok.span;
                            self.advance();
                            let value = self.parse_match_arm_expr()?;
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::MethodCall {
                                receiver: Box::new(expr),
                                method: Ident::new("添加".to_string(), tok_span),
                                args: vec![value],
                                span,
                            };
                        }
                        TokenKind::LParen => {
                            self.advance();
                            let mut args = Vec::new();
                            while !self.check(TokenKind::RParen) && !self.is_eof()
                                && !self.check(TokenKind::End) && !self.check(TokenKind::RBrace)
                                && !self.check(TokenKind::Semicolon)
                                && !self.check(TokenKind::FatArrow)
                            {
                                // 使用 parse_match_arm_expr 来解析参数
                                args.push(self.parse_match_arm_expr()?);
                                
                                if self.eat(TokenKind::Comma).is_none() {
                                    break;
                                }
                            }
                            let span = expr.span().merge(self.current_span());
                            self.expect(TokenKind::RParen, ")")?;
                            expr = Expr::Call {
                                func: Box::new(expr),
                                args,
                                span,
                            };
                        }
                        _ => break,
                    }
                }
                None => break,
            }
        }
        Ok(expr)
    }
    
    fn parse_prefix(&mut self) -> Result<Expr, ParseError> {
        let tok = self.current().ok_or_else(|| ParseError::UnexpectedEof {
            expected: "expression".to_string(),
        })?.clone();
        let span = tok.span;
        
        // 首先检查：如果当前 token 是运算符或类型关键词，且下一个 token 是左括号或左大括号，则将其作为标识符
        if matches!(tok.kind,
                TokenKind::Add | TokenKind::Sub | TokenKind::Mul | TokenKind::Div | TokenKind::Mod |
                TokenKind::And | TokenKind::Or | TokenKind::Not |
                TokenKind::Gt | TokenKind::Lt | TokenKind::Eq | TokenKind::Ge | TokenKind::Le | TokenKind::Ne |
                TokenKind::TypeInt | TokenKind::TypeI8 | TokenKind::TypeI16 | TokenKind::TypeI32 | TokenKind::TypeI64 |
                TokenKind::TypeU8 | TokenKind::TypeU16 | TokenKind::TypeU32 | TokenKind::TypeU64 |
                TokenKind::TypeF32 | TokenKind::TypeF64 |
                TokenKind::TypeBool | TokenKind::TypeChar | TokenKind::TypeString | TokenKind::TypeUnit |
                TokenKind::TypePtr | TokenKind::TypeList | TokenKind::TypeArray | TokenKind::TypeOption |
                TokenKind::TypeMap | TokenKind::Print
            ) {
            // 检查下一个 token 是否是左括号或左大括号
            if let Some(next_tok) = self.peek() {
                if next_tok.kind == TokenKind::LParen || next_tok.kind == TokenKind::LBrace {
                    // 将此关键词当作标识符处理
                    let lexeme = tok.lexeme.clone();
                    self.advance();
                    return Ok(Expr::Ident(Ident::new(lexeme, span)));
                }
            }
            // 对于 Print 关键词，即使下一个不是 LParen，也要处理成 Ident，因为可能是不带括号的调用
            if matches!(tok.kind, TokenKind::Print) {
                let lexeme = tok.lexeme.clone();
                self.advance();
                return Ok(Expr::Ident(Ident::new(lexeme, span)));
            }
        }

        match &tok.kind {
            TokenKind::IntLit(n) => {
                self.advance();
                Ok(Expr::IntLit(*n, span))
            }
            TokenKind::FloatLit(n) => {
                self.advance();
                Ok(Expr::FloatLit(*n, span))
            }
            TokenKind::StringLit(s) => {
                self.advance();
                Ok(Expr::StringLit(s.clone(), span))
            }
            TokenKind::CharLit(c) => {
                self.advance();
                Ok(Expr::CharLit(*c, span))
            }
            TokenKind::True => {
                self.advance();
                Ok(Expr::BoolLit(true, span))
            }
            TokenKind::False => {
                self.advance();
                Ok(Expr::BoolLit(false, span))
            }
            TokenKind::LBracket => {
                self.advance();
                let mut elements = Vec::new();
                while !self.check(TokenKind::RBracket) && !self.is_eof() {
                    elements.push(self.parse_expr()?);
                    if self.eat(TokenKind::Comma).is_none() {
                        break;
                    }
                }
                self.expect(TokenKind::RBracket, "]")?;
                Ok(Expr::List(elements, span))
            }
            TokenKind::Null => {
                self.advance();
                Ok(Expr::Null(span))
            }
            TokenKind::Ident(name) => {
                let name = name.clone();
                self.advance();
                Ok(Expr::Ident(Ident::new(name, span)))
            }
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.expect(TokenKind::RParen, ")")?;
                Ok(expr)
            }
            TokenKind::LBrace => {
                // 检查是否是结构体/Map 字面量：{ 标识符: 表达式, ... }
                // 如果不是，就说明这是语句块的开始，parse_expr 应该停止
                // 重要：不要在这里消费 LBrace，让 parse_expr 的循环检测到它并停止
                
                // peek 下一个 token（不消费 LBrace）
                let saved_pos = self.pos;
                self.advance(); // 消费 LBrace
                let peek_tok = self.peek();
                
                let is_struct_or_map_literal = if let Some(tok) = peek_tok {
                    // 结构体/Map 字面量的下一个 token 应该是标识符（字段名）
                    if matches!(&tok.kind, TokenKind::Ident(_)) {
                        true
                    } else {
                        false
                    }
                } else {
                    false
                };
                
                // 回退到 LBrace 之前的状态
                self.pos = saved_pos;
                
                if is_struct_or_map_literal {
                    // 是 struct/map literal，调用 parse_map_or_struct_literal 解析
                    self.parse_map_or_struct_literal()
                } else {
                    // 不是 struct/map literal，说明这是语句块的开始
                    // 返回 Null 作为占位符，但更重要的是不消费 LBrace
                    // 这样 parse_expr 的调用者会看到 LBrace 并正确处理
                    Ok(Expr::Null(span))
                }
            }
            TokenKind::Minus | TokenKind::Sub => {
                self.advance();
                let expr = self.parse_expr_with_precedence(PREC_UNARY)?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Neg,
                    expr: Box::new(expr),
                    span: span.merge(self.current_span()),
                })
            }
            TokenKind::Not => {
                self.advance();
                let expr = self.parse_expr_with_precedence(PREC_UNARY)?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Not,
                    expr: Box::new(expr),
                    span: span.merge(self.current_span()),
                })
            }
            TokenKind::QuestionMark => {
                // 前缀 ? 运算符，用于错误传播
                self.advance();
                let expr = self.parse_expr_with_precedence(PREC_UNARY)?;
                Ok(Expr::Try {
                    expr: Box::new(expr),
                    span: span.merge(self.current_span()),
                })
            }
            TokenKind::BitAnd => {
                // 取地址操作符 &
                self.advance();
                let expr = self.parse_expr_with_precedence(PREC_UNARY)?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Ref,
                    expr: Box::new(expr),
                    span: span.merge(self.current_span()),
                })
            }
            TokenKind::Star => {
                // 解引用操作符 *
                self.advance();
                let expr = self.parse_expr_with_precedence(PREC_UNARY)?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Deref,
                    expr: Box::new(expr),
                    span: span.merge(self.current_span()),
                })
            }
            TokenKind::Asm => {
                self.advance();
                self.parse_asm(span)
            }
            TokenKind::Async => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Expr::Async {
                    expr: Box::new(expr),
                    span: span.merge(self.current_span()),
                })
            }
            TokenKind::Await => {
                self.advance();
                let expr = self.parse_expr_with_precedence(PREC_UNARY)?;
                Ok(Expr::Await {
                    expr: Box::new(expr),
                    span: span.merge(self.current_span()),
                })
            }
            TokenKind::Spawn => {
                self.advance();
                let expr = self.parse_expr()?;
                Ok(Expr::Spawn {
                    expr: Box::new(expr),
                    span: span.merge(self.current_span()),
                })
            }
            TokenKind::Match => {
                // 处理匹配表达式
                self.advance();
                return self.parse_match_expr();
            }
            TokenKind::Or => {
                // 检查是否是闭包语法 || { ... }
                // 由于 lexer 将 || 识别为单个 Or token，我们需要检查后面是否是 {
                if let Some(next) = self.peek() {
                    if next.kind == TokenKind::LBrace {
                        self.advance(); // 消费 Or token
                        return self.parse_closure();
                    }
                }
                // 否则是逻辑或操作符，由二元表达式处理
                Err(ParseError::UnexpectedToken {
                    expected: "expression".to_string(),
                    found: tok.clone(),
                    span: tok.span,
                })
            }
            TokenKind::BitOr => {
                // 检查是否是闭包语法 |param| { ... }
                if let Some(next) = self.peek() {
                    if matches!(next.kind, TokenKind::Ident(_)) {
                        self.advance(); // 消费 BitOr token (第一个 |)
                        return self.parse_closure_with_params();
                    }
                }
                // 否则是位或操作符，由二元表达式处理
                Err(ParseError::UnexpectedToken {
                    expected: "expression".to_string(),
                    found: tok.clone(),
                    span: tok.span,
                })
            }
            TokenKind::Eof => Err(ParseError::UnexpectedEof {
                expected: "expression".to_string(),
            }),
            _ => Err(ParseError::UnexpectedToken {
                expected: "expression".to_string(),
                found: tok.clone(),
                span: tok.span,
            }),
        }
    }

    fn parse_infix(&mut self, lhs: Expr, _min_prec: u8) -> Result<Expr, ParseError> {
        let tok = self.current().ok_or_else(|| ParseError::UnexpectedEof {
            expected: "operator".to_string(),
        })?.clone();
        let span = tok.span;
        let prec = self.get_infix_precedence(&tok.kind);

        self.advance();

        let rhs = self.parse_expr_with_precedence(prec + 1)?;

        let op = match tok.kind {
            TokenKind::Plus | TokenKind::Add => BinaryOp::Add,
            TokenKind::Minus | TokenKind::Sub => BinaryOp::Sub,
            TokenKind::Star | TokenKind::Mul => BinaryOp::Mul,
            TokenKind::Slash | TokenKind::Div => BinaryOp::Div,
            TokenKind::Percent | TokenKind::Mod => BinaryOp::Mod,
            TokenKind::And => BinaryOp::And,
            TokenKind::Or => BinaryOp::Or,
            TokenKind::Eq | TokenKind::EqEq => BinaryOp::Eq,
            TokenKind::Ne => BinaryOp::Ne,
            TokenKind::Lt => BinaryOp::Lt,
            TokenKind::Gt => BinaryOp::Gt,
            TokenKind::Le => BinaryOp::Le,
            TokenKind::Ge => BinaryOp::Ge,
            TokenKind::BitAnd => BinaryOp::BitAnd,
            TokenKind::BitOr => BinaryOp::BitOr,
            TokenKind::BitXor => BinaryOp::BitXor,
            TokenKind::Shl => BinaryOp::Shl,
            TokenKind::Shr => BinaryOp::Shr,
            TokenKind::Assign => BinaryOp::Assign,
            TokenKind::PlusEq => BinaryOp::AddAssign,
            TokenKind::MinusEq => BinaryOp::SubAssign,
            TokenKind::StarEq => BinaryOp::MulAssign,
            TokenKind::SlashEq => BinaryOp::DivAssign,
            TokenKind::PercentEq => BinaryOp::ModAssign,
            _ => BinaryOp::Add,
        };

        Ok(Expr::BinaryOp {
            op,
            left: Box::new(lhs),
            right: Box::new(rhs),
            span: span.merge(self.current_span()),
        })
    }

    fn parse_list_literal(&mut self) -> Result<Expr, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::LBracket);
        let mut elements = Vec::new();
        while !self.check(TokenKind::RBracket) && !self.is_eof() {
            elements.push(self.parse_expr()?);
            if self.eat(TokenKind::Comma).is_none() {
                break;
            }
        }
        self.expect(TokenKind::RBracket, "]")?;
        let span = start.merge(self.current_span());
        Ok(Expr::List(elements, span))
    }

    fn parse_map_or_struct_literal(&mut self) -> Result<Expr, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::LBrace);
        let mut fields = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            let key = self.parse_expr()?;
            if self.eat(TokenKind::Colon).is_some() {
                let value = self.parse_expr()?;
                fields.push((key, value));
            } else {
                fields.push((key.clone(), key));
            }
            if self.eat(TokenKind::Comma).is_none() {
                break;
            }
        }
        self.expect(TokenKind::RBrace, "}")?;
        let span = start.merge(self.current_span());
        Ok(Expr::Map(fields, span))
    }

    fn parse_const(&mut self, _public: bool) -> Result<Item, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Const);
        let name_tok = self.expect_ident("constant name")?;
        let name = Ident::new(name_tok.lexeme.clone(), name_tok.span);
        let ty = if self.eat(TokenKind::Colon).is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };
        self.expect(TokenKind::Assign, "=")?;
        let value = self.parse_expr()?;
        self.eat(TokenKind::Semicolon);
        let span = start.merge(self.current_span());
        Ok(Item::Global(Global {
            mutable: false,
            name,
            ty,
            value: Box::new(value),
            span,
        }))
    }

    fn parse_extern(&mut self) -> Result<Item, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Extern);

        let abi = if let Some(tok) = self.current() {
            if let TokenKind::StringLit(s) = &tok.kind {
                let s_clone = s.clone();
                self.advance();
                match s_clone.as_str() {
                    "C" => ExternAbi::C,
                    "Rust" => ExternAbi::Rust,
                    "System" => ExternAbi::System,
                    _ => ExternAbi::Custom(s_clone),
                }
            } else {
                ExternAbi::C
            }
        } else {
            ExternAbi::C
        };
        
        let mut items = Vec::new();
        if self.eat(TokenKind::LBrace).is_some() {
            while !self.check(TokenKind::RBrace) && !self.is_eof() {
                if self.check(TokenKind::Func) {
                    self.eat(TokenKind::Func);
                    let name_tok = self.expect_ident("function name")?;
                    let name = Ident::new(name_tok.lexeme.clone(), name_tok.span);
                    self.expect(TokenKind::LParen, "(")?;
                    let mut params = Vec::new();
                    while !self.check(TokenKind::RParen) && !self.is_eof() {
                        let param_name_tok = self.expect_ident("parameter name")?;
                        let param_name = Ident::new(param_name_tok.lexeme.clone(), param_name_tok.span);
                        self.expect(TokenKind::Colon, ":")?;
                        let param_type = self.parse_type()?;
                        params.push((param_name, param_type));
                        if self.eat(TokenKind::Comma).is_none() {
                            break;
                        }
                    }
                    self.expect(TokenKind::RParen, ")")?;
                    let return_type = if self.eat(TokenKind::Arrow).is_some() {
                        self.parse_type()?
                    } else {
                        Type::Unit
                    };
                    items.push(ExternItem::Function {
                        name,
                        generics: vec![],
                        params,
                        return_type,
                        variadic: false,
                    });
                    self.eat(TokenKind::Semicolon);
                } else if self.check(TokenKind::Const) {
                    self.eat(TokenKind::Const);
                    let name_tok = self.expect_ident("constant name")?;
                    let name = Ident::new(name_tok.lexeme.clone(), name_tok.span);
                    self.expect(TokenKind::Colon, ":")?;
                    let ty = self.parse_type()?;
                    items.push(ExternItem::Static {
                        name,
                        ty,
                        mutable: false,
                    });
                    self.eat(TokenKind::Semicolon);
                } else if self.check(TokenKind::Import) {
                    // 跳过导入声明，因为 ExternItem 不支持导入
                    self.eat(TokenKind::Import);
                    // 跳过下一个标记，不管是什么类型
                    if !self.is_eof() {
                        self.advance();
                    }
                    self.eat(TokenKind::Semicolon);
                } else {
                    // 如果遇到不认识的标记，先前进一个，避免无限循环
                    self.advance();
                }
            }
            self.expect(TokenKind::RBrace, "}")?;
        }
        
        let span = start.merge(self.current_span());
        Ok(Item::Extern(ExternBlock {
            abi,
            items,
            span,
        }))
    }

    fn parse_type_alias(&mut self, public: bool) -> Result<Item, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Type);
        let name_tok = self.expect_ident("type name")?;
        let name = Ident::new(name_tok.lexeme.clone(), name_tok.span);
        self.expect(TokenKind::Eq, "=")?;
        let ty = self.parse_type()?;
        self.eat(TokenKind::Semicolon);
        let span = start.merge(self.current_span());
        Ok(Item::TypeAlias(TypeAlias {
            public,
            name,
            generics: vec![],
            ty,
            span,
        }))
    }

    fn parse_global(&mut self, _public: bool) -> Result<Item, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Global);
        let mutable = self.eat(TokenKind::Mut).is_some();
        let name_tok = self.expect_ident("global name")?;
        let name = Ident::new(name_tok.lexeme.clone(), name_tok.span);
        let ty = if self.eat(TokenKind::Colon).is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };
        self.expect(TokenKind::Assign, "=")?;
        let value = self.parse_expr()?;
        self.eat(TokenKind::Semicolon);
        let span = start.merge(self.current_span());
        Ok(Item::Global(Global {
            mutable,
            name,
            ty,
            value: Box::new(value),
            span,
        }))
    }

    fn parse_asm(&mut self, start_span: SourceSpan) -> Result<Expr, ParseError> {
        // 期望看到 '!' 和 '('
        if let Some(tok) = self.current() {
            if tok.lexeme == "!" {
                self.advance();
            }
        }
        self.expect(TokenKind::LParen, "(")?;

        let mut templates = Vec::new();
        let mut outputs = Vec::new();
        let mut inputs = Vec::new();
        let mut clobbers = Vec::new();
        let mut options = AsmOptions::default();

        // 首先解析汇编模板字符串
        while let Some(tok) = self.current() {
            if let TokenKind::StringLit(s) = &tok.kind {
                templates.push(s.clone());
                self.advance();
                // 尝试消费逗号
                if self.eat(TokenKind::Comma).is_none() {
                    break;
                }
            } else {
                break;
            }
        }

        // 解析操作数、clobber、options
        while !self.check(TokenKind::RParen) && !self.is_eof() {
            // 检查是输出、输入还是clobber
            let tok = match self.current() {
                Some(t) => t,
                None => break,
            };
            
            // 检查是否是'输出'或'input'关键词
            let lexeme_lower = tok.lexeme.to_lowercase();
            if lexeme_lower == "输出" || lexeme_lower == "output" {
                self.advance();
                outputs.push(self.parse_asm_operand()?);
            } else if lexeme_lower == "输入" || lexeme_lower == "input" {
                self.advance();
                inputs.push(self.parse_asm_operand()?);
            } else if lexeme_lower == "破坏" || lexeme_lower == "clobber" {
                self.advance();
                clobbers.extend(self.parse_asm_clobbers()?);
            } else if lexeme_lower == "易失" || lexeme_lower == "volatile" {
                self.advance();
                options.volatile = true;
            } else if lexeme_lower == "纯" || lexeme_lower == "pure" {
                self.advance();
                options.pure = true;
            } else if lexeme_lower == "无内存" || lexeme_lower == "nomem" {
                self.advance();
                options.nomem = true;
            } else if lexeme_lower == "保留标志" || lexeme_lower == "preserves_flags" {
                self.advance();
                options.preserves_flags = true;
            } else if lexeme_lower == "不可达" || lexeme_lower == "noreturn" {
                self.advance();
                options.noreturn = true;
            } else if lexeme_lower == "对齐栈" || lexeme_lower == "alignstack" {
                self.advance();
                options.alignstack = true;
            } else if lexeme_lower == "英特尔语法" || lexeme_lower == "intel_syntax" {
                self.advance();
                options.intel_syntax = true;
            } else {
                // 检查是否是函数调用（如 选项(易失)）
                if let Some(next) = self.current() {
                    if let TokenKind::Ident(name) = &next.kind {
                        let name_lower = name.to_lowercase();
                        if name_lower == "选项" || name_lower == "option" {
                            self.advance(); // 消费 "选项"
                            self.expect(TokenKind::LParen, "(")?;
                            // 解析选项参数
                            if let Some(opt_tok) = self.current() {
                                let opt_lower = opt_tok.lexeme.to_lowercase();
                                match opt_lower.as_str() {
                                    "易失" | "volatile" => options.volatile = true,
                                    "纯" | "pure" => options.pure = true,
                                    "无内存" | "nomem" => options.nomem = true,
                                    "保留标志" | "preserves_flags" => options.preserves_flags = true,
                                    "不可达" | "noreturn" => options.noreturn = true,
                                    "对齐栈" | "alignstack" => options.alignstack = true,
                                    "英特尔语法" | "intel_syntax" => options.intel_syntax = true,
                                    _ => {}
                                }
                                self.advance();
                            }
                            self.expect(TokenKind::RParen, ")")?;
                            // 如果下一个 token 是右括号，停止循环
                            if self.check(TokenKind::RParen) {
                                break;
                            }
                            // 消费逗号
                            self.eat(TokenKind::Comma);
                            continue;
                        }
                        
                        let next_next = self.tokens.get(self.pos + 1);
                        if let Some(nn) = next_next {
                            if nn.kind == TokenKind::LParen {
                                // 是其他函数调用，直接解析为表达式并忽略
                                let _ = self.parse_expr()?;
                                // 如果下一个 token 是右括号，停止循环
                                if self.check(TokenKind::RParen) {
                                    break;
                                }
                                // 消费逗号
                                self.eat(TokenKind::Comma);
                                continue;
                            }
                        }
                    }
                }
                // 可能是第一个操作数没有关键词前缀，尝试解析为输入操作数
                inputs.push(self.parse_asm_operand()?);
            }
            
            // 如果下一个 token 是右括号，停止循环
            if self.check(TokenKind::RParen) {
                break;
            }
            
            // 消费逗号
            self.eat(TokenKind::Comma);
        }

        self.expect(TokenKind::RParen, ")")?;

        let span = start_span.merge(self.current_span());

        Ok(Expr::Asm(InlineAsm {
            templates,
            outputs,
            inputs,
            clobbers,
            options,
            span,
        }))
    }

    fn parse_asm_operand(&mut self) -> Result<AsmOperand, ParseError> {
        let mut name = None;
        let mut constraint = String::new();

        // 尝试解析约束，可能在括号中
        if self.eat(TokenKind::LParen).is_some() {
            if let Some(tok) = self.current() {
                if let TokenKind::StringLit(s) = &tok.kind {
                    constraint = s.clone();
                } else {
                    constraint = tok.lexeme.clone();
                }
                self.advance();
            }
            self.expect(TokenKind::RParen, ")")?;
        } else {
            // 没有显式约束，使用默认的约束字符
            constraint = "r".to_string();
        }

        // 尝试解析变量名（如果是输出操作数）
        if let Some(tok) = self.current().cloned() {
            if let TokenKind::Ident(n) = tok.kind {
                let ident = Ident::new(n.clone(), tok.span);
                name = Some(ident.clone());
                self.advance();
                // 如果有 => 则后面是表达式
                if self.eat(TokenKind::FatArrow).is_some() || self.eat(TokenKind::Assign).is_some() {
                    let expr = self.parse_expr()?;
                    return Ok(AsmOperand {
                        name,
                        constraint,
                        expr,
                    });
                }
                // 没有赋值操作符，标识符本身就是表达式
                return Ok(AsmOperand {
                    name,
                    constraint,
                    expr: Expr::Ident(ident),
                });
            }
        }

        // 如果没有变量名，则解析表达式作为输入
        let expr = self.parse_expr()?;
        Ok(AsmOperand {
            name,
            constraint,
            expr,
        })
    }

    fn parse_asm_clobbers(&mut self) -> Result<Vec<String>, ParseError> {
        let mut clobbers = Vec::new();
        
        self.expect(TokenKind::LParen, "(")?;
        
        while !self.check(TokenKind::RParen) && !self.is_eof() {
            if let Some(tok) = self.current() {
                if let TokenKind::StringLit(s) = &tok.kind {
                    clobbers.push(s.clone());
                } else {
                    clobbers.push(tok.lexeme.clone());
                }
                self.advance();
                self.eat(TokenKind::Comma);
            } else {
                break;
            }
        }
        
        self.expect(TokenKind::RParen, ")")?;
        Ok(clobbers)
    }

    fn parse_peripheral(&mut self) -> Result<Item, ParseError> {
        let start = self.current_span();
        self.eat(TokenKind::Peripheral);
        
        let name_tok = self.expect_ident("peripheral name")?;
        let name = Ident::new(name_tok.lexeme.clone(), name_tok.span);
        
        let base_addr = if self.eat(TokenKind::Assign).is_some() {
            // 语法：外设 GPIOA = 0x40010800
            if let Some(tok) = self.current() {
                if let TokenKind::IntLit(n) = tok.kind {
                    self.advance();
                    n as u64
                } else {
                    0
                }
            } else {
                0
            }
        } else if let Some(tok) = self.current() {
            // 语法：外设 GPIOA 基址 0x40010800
            let lexeme_lower = tok.lexeme.to_lowercase();
            if lexeme_lower == "基址" || lexeme_lower == "base" {
                self.advance();
                if let Some(tok) = self.current() {
                    if let TokenKind::IntLit(n) = tok.kind {
                        self.advance();
                        n as u64
                    } else {
                        0
                    }
                } else {
                    0
                }
            } else {
                0
            }
        } else {
            0
        };
        
        let mut registers = Vec::new();
        
        self.expect(TokenKind::LBrace, "{")?;
        
        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            // 检查是否有 "寄存器" 关键词
            let (reg_name, offset, ty, access) = if let Some(tok) = self.current() {
                let lexeme_lower = tok.lexeme.to_lowercase();
                if lexeme_lower == "寄存器" || lexeme_lower == "register" {
                    self.advance(); // 消费 "寄存器" 关键词
                    let reg_name_tok = self.expect_ident("register name")?;
                    let reg_name = Ident::new(reg_name_tok.lexeme.clone(), reg_name_tok.span);
                    
                    // 解析 "偏移" 或 "offset"
                    let offset = if let Some(tok) = self.current() {
                        let lexeme_lower = tok.lexeme.to_lowercase();
                        if lexeme_lower == "偏移" || lexeme_lower == "offset" {
                            self.advance();
                            if let Some(tok) = self.current() {
                                if let TokenKind::IntLit(n) = tok.kind {
                                    self.advance();
                                    n as u64
                                } else {
                                    0
                                }
                            } else {
                                0
                            }
                        } else {
                            0
                        }
                    } else {
                        0
                    };
                    
                    self.expect(TokenKind::Comma, ",")?;
                    
                    // 解析 "类型" 或 "type"
                    let ty = if let Some(tok) = self.current() {
                        let lexeme_lower = tok.lexeme.to_lowercase();
                        if lexeme_lower == "类型" || lexeme_lower == "type" {
                            self.advance();
                            self.parse_type()?
                        } else {
                            self.parse_type()?
                        }
                    } else {
                        self.parse_type()?
                    };
                    
                    self.expect(TokenKind::Comma, ",")?;
                    
                    // 解析 "访问" 或 "access"
                    let access = if let Some(tok) = self.current() {
                        let lexeme_lower = tok.lexeme.to_lowercase();
                        if lexeme_lower == "访问" || lexeme_lower == "access" {
                            self.advance();
                            if let Some(tok) = self.current() {
                                let lexeme_lower = tok.lexeme.to_lowercase();
                                if lexeme_lower == "只读" || lexeme_lower == "readonly" || lexeme_lower == "ro" {
                                    self.advance();
                                    RegisterAccess::ReadOnly
                                } else if lexeme_lower == "只写" || lexeme_lower == "writeonly" || lexeme_lower == "wo" {
                                    self.advance();
                                    RegisterAccess::WriteOnly
                                } else {
                                    self.advance();
                                    RegisterAccess::ReadWrite
                                }
                            } else {
                                RegisterAccess::ReadWrite
                            }
                        } else {
                            let lexeme_lower = tok.lexeme.to_lowercase();
                            if lexeme_lower == "只读" || lexeme_lower == "readonly" || lexeme_lower == "ro" {
                                self.advance();
                                RegisterAccess::ReadOnly
                            } else if lexeme_lower == "只写" || lexeme_lower == "writeonly" || lexeme_lower == "wo" {
                                self.advance();
                                RegisterAccess::WriteOnly
                            } else {
                                self.advance();
                                RegisterAccess::ReadWrite
                            }
                        }
                    } else {
                        RegisterAccess::ReadWrite
                    };
                    
                    // 消费分号
                    self.eat(TokenKind::Semicolon);
                    
                    (reg_name, offset, ty, access)
                } else {
                    // 使用旧语法
                    let reg_name_tok = self.expect_ident("register name")?;
                    let reg_name = Ident::new(reg_name_tok.lexeme.clone(), reg_name_tok.span);
                    
                    self.expect(TokenKind::Comma, ",")?;
                    
                    let offset = if let Some(tok) = self.current() {
                        if let TokenKind::IntLit(n) = tok.kind {
                            self.advance();
                            n as u64
                        } else {
                            0
                        }
                    } else {
                        0
                    };
                    
                    self.expect(TokenKind::Comma, ",")?;
                    
                    let ty = self.parse_type()?;
                    
                    let access = if self.eat(TokenKind::Comma).is_some() {
                        if let Some(tok) = self.current() {
                            let lexeme_lower = tok.lexeme.to_lowercase();
                            if lexeme_lower == "只读" || lexeme_lower == "readonly" || lexeme_lower == "ro" {
                                self.advance();
                                RegisterAccess::ReadOnly
                            } else if lexeme_lower == "只写" || lexeme_lower == "writeonly" || lexeme_lower == "wo" {
                                self.advance();
                                RegisterAccess::WriteOnly
                            } else {
                                self.advance();
                                RegisterAccess::ReadWrite
                            }
                        } else {
                            RegisterAccess::ReadWrite
                        }
                    } else {
                        RegisterAccess::ReadWrite
                    };
                    
                    // 尝试消费逗号，但如果下一个是右括号则停止
                    if self.check(TokenKind::RBrace) {
                        break;
                    }
                    self.eat(TokenKind::Comma);
                    
                    // 再次检查右括号
                    if self.check(TokenKind::RBrace) {
                        break;
                    }
                    
                    (reg_name, offset, ty, access)
                }
            } else {
                break;
            };
            
            registers.push(PeripheralRegisterDef {
                name: reg_name,
                offset,
                ty,
                access,
                span: start.merge(self.current_span()),
            });
        }
        
        self.expect(TokenKind::RBrace, "}")?;
        
        let span = start.merge(self.current_span());
        
        Ok(Item::Peripheral(PeripheralDef {
            name,
            base_addr,
            registers,
            span,
        }))
    }

    fn parse_memory_layout(&mut self) -> Result<Item, ParseError> {
        let start = self.current_span();
        
        if self.check(TokenKind::Memory) {
            self.eat(TokenKind::Memory);
        } else {
            self.eat(TokenKind::Layout);
        }
        
        let name_tok = if let Some(tok) = self.current() {
            if let TokenKind::Ident(_) = tok.kind {
                Some(self.expect_ident("layout name")?)
            } else {
                None
            }
        } else {
            None
        };
        
        let name = name_tok.map(|t| Ident::new(t.lexeme.clone(), t.span))
            .unwrap_or_else(|| Ident::new("default".to_string(), start));
        
        let mut regions = Vec::new();
        let mut segments = Vec::new();
        
        self.expect(TokenKind::LBrace, "{")?;
        
        while !self.is_eof() {
            // 检查是否是右括号
            if self.check(TokenKind::RBrace) {
                break;
            }
            
            if let Some(tok) = self.current() {
                let lexeme_lower = tok.lexeme.to_lowercase();
                if lexeme_lower == "闪存" || lexeme_lower == "flash" || 
                   lexeme_lower == "内存" || lexeme_lower == "memory" ||
                   lexeme_lower == "ram" || lexeme_lower == "rom" {
                    // 标准内存区域类型关键词
                    self.advance();
                    regions.push(self.parse_memory_region()?);
                } else if matches!(tok.kind, TokenKind::Ident(_)) && 
                          lexeme_lower != "段" && lexeme_lower != "segment" && 
                          lexeme_lower != "段定义" {
                    // 任意标识符作为区域名称（如 RAM, ROM, Flash 等）
                    regions.push(self.parse_memory_region()?);
                } else if lexeme_lower == "段" || lexeme_lower == "segment" || 
                          lexeme_lower == "段定义" {
                    self.advance();
                    segments.push(self.parse_segment()?);
                } else if tok.kind == TokenKind::Dot || 
                          (tok.lexeme.starts_with('.') && matches!(tok.kind, TokenKind::Ident(_))) {
                    // 以点开头的段定义（如 ".向量表"）
                    segments.push(self.parse_segment()?);
                } else if tok.kind == TokenKind::Comma {
                    // 跳过逗号
                    self.advance();
                } else if tok.kind == TokenKind::RBrace {
                    // 右括号，退出循环
                    break;
                } else {
                    // 遇到无法识别的 token，返回错误
                    return Err(ParseError::UnexpectedToken {
                        expected: "memory region or segment definition".to_string(),
                        found: tok.clone(),
                        span: tok.span,
                    });
                }
            } else {
                break;
            }
        }
        
        self.expect(TokenKind::RBrace, "}")?;
        
        let span = start.merge(self.current_span());
        
        Ok(Item::MemoryLayout(MemoryLayout {
            name,
            regions,
            segments,
            span,
        }))
    }

    fn parse_memory_region(&mut self) -> Result<MemoryRegionDef, ParseError> {
        let start = self.current_span();
        
        let name = if let Some(tok) = self.current() {
            if let TokenKind::Ident(_) = tok.kind {
                let name = tok.lexeme.clone();
                self.advance();
                // 跳过可选的冒号
                self.eat(TokenKind::Colon);
                name
            } else if tok.kind == TokenKind::Colon {
                // 如果当前是冒号，说明内存区域类型关键词已经被消费了
                self.advance();
                "memory".to_string()
            } else {
                "memory".to_string()
            }
        } else {
            "memory".to_string()
        };
        
        let mut start_addr = 0u64;
        let mut size = 0u64;
        let mut attrs = Vec::new();
        
        // 解析属性，直到遇到逗号、右括号或新的区域/段定义
        loop {
            // 消费逗号（如果存在）
            self.eat(TokenKind::Comma);
            
            // 检查是否是右括号
            if self.check(TokenKind::RBrace) {
                break;
            }
            
            // 获取当前 token
            let tok = match self.current() {
                Some(t) => t.clone(),
                None => break,
            };
            
            let lexeme_lower = tok.lexeme.to_lowercase();
            
            // 检查是否是新的区域或段定义
            if lexeme_lower == "闪存" || lexeme_lower == "flash" ||
               lexeme_lower == "内存" || lexeme_lower == "memory" ||
               lexeme_lower == "段" || lexeme_lower == "segment" {
                break;
            }
            
            self.advance();
            
            if lexeme_lower == "起始" || lexeme_lower == "start" {
                if let Some(tok) = self.current() {
                    match &tok.kind {
                        TokenKind::IntLit(n) => {
                            start_addr = *n as u64;
                            self.advance();
                        }
                        _ => {}
                    }
                }
            } else if lexeme_lower == "长度" || lexeme_lower == "size" || lexeme_lower == "length" {
                if let Some(tok) = self.current() {
                    match &tok.kind {
                        TokenKind::IntLit(n) => {
                            size = *n as u64;
                            self.advance();
                        }
                        TokenKind::Ident(s) => {
                            // 处理类似 64K, 1M 这样的大小表示
                            let value_str = s.to_lowercase();
                            if value_str.ends_with('k') {
                                if let Ok(num) = value_str[..value_str.len()-1].parse::<u64>() {
                                    size = num * 1024;
                                }
                            } else if value_str.ends_with('m') {
                                if let Ok(num) = value_str[..value_str.len()-1].parse::<u64>() {
                                    size = num * 1024 * 1024;
                                }
                            } else if value_str.ends_with('g') {
                                if let Ok(num) = value_str[..value_str.len()-1].parse::<u64>() {
                                    size = num * 1024 * 1024 * 1024;
                                }
                            }
                            self.advance();
                        }
                        _ => {}
                    }
                }
            } else if lexeme_lower == "可读" || lexeme_lower == "readable" {
                attrs.push(MemoryAttr::Readable);
            } else if lexeme_lower == "可写" || lexeme_lower == "writable" {
                attrs.push(MemoryAttr::Writable);
            } else if lexeme_lower == "可执行" || lexeme_lower == "executable" {
                attrs.push(MemoryAttr::Executable);
            } else if let TokenKind::IntLit(n) = tok.kind {
                // 如果是整数，且还没有设置起始地址，则作为起始地址
                if start_addr == 0 {
                    start_addr = n as u64;
                } else if size == 0 {
                    // 如果起始地址已设置，则作为长度
                    size = n as u64;
                }
            } else if let TokenKind::Ident(s) = tok.kind {
                // 处理类似 64K, 1M 这样的大小表示
                let value_str = s.to_lowercase();
                if value_str.ends_with('k') {
                    if let Ok(num) = value_str[..value_str.len()-1].parse::<u64>() {
                        if size == 0 {
                            size = num * 1024;
                        }
                    }
                } else if value_str.ends_with('m') {
                    if let Ok(num) = value_str[..value_str.len()-1].parse::<u64>() {
                        if size == 0 {
                            size = num * 1024 * 1024;
                        }
                    }
                } else if value_str.ends_with('g') {
                    if let Ok(num) = value_str[..value_str.len()-1].parse::<u64>() {
                        if size == 0 {
                            size = num * 1024 * 1024 * 1024;
                        }
                    }
                }
            }
        }
        
        let span = start.merge(self.current_span());
        
        Ok(MemoryRegionDef {
            name,
            start: start_addr,
            size,
            attributes: attrs,
            span,
        })
    }

    fn parse_segment(&mut self) -> Result<SegmentDef, ParseError> {
        let start = self.current_span();
        
        let name = if let Some(tok) = self.current() {
            if let TokenKind::Ident(_) = tok.kind {
                let name = tok.lexeme.clone();
                self.advance();
                // 跳过可选的冒号
                self.eat(TokenKind::Colon);
                name
            } else if tok.kind == TokenKind::Dot {
                // 处理以点开头的段名，如 .向量表
                let mut name = String::new();
                // 消费点号
                self.advance();
                // 读取点号后面的标识符
                if let Some(tok) = self.current() {
                    if let TokenKind::Ident(id) = &tok.kind {
                        name = format!(".{}", id);
                        self.advance();
                    } else if tok.kind == TokenKind::Colon {
                        // 点号后面直接是冒号，说明段名就是 "."（后面会读取下一个标识符）
                        name = ".".to_string();
                    }
                }
                // 跳过可选的冒号
                self.eat(TokenKind::Colon);
                if name.is_empty() {
                    name = ".text".to_string();
                }
                name
            } else if tok.lexeme.starts_with('.') && matches!(tok.kind, TokenKind::Ident(_)) {
                // 处理段名被识别为单个 Ident 的情况（如 ".向量表"）
                let name = tok.lexeme.clone();
                self.advance();
                name
            } else {
                ".text".to_string()
            }
        } else {
            ".text".to_string()
        };
        
        let mut region = String::new();
        let mut alignment = 4u64;
        let mut _load_addr = String::new();
        let mut _clear_bss = false;
        
        loop {
            // 检查是否是右括号或分号
            if self.check(TokenKind::RBrace) || self.check(TokenKind::Semicolon) {
                break;
            }
            
            // 检查是否是文件结束
            if self.is_eof() {
                break;
            }
            
            // 获取当前 token
            let tok = match self.current() {
                Some(t) => t.clone(),
                None => break,
            };
            
            let lexeme_lower = tok.lexeme.to_lowercase();
            
            // 检查是否是新的段定义（以点开头）
            if lexeme_lower.starts_with('.') {
                // 这是一个新的段定义，结束当前段
                break;
            }
            
            // 检查是否是关键字
            if lexeme_lower == "放入" || lexeme_lower == "in" {
                self.advance();
                if let Some(tok) = self.current() {
                    region = tok.lexeme.clone();
                    self.advance();
                }
            } else if lexeme_lower == "对齐" || lexeme_lower == "align" {
                self.advance();
                if let Some(tok) = self.current() {
                    if let TokenKind::IntLit(n) = tok.kind {
                        alignment = n as u64;
                        self.advance();
                    }
                }
            } else if lexeme_lower == "加载地址在" || lexeme_lower == "loadaddr" {
                self.advance();
                if let Some(tok) = self.current() {
                    _load_addr = tok.lexeme.clone();
                    self.advance();
                }
            } else if lexeme_lower == "清零" || lexeme_lower == "clear" {
                _clear_bss = true;
                self.advance();
            } else {
                // 未识别的 token，跳过
                self.advance();
            }
        }
        
        let span = start.merge(self.current_span());
        
        Ok(SegmentDef {
            name,
            region,
            alignment,
            span,
        })
    }
    
    fn parse_segment_block(&mut self) -> Result<Vec<SegmentDef>, ParseError> {
        let start = self.current_span();
        
        // 消费左括号
        self.expect(TokenKind::LBrace, "{")?;
        
        let mut segments = Vec::new();
        
        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            // 跳过分号
            self.eat(TokenKind::Semicolon);
            
            // 检查是否是右括号
            if self.check(TokenKind::RBrace) {
                break;
            }
            
            // 解析一个段定义
            let segment = self.parse_segment()?;
            segments.push(segment);
            
            // 消费分号
            self.eat(TokenKind::Semicolon);
        }
        
        // 消费右括号
        self.expect(TokenKind::RBrace, "}")?;
        
        let _span = start.merge(self.current_span());
        
        Ok(segments)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_source(source: &str) -> Result<Program, ParseError> {
        let mut lexer = crate::core::lexer::Lexer::new(source);
        let (tokens, lex_errors) = lexer.tokenize();
        if !lex_errors.is_empty() {
            return Err(ParseError::Fatal(format!("Lexer errors: {:?}", lex_errors)));
        }
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn test_parse_simple_integer() {
        let source = "42";
        let result = parse_source(source);
        assert!(result.is_ok());
        let program = result.unwrap();
        assert_eq!(program.len(), 1);
    }

    #[test]
    fn test_parse_simple_string() {
        let source = r#""hello world""#;
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_let_statement() {
        let source = "let x: int = 42";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_binary_expression() {
        let source = "1 + 2";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_nested_binary_expression() {
        let source = "1 + 2 * 3";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_function_call() {
        let source = "print(42)";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_list_literal() {
        let source = "[1, 2, 3]";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_map_literal() {
        let source = r#"{"key": "value"}"#;
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_field_access() {
        let source = "point.x";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_index_access() {
        let source = "arr[0]";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_chinese_let_statement() {
        let source = "let x = 25";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_if_statement() {
        let source = "if x > 0 { return 1 }";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_while_statement() {
        let source = "while x < 10 { x = x + 1 }";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_repeat_statement() {
        let source = "repeat 5 { x = x + 1 }";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_return_statement() {
        let source = "return 42";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_assignment() {
        let source = "x = 42";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_function_definition() {
        let source = "function add(a: int, b: int) -> int { return a + b }";
        let result = parse_source(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_comparison_operators() {
        let sources = [
            "x > 0",
            "x < 0",
            "x >= 0",
            "x <= 0",
            "x == 0",
            "x != 0",
        ];
        for source in sources {
            let result = parse_source(source);
            assert!(result.is_ok(), "Failed to parse: {}", source);
        }
    }

    #[test]
    fn test_parse_logical_operators() {
        let sources = [
            "x && y",
            "x || y",
            "!x",
        ];
        for source in sources {
            let result = parse_source(source);
            assert!(result.is_ok(), "Failed to parse: {}", source);
        }
    }
}

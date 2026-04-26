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
use crate::core::ast::*;
use crate::core::sema::{SemanticAnalyzer, SemanticError};

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

#[derive(Debug, Clone)]
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
    semantic_analyzer: Option<SemanticAnalyzer>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
            semantic_analyzer: None,
        }
    }

    fn current(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
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
                if tok.lexeme == *kw {
                    return true;
                }
            }
        }
        false
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
                TokenKind::Gt | TokenKind::Lt | TokenKind::Eq | TokenKind::Ge | TokenKind::Le | TokenKind::Ne
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
    }

    fn parse_item(&mut self) -> Result<Item, ParseError> {
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

        let mut fields = Vec::new();
        if self.eat(TokenKind::LBrace).is_some() {
            while !self.check(TokenKind::RBrace) && !self.is_eof() {
                let field_name_tok = self.expect_ident("field name")?;
                let field_name = Ident::new(field_name_tok.lexeme.clone(), field_name_tok.span);
                self.expect(TokenKind::Colon, ":")?;
                let field_type = self.parse_type()?;
                fields.push((field_name, field_type, None));
                if self.eat(TokenKind::Comma).is_none() {
                    break;
                }
            }
            self.expect(TokenKind::RBrace, "}")?;
        }

        let span = start.merge(self.current_span());
        Ok(Item::Struct(Struct {
            public,
            name,
            generics: vec![],
            where_clause: vec![],
            fields,
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
        
        let span = start.merge(self.current_span());
        Ok(Item::Import(Import {
            items: None,
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

        match &tok.kind {
            TokenKind::Ident(name) => {
                self.advance();
                Ok(Type::Named(Path {
                    segments: vec![Ident::new(name.clone(), tok.span)],
                    generics: None,
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
            _ => Ok(Type::Named(Path {
                segments: vec![Ident::new("Unknown".to_string(), tok.span)],
                generics: None,
                span: tok.span,
            })),
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
            while !self.check(TokenKind::RBrace) && !self.is_eof()
                && !self.check(TokenKind::End)
                && !self.check(TokenKind::Else)
                && !self.check(TokenKind::If)
                && !self.check(TokenKind::While)
                && !self.check(TokenKind::Repeat)
                && !self.check(TokenKind::For)
                && !self.check(TokenKind::Match)
                && !self.check(TokenKind::Break)
                && !self.check(TokenKind::Continue)
            {
                stmts.push(self.parse_stmt()?);
            }

            if !self.is_eof() && self.check(TokenKind::RBrace) {
                self.advance();
            }
            Ok(stmts)
        } else {
            let mut stmts = Vec::new();
            while !self.check(TokenKind::End) && !self.is_eof()
                && !self.check(TokenKind::RBrace)
                && !self.check(TokenKind::Else)
                && !self.check(TokenKind::If)
                && !self.check(TokenKind::While)
                && !self.check(TokenKind::Repeat)
                && !self.check(TokenKind::For)
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
    }

    fn parse_statements_until_end(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while !self.check(TokenKind::End) && !self.check(TokenKind::RBrace) && !self.is_eof()
            && !self.check(TokenKind::Else)
            && !self.check(TokenKind::While)
            && !self.check(TokenKind::Repeat)
            && !self.check(TokenKind::For)
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

        if self.check(TokenKind::Match) {
            return self.parse_match_stmt();
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
        if self.check(TokenKind::Arrow) {
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
                    TokenKind::Ident(_) | TokenKind::IntLit(_) | TokenKind::StringLit(_) | TokenKind::Default | TokenKind::Arrow => {
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
        if self.check(TokenKind::Then) && !self.check_keyword(&["开始", "begin", "显示", "print", "xianshi", "dayin"]) {
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
                if self.check(TokenKind::Then) && !self.check_keyword(&["开始", "begin", "显示", "print", "xianshi", "dayin"]) {
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
        // 跳过可选的 "循环" 关键词
        if self.check(TokenKind::While) && !self.check_keyword(&["开始", "begin"]) {
            self.advance();
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
            while self.current().is_some() && (self.check(TokenKind::Semicolon) || self.current().unwrap().kind == TokenKind::Whitespace || self.check(TokenKind::Arrow)) {
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
                self.expect(TokenKind::Arrow, "=>")?;
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
            if let Err(_) = self.expect(TokenKind::Arrow, "=>") {
                // 没有箭头，退出循环
                break;
            };
            
            let mut arm_stmts = Vec::new();
            
            // 解析 arm 体语句，直到遇到新的模式、结束标记或右大括号
            loop {
                // 检查是否遇到结束标记
                if self.check(TokenKind::RBrace) || self.check(TokenKind::End) || self.check(TokenKind::Default) || self.check(TokenKind::Match) || self.is_eof() {
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
                        TokenKind::Arrow => {
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
                        TokenKind::Ident(_) | TokenKind::IntLit(_) | TokenKind::StringLit(_) | TokenKind::Default | TokenKind::Arrow => {
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
                Ok(Pattern::Ident(Ident::new(name.clone(), span)))
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
                // 对于其他类型的 token，返回错误
                Err(ParseError::UnexpectedToken {
                    expected: "pattern".to_string(),
                    found: tok,
                    span,
                })
            }
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_expr_with_precedence(PREC_LOWEST)
    }

    fn parse_expr_with_precedence(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_prefix()?;

        loop {
            let Some(tok) = self.current() else {
                break;
            };
            if !self.is_infix_operator(&tok.kind) {
                break;
            }
            let prec = self.get_infix_precedence(&tok.kind);
            if prec < min_prec {
                break;
            }
            lhs = self.parse_infix(lhs, prec)?;
        }

        lhs = self.parse_postfix(lhs)?;

        // 再次检查当前 Token 是否是中缀运算符，处理类似 "a + b.c" 的情况
        if let Some(tok) = self.current() {
            let prec = self.get_infix_precedence(&tok.kind);
            if prec >= min_prec && self.is_infix_operator(&tok.kind) {
                let op_tok = tok.clone();
                self.advance();
                let rhs = self.parse_expr_with_precedence(prec + 1)?;
                let span = lhs.span().merge(rhs.span());
                let op = match op_tok.kind {
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
                let new_lhs = Expr::BinaryOp {
                    left: Box::new(lhs),
                    right: Box::new(rhs),
                    op,
                    span,
                };
                return Ok(new_lhs);
            }
        }

        Ok(lhs)
    }

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
                            let index = self.parse_expr()?;
                            self.expect(TokenKind::RBracket, "]")?;
                            let span = expr.span().merge(self.current_span());
                            expr = Expr::Index {
                                target: Box::new(expr),
                                index: Box::new(index),
                                span,
                            };
                        }
                        TokenKind::LParen => {
                    self.advance();
                    let mut args = Vec::new();
                    while !self.check(TokenKind::RParen) && !self.is_eof()
                        && !self.check(TokenKind::End) && !self.check(TokenKind::RBrace)
                        && !self.check(TokenKind::Semicolon) && !self.check(TokenKind::Comma)
                        && !self.check(TokenKind::Arrow)
                    {
                        args.push(self.parse_expr()?);

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

    fn parse_prefix(&mut self) -> Result<Expr, ParseError> {
        let tok = self.current().ok_or_else(|| ParseError::UnexpectedEof {
            expected: "expression".to_string(),
        })?.clone();
        let span = tok.span;

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
            TokenKind::LBracket => self.parse_list_literal(),
            TokenKind::LBrace => self.parse_map_or_struct_literal(),
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
        let result = parser.parse();
        if result.is_err() {
            eprintln!("Parsing failed for source: {}", source);
            eprintln!("Error: {:?}", result);
        }
        result
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

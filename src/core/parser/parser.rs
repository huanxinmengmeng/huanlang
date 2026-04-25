// Copyright (c) 2026 Huanxinmengmeng
// Licensed under the Huan Language License

use crate::core::lexer::token::{Token, TokenKind, SourceSpan, SourcePosition};
use crate::core::lexer::keywords::KeywordTable;
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
const PREC_CALL: u8 = 13;

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
    keyword_table: KeywordTable,
    semantic_analyzer: Option<SemanticAnalyzer>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            pos: 0,
            errors: Vec::new(),
            keyword_table: KeywordTable::new(),
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
            if self.check(TokenKind::RBrace) {
                self.advance();
                continue;
            }
            eprintln!("DEBUG parse: current token: {:?}", self.current());
            match self.parse_item() {
                Ok(item) => {
                    eprintln!("DEBUG parse: parsed item: {:?}", item);
                    items.push(item);
                }
                Err(e) => {
                    eprintln!("DEBUG parse: parse_item error: {:?}", e);
                    self.errors.push(e.clone());
                    if matches!(e, ParseError::UnexpectedEof { .. }) {
                        if !items.is_empty() {
                            return Ok(items);
                        }
                        break;
                    }
                    if matches!(e, ParseError::UnexpectedToken { .. }) && !items.is_empty() {
                        if let ParseError::UnexpectedToken { found, .. } = &e {
                            if matches!(found.kind, TokenKind::Eof) {
                                return Ok(items);
                            }
                        }
                        return Ok(items);
                    }
                    self.synchronize();
                    if matches!(e, ParseError::Fatal(_)) {
                        return Err(e);
                    }
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
        let _public = self.eat(TokenKind::Pub).is_some();

        if self.check(TokenKind::Module) {
            self.parse_module(false)
        } else if self.check(TokenKind::Import) {
            self.parse_import()
        } else if self.check(TokenKind::Func) {
            self.parse_function(false)
        } else if self.check(TokenKind::Struct) {
            self.parse_struct(false)
        } else if self.check(TokenKind::Trait) {
            self.parse_trait(false)
        } else if self.check(TokenKind::Impl) {
            self.parse_impl()
        } else if !self.is_eof() {
            let expr = self.parse_expr()?;
            let span = expr.span();
            let temp_name = Ident::new("_expr".to_string(), span);
            Ok(Item::Global(Global {
                mutable: false,
                name: temp_name,
                ty: None,
                value: Box::new(expr),
                span,
            }))
        } else {
            Err(ParseError::UnexpectedEof {
                expected: "item".to_string(),
            })
        }
    }

    fn parse_function(&mut self, _public: bool) -> Result<Item, ParseError> {
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
        } else {
            Type::Unit
        };

        let body = if self.check(TokenKind::Begin) {
            self.advance();
            self.parse_statements_until_end()?
        } else {
            self.parse_block()?
        };

        let span = start.merge(self.current_span());
        Ok(Item::Function(Function {
            public: false,
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

    fn parse_struct(&mut self, _public: bool) -> Result<Item, ParseError> {
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
            public: false,
            name,
            generics: vec![],
            where_clause: vec![],
            fields,
            span,
        }))
    }

    fn parse_module(&mut self, _public: bool) -> Result<Item, ParseError> {
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
            public: false,
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

    fn parse_trait(&mut self, _public: bool) -> Result<Item, ParseError> {
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
            public: false,
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

        if self.eat(TokenKind::LBrace).is_none() && !self.check(TokenKind::Begin) {
            return Err(ParseError::UnexpectedToken {
                expected: "{{ or begin".to_string(),
                found: self.current().cloned().unwrap_or_else(|| Token::eof(self.current_span().start)),
                span: self.current_span(),
            });
        }
        if self.check(TokenKind::Begin) {
            return self.parse_statements_until_end();
        }

        let mut stmts = Vec::new();
        while !self.check(TokenKind::RBrace) && !self.is_eof()
            && !self.check(TokenKind::End)
        {
            stmts.push(self.parse_stmt()?);
        }

        if self.is_eof() {
            Ok(stmts)
        } else {
            self.expect(TokenKind::RBrace, "}")?;
            Ok(stmts)
        }
    }

    fn parse_statements_until_end(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut stmts = Vec::new();
        while !self.check(TokenKind::End) && !self.is_eof() {
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
            self.expect(TokenKind::Assign, "=")?;
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

        let expr = self.parse_expr()?;
        if !self.check(TokenKind::RBrace) && !self.is_eof() {
            self.eat(TokenKind::Semicolon);
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
        let then_block = self.parse_block()?;

        let mut else_ifs = Vec::new();
        let mut else_block = None;

        while self.check(TokenKind::Else) {
            self.advance();
            if self.check(TokenKind::If) {
                self.advance();
                let else_if_cond = self.parse_expr()?;
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
        let body = self.parse_block()?;
        let span = start.merge(self.current_span());
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
        self.expect(TokenKind::LBrace, "{")?;

        let mut arms = Vec::new();
        let mut default = None;

        while !self.check(TokenKind::RBrace) && !self.is_eof() {
            if self.check(TokenKind::Default) {
                self.advance();
                self.expect(TokenKind::Arrow, "=>")?;
                let mut stmts = Vec::new();
                while !self.check(TokenKind::RBrace) && !self.is_eof() {
                    stmts.push(self.parse_stmt()?);
                }
                default = Some(stmts);
                break;
            }

            let pattern = self.parse_pattern()?;
            self.expect(TokenKind::Arrow, "=>")?;
            let mut arm_stmts = Vec::new();
            while !self.check(TokenKind::RBrace) && !self.check(TokenKind::Default) && !self.is_eof() {
                arm_stmts.push(self.parse_stmt()?);
            }
            arms.push((pattern, arm_stmts));
        }

        self.expect(TokenKind::RBrace, "}")?;
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
            _ => Ok(Pattern::Wildcard(span)),
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, ParseError> {
        self.parse_expr_with_precedence(PREC_LOWEST)
    }

    fn parse_expr_with_precedence(&mut self, min_prec: u8) -> Result<Expr, ParseError> {
        let mut lhs = self.parse_prefix()?;

        while let Some(tok) = self.current() {
            let prec = self.get_infix_precedence(&tok.kind);
            if prec < min_prec || !self.is_infix_operator(&tok.kind) {
                break;
            }
            lhs = self.parse_infix(lhs, prec)?;
        }

        Ok(lhs)
    }

    fn is_infix_operator(&self, kind: &TokenKind) -> bool {
        matches!(kind,
            TokenKind::Assign
            | TokenKind::PlusEq | TokenKind::MinusEq | TokenKind::StarEq
            | TokenKind::SlashEq | TokenKind::PercentEq
            | TokenKind::Or
            | TokenKind::And
            | TokenKind::Eq | TokenKind::Ne
            | TokenKind::Gt | TokenKind::Lt | TokenKind::Ge | TokenKind::Le
            | TokenKind::BitOr
            | TokenKind::BitXor
            | TokenKind::BitAnd
            | TokenKind::Shl | TokenKind::Shr
            | TokenKind::Plus | TokenKind::Minus | TokenKind::Add | TokenKind::Sub
            | TokenKind::Star | TokenKind::Slash | TokenKind::Percent | TokenKind::Mul | TokenKind::Div | TokenKind::Mod
            | TokenKind::LParen | TokenKind::LBracket | TokenKind::Dot
        )
    }

    fn get_infix_precedence(&self, kind: &TokenKind) -> u8 {
        match kind {
            TokenKind::Or => PREC_OR,
            TokenKind::And => PREC_AND,
            TokenKind::Eq | TokenKind::Ne => PREC_EQ,
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
            TokenKind::LParen | TokenKind::LBracket | TokenKind::Dot => PREC_CALL,
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
            TokenKind::Eq => BinaryOp::Eq,
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
}

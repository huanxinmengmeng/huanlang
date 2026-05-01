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

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourcePosition {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

impl Default for SourcePosition {
    fn default() -> Self {
        Self {
            offset: 0,
            line: 1,
            column: 1,
        }
    }
}

impl SourcePosition {
    pub fn new(offset: usize, line: usize, column: usize) -> Self {
        Self { offset, line, column }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SourceSpan {
    pub start: SourcePosition,
    pub end: SourcePosition,
}

impl Default for SourceSpan {
    fn default() -> Self {
        Self {
            start: SourcePosition::default(),
            end: SourcePosition::default(),
        }
    }
}

impl SourceSpan {
    pub fn new(start: SourcePosition, end: SourcePosition) -> Self {
        Self { start, end }
    }

    pub fn merge(self, other: Self) -> Self {
        Self {
            start: self.start,
            end: other.end,
        }
    }
    
    pub fn dummy() -> Self {
        Self::default()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Let,
    Const,
    Be,
    TypeAnno,
    If,
    Then,
    Else,
    End,
    While,
    Loop,
    Repeat,
    Times,
    For,
    Each,
    In,
    Do,
    Begin,
    Func,
    Return,
    Struct,
    Field,
    Trait,
    Requires,
    Impl,
    Interface,
    Abstract,
    Class,
    Extends,
    Method,
    SelfKeyword,
    Print,
    ForTy,
    Module,
    Pub,
    Import,
    From,
    And,
    Or,
    Not,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Gt,
    Lt,
    Eq,
    Ge,
    Le,
    Ne,
    Assign,
    True,
    False,
    Null,
    Match,
    Default,
    Break,
    Continue,
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
    Asm,
    Volatile,
    Global,
    Section,
    Align,
    Peripheral,
    Register,
    Memory,
    Layout,
    Segment,
    Spawn,
    TaskGroup,
    Channel,
    Mutex,
    RwLock,
    Atomic,
    Barrier,
    Once,
    Send,
    Sync,
    Async,
    Await,
    Where,
    Super,
    Static,
    TypeInt,
    Extern,
    Type,
    Mut,
    TypeFn,
    TypeI8,
    TypeI16,
    TypeI32,
    TypeI64,
    TypeU8,
    TypeU16,
    TypeU32,
    TypeU64,
    TypeF32,
    TypeF64,
    TypeBool,
    TypeChar,
    TypeString,
    TypeUnit,
    TypeList,
    TypeArray,
    TypeMap,
    TypePtr,
    TypeOption,
    IntLit(i64),
    FloatLit(f64),
    StringLit(String),
    CharLit(char),
    Ident(String),
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Comma,
    Dot,
    Colon,
    Semicolon,
    Arrow,
    FatArrow,
    DoubleColon,
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    EqEq,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    PercentEq,
    ShlEq,
    ShrEq,
    BitAndEq,
    BitOrEq,
    BitXorEq,
    QuestionMark,
    At,
    Comment(String),
    Whitespace,
    Newline,
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub span: SourceSpan,
    pub lexeme: String,
}

impl Token {
    pub fn new(kind: TokenKind, span: SourceSpan, lexeme: String) -> Self {
        Self { kind, span, lexeme }
    }

    pub fn eof(position: SourcePosition) -> Self {
        Self {
            kind: TokenKind::Eof,
            span: SourceSpan::new(position, position),
            lexeme: String::new(),
        }
    }
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Let => write!(f, "let"),
            TokenKind::Const => write!(f, "const"),
            TokenKind::Be => write!(f, "be"),
            TokenKind::TypeAnno => write!(f, "type"),
            TokenKind::If => write!(f, "if"),
            TokenKind::Then => write!(f, "then"),
            TokenKind::Else => write!(f, "else"),
            TokenKind::End => write!(f, "end"),
            TokenKind::While => write!(f, "while"),
            TokenKind::Loop => write!(f, "loop"),
            TokenKind::Repeat => write!(f, "repeat"),
            TokenKind::Times => write!(f, "times"),
            TokenKind::For => write!(f, "for"),
            TokenKind::Each => write!(f, "each"),
            TokenKind::In => write!(f, "in"),
            TokenKind::Do => write!(f, "do"),
            TokenKind::Begin => write!(f, "begin"),
            TokenKind::Func => write!(f, "func"),
            TokenKind::Return => write!(f, "return"),
            TokenKind::Struct => write!(f, "struct"),
            TokenKind::Field => write!(f, "field"),
            TokenKind::Trait => write!(f, "trait"),
            TokenKind::Requires => write!(f, "requires"),
            TokenKind::Impl => write!(f, "impl"),
            TokenKind::Interface => write!(f, "interface"),
            TokenKind::Abstract => write!(f, "abstract"),
            TokenKind::Class => write!(f, "class"),
            TokenKind::Extends => write!(f, "extends"),
            TokenKind::Method => write!(f, "method"),
            TokenKind::SelfKeyword => write!(f, "self"),
            TokenKind::Print => write!(f, "print"),
            TokenKind::ForTy => write!(f, "for"),
            TokenKind::Module => write!(f, "module"),
            TokenKind::Pub => write!(f, "pub"),
            TokenKind::Import => write!(f, "import"),
            TokenKind::From => write!(f, "from"),
            TokenKind::And => write!(f, "and"),
            TokenKind::Or => write!(f, "or"),
            TokenKind::Not => write!(f, "not"),
            TokenKind::Add => write!(f, "add"),
            TokenKind::Sub => write!(f, "sub"),
            TokenKind::Mul => write!(f, "mul"),
            TokenKind::Div => write!(f, "div"),
            TokenKind::Mod => write!(f, "mod"),
            TokenKind::Gt => write!(f, "gt"),
            TokenKind::Lt => write!(f, "lt"),
            TokenKind::Eq => write!(f, "eq"),
            TokenKind::Ge => write!(f, "ge"),
            TokenKind::Le => write!(f, "le"),
            TokenKind::Ne => write!(f, "ne"),
            TokenKind::Assign => write!(f, "="),
            TokenKind::True => write!(f, "true"),
            TokenKind::False => write!(f, "false"),
            TokenKind::Null => write!(f, "null"),
            TokenKind::Match => write!(f, "match"),
            TokenKind::Default => write!(f, "default"),
            TokenKind::Break => write!(f, "break"),
            TokenKind::Continue => write!(f, "continue"),
            TokenKind::Shl => write!(f, "shl"),
            TokenKind::Shr => write!(f, "shr"),
            TokenKind::BitAnd => write!(f, "bitand"),
            TokenKind::BitOr => write!(f, "bitor"),
            TokenKind::BitXor => write!(f, "bitxor"),
            TokenKind::Asm => write!(f, "asm"),
            TokenKind::Volatile => write!(f, "volatile"),
            TokenKind::Global => write!(f, "global"),
            TokenKind::Section => write!(f, "section"),
            TokenKind::Align => write!(f, "align"),
            TokenKind::Peripheral => write!(f, "peripheral"),
            TokenKind::Register => write!(f, "register"),
            TokenKind::Memory => write!(f, "memory"),
            TokenKind::Layout => write!(f, "layout"),
            TokenKind::Segment => write!(f, "segment"),
            TokenKind::Spawn => write!(f, "spawn"),
            TokenKind::TaskGroup => write!(f, "task_group"),
            TokenKind::Channel => write!(f, "channel"),
            TokenKind::Mutex => write!(f, "mutex"),
            TokenKind::RwLock => write!(f, "rwlock"),
            TokenKind::Atomic => write!(f, "atomic"),
            TokenKind::Barrier => write!(f, "barrier"),
            TokenKind::Once => write!(f, "once"),
            TokenKind::Send => write!(f, "send"),
            TokenKind::Sync => write!(f, "sync"),
            TokenKind::Async => write!(f, "async"),
            TokenKind::Await => write!(f, "await"),
            TokenKind::Where => write!(f, "where"),
            TokenKind::Super => write!(f, "super"),
            TokenKind::Static => write!(f, "static"),
            TokenKind::Extern => write!(f, "extern"),
            TokenKind::Type => write!(f, "type"),
            TokenKind::Mut => write!(f, "mut"),
            TokenKind::TypeFn => write!(f, "fn"),
            TokenKind::TypeInt => write!(f, "int"),
            TokenKind::TypeI8 => write!(f, "i8"),
            TokenKind::TypeI16 => write!(f, "i16"),
            TokenKind::TypeI32 => write!(f, "i32"),
            TokenKind::TypeI64 => write!(f, "i64"),
            TokenKind::TypeU8 => write!(f, "u8"),
            TokenKind::TypeU16 => write!(f, "u16"),
            TokenKind::TypeU32 => write!(f, "u32"),
            TokenKind::TypeU64 => write!(f, "u64"),
            TokenKind::TypeF32 => write!(f, "f32"),
            TokenKind::TypeF64 => write!(f, "f64"),
            TokenKind::TypeBool => write!(f, "bool"),
            TokenKind::TypeChar => write!(f, "char"),
            TokenKind::TypeString => write!(f, "string"),
            TokenKind::TypeUnit => write!(f, "unit"),
            TokenKind::TypeList => write!(f, "list"),
            TokenKind::TypeArray => write!(f, "array"),
            TokenKind::TypeMap => write!(f, "map"),
            TokenKind::TypePtr => write!(f, "ptr"),
            TokenKind::TypeOption => write!(f, "option"),
            TokenKind::IntLit(n) => write!(f, "{}", n),
            TokenKind::FloatLit(n) => write!(f, "{}", n),
            TokenKind::StringLit(s) => write!(f, "\"{}\"", s),
            TokenKind::CharLit(c) => write!(f, "'{}'", c),
            TokenKind::Ident(s) => write!(f, "{}", s),
            TokenKind::LParen => write!(f, "("),
            TokenKind::RParen => write!(f, ")"),
            TokenKind::LBrace => write!(f, "{{"),
            TokenKind::RBrace => write!(f, "}}"),
            TokenKind::LBracket => write!(f, "["),
            TokenKind::RBracket => write!(f, "]"),
            TokenKind::Comma => write!(f, ","),
            TokenKind::Dot => write!(f, "."),
            TokenKind::Colon => write!(f, ":"),
            TokenKind::Semicolon => write!(f, ";"),
            TokenKind::Arrow => write!(f, "->"),
            TokenKind::FatArrow => write!(f, "=>"),
            TokenKind::DoubleColon => write!(f, "::"),
            TokenKind::Plus => write!(f, "+"),
            TokenKind::Minus => write!(f, "-"),
            TokenKind::Star => write!(f, "*"),
            TokenKind::Slash => write!(f, "/"),
            TokenKind::Percent => write!(f, "%"),
            TokenKind::EqEq => write!(f, "=="),
            TokenKind::PlusEq => write!(f, "+="),
            TokenKind::MinusEq => write!(f, "-="),
            TokenKind::StarEq => write!(f, "*="),
            TokenKind::SlashEq => write!(f, "/="),
            TokenKind::PercentEq => write!(f, "%="),
            TokenKind::ShlEq => write!(f, "<<="),
            TokenKind::ShrEq => write!(f, ">>="),
            TokenKind::BitAndEq => write!(f, "&="),
            TokenKind::BitOrEq => write!(f, "|="),
            TokenKind::BitXorEq => write!(f, "^="),
            TokenKind::QuestionMark => write!(f, "?"),
            TokenKind::At => write!(f, "@"),
            TokenKind::Comment(_) => write!(f, "// comment"),
            TokenKind::Whitespace => write!(f, " "),
            TokenKind::Newline => write!(f, "\\n"),
            TokenKind::Eof => write!(f, "<EOF>"),
        }
    }
}

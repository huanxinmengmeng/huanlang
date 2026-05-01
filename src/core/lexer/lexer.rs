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

use super::token::{Token, TokenKind, SourcePosition, SourceSpan};
use super::keywords::KeywordTable;

#[derive(Debug, Clone, PartialEq)]
pub enum LexError {
    UnexpectedChar { ch: char, pos: SourcePosition },
    UnclosedString { start: SourcePosition },
    UnclosedChar { start: SourcePosition },
    InvalidEscape { seq: String, pos: SourcePosition },
    NumericOverflow { pos: SourcePosition },
    CharLiteralTooLong { pos: SourcePosition },
    InvalidUnicode { value: u32, pos: SourcePosition },
}

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
    keyword_table: KeywordTable,
    errors: Vec<LexError>,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Self {
            chars: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
            keyword_table: KeywordTable::new(),
            errors: Vec::new(),
        }
    }

    #[inline]
    fn current_position(&self) -> SourcePosition {
        SourcePosition::new(self.pos, self.line, self.column)
    }

    #[inline]
    fn current_char(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    #[inline]
    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    #[inline]
    fn advance(&mut self) -> Option<char> {
        if let Some(ch) = self.current_char() {
            self.pos += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            } else {
                self.column += 1;
            }
            Some(ch)
        } else {
            None
        }
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.pos >= self.chars.len()
    }

    pub fn tokenize(&mut self) -> (Vec<Token>, Vec<LexError>) {
        let mut tokens = Vec::new();
        while !self.is_eof() {
            match self.next_token() {
                Ok(Some(token)) => tokens.push(token),
                Ok(None) => continue,
                Err(e) => {
                    self.errors.push(e);
                    self.advance();
                }
            }
        }
        tokens.push(Token::eof(self.current_position()));
        (tokens, std::mem::take(&mut self.errors))
    }

    fn next_token(&mut self) -> Result<Option<Token>, LexError> {
        self.skip_whitespace_and_comments();

        if self.is_eof() {
            return Ok(None);
        }

        let start = self.current_position();
        let ch = self.current_char().unwrap();

        if ch == '#' && start.offset == 0 && start.line == 1 && start.column == 1 {
            if self.peek_char() == Some('!') {
                self.advance();
                self.advance();
                self.skip_shebang_line();
                return self.next_token();
            }
        }

        let token = match ch {
            '0'..='9' => self.lex_number(start)?,
            '"' => self.lex_string(start)?,
            '\'' => self.lex_char(start)?,
            c if is_identifier_start(c) => self.lex_identifier_or_keyword(start)?,
            _ => self.lex_symbol(start)?,
        };

        Ok(Some(token))
    }

    fn skip_shebang_line(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn skip_whitespace_and_comments(&mut self) {
        while let Some(ch) = self.current_char() {
            match ch {
                c if is_whitespace(c) => {
                    self.advance();
                }
                '\n' | '\r' => {
                    self.advance();
                }
                '#' => {
                    self.advance();
                    self.skip_line_comment();
                }
                '-' if self.peek_char() == Some('-') => {
                    self.advance();
                    self.advance();
                    self.skip_line_comment();
                }
                '/' if self.peek_char() == Some('/') => {
                    self.advance();
                    self.advance();
                    self.skip_line_comment();
                }
                '/' if self.peek_char() == Some('*') => {
                    self.advance();
                    self.advance();
                    self.skip_block_comment();
                }
                _ => break,
            }
        }
    }

    fn skip_line_comment(&mut self) {
        while let Some(ch) = self.current_char() {
            if ch == '\n' {
                break;
            }
            self.advance();
        }
    }

    fn skip_block_comment(&mut self) {
        let mut depth = 1;
        while depth > 0 {
            if self.is_eof() {
                return;
            }
            match (self.current_char(), self.peek_char()) {
                (Some('/'), Some('*')) => {
                    self.advance();
                    self.advance();
                    depth += 1;
                }
                (Some('*'), Some('/')) => {
                    self.advance();
                    self.advance();
                    depth -= 1;
                }
                _ => {
                    self.advance();
                }
            }
        }
    }

    fn lex_number(&mut self, start: SourcePosition) -> Result<Token, LexError> {
        let mut lexeme = String::new();
        let mut is_float = false;
        let mut base = 10;

        if self.current_char() == Some('0') {
            match self.peek_char() {
                Some('x') | Some('X') => {
                    base = 16;
                    lexeme.push(self.advance().unwrap());
                    lexeme.push(self.advance().unwrap());
                }
                Some('b') | Some('B') => {
                    base = 2;
                    lexeme.push(self.advance().unwrap());
                    lexeme.push(self.advance().unwrap());
                }
                Some('o') | Some('O') => {
                    base = 8;
                    lexeme.push(self.advance().unwrap());
                    lexeme.push(self.advance().unwrap());
                }
                _ => {}
            }
        }

        while let Some(ch) = self.current_char() {
            match ch {
                '_' => {
                    lexeme.push(ch);
                    self.advance();
                }
                '0'..='9' => {
                    lexeme.push(ch);
                    self.advance();
                }
                'a'..='f' | 'A'..='F' if base == 16 => {
                    lexeme.push(ch);
                    self.advance();
                }
                '.' if base == 10 && !is_float => {
                    if let Some(next) = self.peek_char() {
                        if next.is_ascii_digit() {
                            is_float = true;
                            lexeme.push(ch);
                            self.advance();
                            continue;
                        }
                    }
                    break;
                }
                'e' | 'E' if base == 10 => {
                    is_float = true;
                    lexeme.push(ch);
                    self.advance();
                    if let Some(sign) = self.current_char() {
                        if sign == '+' || sign == '-' {
                            lexeme.push(sign);
                            self.advance();
                        }
                    }
                }
                _ => break,
            }
        }

        let end = self.current_position();
        let span = SourceSpan::new(start, end);

        let kind = if is_float {
            let val: f64 = lexeme.parse()
                .map_err(|_| LexError::NumericOverflow { pos: start })?;
            TokenKind::FloatLit(val)
        } else {
            let val = match base {
                16 => i64::from_str_radix(lexeme.trim_start_matches("0x").trim_start_matches("0X"), 16),
                2 => i64::from_str_radix(lexeme.trim_start_matches("0b").trim_start_matches("0B"), 2),
                8 => i64::from_str_radix(lexeme.trim_start_matches("0o").trim_start_matches("0O"), 8),
                _ => lexeme.parse::<i64>(),
            }.map_err(|_| LexError::NumericOverflow { pos: start })?;
            TokenKind::IntLit(val)
        };

        Ok(Token::new(kind, span, lexeme))
    }

    fn lex_string(&mut self, start: SourcePosition) -> Result<Token, LexError> {
        self.advance();
        let mut content = String::new();
        let mut lexeme = String::new();
        lexeme.push('"');

        while let Some(ch) = self.current_char() {
            if ch == '"' {
                lexeme.push(ch);
                self.advance();
                let end = self.current_position();
                let span = SourceSpan::new(start, end);
                return Ok(Token::new(TokenKind::StringLit(content), span, lexeme));
            }

            if ch == '\\' {
                lexeme.push(ch);
                self.advance();
                let escaped = self.parse_escape_sequence(start)?;
                content.push(escaped);
                lexeme.push(escaped);
                continue;
            }

            lexeme.push(ch);
            content.push(ch);
            self.advance();
        }

        Err(LexError::UnclosedString { start })
    }

    fn parse_escape_sequence(&mut self, pos: SourcePosition) -> Result<char, LexError> {
        match self.current_char() {
            Some('n') => { self.advance(); Ok('\n') }
            Some('r') => { self.advance(); Ok('\r') }
            Some('t') => { self.advance(); Ok('\t') }
            Some('\\') => { self.advance(); Ok('\\') }
            Some('"') => { self.advance(); Ok('"') }
            Some('\'') => { self.advance(); Ok('\'') }
            Some('0') => { self.advance(); Ok('\0') }
            Some('u') => {
                self.advance();
                if self.current_char() != Some('{') {
                    return Err(LexError::InvalidEscape { seq: "\\u".to_string(), pos });
                }
                self.advance();
                let mut hex = String::new();
                while let Some(ch) = self.current_char() {
                    if ch == '}' {
                        break;
                    }
                    if !ch.is_ascii_hexdigit() {
                        return Err(LexError::InvalidEscape { seq: format!("\\u{{{}}}", hex), pos });
                    }
                    hex.push(ch);
                    self.advance();
                }
                if self.current_char() != Some('}') {
                    return Err(LexError::InvalidEscape { seq: format!("\\u{{{}}}", hex), pos });
                }
                self.advance();

                let codepoint = u32::from_str_radix(&hex, 16)
                    .map_err(|_| LexError::InvalidEscape { seq: format!("\\u{{{}}}", hex), pos })?;
                char::from_u32(codepoint)
                    .ok_or_else(|| LexError::InvalidUnicode { value: codepoint, pos })
            }
            Some(c) => Err(LexError::InvalidEscape { seq: format!("\\{}", c), pos }),
            None => Err(LexError::UnclosedString { start: pos }),
        }
    }

    fn lex_char(&mut self, start: SourcePosition) -> Result<Token, LexError> {
        self.advance();
        let mut lexeme = String::new();
        lexeme.push('\'');

        let ch_val = if self.current_char() == Some('\\') {
            lexeme.push('\\');
            self.advance();
            let escaped = self.parse_escape_sequence(start)?;
            lexeme.push(escaped);
            escaped
        } else {
            let ch = self.current_char().ok_or(LexError::UnclosedChar { start })?;
            lexeme.push(ch);
            self.advance();
            ch
        };

        if self.current_char() != Some('\'') {
            return Err(LexError::UnclosedChar { start });
        }
        lexeme.push('\'');
        self.advance();

        let end = self.current_position();
        let span = SourceSpan::new(start, end);
        Ok(Token::new(TokenKind::CharLit(ch_val), span, lexeme))
    }

    fn lex_identifier_or_keyword(&mut self, start: SourcePosition) -> Result<Token, LexError> {
        // 先看看第一个字符是不是 CJK（中文）字符
        let first_char = self.current_char().unwrap();
        if is_cjk_char(first_char) {
            // 对于 CJK 字符序列，使用最长关键词匹配
            // 先收集所有可能的标识符字符
            let mut all_chars = Vec::new();
            let mut saved_pos = self.pos;
            let saved_line = self.line;
            let saved_column = self.column;

            while let Some(ch) = self.current_char() {
                if is_identifier_char(ch) {
                    all_chars.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }

            // 现在尝试从长到短匹配关键词
            for len in (1..=all_chars.len()).rev() {
                let candidate: String = all_chars[0..len].iter().collect();
                if let Some(kind) = self.keyword_table.get(candidate.as_str()) {
                    let is_valid = if len == all_chars.len() {
                        true
                    } else if len == 1 {
                        // 单字符关键词，只在后面跟着非标识符字符时匹配
                        !is_identifier_char(all_chars[len])
                    } else {
                        // 多字符关键词，只要 len+1 不是关键词就匹配
                        let longer_candidate: String = all_chars[0..len+1].iter().collect();
                        self.keyword_table.get(longer_candidate.as_str()).is_none()
                    };
                    if is_valid {
                        // 匹配成功！回退到 len 个字符之后
                        self.pos = saved_pos;
                        self.line = saved_line;
                        self.column = saved_column;
                        let mut lexeme = String::new();
                        for _ in 0..len {
                            if let Some(ch) = self.current_char() {
                                lexeme.push(ch);
                                self.advance();
                            }
                        }
                        let end = self.current_position();
                        let span = SourceSpan::new(start, end);
                        return Ok(Token::new(kind, span, lexeme));
                    }
                }
            }

            // 如果没有匹配到任何关键词，就把所有字符作为标识符
            let lexeme: String = all_chars.into_iter().collect();
            let end = self.current_position();
            let span = SourceSpan::new(start, end);

            return Ok(Token::new(TokenKind::Ident(lexeme.clone()), span, lexeme));
        } else {
            // 对于非 CJK 字符（英文、拼音），使用原来的方法
            let mut lexeme = String::new();
            while let Some(ch) = self.current_char() {
                if is_identifier_char(ch) {
                    lexeme.push(ch);
                    self.advance();
                } else {
                    break;
                }
            }

            let end = self.current_position();
            let span = SourceSpan::new(start, end);

            let kind = if let Some(kind) = self.keyword_table.get(lexeme.as_str()) {
                kind
            } else {
                TokenKind::Ident(lexeme.clone())
            };

            return Ok(Token::new(kind, span, lexeme));
        }
    }

    fn lex_symbol(&mut self, start: SourcePosition) -> Result<Token, LexError> {
        let mut lexeme = String::new();
        let ch = self.current_char().unwrap();
        lexeme.push(ch);
        self.advance();

        let kind = match ch {
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            ',' => TokenKind::Comma,
            '.' => TokenKind::Dot,
            ';' => TokenKind::Semicolon,
            ':' => {
                if self.current_char() == Some(':') {
                    lexeme.push(':');
                    self.advance();
                    TokenKind::DoubleColon
                } else {
                    TokenKind::Colon
                }
            }
            '=' => {
                if self.current_char() == Some('=') {
                    lexeme.push('=');
                    self.advance();
                    TokenKind::EqEq
                } else if self.current_char() == Some('>') {
                    lexeme.push('>');
                    self.advance();
                    TokenKind::FatArrow
                } else {
                    TokenKind::Assign
                }
            }
            '+' => {
                if self.current_char() == Some('=') {
                    lexeme.push('=');
                    self.advance();
                    TokenKind::PlusEq
                } else {
                    TokenKind::Plus
                }
            }
            '-' => {
                if self.current_char() == Some('=') {
                    lexeme.push('=');
                    self.advance();
                    TokenKind::MinusEq
                } else if self.current_char() == Some('>') {
                    lexeme.push('>');
                    self.advance();
                    TokenKind::Arrow
                } else {
                    TokenKind::Minus
                }
            }
            '*' => {
                if self.current_char() == Some('=') {
                    lexeme.push('=');
                    self.advance();
                    TokenKind::StarEq
                } else {
                    TokenKind::Star
                }
            }
            '/' => {
                if self.current_char() == Some('=') {
                    lexeme.push('=');
                    self.advance();
                    TokenKind::SlashEq
                } else {
                    TokenKind::Slash
                }
            }
            '%' => {
                if self.current_char() == Some('=') {
                    lexeme.push('=');
                    self.advance();
                    TokenKind::PercentEq
                } else {
                    TokenKind::Percent
                }
            }
            '<' => {
                if self.current_char() == Some('=') {
                    lexeme.push('=');
                    self.advance();
                    TokenKind::Le
                } else if self.current_char() == Some('<') {
                    lexeme.push('<');
                    self.advance();
                    if self.current_char() == Some('=') {
                        lexeme.push('=');
                        self.advance();
                        TokenKind::ShlEq
                    } else {
                        TokenKind::Shl
                    }
                } else {
                    TokenKind::Lt
                }
            }
            '>' => {
                if self.current_char() == Some('=') {
                    lexeme.push('=');
                    self.advance();
                    TokenKind::Ge
                } else if self.current_char() == Some('>') {
                    lexeme.push('>');
                    self.advance();
                    if self.current_char() == Some('=') {
                        lexeme.push('=');
                        self.advance();
                        TokenKind::ShrEq
                    } else {
                        TokenKind::Shr
                    }
                } else {
                    TokenKind::Gt
                }
            }
            '!' => {
                if self.current_char() == Some('=') {
                    lexeme.push('=');
                    self.advance();
                    TokenKind::Ne
                } else {
                    TokenKind::Not
                }
            }
            '&' => {
                if self.current_char() == Some('&') {
                    lexeme.push('&');
                    self.advance();
                    TokenKind::And
                } else if self.current_char() == Some('=') {
                    lexeme.push('=');
                    self.advance();
                    TokenKind::BitAndEq
                } else {
                    TokenKind::BitAnd
                }
            }
            '|' => {
                if self.current_char() == Some('|') {
                    lexeme.push('|');
                    self.advance();
                    TokenKind::Or
                } else if self.current_char() == Some('=') {
                    lexeme.push('=');
                    self.advance();
                    TokenKind::BitOrEq
                } else {
                    TokenKind::BitOr
                }
            }
            '^' => {
                if self.current_char() == Some('=') {
                    lexeme.push('=');
                    self.advance();
                    TokenKind::BitXorEq
                } else {
                    TokenKind::BitXor
                }
            }
            '?' => {
                TokenKind::QuestionMark
            }
            '@' => {
                TokenKind::At
            }
            _ => return Err(LexError::UnexpectedChar { ch, pos: start }),
        };

        let end = self.current_position();
        let span = SourceSpan::new(start, end);

        Ok(Token::new(kind, span, lexeme))
    }
}

fn is_cjk_char(ch: char) -> bool {
    // 简化的中文字符检查
    ch as u32 >= 0x4E00 && ch as u32 <= 0x9FFF
}

fn is_identifier_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || is_cjk_char(ch)
}

fn is_identifier_start(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_' || is_cjk_char(ch)
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\r'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex_mixed_keywords() {
        let source = "let 年龄 = 25";
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Ident("年龄".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Assign);
        assert_eq!(tokens[3].kind, TokenKind::IntLit(25));
    }

    #[test]
    fn test_lex_chinese_keywords() {
        let source = "令 x 为 10";
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Ident("x".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Be);
        assert_eq!(tokens[3].kind, TokenKind::IntLit(10));
    }

    #[test]
    fn test_lex_pinyin_keywords() {
        let source = "ruo x dengyu 10 ze xiaoshi";
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::If);
    }

    #[test]
    fn test_lex_string_with_escape() {
        let source = r#""Hello\n世界""#;
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        if let TokenKind::StringLit(s) = &tokens[0].kind {
            assert_eq!(s, "Hello\n世界");
        } else {
            panic!("Expected string literal");
        }
    }

    #[test]
    fn test_lex_comments() {
        let source = "-- 这是注释\n令 x = 1 /* 块注释 */";
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Let);
    }

    #[test]
    fn test_lex_numbers() {
        let source = "42 3.14 0xFF 0b1010 0o77";
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::IntLit(42));
        assert_eq!(tokens[1].kind, TokenKind::FloatLit(3.14));
        assert_eq!(tokens[2].kind, TokenKind::IntLit(255));
        assert_eq!(tokens[3].kind, TokenKind::IntLit(10));
        assert_eq!(tokens[4].kind, TokenKind::IntLit(63));
    }

    #[test]
    fn test_lex_symbols() {
        let source = "( ) { } [ ] , . ; : :: = == + += - -= * *= / /= % %= < <= << <<= > >= >> >>= ! != & && &= | || |= ^ ^=";
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::LParen);
        assert_eq!(tokens[1].kind, TokenKind::RParen);
        assert_eq!(tokens[2].kind, TokenKind::LBrace);
        assert_eq!(tokens[3].kind, TokenKind::RBrace);
        assert_eq!(tokens[4].kind, TokenKind::LBracket);
        assert_eq!(tokens[5].kind, TokenKind::RBracket);
        assert_eq!(tokens[6].kind, TokenKind::Comma);
        assert_eq!(tokens[7].kind, TokenKind::Dot);
        assert_eq!(tokens[8].kind, TokenKind::Semicolon);
        assert_eq!(tokens[9].kind, TokenKind::Colon);
        assert_eq!(tokens[10].kind, TokenKind::DoubleColon);
        assert_eq!(tokens[11].kind, TokenKind::Assign);
        assert_eq!(tokens[12].kind, TokenKind::EqEq);
        assert_eq!(tokens[13].kind, TokenKind::Plus);
        assert_eq!(tokens[14].kind, TokenKind::PlusEq);
        assert_eq!(tokens[15].kind, TokenKind::Minus);
        assert_eq!(tokens[16].kind, TokenKind::MinusEq);
        assert_eq!(tokens[17].kind, TokenKind::Star);
        assert_eq!(tokens[18].kind, TokenKind::StarEq);
        assert_eq!(tokens[19].kind, TokenKind::Slash);
        assert_eq!(tokens[20].kind, TokenKind::SlashEq);
        assert_eq!(tokens[21].kind, TokenKind::Percent);
        assert_eq!(tokens[22].kind, TokenKind::PercentEq);
        assert_eq!(tokens[23].kind, TokenKind::Lt);
        assert_eq!(tokens[24].kind, TokenKind::Le);
        assert_eq!(tokens[25].kind, TokenKind::Shl);
        assert_eq!(tokens[26].kind, TokenKind::ShlEq);
        assert_eq!(tokens[27].kind, TokenKind::Gt);
        assert_eq!(tokens[28].kind, TokenKind::Ge);
        assert_eq!(tokens[29].kind, TokenKind::Shr);
        assert_eq!(tokens[30].kind, TokenKind::ShrEq);
        assert_eq!(tokens[31].kind, TokenKind::Not);
        assert_eq!(tokens[32].kind, TokenKind::Ne);
        assert_eq!(tokens[33].kind, TokenKind::BitAnd);
        assert_eq!(tokens[34].kind, TokenKind::And);
        assert_eq!(tokens[35].kind, TokenKind::BitAndEq);
        assert_eq!(tokens[36].kind, TokenKind::BitOr);
        assert_eq!(tokens[37].kind, TokenKind::Or);
        assert_eq!(tokens[38].kind, TokenKind::BitOrEq);
        assert_eq!(tokens[39].kind, TokenKind::BitXor);
        assert_eq!(tokens[40].kind, TokenKind::BitXorEq);
    }

    #[test]
    fn test_lex_arrow() {
        let source = "->";
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Arrow);
    }

    #[test]
    fn test_lex_literals() {
        let source = r#"true false null 'a' '\n' '中' "hello" "line1\nline2" "unicode \u{4E2D}""#;
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::True);
        assert_eq!(tokens[1].kind, TokenKind::False);
        assert_eq!(tokens[2].kind, TokenKind::Null);
        assert_eq!(tokens[3].kind, TokenKind::CharLit('a'));
        assert_eq!(tokens[4].kind, TokenKind::CharLit('\n'));
        assert_eq!(tokens[5].kind, TokenKind::CharLit('中'));
        if let TokenKind::StringLit(s) = &tokens[6].kind {
            assert_eq!(s, "hello");
        }
        if let TokenKind::StringLit(s) = &tokens[7].kind {
            assert_eq!(s, "line1\nline2");
        }
    }

    #[test]
    fn test_lex_cjk_identifiers() {
        let source = "数量 中文变量 my_var _private_var";
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Ident("数量".to_string()));
        assert_eq!(tokens[1].kind, TokenKind::Ident("中文变量".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Ident("my_var".to_string()));
        assert_eq!(tokens[3].kind, TokenKind::Ident("_private_var".to_string()));
    }

    #[test]
    fn test_lex_nested_comments() {
        let source = "/* outer /* inner */ */ 令 x = 5";
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Let);
    }

    #[test]
    fn test_lex_shebang() {
        let source = "#!/usr/bin/huanyu\n令 x = 10";
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Let);
    }

    #[test]
    fn test_lex_errors_unclosed_string() {
        let source = r#""unclosed string"#;
        let mut lexer = Lexer::new(source);
        let (_tokens, errors) = lexer.tokenize();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_lex_errors_unclosed_char() {
        let source = r#"'a"#;
        let mut lexer = Lexer::new(source);
        let (_tokens, errors) = lexer.tokenize();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_lex_errors_invalid_escape() {
        let source = r#""\z""#;
        let mut lexer = Lexer::new(source);
        let (_tokens, errors) = lexer.tokenize();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_lex_errors_unexpected_char() {
        let source = "@ ` ~";
        let mut lexer = Lexer::new(source);
        let (_tokens, errors) = lexer.tokenize();
        assert_eq!(errors.len(), 3);
    }

    #[test]
    fn test_lex_complete_program() {
        let source = r#"
-- 计算阶乘的函数
令 阶乘 为 函数(n 类型 整数): 整数
    若 n 不大于 1
        返回 1
    否则
        返回 n 乘 阶乘(n 减 1)
结束
-- 主程序
令 结果 为 阶乘(5)
显示(结果)
        "#;
        let mut lexer = Lexer::new(source);
        let (tokens, errors) = lexer.tokenize();
        assert!(errors.is_empty());
        assert_eq!(tokens[0].kind, TokenKind::Let);
        assert_eq!(tokens[1].kind, TokenKind::Ident("阶乘".to_string()));
        assert_eq!(tokens[2].kind, TokenKind::Be);
        assert_eq!(tokens[3].kind, TokenKind::Func);
        assert_eq!(tokens[4].kind, TokenKind::LParen);
        assert_eq!(tokens[5].kind, TokenKind::Ident("n".to_string()));
        assert_eq!(tokens[6].kind, TokenKind::TypeAnno);
        assert_eq!(tokens[7].kind, TokenKind::TypeInt);
        assert_eq!(tokens[8].kind, TokenKind::RParen);
        assert_eq!(tokens[9].kind, TokenKind::Colon);
        assert_eq!(tokens[10].kind, TokenKind::TypeInt);
        assert_eq!(tokens[11].kind, TokenKind::If);
        assert_eq!(tokens[12].kind, TokenKind::Ident("n".to_string()));
        assert_eq!(tokens[13].kind, TokenKind::Le);
        assert_eq!(tokens[14].kind, TokenKind::IntLit(1));
        assert_eq!(tokens[15].kind, TokenKind::Return);
        assert_eq!(tokens[16].kind, TokenKind::IntLit(1));
        assert_eq!(tokens[17].kind, TokenKind::Else);
        assert_eq!(tokens[18].kind, TokenKind::Return);
        assert_eq!(tokens[19].kind, TokenKind::Ident("n".to_string()));
        assert_eq!(tokens[20].kind, TokenKind::Mul);
        assert_eq!(tokens[21].kind, TokenKind::Ident("阶乘".to_string()));
        assert_eq!(tokens[22].kind, TokenKind::LParen);
        assert_eq!(tokens[23].kind, TokenKind::Ident("n".to_string()));
        assert_eq!(tokens[24].kind, TokenKind::Sub);
        assert_eq!(tokens[25].kind, TokenKind::IntLit(1));
        assert_eq!(tokens[26].kind, TokenKind::RParen);
        assert_eq!(tokens[27].kind, TokenKind::End);
        assert_eq!(tokens[28].kind, TokenKind::Let);
        assert_eq!(tokens[29].kind, TokenKind::Ident("结果".to_string()));
        assert_eq!(tokens[30].kind, TokenKind::Be);
        assert_eq!(tokens[31].kind, TokenKind::Ident("阶乘".to_string()));
        assert_eq!(tokens[32].kind, TokenKind::LParen);
        assert_eq!(tokens[33].kind, TokenKind::IntLit(5));
        assert_eq!(tokens[34].kind, TokenKind::RParen);
        assert_eq!(tokens[35].kind, TokenKind::Ident("显示".to_string()));
        assert_eq!(tokens[36].kind, TokenKind::LParen);
        assert_eq!(tokens[37].kind, TokenKind::Ident("结果".to_string()));
        assert_eq!(tokens[38].kind, TokenKind::RParen);
    }
}

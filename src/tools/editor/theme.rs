
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn from_hex(hex: &str) -> Self {
        let hex = hex.trim_start_matches('#');
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
        Self { r, g, b }
    }
    
    pub fn to_ansi_fg(&self) -> String {
        format!("\x1b[38;2;{};{};{}m", self.r, self.g, self.b)
    }
    
    pub fn to_ansi_bg(&self) -> String {
        format!("\x1b[48;2;{};{};{}m", self.r, self.g, self.b)
    }
}

impl Default for Color {
    fn default() -> Self {
        Self { r: 255, g: 255, b: 255 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub background: Color,
    pub foreground: Color,
    pub keyword: Color,
    pub type_name: Color,
    pub function: Color,
    pub string: Color,
    pub comment: Color,
    pub number: Color,
    pub operator: Color,
    pub selection: Color,
    pub line_number: Color,
    pub status_line_bg: Color,
    pub status_line_fg: Color,
    pub error: Color,
    pub warning: Color,
    pub info: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "幻语默认".to_string(),
            background: Color::from_hex("#1E1E1E"),
            foreground: Color::from_hex("#D4D4D4"),
            keyword: Color::from_hex("#569CD6"),
            type_name: Color::from_hex("#4EC9B0"),
            function: Color::from_hex("#DCDCAA"),
            string: Color::from_hex("#CE9178"),
            comment: Color::from_hex("#6A9955"),
            number: Color::from_hex("#B5CEA8"),
            operator: Color::from_hex("#D4D4D4"),
            selection: Color::from_hex("#264F78"),
            line_number: Color::from_hex("#858585"),
            status_line_bg: Color::from_hex("#007ACC"),
            status_line_fg: Color::from_hex("#FFFFFF"),
            error: Color::from_hex("#F44747"),
            warning: Color::from_hex("#CCA700"),
            info: Color::from_hex("#75BEFF"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HighlightSpan {
    pub start_offset: usize,
    pub end_offset: usize,
    pub highlight_type: HighlightType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HighlightType {
    Keyword,
    Type,
    Function,
    String,
    Comment,
    Number,
    Operator,
    None,
}

pub struct SyntaxHighlighter {
    theme: Theme,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }
    
    pub fn new_with_theme(theme: Theme) -> Self {
        Self { theme }
    }
    
    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = theme;
    }
    
    pub fn theme(&self) -> &Theme {
        &self.theme
    }
    
    pub fn highlight_line(&self, line: &str, line_offset: usize) -> Vec<HighlightSpan> {
        let mut spans = Vec::new();
        let mut i = 0;
        let chars: Vec<char> = line.chars().collect();
        
        while i < chars.len() {
            if chars[i].is_whitespace() {
                i += 1;
                continue;
            }
            
            if chars[i] == '/' && i + 1 < chars.len() && chars[i + 1] == '/' {
                spans.push(HighlightSpan {
                    start_offset: line_offset + i,
                    end_offset: line_offset + chars.len(),
                    highlight_type: HighlightType::Comment,
                });
                break;
            }
            
            if chars[i] == '"' || chars[i] == '\'' {
                let quote = chars[i];
                let start = i;
                i += 1;
                while i < chars.len() && chars[i] != quote {
                    if chars[i] == '\\' {
                        i += 2;
                    } else {
                        i += 1;
                    }
                }
                i += 1;
                spans.push(HighlightSpan {
                    start_offset: line_offset + start,
                    end_offset: line_offset + i,
                    highlight_type: HighlightType::String,
                });
                continue;
            }
            
            if chars[i].is_ascii_digit() {
                let start = i;
                while i < chars.len() && chars[i].is_ascii_digit() {
                    i += 1;
                }
                spans.push(HighlightSpan {
                    start_offset: line_offset + start,
                    end_offset: line_offset + i,
                    highlight_type: HighlightType::Number,
                });
                continue;
            }
            
            if chars[i].is_alphabetic() || chars[i] == '_' {
                let start = i;
                while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                    i += 1;
                }
                let word: String = chars[start..i].iter().collect();
                
                let highlight_type = self.classify_word(&word);
                spans.push(HighlightSpan {
                    start_offset: line_offset + start,
                    end_offset: line_offset + i,
                    highlight_type,
                });
                continue;
            }
            
            let operators = ['=', '+', '-', '*', '/', '%', '&', '|', '^', '!', '<', '>', '?', ':'];
            if operators.contains(&chars[i]) {
                spans.push(HighlightSpan {
                    start_offset: line_offset + i,
                    end_offset: line_offset + i + 1,
                    highlight_type: HighlightType::Operator,
                });
                i += 1;
                continue;
            }
            
            i += 1;
        }
        
        spans
    }
    
    fn classify_word(&self, word: &str) -> HighlightType {
        let keywords = [
            "令", "使", "是", "为", "如", "果", "则", "否", "则", "循",
            "环", "返", "回", "函", "数", "结", "构", "枚", "举", "类",
            "型", "模", "块", "导", "入", "pub", "fn", "struct", "enum",
            "let", "mut", "if", "else", "while", "for", "return", "use",
            "mod", "type", "impl", "trait", "match",
        ];
        
        if keywords.contains(&word) {
            return HighlightType::Keyword;
        }
        
        if word.chars().next().map_or(false, |c| c.is_uppercase()) {
            return HighlightType::Type;
        }
        
        HighlightType::None
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self { theme: Theme::default() }
    }
}

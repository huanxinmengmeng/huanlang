// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 文件格式处理模块
//! 
//! 根据「幻语编程语言 - 完整详细的开发规范文档.md」第 1.2 - 1.4 节
//! 实现文件编码检查、Shebang 解析、行结束符规范化、文件类型识别等功能。

use crate::lang_identity::*;
use std::fs;
use std::io::Read;
use std::path::Path;

/// 文件加载结果
#[derive(Debug, Clone)]
pub struct LoadedFile {
    /// 文件路径
    pub path: String,
    /// 文件类型
    pub file_type: FileType,
    /// 规范化后的源码内容
    pub content: String,
    /// Shebang 内容（如果有）
    pub shebang: Option<String>,
    /// Shebang 参数（如果有）
    pub shebang_args: Vec<String>,
    /// 是否有 BOM
    pub has_bom: bool,
    /// 原始文件中的换行符类型
    pub original_line_endings: LineEnding,
    /// 原始行号与规范化行号的映射
    pub line_number_mapping: Vec<(usize, usize)>,
}

/// 换行符类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LineEnding {
    /// LF (`\n`)
    Lf,
    /// CRLF (`\r\n`)
    Crlf,
    /// CR (`\r`)
    Cr,
    /// 混合类型
    Mixed,
}

/// 文件加载错误类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadFileError {
    /// IO 错误
    IoError(String),
    /// 不是 UTF-8 编码
    NotUtf8(String),
    /// 魔法数字不匹配
    InvalidMagicNumber(String),
    /// 文件为空
    EmptyFile,
    /// 其他错误
    Other(String),
}

impl std::fmt::Display for LoadFileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LoadFileError::IoError(msg) => write!(f, "IO 错误: {}", msg),
            LoadFileError::NotUtf8(msg) => write!(f, "编码错误: {}", msg),
            LoadFileError::InvalidMagicNumber(msg) => write!(f, "无效的魔法数字: {}", msg),
            LoadFileError::EmptyFile => write!(f, "文件为空"),
            LoadFileError::Other(msg) => write!(f, "其他错误: {}", msg),
        }
    }
}

/// 加载并处理幻语源文件
/// 
/// 根据规范文档第 1.2 - 1.4 节的要求：
/// - 检查 UTF-8 编码，跳过 BOM
/// - 解析 Shebang 行（如果有）
/// - 规范化行结束符为 LF
/// - 识别文件类型
pub fn load_file(path: &str) -> Result<LoadedFile, LoadFileError> {
    let path_obj = Path::new(path);
    
    let mut bytes = Vec::new();
    fs::File::open(path_obj)
        .and_then(|mut file| file.read_to_end(&mut bytes))
        .map_err(|e| LoadFileError::IoError(e.to_string()))?;
    
    if bytes.is_empty() {
        return Err(LoadFileError::EmptyFile);
    }
    
    let file_type = FileType::from_path(path);
    
    check_magic_number(&bytes, file_type)?;
    
    let (has_bom, content_str) = process_utf8(&bytes)?;
    
    let (shebang, remaining_content, shebang_args) = parse_shebang(&content_str);
    
    let (normalized, line_ending, mapping) = normalize_line_endings(&remaining_content);
    
    Ok(LoadedFile {
        path: path.to_string(),
        file_type,
        content: normalized,
        shebang,
        shebang_args,
        has_bom,
        original_line_endings: line_ending,
        line_number_mapping: mapping,
    })
}

/// 检查魔术数字
fn check_magic_number(bytes: &[u8], file_type: FileType) -> Result<(), LoadFileError> {
    match file_type {
        FileType::Package => {
            if bytes.len() < 2 || &bytes[0..2] != MAGIC_HLP {
                return Err(LoadFileError::InvalidMagicNumber(
                    "包文件 (.hlp) 必须是 gzip 格式 (0x1F 0x8B)".to_string()
                ));
            }
        }
        _ => {}
    }
    
    Ok(())
}

/// 处理 UTF-8 编码，跳过 BOM
fn process_utf8(bytes: &[u8]) -> Result<(bool, String), LoadFileError> {
    let has_bom = bytes.starts_with(UTF8_BOM);
    let content_bytes = if has_bom {
        &bytes[3..]
    } else {
        bytes
    };
    
    let content_str = String::from_utf8(content_bytes.to_vec())
        .map_err(|e| LoadFileError::NotUtf8(e.to_string()))?;
    
    Ok((has_bom, content_str))
}

/// 解析 Shebang 行
fn parse_shebang(content: &str) -> (Option<String>, String, Vec<String>) {
    if !content.starts_with(SHEBANG_PREFIX) {
        return (None, content.to_string(), Vec::new());
    }
    
    let lines: Vec<&str> = content.splitn(2, '\n').collect();
    if lines.len() < 2 {
        let shebang = lines[0].to_string();
        let args = parse_shebang_args(&shebang);
        return (Some(shebang), String::new(), args);
    }
    
    let shebang = lines[0].to_string();
    let args = parse_shebang_args(&shebang);
    let remaining_content = lines[1].to_string();
    
    (Some(shebang), remaining_content, args)
}

/// 解析 Shebang 中的参数
fn parse_shebang_args(shebang: &str) -> Vec<String> {
    let without_prefix = shebang.strip_prefix(SHEBANG_PREFIX).unwrap_or(shebang);
    let parts: Vec<String> = without_prefix.split_whitespace()
        .map(|s| s.to_string())
        .collect();
    
    // 找到 "huan" 之后的参数
    if let Some(index) = parts.iter().position(|p| p == "huan") {
        parts[index + 1..].to_vec()
    } else {
        // 如果没有找到 "huan"，返回所有参数除了第一个
        if !parts.is_empty() {
            parts[1..].to_vec()
        } else {
            Vec::new()
        }
    }
}

/// 规范化行结束符
fn normalize_line_endings(content: &str) -> (String, LineEnding, Vec<(usize, usize)>) {
    let mut result = String::new();
    let mut mapping = Vec::new();
    let mut has_lf = false;
    let mut has_crlf = false;
    let mut has_cr = false;
    
    let mut chars = content.chars().peekable();
    let mut original_line = 1;
    let mut normalized_line = 1;
    
    while let Some(c) = chars.next() {
        match c {
            '\r' => {
                if let Some(&'\n') = chars.peek() {
                    has_crlf = true;
                    chars.next();
                } else {
                    has_cr = true;
                }
                result.push('\n');
                mapping.push((original_line, normalized_line));
                original_line += 1;
                normalized_line += 1;
            }
            '\n' => {
                has_lf = true;
                result.push('\n');
                mapping.push((original_line, normalized_line));
                original_line += 1;
                normalized_line += 1;
            }
            _ => {
                result.push(c);
            }
        }
    }
    
    let line_ending = match (has_lf, has_crlf, has_cr) {
        (true, false, false) => LineEnding::Lf,
        (false, true, false) => LineEnding::Crlf,
        (false, false, true) => LineEnding::Cr,
        _ => LineEnding::Mixed,
    };
    
    (result, line_ending, mapping)
}

/// 获取规范化后的行号
pub fn get_normalized_line_number(mapping: &[(usize, usize)], original_line: usize) -> usize {
    mapping.iter()
        .find(|&&(orig, _)| orig == original_line)
        .map(|&(_, norm)| norm)
        .unwrap_or(original_line)
}

/// 检查是否为有效的标识符
pub fn is_valid_identifier(s: &str) -> bool {
    if s.is_empty() || s.len() > MAX_IDENTIFIER_LENGTH {
        return false;
    }
    
    let mut chars = s.chars();
    let first_char = match chars.next() {
        None => return false,
        Some(c) => c,
    };
    
    if !is_identifier_start_char(first_char) {
        return false;
    }
    
    for c in chars {
        if !is_identifier_char(c) {
            return false;
        }
    }
    
    true
}

/// 检查是否为有效的标识符起始字符
fn is_identifier_start_char(c: char) -> bool {
    match c {
        'a'..='z' | 'A'..='Z' | '_' => true,
        c if is_cjk_character(c) => true,
        _ => false,
    }
}

/// 检查是否为有效的标识符字符
fn is_identifier_char(c: char) -> bool {
    match c {
        '0'..='9' => true,
        'a'..='z' | 'A'..='Z' | '_' => true,
        c if is_cjk_character(c) => true,
        _ => false,
    }
}

/// 检查是否为 CJK 字符
fn is_cjk_character(c: char) -> bool {
    let code_point = c as u32;
    
    // CJK 统一表意字符 (U+4E00 - U+9FFF)
    (code_point >= 0x4E00 && code_point <= 0x9FFF)
    // CJK 扩展 A (U+3400 - U+4DBF)
    || (code_point >= 0x3400 && code_point <= 0x4DBF)
    // CJK 扩展 B (U+20000 - U+2A6DF)
    || (code_point >= 0x20000 && code_point <= 0x2A6DF)
    // CJK 兼容表意字符 (U+F900 - U+FAFF)
    || (code_point >= 0xF900 && code_point <= 0xFAFF)
}

/// 检查是否为空白字符（根据规范第 1.3.4 节）
pub fn is_whitespace(c: char) -> bool {
    matches!(c, '\u{0020}' | '\u{0009}' | '\u{000A}' | '\u{000D}')
}

/// 格式化版本号比较结果
pub fn format_version_compare(v1: &str, v2: &str) -> String {
    match compare_version(v1, v2) {
        Some(std::cmp::Ordering::Less) => format!("{} < {}", v1, v2),
        Some(std::cmp::Ordering::Equal) => format!("{} == {}", v1, v2),
        Some(std::cmp::Ordering::Greater) => format!("{} > {}", v1, v2),
        None => "版本号格式错误".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shebang() {
        let content = "#!/usr/bin/env huan --ownership\n导入 \"标准库/核心\"";
        let (shebang, remaining, args) = parse_shebang(content);
        
        assert_eq!(shebang, Some("#!/usr/bin/env huan --ownership".to_string()));
        assert!(remaining.starts_with("导入"));
        assert_eq!(args, vec!["--ownership".to_string()]);
    }

    #[test]
    fn test_normalize_line_endings() {
        let content = "第一行\r\n第二行\r第三行\n";
        let (normalized, ending, _) = normalize_line_endings(content);
        
        assert_eq!(ending, LineEnding::Mixed);
        assert_eq!(normalized, "第一行\n第二行\n第三行\n");
    }

    #[test]
    fn test_valid_identifier() {
        assert!(is_valid_identifier("hello"));
        assert!(is_valid_identifier("你好"));
        assert!(is_valid_identifier("_hello"));
        assert!(is_valid_identifier("hello123"));
        assert!(!is_valid_identifier(""));
        assert!(!is_valid_identifier("123hello"));
    }

    #[test]
    fn test_cjk_identifier() {
        assert!(is_valid_identifier("幻语"));
        assert!(is_valid_identifier("函数_主"));
        assert!(!is_valid_identifier("123测试"));
    }

    #[test]
    fn test_is_whitespace() {
        assert!(is_whitespace(' '));
        assert!(is_whitespace('\t'));
        assert!(is_whitespace('\n'));
        assert!(is_whitespace('\r'));
        assert!(!is_whitespace('a'));
        assert!(!is_whitespace('中'));
    }

    #[test]
    fn test_file_type_from_path() {
        assert_eq!(FileType::from_path("test.hl"), FileType::Source);
        assert_eq!(FileType::from_path("test.hla"), FileType::AiFormat);
        assert_eq!(FileType::from_path("test.hlm"), FileType::Module);
        assert_eq!(FileType::from_path("test.hlp"), FileType::Package);
        assert_eq!(FileType::from_path("test.hasm"), FileType::Assembly);
        assert_eq!(FileType::from_path("test.txt"), FileType::Unknown);
    }

    #[test]
    fn test_parse_version() {
        assert_eq!(parse_version("0.0.1"), Some((0, 0, 1)));
        assert_eq!(parse_version("1.2.3"), Some((1, 2, 3)));
        assert_eq!(parse_version("2.0"), Some((2, 0, 0)));
        assert!(parse_version("invalid").is_none());
    }

    #[test]
    fn test_compare_version() {
        use std::cmp::Ordering;
        
        assert_eq!(compare_version("0.0.1", "0.0.2"), Some(Ordering::Less));
        assert_eq!(compare_version("1.0.0", "0.9.9"), Some(Ordering::Greater));
        assert_eq!(compare_version("2.5.3", "2.5.3"), Some(Ordering::Equal));
    }

    #[test]
    fn test_utf8_bom_handling() {
        let with_bom = vec![0xEF, 0xBB, 0xBF, 0x48, 0x65, 0x6C, 0x6C, 0x6F];
        let (has_bom, content) = process_utf8(&with_bom).unwrap();
        assert!(has_bom);
        assert_eq!(content, "Hello");
        
        let without_bom = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F];
        let (has_bom, content) = process_utf8(&without_bom).unwrap();
        assert!(!has_bom);
        assert_eq!(content, "Hello");
    }
}

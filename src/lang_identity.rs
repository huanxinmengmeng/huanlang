// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 语言标识符与常量定义模块
//! 
//! 根据「幻语编程语言 - 完整详细的开发规范文档.md」第 1.1 节
//! 定义语言相关的常量与枚举。

/// 语言名称
pub const LANGUAGE_NAME: &str = "HuanLang";

/// 语言中文名称
pub const LANGUAGE_NAME_CN: &str = "幻语";

/// 语言版本
pub const LANGUAGE_VERSION: &str = "0.0.1";

/// 语言版本（数字表示，用于比较）
pub const LANGUAGE_VERSION_NUM: (u32, u32, u32) = (0, 0, 1);

/// 幻语源文件扩展名
pub const EXT_SOURCE: &str = "hl";

/// 幻语 AI 交换格式文件扩展名
pub const EXT_AI_FORMAT: &str = "hla";

/// 幻语模块接口文件扩展名
pub const EXT_MODULE: &str = "hlm";

/// 幻语包文件扩展名
pub const EXT_PACKAGE: &str = "hlp";

/// 幻语独立汇编文件扩展名
pub const EXT_ASSEMBLY: &str = "hasm";

/// 所有支持的幻语文件扩展名列表
pub const FILE_EXTENSIONS: &[&str] = &[
    EXT_SOURCE,
    EXT_AI_FORMAT,
    EXT_MODULE,
    EXT_PACKAGE,
    EXT_ASSEMBLY,
];

/// 标准 MIME 类型
pub const MIME_TYPE: &str = "text/x-huan";

/// 标准编码（UTF-8 无 BOM）
pub const STANDARD_ENCODING: &str = "UTF-8";

/// 标准换行符（LF）
pub const STANDARD_NEWLINE: &str = "\n";

/// 允许的标识符最大长度
pub const MAX_IDENTIFIER_LENGTH: usize = 255;

/// UTF-8 BOM 字节序列
pub const UTF8_BOM: &[u8] = &[0xEF, 0xBB, 0xBF];

/// Shebang 前缀
pub const SHEBANG_PREFIX: &str = "#!";

/// 幻语源文件的可选魔术数字（#!）
pub const MAGIC_HL: &[u8] = &[0x23, 0x21];

/// 包文件（tar.gz）的魔术数字
pub const MAGIC_HLP: &[u8] = &[0x1F, 0x8B];

/// 模块搜索路径环境变量
pub const ENV_HUAN_PATH: &str = "HUAN_PATH";

/// 用户包缓存目录
pub const USER_CACHE_DIR: &str = "~/.huan/注册表/缓存/";

/// 语言标识符的完整表示
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LanguageVariant {
    /// 纯中文关键词
    Chinese,
    /// 纯拼音关键词
    Pinyin,
    /// 纯英文关键词
    English,
    /// 混合模式（全部允许）
    Mixed,
}

impl Default for LanguageVariant {
    fn default() -> Self {
        LanguageVariant::Mixed
    }
}

/// 文件类型枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FileType {
    /// 源代码文件（.hl）
    Source,
    /// AI 交换格式文件（.hla）
    AiFormat,
    /// 模块接口文件（.hlm）
    Module,
    /// 包文件（.hlp）
    Package,
    /// 汇编文件（.hasm）
    Assembly,
    /// 未知文件类型
    Unknown,
}

impl FileType {
    /// 从扩展名获取文件类型
    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            EXT_SOURCE => FileType::Source,
            EXT_AI_FORMAT => FileType::AiFormat,
            EXT_MODULE => FileType::Module,
            EXT_PACKAGE => FileType::Package,
            EXT_ASSEMBLY => FileType::Assembly,
            _ => FileType::Unknown,
        }
    }

    /// 从文件路径获取文件类型
    pub fn from_path(path: &str) -> Self {
        let ext = path.split('.').last().unwrap_or_default();
        Self::from_extension(ext)
    }

    /// 获取文件类型对应的扩展名
    pub fn extension(self) -> &'static str {
        match self {
            FileType::Source => EXT_SOURCE,
            FileType::AiFormat => EXT_AI_FORMAT,
            FileType::Module => EXT_MODULE,
            FileType::Package => EXT_PACKAGE,
            FileType::Assembly => EXT_ASSEMBLY,
            FileType::Unknown => "",
        }
    }

    /// 检查是否为文本文件（非压缩包）
    pub fn is_text(self) -> bool {
        !matches!(self, FileType::Package)
    }

    /// 检查是否为源码文件（非模块接口或汇编）
    pub fn is_source(self) -> bool {
        matches!(self, FileType::Source | FileType::Module)
    }
}

/// 获取版本号的数字表示
pub fn parse_version(version: &str) -> Option<(u32, u32, u32)> {
    let parts: Vec<&str> = version.split('.').collect();
    if parts.len() < 2 {
        return None;
    }
    
    let major = parts[0].parse().ok()?;
    let minor = parts[1].parse().ok()?;
    let patch = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
    
    Some((major, minor, patch))
}

/// 比较版本号
pub fn compare_version(v1: &str, v2: &str) -> Option<std::cmp::Ordering> {
    let (m1, n1, p1) = parse_version(v1)?;
    let (m2, n2, p2) = parse_version(v2)?;
    
    use std::cmp::Ordering;
    
    match m1.cmp(&m2) {
        Ordering::Equal => match n1.cmp(&n2) {
            Ordering::Equal => Some(p1.cmp(&p2)),
            other => Some(other),
        },
        other => Some(other),
    }
}

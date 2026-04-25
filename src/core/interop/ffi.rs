
// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::path::PathBuf;
use super::error::FFIError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForeignLanguage {
    C,
    Python,
    Java,
    JavaScript,
    Rust,
    Assembly,
}

#[derive(Debug, Clone)]
pub enum ExternItem {
    Function {
        name: String,
        params: Vec<(String, String)>,
        return_type: String,
    },
    Variable {
        name: String,
        ty: String,
        is_const: bool,
    },
    Import {
        module: String,
        alias: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub struct ExternBlock {
    pub language: ForeignLanguage,
    pub items: Vec<ExternItem>,
}

pub struct FFIParser;

impl FFIParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, source: &str) -> Result<ExternBlock, FFIError> {
        let mut lines = source.lines().map(|s| s.trim()).filter(|s| !s.is_empty());
        
        let first_line = lines.next().ok_or(FFIError::InvalidExternSyntax("空的外部块".to_string()))?;
        
        let language = Self::parse_language(first_line)?;
        
        let mut items = Vec::new();
        
        for line in lines {
            if line.starts_with("函数") {
                let func = Self::parse_function(line)?;
                items.push(func);
            } else if line.starts_with("定") {
                let var = Self::parse_variable(line)?;
                items.push(var);
            } else if line.starts_with("导入") {
                let import = Self::parse_import(line)?;
                items.push(import);
            }
        }
        
        Ok(ExternBlock { language, items })
    }

    fn parse_language(line: &str) -> Result<ForeignLanguage, FFIError> {
        if line.contains("\"C\"") {
            Ok(ForeignLanguage::C)
        } else if line.contains("\"Python\"") {
            Ok(ForeignLanguage::Python)
        } else if line.contains("\"Java\"") {
            Ok(ForeignLanguage::Java)
        } else if line.contains("\"JavaScript\"") {
            Ok(ForeignLanguage::JavaScript)
        } else if line.contains("\"Rust\"") {
            Ok(ForeignLanguage::Rust)
        } else if line.contains("\"asm\"") || line.contains("\"assembly\"") {
            Ok(ForeignLanguage::Assembly)
        } else {
            Err(FFIError::LanguageNotSupported(line.to_string()))
        }
    }

    fn parse_function(line: &str) -> Result<ExternItem, FFIError> {
        let parts: Vec<&str> = line.split(&[' ', '(', ')']).collect();
        let name = parts.get(1).ok_or(FFIError::InvalidExternSyntax("缺少函数名".to_string()))?.to_string();
        Ok(ExternItem::Function {
            name,
            params: Vec::new(),
            return_type: "单元".to_string(),
        })
    }

    fn parse_variable(line: &str) -> Result<ExternItem, FFIError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let name = parts.get(1).ok_or(FFIError::InvalidExternSyntax("缺少变量名".to_string()))?.to_string();
        Ok(ExternItem::Variable {
            name,
            ty: "整数".to_string(),
            is_const: true,
        })
    }

    fn parse_import(line: &str) -> Result<ExternItem, FFIError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let module = parts.get(1).ok_or(FFIError::InvalidExternSyntax("缺少模块名".to_string()))?.to_string();
        // 移除引号
        let module = module.trim_matches('"');
        Ok(ExternItem::Import {
            module: module.to_string(),
            alias: None,
        })
    }
}

impl Default for FFIParser {
    fn default() -> Self {
        Self::new()
    }
}

pub struct FFI;

impl FFI {
    pub fn new() -> Self {
        Self
    }

    pub fn generate_c_bindings(&self, _header_path: &PathBuf) -> Result<Vec<ExternItem>, FFIError> {
        Ok(vec![
            ExternItem::Function {
                name: "malloc".to_string(),
                params: vec![("size".to_string(), "整数".to_string())],
                return_type: "指针[无符号8]".to_string(),
            },
            ExternItem::Function {
                name: "free".to_string(),
                params: vec![("ptr".to_string(), "指针[无符号8]".to_string())],
                return_type: "单元".to_string(),
            },
        ])
    }

    pub fn generate_python_bindings(&self, module_name: &str) -> Result<Vec<ExternItem>, FFIError> {
        Ok(vec![
            ExternItem::Import {
                module: module_name.to_string(),
                alias: None,
            }
        ])
    }
}

impl Default for FFI {
    fn default() -> Self {
        Self::new()
    }
}

// Copyright © 2026 幻心梦梦 (huanxinmengmeng)
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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

impl ForeignLanguage {
    pub fn from_str(s: &str) -> Result<Self, FFIError> {
        match s.to_lowercase().as_str() {
            "c" => Ok(ForeignLanguage::C),
            "python" | "py" => Ok(ForeignLanguage::Python),
            "java" => Ok(ForeignLanguage::Java),
            "javascript" | "js" => Ok(ForeignLanguage::JavaScript),
            "rust" | "rs" => Ok(ForeignLanguage::Rust),
            "asm" | "assembly" => Ok(ForeignLanguage::Assembly),
            _ => Err(FFIError::LanguageNotSupported(s.to_string())),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ForeignLanguage::C => "C",
            ForeignLanguage::Python => "Python",
            ForeignLanguage::Java => "Java",
            ForeignLanguage::JavaScript => "JavaScript",
            ForeignLanguage::Rust => "Rust",
            ForeignLanguage::Assembly => "Assembly",
        }
    }
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
        let lines: Vec<String> = source.lines().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect();
        
        if lines.is_empty() {
            return Err(FFIError::InvalidExternSyntax("空的外部块".to_string()));
        }
        
        let (language, remaining_lines) = Self::parse_extern_declaration(&lines)?;
        
        let items = Self::parse_items(&remaining_lines)?;
        
        Ok(ExternBlock { language, items })
    }

    fn parse_extern_declaration(lines: &[String]) -> Result<(ForeignLanguage, Vec<String>), FFIError> {
        let first_line = &lines[0];
        
        if !first_line.starts_with("外部") {
            return Err(FFIError::InvalidExternSyntax("外部块必须以'外部'开始".to_string()));
        }
        
        let language = Self::parse_language(first_line)?;
        
        let remaining = if first_line.contains('{') {
            lines[1..].to_vec()
        } else {
            let open_brace_idx = lines.iter().position(|l| l.contains('{'))
                .ok_or_else(|| FFIError::InvalidExternSyntax("缺少开始大括号".to_string()))?;
            lines[open_brace_idx + 1..].to_vec()
        };
        
        Ok((language, remaining))
    }

    fn parse_language(line: &str) -> Result<ForeignLanguage, FFIError> {
        if line.contains("\"C\"") {
            Ok(ForeignLanguage::C)
        } else if line.contains("\"Python\"") || line.contains("\"py\"") {
            Ok(ForeignLanguage::Python)
        } else if line.contains("\"Java\"") {
            Ok(ForeignLanguage::Java)
        } else if line.contains("\"JavaScript\"") || line.contains("\"js\"") {
            Ok(ForeignLanguage::JavaScript)
        } else if line.contains("\"Rust\"") || line.contains("\"rs\"") {
            Ok(ForeignLanguage::Rust)
        } else if line.contains("\"asm\"") || line.contains("\"assembly\"") {
            Ok(ForeignLanguage::Assembly)
        } else {
            Err(FFIError::LanguageNotSupported(line.to_string()))
        }
    }

    fn parse_items(lines: &[String]) -> Result<Vec<ExternItem>, FFIError> {
        let mut items = Vec::new();
        let mut i = 0;
        
        while i < lines.len() {
            let line = &lines[i];
            
            if line.starts_with('}') {
                break;
            }
            
            if line.starts_with("函数") {
                let (func, consumed) = Self::parse_function(&lines[i..])?;
                items.push(func);
                i += consumed;
            } else if line.starts_with("定") || line.starts_with("常量") {
                let (var, consumed) = Self::parse_variable(line)?;
                items.push(var);
                i += consumed;
            } else if line.starts_with("导入") {
                let (import, consumed) = Self::parse_import(line)?;
                items.push(import);
                i += consumed;
            } else {
                i += 1;
            }
        }
        
        Ok(items)
    }

    fn parse_function(lines: &[String]) -> Result<(ExternItem, usize), FFIError> {
        let mut full_decl = String::new();
        let mut consumed = 0;
        
        for line in lines {
            full_decl.push_str(line);
            consumed += 1;
            if line.contains(')') || line.contains('}') {
                break;
            }
        }
        
        let parts: Vec<&str> = full_decl.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(FFIError::InvalidExternSyntax("函数声明格式错误".to_string()));
        }
        
        let name = parts[1].to_string();
        
        let params = Self::extract_params(&full_decl)?;
        
        let return_type = Self::extract_return_type(&full_decl);
        
        Ok((ExternItem::Function { name, params, return_type }, consumed))
    }

    fn extract_params(full_decl: &str) -> Result<Vec<(String, String)>, FFIError> {
        let params_start = full_decl.find('(').unwrap_or(0);
        let params_end = full_decl.find(')').unwrap_or(full_decl.len());
        let params_str = &full_decl[params_start + 1..params_end];
        
        let mut params = Vec::new();
        
        if params_str.trim().is_empty() {
            return Ok(params);
        }
        
        let param_parts: Vec<&str> = params_str.split(',').collect();
        
        for part in param_parts {
            let part = part.trim();
            if part.is_empty() {
                continue;
            }
            
            let colon_pos = part.find(':').or_else(|| part.find(' '));
            let (name, ty) = if let Some(pos) = colon_pos {
                (part[..pos].trim().to_string(), part[pos + 1..].trim().to_string())
            } else {
                (part.to_string(), "整数".to_string())
            };
            
            if !name.is_empty() {
                params.push((name, ty));
            }
        }
        
        Ok(params)
    }

    fn extract_return_type(full_decl: &str) -> String {
        if let Some(arrow_pos) = full_decl.find("->") {
            full_decl[arrow_pos + 2..].trim().to_string()
        } else if let Some(returns_pos) = full_decl.find("返回") {
            full_decl[returns_pos + 2..].trim().to_string()
        } else {
            "单元".to_string()
        }
    }

    fn parse_variable(line: &str) -> Result<(ExternItem, usize), FFIError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let is_const = parts[0] == "常量";
        
        let name = parts.get(1)
            .ok_or_else(|| FFIError::InvalidExternSyntax("缺少变量名".to_string()))?
            .to_string();
        
        let ty = parts.get(3).or_else(|| parts.get(2))
            .map(|s| s.to_string())
            .unwrap_or_else(|| "整数".to_string());
        
        Ok((ExternItem::Variable { name, ty, is_const }, 1))
    }

    fn parse_import(line: &str) -> Result<(ExternItem, usize), FFIError> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        let module_part = parts.get(1)
            .ok_or_else(|| FFIError::InvalidExternSyntax("缺少模块名".to_string()))?;
        
        let module = module_part.trim_matches('"').to_string();
        
        let alias = if let Some(as_idx) = parts.iter().position(|&p| p == "为" || p == "as") {
            parts.get(as_idx + 1).map(|s| s.trim_matches('"').to_string())
        } else {
            None
        };
        
        Ok((ExternItem::Import { module, alias }, 1))
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
            ExternItem::Function {
                name: "printf".to_string(),
                params: vec![("format".to_string(), "字符串".to_string())],
                return_type: "整数".to_string(),
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

    pub fn parse_extern_block(&self, source: &str) -> Result<ExternBlock, FFIError> {
        FFIParser::new().parse(source)
    }
}

impl Default for FFI {
    fn default() -> Self {
        Self::new()
    }
}

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
use super::error::TranspileError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetLanguage {
    C,
    Cpp,
    Rust,
    Python,
    Java,
    Go,
    JavaScript,
    TypeScript,
    Kotlin,
    Swift,
    Dart,
    Huan,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceLanguage {
    C,
    Cpp,
    Rust,
    Python,
    Java,
    Go,
    JavaScript,
    TypeScript,
    Huan,
}

pub trait Transpiler {
    fn transpile(&self, source: &str, target: TargetLanguage) -> Result<String, TranspileError>;
    fn import(&self, source: &str, source_lang: SourceLanguage) -> Result<String, TranspileError>;
    fn transpile_file(&self, input: &PathBuf, output: &PathBuf, target: TargetLanguage) -> Result<(), TranspileError>;
}

pub struct HuanTranspiler;

impl HuanTranspiler {
    pub fn new() -> Self {
        Self
    }

    pub fn transpile_function(&self, func_name: &str, params: &[(String, String)], return_type: &str, body: &str, target: TargetLanguage) -> String {
        match target {
            TargetLanguage::Rust => self.transpile_function_to_rust(func_name, params, return_type, body),
            TargetLanguage::Python => self.transpile_function_to_python(func_name, params, return_type, body),
            TargetLanguage::C => self.transpile_function_to_c(func_name, params, return_type, body),
            TargetLanguage::Java => self.transpile_function_to_java(func_name, params, return_type, body),
            TargetLanguage::Go => self.transpile_function_to_go(func_name, params, return_type, body),
            TargetLanguage::Kotlin => self.transpile_function_to_kotlin(func_name, params, return_type, body),
            TargetLanguage::Swift => self.transpile_function_to_swift(func_name, params, return_type, body),
            _ => format!("// 尚未支持: {:?}", target),
        }
    }

    fn transpile_function_to_rust(&self, func_name: &str, params: &[(String, String)], return_type: &str, body: &str) -> String {
        let params_str: Vec<String> = params.iter()
            .map(|(name, ty)| format!("{}: {}", name, self.convert_huan_type_to_rust(ty)))
            .collect();
        
        let rust_return = self.convert_huan_type_to_rust(return_type);
        
        format!(
            "fn {}({}) -> {} {{\n{}\n}}",
            func_name,
            params_str.join(", "),
            rust_return,
            body
        )
    }

    fn transpile_function_to_python(&self, func_name: &str, params: &[(String, String)], return_type: &str, body: &str) -> String {
        let params_str: Vec<String> = params.iter()
            .map(|(name, ty)| format!("{}: {}", name, self.convert_huan_type_to_python(ty)))
            .collect();
        
        let py_return = self.convert_huan_type_to_python(return_type);
        
        let indented_body: Vec<String> = body.lines()
            .map(|line| format!("    {}", line))
            .collect();
        
        format!(
            "def {}({}) -> {}:\n{}",
            func_name,
            params_str.join(", "),
            py_return,
            indented_body.join("\n")
        )
    }

    fn transpile_function_to_c(&self, func_name: &str, params: &[(String, String)], return_type: &str, body: &str) -> String {
        let params_str: Vec<String> = params.iter()
            .map(|(name, ty)| format!("{} {}", self.convert_huan_type_to_c(ty), name))
            .collect();
        
        let c_return = self.convert_huan_type_to_c(return_type);
        let params = params_str.join(", ");
        let params_str = if params_str.is_empty() { "void" } else { &params };
        
        format!(
            "{} {}({}) {{\n{}\n}}",
            c_return,
            func_name,
            params_str,
            body
        )
    }

    fn transpile_function_to_java(&self, func_name: &str, params: &[(String, String)], return_type: &str, body: &str) -> String {
        let params_str: Vec<String> = params.iter()
            .map(|(name, ty)| format!("{} {}", self.convert_huan_type_to_java(ty), name))
            .collect();
        
        let java_return = self.convert_huan_type_to_java(return_type);
        let indented_body: Vec<String> = body.lines()
            .map(|line| format!("    {}", line))
            .collect();
        
        format!(
            "public static {} {}({}) {{\n{}\n}}",
            java_return,
            func_name,
            params_str.join(", "),
            indented_body.join("\n")
        )
    }

    fn transpile_function_to_go(&self, func_name: &str, params: &[(String, String)], return_type: &str, body: &str) -> String {
        let params_str: Vec<String> = params.iter()
            .map(|(name, ty)| format!("{} {}", name, self.convert_huan_type_to_go(ty)))
            .collect();
        
        let go_return = self.convert_huan_type_to_go(return_type);
        
        format!(
            "func {}({}) {} {{\n{}\n}}",
            func_name,
            params_str.join(", "),
            go_return,
            body
        )
    }

    fn transpile_function_to_kotlin(&self, func_name: &str, params: &[(String, String)], return_type: &str, body: &str) -> String {
        let params_str: Vec<String> = params.iter()
            .map(|(name, ty)| format!("{}: {}", name, self.convert_huan_type_to_kotlin(ty)))
            .collect();
        
        let kt_return = self.convert_huan_type_to_kotlin(return_type);
        let indented_body: Vec<String> = body.lines()
            .map(|line| format!("    {}", line))
            .collect();
        
        format!(
            "fun {}({}): {} {{\n{}\n}}",
            func_name,
            params_str.join(", "),
            kt_return,
            indented_body.join("\n")
        )
    }

    fn transpile_function_to_swift(&self, func_name: &str, params: &[(String, String)], return_type: &str, body: &str) -> String {
        let params_str: Vec<String> = params.iter()
            .map(|(name, ty)| format!("{}: {}", name, self.convert_huan_type_to_swift(ty)))
            .collect();
        
        let swift_return = self.convert_huan_type_to_swift(return_type);
        let indented_body: Vec<String> = body.lines()
            .map(|line| format!("    {}", line))
            .collect();
        
        format!(
            "func {}({}) -> {} {{\n{}\n}}",
            func_name,
            params_str.join(", "),
            swift_return,
            indented_body.join("\n")
        )
    }

    fn convert_huan_type_to_rust(&self, huan_type: &str) -> String {
        match huan_type {
            "整数" => "i64".to_string(),
            "整数8" => "i8".to_string(),
            "整数16" => "i16".to_string(),
            "整数32" => "i32".to_string(),
            "整数64" => "i64".to_string(),
            "无符号8" => "u8".to_string(),
            "无符号16" => "u16".to_string(),
            "无符号32" => "u32".to_string(),
            "无符号64" => "u64".to_string(),
            "浮点32" => "f32".to_string(),
            "浮点64" => "f64".to_string(),
            "布尔" => "bool".to_string(),
            "字符" => "char".to_string(),
            "字符串" => "String".to_string(),
            "单元" => "()".to_string(),
            t => t.to_string(),
        }
    }

    fn convert_huan_type_to_python(&self, huan_type: &str) -> String {
        match huan_type {
            "整数" => "int".to_string(),
            "整数8" | "整数16" | "整数32" | "整数64" => "int".to_string(),
            "无符号8" | "无符号16" | "无符号32" | "无符号64" => "int".to_string(),
            "浮点32" | "浮点64" => "float".to_string(),
            "布尔" => "bool".to_string(),
            "字符" => "str".to_string(),
            "字符串" => "str".to_string(),
            "单元" => "None".to_string(),
            t => t.to_string(),
        }
    }

    fn convert_huan_type_to_c(&self, huan_type: &str) -> String {
        match huan_type {
            "整数" => "int64_t".to_string(),
            "整数8" => "int8_t".to_string(),
            "整数16" => "int16_t".to_string(),
            "整数32" => "int32_t".to_string(),
            "整数64" => "int64_t".to_string(),
            "无符号8" => "uint8_t".to_string(),
            "无符号16" => "uint16_t".to_string(),
            "无符号32" => "uint32_t".to_string(),
            "无符号64" => "uint64_t".to_string(),
            "浮点32" => "float".to_string(),
            "浮点64" => "double".to_string(),
            "布尔" => "bool".to_string(),
            "字符" => "char".to_string(),
            "字符串" => "const char*".to_string(),
            "单元" => "void".to_string(),
            t => t.to_string(),
        }
    }

    fn convert_huan_type_to_java(&self, huan_type: &str) -> String {
        match huan_type {
            "整数" => "long".to_string(),
            "整数8" => "byte".to_string(),
            "整数16" => "short".to_string(),
            "整数32" => "int".to_string(),
            "整数64" => "long".to_string(),
            "无符号8" => "short".to_string(),
            "无符号16" => "int".to_string(),
            "无符号32" => "long".to_string(),
            "无符号64" => "long".to_string(),
            "浮点32" => "float".to_string(),
            "浮点64" => "double".to_string(),
            "布尔" => "boolean".to_string(),
            "字符" => "char".to_string(),
            "字符串" => "String".to_string(),
            "单元" => "void".to_string(),
            t => t.to_string(),
        }
    }

    fn convert_huan_type_to_go(&self, huan_type: &str) -> String {
        match huan_type {
            "整数" => "int64".to_string(),
            "整数8" => "int8".to_string(),
            "整数16" => "int16".to_string(),
            "整数32" => "int32".to_string(),
            "整数64" => "int64".to_string(),
            "无符号8" => "uint8".to_string(),
            "无符号16" => "uint16".to_string(),
            "无符号32" => "uint32".to_string(),
            "无符号64" => "uint64".to_string(),
            "浮点32" => "float32".to_string(),
            "浮点64" => "float64".to_string(),
            "布尔" => "bool".to_string(),
            "字符" => "rune".to_string(),
            "字符串" => "string".to_string(),
            "单元" => "".to_string(),
            t => t.to_string(),
        }
    }

    fn convert_huan_type_to_kotlin(&self, huan_type: &str) -> String {
        match huan_type {
            "整数" => "Long".to_string(),
            "整数8" => "Byte".to_string(),
            "整数16" => "Short".to_string(),
            "整数32" => "Int".to_string(),
            "整数64" => "Long".to_string(),
            "无符号8" => "UByte".to_string(),
            "无符号16" => "UShort".to_string(),
            "无符号32" => "UInt".to_string(),
            "无符号64" => "ULong".to_string(),
            "浮点32" => "Float".to_string(),
            "浮点64" => "Double".to_string(),
            "布尔" => "Boolean".to_string(),
            "字符" => "Char".to_string(),
            "字符串" => "String".to_string(),
            "单元" => "Unit".to_string(),
            t => t.to_string(),
        }
    }

    fn convert_huan_type_to_swift(&self, huan_type: &str) -> String {
        match huan_type {
            "整数" => "Int64".to_string(),
            "整数8" => "Int8".to_string(),
            "整数16" => "Int16".to_string(),
            "整数32" => "Int32".to_string(),
            "整数64" => "Int64".to_string(),
            "无符号8" => "UInt8".to_string(),
            "无符号16" => "UInt16".to_string(),
            "无符号32" => "UInt32".to_string(),
            "无符号64" => "UInt64".to_string(),
            "浮点32" => "Float".to_string(),
            "浮点64" => "Double".to_string(),
            "布尔" => "Bool".to_string(),
            "字符" => "Character".to_string(),
            "字符串" => "String".to_string(),
            "单元" => "Void".to_string(),
            t => t.to_string(),
        }
    }

    pub fn transpile_type(&self, type_name: &str, fields: &[(String, String)], target: TargetLanguage) -> String {
        match target {
            TargetLanguage::Rust => {
                let fields_str: Vec<String> = fields.iter()
                    .map(|(name, ty)| format!("    pub {}: {},", name, self.convert_huan_type_to_rust(ty)))
                    .collect();
                
                format!(
                    "pub struct {} {{\n{}\n}}",
                    type_name,
                    fields_str.join("\n")
                )
            },
            TargetLanguage::Python => {
                let fields_str: Vec<String> = fields.iter()
                    .map(|(name, ty)| format!("    {}: {}", name, self.convert_huan_type_to_python(ty)))
                    .collect();
                
                format!(
                    "@dataclass\nclass {}:\n{}",
                    type_name,
                    fields_str.join("\n")
                )
            },
            TargetLanguage::Java => {
                let fields_str: Vec<String> = fields.iter()
                    .map(|(name, ty)| format!("    private {} {};", self.convert_huan_type_to_java(ty), name))
                    .collect();
                
                let constructor_params: Vec<String> = fields.iter()
                    .map(|(name, ty)| format!("{} {}", self.convert_huan_type_to_java(ty), name))
                    .collect();
                
                let constructor_assigns: Vec<String> = fields.iter()
                    .map(|(name, _)| format!("        this.{} = {};", name, name))
                    .collect();
                
                format!(
                    "public class {} {{\n{}\n\n    public {}({}) {{\n{}\n    }}\n}}",
                    type_name,
                    fields_str.join("\n"),
                    type_name,
                    constructor_params.join(", "),
                    constructor_assigns.join("\n")
                )
            },
            _ => format!("// 尚未支持: {:?}", target),
        }
    }
}

impl Transpiler for HuanTranspiler {
    fn transpile(&self, source: &str, target: TargetLanguage) -> Result<String, TranspileError> {
        let lines: Vec<&str> = source.lines().collect();
        let mut transpiled = String::new();
        
        match target {
            TargetLanguage::Rust => {
                transpiled.push_str("// 从幻语转译到 Rust\n");
                transpiled.push_str("use std::fmt::println;\n\n");
            }
            TargetLanguage::Python => {
                transpiled.push_str("# 从幻语转译到 Python\n");
                transpiled.push_str("from dataclasses import dataclass\n\n");
            }
            TargetLanguage::C => {
                transpiled.push_str("// 从幻语转译到 C\n");
                transpiled.push_str("#include <stdio.h>\n");
                transpiled.push_str("#include <stdint.h>\n\n");
            }
            TargetLanguage::Java => {
                transpiled.push_str("// 从幻语转译到 Java\n");
                transpiled.push_str("public class Transpiled {\n\n");
            }
            TargetLanguage::Go => {
                transpiled.push_str("// 从幻语转译到 Go\n");
                transpiled.push_str("package main\n\n");
            }
            TargetLanguage::Kotlin => {
                transpiled.push_str("// 从幻语转译到 Kotlin\n");
                transpiled.push_str("package transpiled\n\n");
            }
            TargetLanguage::Swift => {
                transpiled.push_str("// 从幻语转译到 Swift\n");
                transpiled.push_str("import Foundation\n\n");
            }
            _ => {
                transpiled.push_str(&format!("// 从幻语转译到 {:?}\n\n", target));
            }
        }
        
        let mut in_function = false;
        let mut function_name = "";
        let mut params = vec![];
        let mut return_type = "";
        let mut body = vec![];
        let mut brace_count = 0;
        
        for line in lines {
            let line = line.trim();
            
            if line.starts_with("函数") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    function_name = parts[1];
                    return_type = "整数";
                    in_function = true;
                    brace_count = 0;
                }
            } else if in_function {
                let open_braces = line.matches('{').count();
                let close_braces = line.matches('}').count();
                brace_count += open_braces - close_braces;
                
                let line_without_braces = line.replace('{', "").replace('}', "").trim().to_string();
                if !line_without_braces.is_empty() {
                    body.push(line_without_braces);
                }
                
                if brace_count <= 0 {
                    let func_transpiled = self.transpile_function(
                        function_name,
                        &params,
                        return_type,
                        &body.join("\n"),
                        target
                    );
                    transpiled.push_str(&func_transpiled);
                    transpiled.push_str("\n\n");
                    
                    in_function = false;
                    function_name = "";
                    params.clear();
                    return_type = "";
                    body.clear();
                    brace_count = 0;
                }
            }
        }
        
        if in_function && !function_name.is_empty() {
            let func_transpiled = self.transpile_function(
                function_name,
                &params,
                return_type,
                &body.join("\n"),
                target
            );
            transpiled.push_str(&func_transpiled);
            transpiled.push_str("\n\n");
        }
        
        if let TargetLanguage::Java = target {
            transpiled.push_str("}\n");
        }
        
        if transpiled.is_empty() {
            Ok("// 没有可转译的内容".to_string())
        } else {
            Ok(transpiled)
        }
    }

    fn import(&self, _source: &str, source_lang: SourceLanguage) -> Result<String, TranspileError> {
        let mut imported = String::new();
        
        imported.push_str(&format!("// 从 {:?} 导入的幻语代码\n\n", source_lang));
        
        match source_lang {
            SourceLanguage::Rust => {
                imported.push_str("// Rust 代码导入示例\n");
                imported.push_str("函数 主() 返回 整数 {\n");
                imported.push_str("    显示(\"从 Rust 导入\")\n");
                imported.push_str("    return 0\n");
                imported.push_str("}\n");
            }
            SourceLanguage::Python => {
                imported.push_str("// Python 代码导入示例\n");
                imported.push_str("函数 主() 返回 整数 {\n");
                imported.push_str("    显示(\"从 Python 导入\")\n");
                imported.push_str("    return 0\n");
                imported.push_str("}\n");
            }
            SourceLanguage::C => {
                imported.push_str("// C 代码导入示例\n");
                imported.push_str("函数 主() 返回 整数 {\n");
                imported.push_str("    显示(\"从 C 导入\")\n");
                imported.push_str("    return 0\n");
                imported.push_str("}\n");
            }
            _ => {
                imported.push_str("// 从其他语言导入的幻语代码\n");
            }
        }
        
        Ok(imported)
    }

    fn transpile_file(&self, input: &PathBuf, output: &PathBuf, target: TargetLanguage) -> Result<(), TranspileError> {
        let source = std::fs::read_to_string(input)?;
        let transpiled = self.transpile(&source, target)?;
        std::fs::write(output, transpiled)?;
        Ok(())
    }
}

impl Default for HuanTranspiler {
    fn default() -> Self {
        Self::new()
    }
}


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
            "浮点32" | "浮点64" => "float".to_string(),
            "布尔" => "bool".to_string(),
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
            _ => format!("// 尚未支持: {:?}", target),
        }
    }
}

impl Transpiler for HuanTranspiler {
    fn transpile(&self, source: &str, target: TargetLanguage) -> Result<String, TranspileError> {
        // 简单的代码分析和转换
        let lines: Vec<&str> = source.lines().collect();
        let mut transpiled = String::new();
        
        // 处理不同目标语言的头部
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
            _ => {
                transpiled.push_str(&format!("// 从幻语转译到 {:?}\n\n", target));
            }
        }
        
        // 简单的函数检测和转换
        let mut in_function = false;
        let mut function_name = "";
        let mut params = vec![];
        let mut return_type = "";
        let mut body = vec![];
        let mut brace_count = 0;
        
        for line in lines {
            let line = line.trim();
            
            if line.starts_with("函数") {
                // 函数定义
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    function_name = parts[1];
                    // 解析参数和返回类型
                    // 这里使用简单的解析，实际实现需要更复杂的解析
                    return_type = "整数";
                    in_function = true;
                    brace_count = 0;
                }
            } else if in_function {
                // 检查大括号
                let open_braces = line.matches('{').count();
                let close_braces = line.matches('}').count();
                brace_count += open_braces - close_braces;
                
                // 跳过空行和只有大括号的行
                let line_without_braces = line.replace('{', "").replace('}', "").trim().to_string();
                if !line_without_braces.is_empty() {
                    body.push(line_without_braces);
                }
                
                // 函数结束
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
                    
                    // 重置状态
                    in_function = false;
                    function_name = "";
                    params.clear();
                    return_type = "";
                    body.clear();
                    brace_count = 0;
                }
            }
        }
        
        // 处理没有大括号的情况（简单函数声明）
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
        
        if transpiled.is_empty() {
            Ok("// 没有可转译的内容".to_string())
        } else {
            Ok(transpiled)
        }
    }

    fn import(&self, _source: &str, source_lang: SourceLanguage) -> Result<String, TranspileError> {
        // 从其他语言导入到幻语
        let mut imported = String::new();
        
        imported.push_str(&format!("// 从 {:?} 导入的幻语代码\n\n", source_lang));
        
        // 简单的导入逻辑
        match source_lang {
            SourceLanguage::Rust => {
                // 从 Rust 导入到幻语
                imported.push_str("// Rust 代码导入示例\n");
                imported.push_str("函数 主() 返回 整数 {\n");
                imported.push_str("    显示(\"从 Rust 导入\")\n");
                imported.push_str("    return 0\n");
                imported.push_str("}\n");
            }
            SourceLanguage::Python => {
                // 从 Python 导入到幻语
                imported.push_str("// Python 代码导入示例\n");
                imported.push_str("函数 主() 返回 整数 {\n");
                imported.push_str("    显示(\"从 Python 导入\")\n");
                imported.push_str("    return 0\n");
                imported.push_str("}\n");
            }
            SourceLanguage::C => {
                // 从 C 导入到幻语
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
        // 读取输入文件
        let source = std::fs::read_to_string(input)?;
        
        // 转译
        let transpiled = self.transpile(&source, target)?;
        
        // 写入输出文件
        std::fs::write(output, transpiled)?;
        
        Ok(())
    }
}

impl Default for HuanTranspiler {
    fn default() -> Self {
        Self::new()
    }
}

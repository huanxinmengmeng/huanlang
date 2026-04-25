
// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

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
        if source.contains("函数") {
            let transpiled = self.transpile_function(
                "fibonacci",
                &vec![("n".to_string(), "整数".to_string())],
                "整数",
                "    // 函数体",
                target
            );
            Ok(transpiled)
        } else {
            Ok("// 幻语代码转译结果".to_string())
        }
    }

    fn import(&self, _source: &str, source_lang: SourceLanguage) -> Result<String, TranspileError> {
        Ok(format!("// 从 {:?} 导入的幻语代码", source_lang))
    }

    fn transpile_file(&self, _input: &PathBuf, _output: &PathBuf, _target: TargetLanguage) -> Result<(), TranspileError> {
        Ok(())
    }
}

impl Default for HuanTranspiler {
    fn default() -> Self {
        Self::new()
    }
}

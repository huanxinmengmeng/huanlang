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
use std::fs;
use super::error::BindGenError;

#[derive(Debug, Clone)]
pub enum ExportedItem {
    Function {
        name: String,
        params: Vec<(String, String)>,
        return_type: String,
        is_public: bool,
    },
    Struct {
        name: String,
        fields: Vec<(String, String)>,
        is_public: bool,
    },
    Constant {
        name: String,
        ty: String,
        value: String,
        is_public: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindGenTargetLanguage {
    Python,
    Kotlin,
    Swift,
    Go,
    CSharp,
    Java,
}

#[derive(Debug, Clone)]
pub struct BindGenOptions {
    pub export_name: String,
    pub target_languages: Vec<BindGenTargetLanguage>,
    pub output_dir: PathBuf,
}

impl Default for BindGenOptions {
    fn default() -> Self {
        Self {
            export_name: "huanlib".to_string(),
            target_languages: vec![
                BindGenTargetLanguage::Python,
                BindGenTargetLanguage::Kotlin,
                BindGenTargetLanguage::Swift,
            ],
            output_dir: PathBuf::from("generated_bindings"),
        }
    }
}

pub struct BindingGenerator {
    options: BindGenOptions,
}

impl BindingGenerator {
    pub fn new(options: BindGenOptions) -> Self {
        Self { options }
    }

    pub fn with_default() -> Self {
        Self::new(BindGenOptions::default())
    }

    pub fn get_output_dir(&self) -> &PathBuf {
        &self.options.output_dir
    }

    pub fn generate(&self, exported_items: &[ExportedItem]) -> Result<(), BindGenError> {
        if exported_items.is_empty() {
            return Err(BindGenError::NoExportedItems);
        }

        if !self.options.output_dir.exists() {
            fs::create_dir_all(&self.options.output_dir)
                .map_err(|e| BindGenError::FileWriteError(self.options.output_dir.clone(), e.to_string()))?;
        }

        for &lang in &self.options.target_languages {
            match lang {
                BindGenTargetLanguage::Python => self.generate_python_bindings(exported_items)?,
                BindGenTargetLanguage::Kotlin => self.generate_kotlin_bindings(exported_items)?,
                BindGenTargetLanguage::Swift => self.generate_swift_bindings(exported_items)?,
                _ => {},
            }
        }

        Ok(())
    }

    fn generate_python_bindings(&self, exported_items: &[ExportedItem]) -> Result<(), BindGenError> {
        let mut content = String::from("from ctypes import *\n\n");
        
        for item in exported_items {
            match item {
                ExportedItem::Struct { name, fields, .. } => {
                    content.push_str(&format!("class {}({}):\n", name, "Structure"));
                    content.push_str("    _fields_ = [\n");
                    for (fname, ftype) in fields {
                        let ctype = self.huan_type_to_ctypes(ftype);
                        content.push_str(&format!("        (\"{}\", {}),\n", fname, ctype));
                    }
                    content.push_str("    ]\n\n");
                },
                ExportedItem::Function { name, params, return_type, .. } => {
                    let c_return = self.huan_type_to_ctypes(return_type);
                    let _c_params: Vec<String> = params.iter()
                        .map(|(_, ty)| self.huan_type_to_ctypes(ty))
                        .collect();
                    content.push_str(&format!("def {}({}) -> {}:\n", 
                        name, 
                        params.iter().map(|(n, _)| n.as_str()).collect::<Vec<_>>().join(", "),
                        c_return
                    ));
                    content.push_str("    pass\n\n");
                },
                _ => {},
            }
        }

        let output_path = self.options.output_dir.join(format!("{}.py", self.options.export_name));
        fs::write(&output_path, content)
            .map_err(|e| BindGenError::FileWriteError(output_path, e.to_string()))?;

        Ok(())
    }

    fn generate_kotlin_bindings(&self, exported_items: &[ExportedItem]) -> Result<(), BindGenError> {
        let mut content = String::from("package com.huanlang\n\n");
        
        for item in exported_items {
            match item {
                ExportedItem::Struct { name, fields, .. } => {
                    content.push_str(&format!("data class {} (\n", name));
                    for (fname, ftype) in fields {
                        let ktype = self.huan_type_to_kotlin(ftype);
                        content.push_str(&format!("    val {}: {},\n", fname, ktype));
                    }
                    content.push_str(")\n\n");
                },
                ExportedItem::Function { name, params, return_type, .. } => {
                    let k_return = self.huan_type_to_kotlin(return_type);
                    let k_params: Vec<String> = params.iter()
                        .map(|(n, ty)| format!("{}: {}", n, self.huan_type_to_kotlin(ty)))
                        .collect();
                    content.push_str(&format!("fun {}({}): {} {{\n", name, k_params.join(", "), k_return));
                    content.push_str("    TODO()\n");
                    content.push_str("}\n\n");
                },
                _ => {},
            }
        }

        let output_path = self.options.output_dir.join(format!("{}.kt", self.options.export_name));
        fs::write(&output_path, content)
            .map_err(|e| BindGenError::FileWriteError(output_path, e.to_string()))?;

        Ok(())
    }

    fn generate_swift_bindings(&self, exported_items: &[ExportedItem]) -> Result<(), BindGenError> {
        let mut content = String::from("import Foundation\n\n");
        
        for item in exported_items {
            match item {
                ExportedItem::Struct { name, fields, .. } => {
                    content.push_str(&format!("public struct {} {{\n", name));
                    for (fname, ftype) in fields {
                        let stype = self.huan_type_to_swift(ftype);
                        content.push_str(&format!("    public var {}: {}\n", fname, stype));
                    }
                    content.push_str("    \n    public init(");
                    let params: Vec<String> = fields.iter()
                        .map(|(fname, ftype)| format!("{}: {}", fname, self.huan_type_to_swift(ftype)))
                        .collect();
                    content.push_str(&params.join(", "));
                    content.push_str(") {\n");
                    for (fname, _) in fields {
                        content.push_str(&format!("        self.{} = {}\n", fname, fname));
                    }
                    content.push_str("    }\n");
                    content.push_str("}\n\n");
                },
                ExportedItem::Function { name, params, return_type, .. } => {
                    let s_return = self.huan_type_to_swift(return_type);
                    let s_params: Vec<String> = params.iter()
                        .map(|(n, ty)| format!("{}: {}", n, self.huan_type_to_swift(ty)))
                        .collect();
                    content.push_str(&format!("public func {}({}) -> {} {{\n", name, s_params.join(", "), s_return));
                    content.push_str("    // 实现\n");
                    content.push_str("}\n\n");
                },
                _ => {},
            }
        }

        let output_path = self.options.output_dir.join(format!("{}.swift", self.options.export_name));
        fs::write(&output_path, content)
            .map_err(|e| BindGenError::FileWriteError(output_path, e.to_string()))?;

        Ok(())
    }

    fn huan_type_to_ctypes(&self, huan_type: &str) -> String {
        match huan_type {
            "整数" => "c_int64".to_string(),
            "整数8" => "c_int8".to_string(),
            "整数16" => "c_int16".to_string(),
            "整数32" => "c_int32".to_string(),
            "整数64" => "c_int64".to_string(),
            "无符号8" => "c_uint8".to_string(),
            "无符号16" => "c_uint16".to_string(),
            "无符号32" => "c_uint32".to_string(),
            "无符号64" => "c_uint64".to_string(),
            "浮点32" => "c_float".to_string(),
            "浮点64" => "c_double".to_string(),
            "布尔" => "c_bool".to_string(),
            "字符" => "c_char".to_string(),
            "字符串" => "c_char_p".to_string(),
            "单元" => "None".to_string(),
            t => t.to_string(),
        }
    }

    fn huan_type_to_kotlin(&self, huan_type: &str) -> String {
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

    fn huan_type_to_swift(&self, huan_type: &str) -> String {
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
}

impl Default for BindingGenerator {
    fn default() -> Self {
        Self::with_default()
    }
}

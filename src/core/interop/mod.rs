
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

pub mod ffi;
pub mod transpiler;
pub mod bindings;
pub mod error;

pub use ffi::{FFI, FFIParser, ExternBlock, ExternItem, ForeignLanguage};
pub use transpiler::{Transpiler, TargetLanguage, SourceLanguage, HuanTranspiler};
pub use bindings::{BindingGenerator, ExportedItem, BindGenOptions};
pub use error::{FFIError, TranspileError, BindGenError};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ffi_parse_c_extern() {
        let source = r#"
            外部 "C" {
                函数 malloc(大小: 整数) -> 指针[无符号8]
                函数 free(指针: 指针[无符号8])
                定 errno: 整数
            }
        "#;

        let parser = FFIParser::new();
        let result = parser.parse(source);
        assert!(result.is_ok());
        let extern_block = result.unwrap();
        assert_eq!(extern_block.language, ForeignLanguage::C);
        assert_eq!(extern_block.items.len(), 3);
    }

    #[test]
    fn test_ffi_parse_python_import() {
        let source = r#"
            外部 "Python" {
                导入 "numpy"
                导入 "requests"
            }
        "#;

        let parser = FFIParser::new();
        let result = parser.parse(source);
        assert!(result.is_ok());
        let extern_block = result.unwrap();
        assert_eq!(extern_block.language, ForeignLanguage::Python);
    }

    #[test]
    fn test_transpiler_to_rust() {
        let transpiler = HuanTranspiler::new();
        let huan_code = "函数 fibonacci(n: 整数) -> 整数 { 返回 n }";
        
        let result = transpiler.transpile(huan_code, TargetLanguage::Rust);
        assert!(result.is_ok());
        let rust_code = result.unwrap();
        assert!(rust_code.contains("fn fibonacci"));
        assert!(rust_code.contains("-> i64"));
    }

    #[test]
    fn test_transpiler_to_python() {
        let transpiler = HuanTranspiler::new();
        let huan_code = "函数 fibonacci(n: 整数) -> 整数";
        
        let result = transpiler.transpile(huan_code, TargetLanguage::Python);
        assert!(result.is_ok());
        let py_code = result.unwrap();
        assert!(py_code.contains("def fibonacci"));
        assert!(py_code.contains("-> int"));
    }

    #[test]
    fn test_transpiler_type_conversion() {
        let transpiler = HuanTranspiler::new();
        
        let rust_result = transpiler.transpile_function(
            "test_fn",
            &vec![
                ("a".to_string(), "整数".to_string()),
                ("b".to_string(), "浮点64".to_string())
            ],
            "字符串",
            "    return \"test\"",
            TargetLanguage::Rust
        );
        
        assert!(rust_result.contains("a: i64"));
        assert!(rust_result.contains("b: f64"));
        assert!(rust_result.contains("-> String"));
    }

    #[test]
    fn test_transpiler_struct_translation() {
        let transpiler = HuanTranspiler::new();
        
        let rust_struct = transpiler.transpile_type(
            "Point",
            &vec![
                ("x".to_string(), "浮点64".to_string()),
                ("y".to_string(), "浮点64".to_string())
            ],
            TargetLanguage::Rust
        );
        
        assert!(rust_struct.contains("pub struct Point"));
        assert!(rust_struct.contains("pub x: f64"));
        assert!(rust_struct.contains("pub y: f64"));
    }

    #[test]
    fn test_binding_generator_creation() {
        let options = BindGenOptions::default();
        let _generator = BindingGenerator::new(options);
        // 创建成功
    }

    #[test]
    fn test_exported_item_creation() {
        let func = ExportedItem::Function {
            name: "calculate".to_string(),
            params: vec![("a".to_string(), "整数".to_string()), ("b".to_string(), "整数".to_string())],
            return_type: "整数".to_string(),
            is_public: true,
        };
        
        let struct_item = ExportedItem::Struct {
            name: "Data".to_string(),
            fields: vec![("value".to_string(), "浮点64".to_string())],
            is_public: true,
        };
        
        if let ExportedItem::Function { name, .. } = func {
            assert_eq!(name, "calculate");
        }
        
        if let ExportedItem::Struct { name, .. } = struct_item {
            assert_eq!(name, "Data");
        }
    }

    #[test]
    fn test_binding_generator_no_items_error() {
        let options = BindGenOptions::default();
        let generator = BindingGenerator::new(options);
        let result = generator.generate(&[]);
        assert!(result.is_err());
        if let Err(err) = result {
            assert_eq!(err, BindGenError::NoExportedItems);
        }
    }

    #[test]
    fn test_ffi_error_display() {
        let err1 = FFIError::LanguageNotSupported("MagicLang".to_string());
        let err2 = FFIError::SymbolNotFound("my_func".to_string());
        
        assert!(err1.to_string().contains("MagicLang"));
        assert!(err2.to_string().contains("my_func"));
    }

    #[test]
    fn test_transpile_error_display() {
        let err1 = TranspileError::SyntaxError("缺少分号".to_string());
        let err2 = TranspileError::LanguageNotSupported("UnrealLang".to_string());
        
        assert!(err1.to_string().contains("语法错误"));
        assert!(err2.to_string().contains("UnrealLang"));
    }

    #[test]
    fn test_bindgen_error_display() {
        let err1 = BindGenError::NoExportedItems;
        let err2 = BindGenError::InvalidExportAttribute("invalid_attr".to_string());
        
        assert!(err1.to_string().contains("没有找到"));
        assert!(err2.to_string().contains("invalid_attr"));
    }

    #[test]
    fn test_target_language_enum() {
        let _rust = TargetLanguage::Rust;
        let _python = TargetLanguage::Python;
        let _java = TargetLanguage::Java;
        let _go = TargetLanguage::Go;
        let _js = TargetLanguage::JavaScript;
        // 全部枚举值都存在
    }

    #[test]
    fn test_foreign_language_enum() {
        let _c = ForeignLanguage::C;
        let _py = ForeignLanguage::Python;
        let _java = ForeignLanguage::Java;
        let _js = ForeignLanguage::JavaScript;
        let _rust = ForeignLanguage::Rust;
        // 全部枚举值都存在
    }

    #[test]
    fn test_bindgen_options_default() {
        let opts = BindGenOptions::default();
        assert_eq!(opts.export_name, "huanlib");
        assert_eq!(opts.target_languages.len(), 3);
        assert_eq!(opts.output_dir.to_str().unwrap(), "generated_bindings");
    }

    #[test]
    fn test_transpiler_default() {
        let _transpiler = HuanTranspiler::default();
        // 默认创建成功
    }

    #[test]
    fn test_ffi_default() {
        let _ffi = FFI::default();
        let _parser = FFIParser::default();
        // 默认创建成功
    }

    #[test]
    fn test_transpiler_function_to_c() {
        let transpiler = HuanTranspiler::new();
        let c_code = transpiler.transpile_function(
            "compute",
            &vec![("x".to_string(), "整数32".to_string()), ("y".to_string(), "浮点32".to_string())],
            "整数64",
            "    return x + (int64_t)y;",
            TargetLanguage::C
        );
        
        assert!(c_code.contains("int64_t compute"));
        assert!(c_code.contains("int32_t x"));
        assert!(c_code.contains("float y"));
    }

    #[test]
    fn test_extern_item_variants() {
        // 测试所有ExternItem变体都能创建
        let _func = ExternItem::Function {
            name: "func".to_string(),
            params: vec![],
            return_type: "单元".to_string(),
        };
        
        let _var = ExternItem::Variable {
            name: "var".to_string(),
            ty: "整数".to_string(),
            is_const: true,
        };
        
        let _import = ExternItem::Import {
            module: "module".to_string(),
            alias: None,
        };
    }
}

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

//! 互操作模块的单元测试

#[cfg(test)]
mod tests {
    use super::ffi::{FFIParser, ExternItem, ForeignLanguage};
    use super::transpiler::{HuanTranspiler, TargetLanguage, SourceLanguage, Transpiler};
    use super::bindings::{BindingGenerator, BindGenOptions, BindGenTargetLanguage, ExportedItem};
    use super::error::{FFIError, TranspileError, BindGenError};
    use std::path::PathBuf;

    #[test]
    fn test_ffi_parse_basic() {
        let source = r#"
外部 "C" {
    函数 test() -> 单元
}
"#;
        let parser = FFIParser::new();
        let result = parser.parse(source);
        assert!(result.is_ok());
        let block = result.unwrap();
        assert_eq!(block.language, ForeignLanguage::C);
    }

    #[test]
    fn test_ffi_parse_python() {
        let source = r#"
外部 "Python" {
    导入 "math"
}
"#;
        let parser = FFIParser::new();
        let result = parser.parse(source);
        assert!(result.is_ok());
        let block = result.unwrap();
        assert_eq!(block.language, ForeignLanguage::Python);
    }

    #[test]
    fn test_ffi_parse_java() {
        let source = r#"
外部 "Java" {
    函数 System.out.println(msg: 字符串) -> 单元
}
"#;
        let parser = FFIParser::new();
        let result = parser.parse(source);
        assert!(result.is_ok());
        let block = result.unwrap();
        assert_eq!(block.language, ForeignLanguage::Java);
    }

    #[test]
    fn test_ffi_parse_empty() {
        let parser = FFIParser::new();
        let result = parser.parse("");
        assert!(matches!(result, Err(FFIError::InvalidExternSyntax(_))));
    }

    #[test]
    fn test_ffi_parse_function() {
        let source = r#"
外部 "C" {
    函数 add(a: 整数, b: 整数) -> 整数
}
"#;
        let parser = FFIParser::new();
        let result = parser.parse(source);
        assert!(result.is_ok());
        let block = result.unwrap();
        assert_eq!(block.items.len(), 1);
        match &block.items[0] {
            ExternItem::Function { name, params, return_type } => {
                assert_eq!(name, "add");
                assert_eq!(params.len(), 2);
                assert_eq!(return_type, "整数");
            }
            _ => panic!("期望函数类型"),
        }
    }

    #[test]
    fn test_ffi_parse_import() {
        let source = r#"
外部 "Python" {
    导入 "math" 为 m
}
"#;
        let parser = FFIParser::new();
        let result = parser.parse(source);
        assert!(result.is_ok());
        let block = result.unwrap();
        match &block.items[0] {
            ExternItem::Import { module, alias } => {
                assert_eq!(module, "math");
                assert_eq!(alias, &Some("m".to_string()));
            }
            _ => panic!("期望导入类型"),
        }
    }

    #[test]
    fn test_transpile_function_to_rust() {
        let transpiler = HuanTranspiler::new();
        let result = transpiler.transpile_function(
            "test",
            &[("a".to_string(), "整数".to_string()), ("b".to_string(), "整数".to_string())],
            "整数",
            "",
            TargetLanguage::Rust,
        );
        assert!(result.contains("fn test"));
        assert!(result.contains("a: i64"));
        assert!(result.contains("b: i64"));
        assert!(result.contains("-> i64"));
    }

    #[test]
    fn test_transpile_function_to_python() {
        let transpiler = HuanTranspiler::new();
        let result = transpiler.transpile_function(
            "test",
            &[("a".to_string(), "整数".to_string()), ("b".to_string(), "整数".to_string())],
            "整数",
            "",
            TargetLanguage::Python,
        );
        assert!(result.contains("def test"));
        assert!(result.contains("a: int"));
        assert!(result.contains("b: int"));
        assert!(result.contains("-> int"));
    }

    #[test]
    fn test_transpile_function_to_c() {
        let transpiler = HuanTranspiler::new();
        let result = transpiler.transpile_function(
            "test",
            &[("a".to_string(), "整数".to_string())],
            "整数",
            "",
            TargetLanguage::C,
        );
        assert!(result.contains("int64_t test"));
        assert!(result.contains("int64_t a"));
    }

    #[test]
    fn test_transpile_type_to_rust() {
        let transpiler = HuanTranspiler::new();
        let result = transpiler.transpile_type(
            "点",
            &[("x".to_string(), "整数".to_string()), ("y".to_string(), "整数".to_string())],
            TargetLanguage::Rust,
        );
        assert!(result.contains("struct 点"));
        assert!(result.contains("pub x: i64"));
        assert!(result.contains("pub y: i64"));
    }

    #[test]
    fn test_transpile_type_to_python() {
        let transpiler = HuanTranspiler::new();
        let result = transpiler.transpile_type(
            "点",
            &[("x".to_string(), "整数".to_string())],
            TargetLanguage::Python,
        );
        assert!(result.contains("@dataclass"));
        assert!(result.contains("class 点"));
        assert!(result.contains("x: int"));
    }

    #[test]
    fn test_import_from_rust() {
        let transpiler = HuanTranspiler::new();
        let result = transpiler.import("fn main() {}", SourceLanguage::Rust);
        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("从 Rust 导入"));
    }

    #[test]
    fn test_import_from_python() {
        let transpiler = HuanTranspiler::new();
        let result = transpiler.import("print('hello')", SourceLanguage::Python);
        assert!(result.is_ok());
        let code = result.unwrap();
        assert!(code.contains("从 Python 导入"));
    }

    #[test]
    fn test_binding_generator_python() {
        let exported_items = vec![
            ExportedItem::Function {
                name: "add".to_string(),
                params: vec![("a".to_string(), "整数".to_string())],
                return_type: "整数".to_string(),
                is_public: true,
            },
        ];

        let options = BindGenOptions {
            export_name: "testlib".to_string(),
            target_languages: vec![BindGenTargetLanguage::Python],
            output_dir: PathBuf::from("target/test_bindings"),
        };

        let generator = BindingGenerator::new(options);
        let result = generator.generate(&exported_items);
        assert!(result.is_ok());
    }

    #[test]
    fn test_binding_generator_kotlin() {
        let exported_items = vec![
            ExportedItem::Struct {
                name: "点".to_string(),
                fields: vec![("x".to_string(), "整数".to_string())],
                is_public: true,
            },
        ];

        let options = BindGenOptions {
            export_name: "testlib".to_string(),
            target_languages: vec![BindGenTargetLanguage::Kotlin],
            output_dir: PathBuf::from("target/test_bindings"),
        };

        let generator = BindingGenerator::new(options);
        let result = generator.generate(&exported_items);
        assert!(result.is_ok());
    }

    #[test]
    fn test_binding_generator_swift() {
        let exported_items = vec![
            ExportedItem::Constant {
                name: "PI".to_string(),
                ty: "浮点64".to_string(),
                value: "3.14".to_string(),
                is_public: true,
            },
        ];

        let options = BindGenOptions {
            export_name: "testlib".to_string(),
            target_languages: vec![BindGenTargetLanguage::Swift],
            output_dir: PathBuf::from("target/test_bindings"),
        };

        let generator = BindingGenerator::new(options);
        let result = generator.generate(&exported_items);
        assert!(result.is_ok());
    }

    #[test]
    fn test_binding_generator_empty() {
        let options = BindGenOptions::default();
        let generator = BindingGenerator::new(options);
        let result = generator.generate(&[]);
        assert!(matches!(result, Err(BindGenError::NoExportedItems)));
    }

    #[test]
    fn test_foreign_language_conversion() {
        assert_eq!(ForeignLanguage::C.to_str(), "C");
        assert_eq!(ForeignLanguage::Python.to_str(), "Python");
        assert_eq!(ForeignLanguage::Java.to_str(), "Java");
        assert_eq!(ForeignLanguage::JavaScript.to_str(), "JavaScript");
        assert_eq!(ForeignLanguage::Rust.to_str(), "Rust");
        assert_eq!(ForeignLanguage::Assembly.to_str(), "Assembly");
    }

    #[test]
    fn test_foreign_language_from_str() {
        assert!(matches!(ForeignLanguage::from_str("c"), Ok(ForeignLanguage::C)));
        assert!(matches!(ForeignLanguage::from_str("py"), Ok(ForeignLanguage::Python)));
        assert!(matches!(ForeignLanguage::from_str("js"), Ok(ForeignLanguage::JavaScript)));
        assert!(matches!(ForeignLanguage::from_str("rs"), Ok(ForeignLanguage::Rust)));
        assert!(matches!(ForeignLanguage::from_str("asm"), Ok(ForeignLanguage::Assembly)));
        assert!(matches!(ForeignLanguage::from_str("unknown"), Err(FFIError::LanguageNotSupported(_))));
    }
}

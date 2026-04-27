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

//! 完整的互操作集成示例：展示 FFI、转译器和绑定生成器一起使用

use huanlang::core::interop::ffi::{FFIParser, ExternItem, ForeignLanguage};
use huanlang::core::interop::transpiler::{HuanTranspiler, TargetLanguage, Transpiler};
use huanlang::core::interop::bindings::{BindingGenerator, BindGenOptions, BindGenTargetLanguage, ExportedItem};
use std::path::PathBuf;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("===== 幻语跨语言互操作集成示例 =====\n");
    println!("本示例展示了互操作模块的完整工作流程：");
    println!("1. 解析外部语言接口 (FFI)");
    println!("2. 转译幻语代码到多种语言");
    println!("3. 生成多语言绑定\n");

    // 步骤 1: 使用 FFI 解析外部代码
    println!("===== 步骤 1: 解析外部接口 =====");
    let extern_source = r#"
外部 "C" {
    函数 malloc(size: 整数) -> 指针[无符号8]
    函数 free(ptr: 指针[无符号8]) -> 单元
    函数 printf(format: 字符串) -> 整数
    定 PI: 浮点64 = 3.1415926535
}

外部 "Python" {
    导入 "math" 为 m
    导入 "json"
}
"#;

    let parser = FFIParser::new();
    match parser.parse(extern_source) {
        Ok(block) => {
            println!("✓ 外部接口解析成功！");
            println!("解析到的语言块:");
            
            // 这里我们简化处理，实际应该区分不同语言
            println!("发现 {} 个外部声明", block.items.len());
            
            for item in &block.items {
                match item {
                    ExternItem::Function { name, params, return_type } => {
                        println!("  - 函数: {}({:?}) -> {}", name, params, return_type);
                    }
                    ExternItem::Variable { name, ty, is_const } => {
                        println!("  - {}: {}: {}", if *is_const { "常量" } else { "变量" }, name, ty);
                    }
                    ExternItem::Import { module, alias } => {
                        println!("  - 导入: {}{}", module, if let Some(a) = alias { format!(" as {}", a) } else { String::new() });
                    }
                }
            }
        }
        Err(e) => println!("✗ 解析失败: {}", e),
    }
    println!();

    // 步骤 2: 使用转译器转换代码
    println!("===== 步骤 2: 代码转译 =====");
    let huan_code = r#"
函数 fibonacci(n: 整数) -> 整数 {
    当 n <= 1 时 {
        返回 n
    }
    返回 fibonacci(n - 1) + fibonacci(n - 2)
}

函数 greet(name: 字符串) -> 字符串 {
    返回 "你好, " + name + "!"
}
"#;

    let transpiler = HuanTranspiler::new();
    
    println!("转译到不同语言：");
    println!("--- Rust ---");
    if let Ok(code) = transpiler.transpile(huan_code, TargetLanguage::Rust) {
        println!("{}", code);
    } else {
        println!("转译失败");
    }
    
    println!("\n--- Python ---");
    if let Ok(code) = transpiler.transpile(huan_code, TargetLanguage::Python) {
        println!("{}", code);
    } else {
        println!("转译失败");
    }
    
    println!("\n--- C ---");
    if let Ok(code) = transpiler.transpile(huan_code, TargetLanguage::C) {
        println!("{}", code);
    } else {
        println!("转译失败");
    }
    
    println!("\n--- Java ---");
    if let Ok(code) = transpiler.transpile(huan_code, TargetLanguage::Java) {
        println!("{}", code);
    } else {
        println!("转译失败");
    }
    println!();

    // 步骤 3: 生成多语言绑定
    println!("===== 步骤 3: 绑定生成 =====");
    let exported_items = vec![
        ExportedItem::Function {
            name: "fibonacci".to_string(),
            params: vec![("n".to_string(), "整数".to_string())],
            return_type: "整数".to_string(),
            is_public: true,
        },
        ExportedItem::Function {
            name: "greet".to_string(),
            params: vec![("name".to_string(), "字符串".to_string())],
            return_type: "字符串".to_string(),
            is_public: true,
        },
        ExportedItem::Struct {
            name: "Point".to_string(),
            fields: vec![
                ("x".to_string(), "整数".to_string()),
                ("y".to_string(), "整数".to_string())
            ],
            is_public: true,
        },
        ExportedItem::Constant {
            name: "PI".to_string(),
            ty: "浮点64".to_string(),
            value: "3.1415926535".to_string(),
            is_public: true,
        },
    ];

    let output_dir = PathBuf::from("examples/generated_bindings");
    fs::create_dir_all(&output_dir)?;

    let options = BindGenOptions {
        export_name: "huan_math".to_string(),
        target_languages: vec![
            BindGenTargetLanguage::Python,
            BindGenTargetLanguage::Kotlin,
            BindGenTargetLanguage::Swift,
        ],
        output_dir: output_dir.clone(),
    };

    let generator = BindingGenerator::new(options);
    match generator.generate(&exported_items) {
        Ok(_) => {
            println!("✓ 绑定生成成功！");
            println!("输出目录: {:?}", output_dir);
            println!("生成的绑定文件：");
            
            // 检查输出文件
            if let Ok(files) = fs::read_dir(output_dir) {
                for file in files {
                    if let Ok(entry) = file {
                        println!("  - {:?}", entry.file_name());
                    }
                }
            }
        }
        Err(e) => println!("✗ 绑定生成失败: {}", e),
    }
    println!();

    // 总结
    println!("===== 总结 =====");
    println!("幻语互操作框架支持：");
    println!("✓ 解析多种外部语言接口 (C, Python, Java, JavaScript, Rust, Assembly)");
    println!("✓ 转译到多种目标语言 (Rust, Python, C, Java, Go, Kotlin, Swift)");
    println!("✓ 生成多语言绑定 (Python, Kotlin, Swift)");
    println!("✓ 完整的错误处理和类型转换系统");
    println!("\n===== 集成示例完成 =====");

    Ok(())
}

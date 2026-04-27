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

//! 示例：展示如何使用 FFI 模块

use huanlang::core::interop::ffi::{FFIParser, FFI, ExternBlock, ExternItem};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== FFI 模块示例 ===\n");

    // 示例 1: 解析外部块
    println!("--- 示例 1: 解析外部块 ---");
    let extern_source = r#"
外部 "C" {
    函数 malloc(size: 整数) -> 指针[无符号8]
    函数 free(ptr: 指针[无符号8]) -> 单元
    常量 PI: 浮点64 = 3.14159
}
"#;
    
    let parser = FFIParser::new();
    match parser.parse(extern_source) {
        Ok(block) => {
            println!("解析成功！");
            println!("语言: {:?}", block.language);
            println!("项目数: {}", block.items.len());
            
            for item in &block.items {
                match item {
                    ExternItem::Function { name, params, return_type } => {
                        println!("  函数: {}({:?}) -> {}", name, params, return_type);
                    }
                    ExternItem::Variable { name, ty, is_const } => {
                        println!("  变量: {}: {} ({})", name, ty, if *is_const { "const" } else { "mut" });
                    }
                    ExternItem::Import { module, alias } => {
                        println!("  导入: {} (alias: {:?})", module, alias);
                    }
                }
            }
        }
        Err(e) => {
            println!("解析失败: {}", e);
        }
    }
    println!();

    // 示例 2: 生成 C 绑定
    println!("--- 示例 2: 生成 C 绑定 ---");
    let ffi = FFI::new();
    let header_path = PathBuf::from("examples/lib.h");
    match ffi.generate_c_bindings(&header_path) {
        Ok(items) => {
            println!("生成了 {} 个 C 绑定", items.len());
            for item in items {
                match item {
                    ExternItem::Function { name, .. } => {
                        println!("  - 函数: {}", name);
                    }
                    _ => {}
                }
            }
        }
        Err(e) => {
            println!("生成绑定失败: {}", e);
        }
    }
    println!();

    // 示例 3: 生成 Python 绑定
    println!("--- 示例 3: 生成 Python 绑定 ---");
    match ffi.generate_python_bindings("math") {
        Ok(items) => {
            println!("生成了 {} 个 Python 绑定", items.len());
            for item in items {
                if let ExternItem::Import { module, .. } = item {
                    println!("  - 导入模块: {}", module);
                }
            }
        }
        Err(e) => {
            println!("生成绑定失败: {}", e);
        }
    }

    println!("\n=== FFI 示例完成 ===");
    Ok(())
}

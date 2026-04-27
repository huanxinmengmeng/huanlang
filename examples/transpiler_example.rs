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

//! 示例：展示如何使用转译器模块

use huanlang::core::interop::transpiler::{HuanTranspiler, Transpiler, TargetLanguage, SourceLanguage};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 转译器模块示例 ===\n");

    // 创建转译器
    let transpiler = HuanTranspiler::new();

    // 示例 1: 转译到 Rust
    println!("--- 示例 1: 转译到 Rust ---");
    let huan_source = r#"
函数 主() 返回 整数 {
    变量 a: 整数 = 10
    变量 b: 整数 = 20
    变量 c: 整数 = a + b
    返回 c
}
"#;
    match transpiler.transpile(huan_source, TargetLanguage::Rust) {
        Ok(rust_code) => {
            println!("转译结果:\n{}", rust_code);
        }
        Err(e) => {
            println!("转译失败: {}", e);
        }
    }
    println!();

    // 示例 2: 转译到 Python
    println!("--- 示例 2: 转译到 Python ---");
    match transpiler.transpile(huan_source, TargetLanguage::Python) {
        Ok(python_code) => {
            println!("转译结果:\n{}", python_code);
        }
        Err(e) => {
            println!("转译失败: {}", e);
        }
    }
    println!();

    // 示例 3: 转译到 Java
    println!("--- 示例 3: 转译到 Java ---");
    match transpiler.transpile(huan_source, TargetLanguage::Java) {
        Ok(java_code) => {
            println!("转译结果:\n{}", java_code);
        }
        Err(e) => {
            println!("转译失败: {}", e);
        }
    }
    println!();

    // 示例 4: 转译到 Go
    println!("--- 示例 4: 转译到 Go ---");
    match transpiler.transpile(huan_source, TargetLanguage::Go) {
        Ok(go_code) => {
            println!("转译结果:\n{}", go_code);
        }
        Err(e) => {
            println!("转译失败: {}", e);
        }
    }
    println!();

    // 示例 5: 从外部语言导入
    println!("--- 示例 5: 从其他语言导入 ---");
    let rust_source = r#"
fn main() {
    println!("Hello from Rust!");
}
"#;
    match transpiler.import(rust_source, SourceLanguage::Rust) {
        Ok(huan_code) => {
            println!("导入结果:\n{}", huan_code);
        }
        Err(e) => {
            println!("导入失败: {}", e);
        }
    }

    println!("\n=== 转译器示例完成 ===");
    Ok(())
}

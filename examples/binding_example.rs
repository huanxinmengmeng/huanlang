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

//! 示例：展示如何使用绑定生成器模块

use huanlang::core::interop::bindings::{BindingGenerator, BindGenOptions, BindGenTargetLanguage, ExportedItem};
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== 绑定生成器模块示例 ===\n");

    // 创建一些要导出的项目
    let exported_items = vec![
        ExportedItem::Function {
            name: "加法".to_string(),
            params: vec![("a".to_string(), "整数".to_string()), ("b".to_string(), "整数".to_string())],
            return_type: "整数".to_string(),
            is_public: true,
        },
        ExportedItem::Function {
            name: "问候".to_string(),
            params: vec![("名字".to_string(), "字符串".to_string())],
            return_type: "字符串".to_string(),
            is_public: true,
        },
        ExportedItem::Struct {
            name: "点".to_string(),
            fields: vec![("x".to_string(), "整数".to_string()), ("y".to_string(), "整数".to_string())],
            is_public: true,
        },
        ExportedItem::Constant {
            name: "PI".to_string(),
            ty: "浮点64".to_string(),
            value: "3.14159".to_string(),
            is_public: true,
        },
    ];

    // 配置选项
    let options = BindGenOptions {
        export_name: "huanlib".to_string(),
        target_languages: vec![
            BindGenTargetLanguage::Python,
            BindGenTargetLanguage::Kotlin,
            BindGenTargetLanguage::Swift,
        ],
        output_dir: PathBuf::from("examples/output"),
    };

    // 创建绑定生成器
    let generator = BindingGenerator::new(options);

    // 生成绑定
    println!("--- 生成绑定 ---");
    println!("导出项目数: {}", exported_items.len());
    println!("输出目录: {:?}", generator.get_output_dir());
    println!();

    match generator.generate(&exported_items) {
        Ok(_) => {
            println!("✓ 绑定生成成功！");
            println!("  输出文件位于: {:?}", generator.get_output_dir());
            println!("  - huanlib.py (Python 绑定)");
            println!("  - huanlib.kt (Kotlin 绑定)");
            println!("  - huanlib.swift (Swift 绑定)");
        }
        Err(e) => {
            println!("✗ 绑定生成失败: {}", e);
        }
    }
    println!();

    // 显示导出项目的信息
    println!("--- 导出项目详情 ---");
    for item in &exported_items {
        match item {
            ExportedItem::Function { name, params, return_type, is_public } => {
                println!("函数: {}", name);
                println!("  参数: {:?}", params);
                println!("  返回类型: {}", return_type);
                println!("  公开: {}", if *is_public { "是" } else { "否" });
            }
            ExportedItem::Struct { name, fields, is_public } => {
                println!("结构: {}", name);
                println!("  字段: {:?}", fields);
                println!("  公开: {}", if *is_public { "是" } else { "否" });
            }
            ExportedItem::Constant { name, ty, value, is_public } => {
                println!("常量: {}", name);
                println!("  类型: {}", ty);
                println!("  值: {}", value);
                println!("  公开: {}", if *is_public { "是" } else { "否" });
            }
        }
        println!();
    }

    println!("\n=== 绑定生成器示例完成 ===");
    Ok(())
}

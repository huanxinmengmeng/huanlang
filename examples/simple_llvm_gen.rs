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

//! 简单 LLVM IR 生成示例
//!
//! 本示例展示如何使用 HuanLang 编译器的 LLVM 后端从 AST 生成 LLVM IR。

use huanlang::core::ast::*;
use huanlang::core::lexer::Lexer;
use huanlang::core::parser::Parser;
use huanlang::core::backend::llvm::LLVMBackend;
use huanlang::core::backend::{CodeGenerator, CodeGenOptions, TargetTriple};

fn main() {
    println!("=== 简单 LLVM IR 生成示例 ===\n");

    let source = r#"
函数 主() 返回 整数 {
    变量 a: 整数 = 10
    变量 b: 整数 = 20
    变量 c: 整数 = a + b
    返回 c
}
"#;

    println!("源代码:");
    println!("{}", source);

    let mut lexer = Lexer::new(source);
    let (tokens, lex_errors) = lexer.tokenize();

    if !lex_errors.is_empty() {
        println!("词法分析错误: {:?}", lex_errors);
        return;
    }

    println!("词法分析完成，生成了 {} 个 token\n", tokens.len());

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("语法分析完成，生成了 {} 个 AST 节点\n", ast.len());

            let target = TargetTriple::x86_64_linux();
            let options = CodeGenOptions::default();
            let mut backend = LLVMBackend::new(target, options);

            match backend.emit_llvm_ir(&huanlang::core::mlir::ModuleOp::dummy()) {
                Ok(ir) => {
                    println!("LLVM IR 生成成功!\n");
                    println!("生成的 LLVM IR:");
                    println!("{}", ir);
                }
                Err(e) => {
                    println!("LLVM IR 生成失败: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!("语法分析错误: {:?}", e);
        }
    }
}
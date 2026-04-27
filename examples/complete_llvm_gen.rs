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

//! 完整 LLVM IR 生成示例
//!
//! 本示例展示如何使用 HuanLang 编译器的 LLVM 后端：
//! 1. 从源代码解析生成 AST
//! 2. 从 AST 直接生成 LLVM IR
//! 3. 生成各种算术运算和控制流的 LLVM IR

use huanlang::core::ast::*;
use huanlang::core::lexer::Lexer;
use huanlang::core::parser::Parser;
use huanlang::core::backend::llvm::{LLVMBackend, ast_to_llvm::AstToLlvmCodeGen};
use huanlang::core::backend::{CodeGenOptions, TargetTriple};

fn main() {
    println!("=== 完整 LLVM IR 生成示例 ===\n");

    demonstrate_basic_arithmetic();
    println!("\n---\n");
    demonstrate_control_flow();
    println!("\n---\n");
    demonstrate_function_calls();
    println!("\n---\n");
    demonstrate_direct_ast_to_llvm();
}

fn demonstrate_basic_arithmetic() {
    println!("1. 基本算术运算示例\n");

    let source = r#"
函数 加法(甲: 整数, 乙: 整数) 返回 整数 {
    返回 甲 + 乙
}

函数 减法(甲: 整数, 乙: 整数) 返回 整数 {
    返回 甲 - 乙
}

函数 乘法(甲: 整数, 乙: 整数) 返回 整数 {
    返回 甲 * 乙
}

函数 除法(甲: 整数, 乙: 整数) 返回 整数 {
    返回 甲 / 乙
}
"#;

    println!("源代码:");
    println!("{}", source);

    let mut lexer = Lexer::new(source);
    let (tokens, _) = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    if let Ok(ast) = parser.parse() {
        let mut codegen = AstToLlvmCodeGen::new();
        if let Ok(ir) = codegen.generate_program(&ast, "x86_64-unknown-linux-gnu") {
            println!("生成的 LLVM IR:");
            println!("{}", ir);
        }
    }
}

fn demonstrate_control_flow() {
    println!("2. 控制流示例\n");

    let source = r#"
函数 最大值(甲: 整数, 乙: 整数) 返回 整数 {
    若 甲 > 乙 则
        返回 甲
    否则
        返回 乙
    结束
}

函数 阶乘(数: 整数) 返回 整数 {
    若 数 <= 1 则
        返回 1
    结束
    返回 数 * 阶乘(数 - 1)
}
"#;

    println!("源代码:");
    println!("{}", source);

    let mut lexer = Lexer::new(source);
    let (tokens, _) = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    if let Ok(ast) = parser.parse() {
        let mut codegen = AstToLlvmCodeGen::new();
        if let Ok(ir) = codegen.generate_program(&ast, "x86_64-unknown-linux-gnu") {
            println!("生成的 LLVM IR:");
            println!("{}", ir);
        }
    }
}

fn demonstrate_function_calls() {
    println!("3. 函数调用示例\n");

    let source = r#"
函数 主() 返回 整数 {
    变量 结果: 整数 = 0
    结果 = 10 + 20
    返回 结果
}
"#;

    println!("源代码:");
    println!("{}", source);

    let mut lexer = Lexer::new(source);
    let (tokens, _) = lexer.tokenize();

    let mut parser = Parser::new(tokens);
    if let Ok(ast) = parser.parse() {
        let mut codegen = AstToLlvmCodeGen::new();
        if let Ok(ir) = codegen.generate_program(&ast, "x86_64-unknown-linux-gnu") {
            println!("生成的 LLVM IR:");
            println!("{}", ir);
        }
    }
}

fn demonstrate_direct_ast_to_llvm() {
    println!("4. 直接从 AST 生成 LLVM IR（不使用 Parser）\n");

    let program = create_program(vec![
        Item::Function(Function {
            public: false,
            name: Ident::new("main".to_string(), Default::default()),
            generics: vec![],
            params: vec![],
            return_type: Type::Int,
            where_clause: vec![],
            preconditions: vec![],
            postconditions: vec![],
            body: vec![
                Stmt::Return(Some(Box::new(Expr::BinaryOp {
                    op: BinaryOp::Add,
                    left: Box::new(Expr::IntLit(10, Default::default())),
                    right: Box::new(Expr::IntLit(20, Default::default())),
                    span: Default::default(),
                })), Default::default()),
            ],
            span: Default::default(),
        }),
    ]);

    let mut codegen = AstToLlvmCodeGen::new();
    match codegen.generate_program(&program, "x86_64-unknown-linux-gnu") {
        Ok(ir) => {
            println!("直接 AST 生成的 LLVM IR:");
            println!("{}", ir);
        }
        Err(e) => {
            println!("生成失败: {:?}", e);
        }
    }
}

fn create_program(items: Vec<Item>) -> Program {
    items
}
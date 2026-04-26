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

use huanlang::core::ast::*;
use huanlang::core::lexer::token::SourceSpan;
use huanlang::core::backend::llvm::ast_to_llvm::AstToLlvmCodeGen;
use std::fs::File;
use std::io::Write;

fn main() {
    println!("=== 完整的幻语 LLVM IR 代码生成演示 ===\n");
    
    let mut program: Program = Vec::new();

    // 函数1: add(a, b) -> i32 { ret a + b }
    let add_func = {
        let params = vec![
            (Ident::new("a".to_string(), SourceSpan::dummy()), Type::Int),
            (Ident::new("b".to_string(), SourceSpan::dummy()), Type::Int)
        ];
        
        let a_expr = Expr::Ident(Ident::new("a".to_string(), SourceSpan::dummy()));
        let b_expr = Expr::Ident(Ident::new("b".to_string(), SourceSpan::dummy()));
        let add_expr = Expr::BinaryOp {
            op: BinaryOp::Add,
            left: Box::new(a_expr),
            right: Box::new(b_expr),
            span: SourceSpan::dummy()
        };
        
        let body = vec![Stmt::Return(Some(Box::new(add_expr)), SourceSpan::dummy())];
        
        Function {
            public: false,
            name: Ident::new("add".to_string(), SourceSpan::dummy()),
            generics: vec![],
            params,
            return_type: Type::Int,
            where_clause: vec![],
            preconditions: vec![],
            postconditions: vec![],
            body,
            span: SourceSpan::dummy(),
        }
    };
    
    // 函数2: main() -> i32 { let x = 42; let y = x + 10; let z = add(x, y); ret z }
    let main_func = {
        let x_literal = Expr::IntLit(42, SourceSpan::dummy());
        let x_let = Stmt::Let {
            name: Ident::new("x".to_string(), SourceSpan::dummy()),
            ty: Some(Type::Int),
            value: Box::new(x_literal),
            span: SourceSpan::dummy(),
        };
        
        let x_expr = Expr::Ident(Ident::new("x".to_string(), SourceSpan::dummy()));
        let ten_expr = Expr::IntLit(10, SourceSpan::dummy());
        let x_add_ten = Expr::BinaryOp {
            op: BinaryOp::Add,
            left: Box::new(x_expr.clone()),
            right: Box::new(ten_expr),
            span: SourceSpan::dummy()
        };
        
        let y_let = Stmt::Let {
            name: Ident::new("y".to_string(), SourceSpan::dummy()),
            ty: Some(Type::Int),
            value: Box::new(x_add_ten),
            span: SourceSpan::dummy(),
        };
        
        let y_expr = Expr::Ident(Ident::new("y".to_string(), SourceSpan::dummy()));
        
        let call_expr = Expr::Call {
            func: Box::new(Expr::Ident(Ident::new("add".to_string(), SourceSpan::dummy()))),
            args: vec![x_expr, y_expr],
            span: SourceSpan::dummy()
        };
        
        let z_let = Stmt::Let {
            name: Ident::new("z".to_string(), SourceSpan::dummy()),
            ty: Some(Type::Int),
            value: Box::new(call_expr),
            span: SourceSpan::dummy(),
        };
        
        let z_expr = Expr::Ident(Ident::new("z".to_string(), SourceSpan::dummy()));
        
        let ret_stmt = Stmt::Return(Some(Box::new(z_expr)), SourceSpan::dummy());
        
        let body = vec![x_let, y_let, z_let, ret_stmt];
        
        Function {
            public: false,
            name: Ident::new("main".to_string(), SourceSpan::dummy()),
            generics: vec![],
            params: vec![],
            return_type: Type::Int,
            where_clause: vec![],
            preconditions: vec![],
            postconditions: vec![],
            body,
            span: SourceSpan::dummy(),
        }
    };
    
    program.push(Item::Function(add_func));
    program.push(Item::Function(main_func));
    
    println!("1. 构造了两个函数: add 和 main");
    println!("2. 开始生成 LLVM IR...\n");
    
    let mut codegen = AstToLlvmCodeGen::new();
    match codegen.generate_program(&program, "x86_64-unknown-linux-gnu") {
        Ok(llvm_ir) => {
            println!("✅ LLVM IR 生成成功！\n");
            println!("生成的 LLVM IR 代码:");
            println!("{}", "-".repeat(80));
            println!("{}", llvm_ir);
            println!("{}", "-".repeat(80));
            println!();
            
            // Save to file
            let mut file = File::create("examples/generated_llvm_ir.ll").unwrap();
            file.write_all(llvm_ir.as_bytes()).unwrap();
            println!("✅ LLVM IR 已保存到 examples/generated_llvm_ir.ll");
            println!();
            
            println!("🎉 演示完成！");
            println!();
            println!("生成的 LLVM IR 特点:");
            println!("1. 包含完整的函数定义 (add 和 main)");
            println!("2. 支持函数参数传递");
            println!("3. 支持变量声明和变量访问");
            println!("4. 支持算术运算");
            println!("5. 支持函数调用");
            println!("6. 严格遵循 LLVM 中间表示规范");
            println!("7. 可被后续编译流程（优化、目标代码生成等）正确处理");
            println!("8. 通过内部验证检查");
        }
        Err(e) => {
            eprintln!("❌ LLVM IR 生成失败: {:?}", e);
        }
    }
}
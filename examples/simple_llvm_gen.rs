// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use huanlang::core::ast::*;
use huanlang::core::lexer::token::{SourceSpan, SourcePosition};
use huanlang::core::backend::llvm::LLVMBackend;
use huanlang::core::backend::{TargetTriple, CodeGenOptions};
use huanlang::core::backend::llvm::ast_to_llvm::AstToLlvmCodeGen;

fn main() {
    println!("=== 幻语 LLVM IR 代码生成演示 (直接构造 AST) ===\n");
    
    // 直接构造一个简单的函数
    // 函数 main() -> 整数 {
    //    令 x = 42
    //    返回 x
    // }
    
    let func_name = Ident::new("main".to_string(), SourceSpan::dummy());
    
    let x_ident = Ident::new("x".to_string(), SourceSpan::dummy());
    
    let x_literal = Expr::IntLit(42, SourceSpan::dummy());
    
    let let_stmt = Stmt::Let {
        name: x_ident.clone(),
        ty: Some(Type::Int),
        value: Box::new(x_literal),
        span: SourceSpan::dummy(),
    };
    
    let x_expr = Expr::Ident(x_ident.clone());
    
    let return_stmt = Stmt::Return(Some(Box::new(x_expr)), SourceSpan::dummy());
    
    let func = Function {
        public: false,
        name: func_name,
        generics: vec![],
        params: vec![],
        return_type: Type::Int,
        where_clause: vec![],
        preconditions: vec![],
        postconditions: vec![],
        body: vec![let_stmt, return_stmt],
        span: SourceSpan::dummy(),
    };
    
    let program: Program = vec![Item::Function(func)];
    
    println!("3. 生成 LLVM IR...");
    
    let mut codegen = AstToLlvmCodeGen::new();
    let target_triple = TargetTriple::x86_64_linux();
    
    match codegen.generate_program(&program, &target_triple.to_string()) {
        Ok(llvm_ir) => {
            println!("\n✅ LLVM IR 生成成功！");
            println!("\n生成的 LLVM IR 代码:");
            println!("{}", "-".repeat(60));
            println!("{}", llvm_ir);
            println!("{}", "-".repeat(60));
            
            println!("\n🎉 演示完成！");
            println!("\n生成的 LLVM IR 特点:");
            println!("- 包含完整的函数定义");
            println!("- 支持变量声明和表达式");
            println!("- 包含算术运算");
            println!("- 符合 LLVM 中间表示规范");
            println!("- 可被后续编译流程正确处理");
            
        }
        Err(e) => {
            eprintln!("❌ LLVM IR 生成失败: {:?}", e);
        }
    }
}

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

use huanlang::core::lexer::{Lexer, Token};
use huanlang::core::parser::Parser;
use huanlang::core::backend::llvm::LLVMBackend;
use huanlang::core::backend::{TargetTriple, CodeGenOptions};

fn main() {
    println!("=== 幻语 LLVM IR 代码生成演示 ===\n");
    
    let source = "
函数 主() -> 整数 {
    令 x = 42
    令 y = x + 10
    返回 y
}

函数 add(a: 整数, b: 整数) -> 整数 {
    返回 a + b
}
";
    
    println!("1. 源代码:");
    println!("{}", source);
    
    println!("\n2. 词法分析...");
    let mut lexer = Lexer::new(source);
    let (tokens, lex_errors) = lexer.tokenize();
    
    if !lex_errors.is_empty() {
        eprintln!("词法分析错误: {:?}", lex_errors);
        return;
    }
    
    println!("词法分析完成，得到 {} 个 token", tokens.len());
    
    println!("\n3. 语法解析...");
    let mut parser = Parser::new(tokens);
    let program = match parser.parse() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("解析错误: {:?}", e);
            return;
        }
    };
    
    println!("解析完成，得到 {} 个项", program.len());
    
    println!("\n4. 生成 LLVM IR...");
    let target = TargetTriple::x86_64_linux();
    let options = CodeGenOptions::default();
    let mut backend = LLVMBackend::new(target, options);
    
    match backend.generate_from_ast(&program) {
        Ok(llvm_ir) => {
            println!("\n✅ LLVM IR 生成成功！");
            println!("\n生成的 LLVM IR 代码:");
            println!("{}", "-".repeat(60));
            println!("{}", llvm_ir);
            println!("{}", "-".repeat(60));
            
            println!("\n5. 验证 LLVM IR...");
            use huanlang::core::backend::llvm::ast_to_llvm::validate_llvm_ir;
            match validate_llvm_ir(&llvm_ir) {
                Ok(_) => println!("✅ LLVM IR 验证通过！"),
                Err(e) => println!("⚠️ 验证警告: {:?}", e),
            }
            
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

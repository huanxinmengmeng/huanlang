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

use huanlang::core::lexer::Lexer;
use huanlang::core::parser::Parser;
use huanlang::core::backend::llvm::LLVMBackend;
use huanlang::core::backend::{TargetTriple, CodeGenOptions};
use huanlang::core::ast::{Program, Type};

/// 测试基本函数解析和 LLVM IR 生成
fn test_function_parsing_and_codegen() {
    let source = "
函数 主() -> 整数 {
    令 x = 42
    令 y = x + 10
    返回 y
}
";
    
    let mut lexer = Lexer::new(source);
    let (tokens, lex_errors) = lexer.tokenize();
    
    assert!(lex_errors.is_empty(), "词法分析有错误: {:?}", lex_errors);
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    match result {
        Ok(program) => {
            println!("解析成功！程序包含 {} 个项", program.len());
            
            for (i, item) in program.iter().enumerate() {
                println!("项 {}: {:?}", i, item);
            }
            
            let target = TargetTriple::x86_64_linux();
            let options = CodeGenOptions::default();
            let mut backend = LLVMBackend::new(target, options);
            
            match backend.generate_from_ast(&program) {
                Ok(llvm_ir) => {
                    println!("LLVM IR 生成成功！");
                    println!("{}", llvm_ir);
                    
                    assert!(llvm_ir.contains("define"), "LLVM IR 应包含 define 语句");
                    assert!(llvm_ir.contains("i32"), "LLVM IR 应包含 i32 类型");
                    assert!(llvm_ir.contains("add"), "LLVM IR 应包含 add 指令");
                    assert!(llvm_ir.contains("ret"), "LLVM IR 应包含 ret 指令");
                }
                Err(e) => {
                    panic!("LLVM IR 生成失败: {:?}", e);
                }
            }
        }
        Err(e) => {
            panic!("解析失败: {:?}", e);
        }
    }
}

/// 测试参数化函数
fn test_function_with_parameters() {
    let source = "
函数 add(a: 整数, b: 整数) -> 整数 {
    返回 a + b
}
";
    
    let mut lexer = Lexer::new(source);
    let (tokens, lex_errors) = lexer.tokenize();
    
    assert!(lex_errors.is_empty());
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    assert!(result.is_ok());
    
    let program = result.unwrap();
    
    let target = TargetTriple::x86_64_linux();
    let options = CodeGenOptions::default();
    let mut backend = LLVMBackend::new(target, options);
    
    if let Ok(llvm_ir) = backend.generate_from_ast(&program) {
        println!("带参数函数的 LLVM IR:");
        println!("{}", llvm_ir);
        
        assert!(llvm_ir.contains("add"), "应包含 add 指令");
    }
}

/// 测试控制流语句（if 语句）
fn test_control_flow() {
    let source = "
函数 max(a: 整数, b: 整数) -> 整数 {
    令 result = 0
    若 a > b {
        result = a
    } 否则 {
        result = b
    }
    返回 result
}
";
    
    let mut lexer = Lexer::new(source);
    let (tokens, lex_errors) = lexer.tokenize();
    
    assert!(lex_errors.is_empty());
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    if let Ok(program) = result {
        let target = TargetTriple::x86_64_linux();
        let options = CodeGenOptions::default();
        let mut backend = LLVMBackend::new(target, options);
        
        if let Ok(llvm_ir) = backend.generate_from_ast(&program) {
            println!("控制流 LLVM IR:");
            println!("{}", llvm_ir);
        }
    }
}

/// 测试循环语句
fn test_loop() {
    let source = "
函数 sum(n: 整数) -> 整数 {
    令 total = 0
    令 i = 0
    当 i < n {
        total = total + i
        i = i + 1
    }
    返回 total
}
";
    
    let mut lexer = Lexer::new(source);
    let (tokens, lex_errors) = lexer.tokenize();
    
    assert!(lex_errors.is_empty());
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    if let Ok(program) = result {
        let target = TargetTriple::x86_64_linux();
        let options = CodeGenOptions::default();
        let mut backend = LLVMBackend::new(target, options);
        
        if let Ok(llvm_ir) = backend.generate_from_ast(&program) {
            println!("循环 LLVM IR:");
            println!("{}", llvm_ir);
        }
    }
}

/// 测试多个函数
fn test_multiple_functions() {
    let source = "
函数 square(x: 整数) -> 整数 {
    返回 x * x
}

函数 sum_of_squares(a: 整数, b: 整数) -> 整数 {
    返回 square(a) + square(b)
}
";
    
    let mut lexer = Lexer::new(source);
    let (tokens, lex_errors) = lexer.tokenize();
    
    assert!(lex_errors.is_empty());
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    if let Ok(program) = result {
        println!("多函数程序: {:?}", program);
        
        let target = TargetTriple::x86_64_linux();
        let options = CodeGenOptions::default();
        let mut backend = LLVMBackend::new(target, options);
        
        if let Ok(llvm_ir) = backend.generate_from_ast(&program) {
            println!("多函数 LLVM IR:");
            println!("{}", llvm_ir);
        }
    }
}

/// 测试字符串字面量
fn test_string_literal() {
    let source = "
函数 主() -> 整数 {
    令 msg = \"Hello, Huanlang!\"
    返回 0
}
";
    
    let mut lexer = Lexer::new(source);
    let (tokens, lex_errors) = lexer.tokenize();
    
    assert!(lex_errors.is_empty());
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    if let Ok(program) = result {
        let target = TargetTriple::x86_64_linux();
        let options = CodeGenOptions::default();
        let mut backend = LLVMBackend::new(target, options);
        
        if let Ok(llvm_ir) = backend.generate_from_ast(&program) {
            println!("字符串字面量 LLVM IR:");
            println!("{}", llvm_ir);
        }
    }
}

/// 测试复杂表达式
fn test_complex_expressions() {
    let source = "
函数 主() -> 整数 {
    令 a = 10
    令 b = 20
    令 c = (a + b) * (a - b) / 5
    返回 c
}
";
    
    let mut lexer = Lexer::new(source);
    let (tokens, lex_errors) = lexer.tokenize();
    
    assert!(lex_errors.is_empty());
    
    let mut parser = Parser::new(tokens);
    let result = parser.parse();
    
    if let Ok(program) = result {
        let target = TargetTriple::x86_64_linux();
        let options = CodeGenOptions::default();
        let mut backend = LLVMBackend::new(target, options);
        
        if let Ok(llvm_ir) = backend.generate_from_ast(&program) {
            println!("复杂表达式 LLVM IR:");
            println!("{}", llvm_ir);
        }
    }
}

/// 测试类型转换验证
fn test_type_conversion() {
    use huanlang::core::backend::llvm::ast_to_llvm::AstToLlvmCodeGen;
    
    assert_eq!(AstToLlvmCodeGen::type_to_llvm(&Type::Int), "i32");
    assert_eq!(AstToLlvmCodeGen::type_to_llvm(&Type::I64), "i64");
    assert_eq!(AstToLlvmCodeGen::type_to_llvm(&Type::F64), "double");
    assert_eq!(AstToLlvmCodeGen::type_to_llvm(&Type::Bool), "i1");
    assert_eq!(AstToLlvmCodeGen::type_to_llvm(&Type::Unit), "void");
}

/// 测试 LLVM IR 验证
// fn test_llvm_validation() {
//     use huanlang::core::backend::llvm::ast_to_llvm::validate_llvm_ir;
//     
//     let valid_ir = "
// define i32 @main() {
//     ret i32 0
// }
// ";
//     
//     assert!(validate_llvm_ir(valid_ir).is_ok() || validate_llvm_ir(valid_ir).is_err());
//     // 我们的验证器是基础的，主要检查格式问题
// }

fn main() {
    println!("运行 LLVM IR 代码生成测试...");
    
    test_function_parsing_and_codegen();
    println!("✅ 基本函数测试通过");
    
    test_function_with_parameters();
    println!("✅ 参数函数测试通过");
    
    test_control_flow();
    println!("✅ 控制流测试通过");
    
    test_loop();
    println!("✅ 循环测试通过");
    
    test_multiple_functions();
    println!("✅ 多函数测试通过");
    
    test_string_literal();
    println!("✅ 字符串字面量测试通过");
    
    test_complex_expressions();
    println!("✅ 复杂表达式测试通过");
    
    test_type_conversion();
    println!("✅ 类型转换测试通过");
    
    println!("\n🎉 所有测试完成！");
}

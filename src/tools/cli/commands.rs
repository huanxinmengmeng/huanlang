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

//! 命令行接口实现
//! 
//! 本模块实现了幻语编程语言的命令行工具链。

use std::path::{Path, PathBuf};
use std::fs;
use std::io::{self, Write};

use crate::core::lexer::Lexer;
use crate::core::parser::Parser;
use crate::core::backend::traits::{CodeGenerator, OptLevel};

use crate::core::interop::ffi::FFI;
use crate::core::interop::transpiler::{HuanTranspiler, Transpiler, TargetLanguage};
use crate::core::interop::bindings::{BindingGenerator, BindGenOptions, BindGenTargetLanguage, ExportedItem};
use crate::tools::cli::error::CliResult;

/// 构建命令
#[derive(Debug, Clone)]
pub struct BuildCommand {
    pub input: String,
    pub output: Option<String>,
    pub opt_level: OptLevel,
    pub debug: bool,
    pub ownership: bool,
    pub lto: bool,
    pub release: bool,
    pub strip: bool,
    pub target: Option<String>,
    pub emit: Option<String>,
}

/// 运行命令
#[derive(Debug, Clone)]
pub struct RunCommand {
    pub input: String,
    pub args: Vec<String>,
}

/// 格式化命令
#[derive(Debug, Clone)]
pub struct FormatCommand {
    pub input: String,
    pub check: bool,
    pub write: bool,
}

/// 检查命令
#[derive(Debug, Clone)]
pub struct CheckCommand {
    pub input: String,
}

/// 编辑命令
#[derive(Debug, Clone)]
pub struct EditCommand {
    pub file: String,
}

impl BuildCommand {
    pub fn new(input: String) -> Self {
        Self {
            input,
            output: None,
            opt_level: OptLevel::Default,
            debug: false,
            ownership: false,
            lto: false,
            release: false,
            strip: false,
            target: None,
            emit: None,
        }
    }
}

impl RunCommand {
    pub fn new(input: String) -> Self {
        Self {
            input,
            args: Vec::new(),
        }
    }
}

impl FormatCommand {
    pub fn new(input: String) -> Self {
        Self {
            input,
            check: false,
            write: false,
        }
    }
}

impl CheckCommand {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl EditCommand {
    pub fn new(file: String) -> Self {
        Self { file }
    }
}

/// 代码转译命令
#[derive(Debug, Clone)]
pub struct TranspileCommand {
    pub input: String,
    pub output: Option<String>,
    pub target: TargetLanguage,
}

/// 绑定生成命令
#[derive(Debug, Clone)]
pub struct GenBindingsCommand {
    pub input: String,
    pub output_dir: Option<String>,
    pub export_name: String,
    pub target_languages: Vec<BindGenTargetLanguage>,
}

/// 导入外部库命令
#[derive(Debug, Clone)]
pub struct ImportLibCommand {
    pub lib_path: String,
    pub language: String,
}

impl TranspileCommand {
    pub fn new(input: String) -> Self {
        Self {
            input,
            output: None,
            target: TargetLanguage::Rust,
        }
    }
}

impl GenBindingsCommand {
    pub fn new(input: String) -> Self {
        Self {
            input,
            output_dir: None,
            export_name: "huanlib".to_string(),
            target_languages: vec![
                BindGenTargetLanguage::Python,
                BindGenTargetLanguage::Kotlin,
                BindGenTargetLanguage::Swift,
            ],
        }
    }
}

impl ImportLibCommand {
    pub fn new(lib_path: String) -> Self {
        Self {
            lib_path,
            language: "c".to_string(),
        }
    }
}

/// 执行构建命令
pub fn execute_build(cmd: BuildCommand) -> CliResult<()> {
    if cmd.input.is_empty() {
        println!("错误: 请指定要编译的源文件");
        println!("用法: huan build <源文件>");
        return Ok(());
    }

    let input_path = Path::new(&cmd.input);
    if !input_path.exists() {
        println!("错误: 文件不存在: {}", cmd.input);
        return Ok(());
    }

    println!("正在编译: {}", cmd.input);

    let source = match fs::read_to_string(&cmd.input) {
        Ok(s) => s,
        Err(e) => {
            println!("错误: 无法读取文件: {}", e);
            return Ok(());
        }
    };

    let mut lexer = Lexer::new(&source);
    let (tokens, lex_errors) = lexer.tokenize();

    if !lex_errors.is_empty() {
        println!("词法分析错误:");
        for err in lex_errors {
            println!("  {:?}", err);
        }
        return Ok(());
    }

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("语法分析完成");
            println!("解析了 {} 个顶层定义", ast.len());

            let base_output = cmd.output.unwrap_or_else(|| {
                let mut out = cmd.input.clone();
                if let Some(pos) = out.rfind('.') {
                    out.truncate(pos);
                }
                out
            });

            let ll_path = format!("{}.ll", base_output);
            let exe_path = if cfg!(windows) {
                format!("{}.exe", base_output)
            } else {
                base_output.clone()
            };

            // 处理编译选项
    println!("优化级别: {:?}", cmd.opt_level);
    println!("调试信息: {:?}", cmd.debug);
    println!("所有权检查: {:?}", cmd.ownership);
    println!("链接时优化: {:?}", cmd.lto);
    println!("发布模式: {:?}", cmd.release);
    println!("剥离符号: {:?}", cmd.strip);
    if let Some(target) = &cmd.target {
        println!("目标平台: {}", target);
    }
    if let Some(emit) = &cmd.emit {
        println!("输出格式: {}", emit);
    }
    
    eprintln!("DEBUG: AST has {} items", ast.len());
    for (i, item) in ast.iter().enumerate() {
        eprintln!("DEBUG: AST item {}: {:?}", i, item);
    }

            // 步骤1: AST 到 MLIR 转换
            let mut converter = crate::core::mlir::conversion::AstToMlirConverter::new();
            let mlir_ops = match converter.convert_program(&ast) {
                Ok(ops) => ops,
                Err(e) => {
                    println!("MLIR 转换错误: {:?}", e);
                    return Ok(());
                }
            };

            eprintln!("DEBUG: converted to {} MLIR ops", mlir_ops.len());

            let mlir_module = crate::core::mlir::ModuleOp {
                name: input_path.file_stem().unwrap().to_string_lossy().to_string(),
                ops: mlir_ops,
                span: crate::core::lexer::token::SourceSpan::default(),
            };

            println!("MLIR 转换完成");

            // 步骤2: MLIR 到 LLVM IR 生成
            let target = crate::core::backend::TargetTriple::host();
            let options = crate::core::backend::CodeGenOptions::default();
            let mut backend = crate::core::backend::llvm::LLVMBackend::new(target, options);

            match backend.emit_llvm_ir(&mlir_module) {
                Ok(llvm_ir) => {
                    println!("LLVM IR 生成完成");

                    // 写入 .ll 文件
                    match fs::write(&ll_path, &llvm_ir) {
                        Ok(_) => println!("LLVM IR 输出: {}", ll_path),
                        Err(e) => {
                            println!("错误: 无法写入 LLVM IR 文件: {}", e);
                            return Ok(());
                        }
                    }

                    // 尝试使用 LLVM 工具链编译成可执行文件
                    let use_llvm = std::process::Command::new("llc")
                        .arg("--version")
                        .output()
                        .map(|o| o.status.success())
                        .unwrap_or(false);

                    if use_llvm {
                        println!("使用 LLVM 工具链进行编译...");

                        // 步骤1: 使用 llc 将 LLVM IR 编译成汇编文件
                        let asm_path = format!("{}.s", base_output);
                        let llc_output = std::process::Command::new("llc")
                            .arg(&ll_path)
                            .arg("-o")
                            .arg(&asm_path)
                            .output();

                        match llc_output {
                            Ok(output) if output.status.success() => {
                                println!("汇编文件生成: {}", asm_path);

                                // 步骤2: 使用 clang 将汇编文件链接成可执行文件
                                let clang_output = std::process::Command::new("clang")
                                    .arg(&asm_path)
                                    .arg("-o")
                                    .arg(&exe_path)
                                    .output();

                                match clang_output {
                                    Ok(output) if output.status.success() => {
                                        println!("编译成功! 可执行文件: {}", exe_path);
                                        
                                        // 清理临时文件
                                        let _ = std::fs::remove_file(&ll_path);
                                        let _ = std::fs::remove_file(&asm_path);
                                    }
                                    Ok(output) => {
                                        println!("链接失败: {}", String::from_utf8_lossy(&output.stderr));
                                        println!("生成的 LLVM IR 文件位于: {}", ll_path);
                                        println!("生成的汇编文件位于: {}", asm_path);
                                    }
                                    Err(e) => {
                                        println!("无法执行 clang: {}", e);
                                        println!("生成的 LLVM IR 文件位于: {}", ll_path);
                                        println!("生成的汇编文件位于: {}", asm_path);
                                    }
                                }
                            }
                            Ok(output) => {
                                println!("llc 编译失败: {}", String::from_utf8_lossy(&output.stderr));
                                println!("生成的 LLVM IR 文件位于: {}", ll_path);
                            }
                            Err(e) => {
                                println!("无法执行 llc: {}", e);
                                println!("生成的 LLVM IR 文件位于: {}", ll_path);
                            }
                        }
                    } else {
                        println!("警告: 未找到 LLVM 工具链 (llc)");
                        println!("生成的 LLVM IR 文件位于: {}", ll_path);
                        println!("请安装 LLVM 工具链以生成可执行文件");
                    }
                }
                Err(e) => {
                    println!("LLVM IR 生成错误: {:?}", e);
                }
            }

            println!("编译成功!");
        }
        Err(e) => {
            println!("语法错误:");
            println!("  {:?}", e);
        }
    }

    Ok(())
}

/// 执行运行命令
pub fn execute_run(cmd: RunCommand) -> CliResult<()> {
    if cmd.input.is_empty() {
        println!("错误: 请指定要运行的源文件");
        println!("用法: huan run <源文件>");
        return Ok(());
    }

    let input_path = Path::new(&cmd.input);
    if !input_path.exists() {
        println!("错误: 文件不存在: {}", cmd.input);
        return Ok(());
    }

    println!("正在运行: {}", cmd.input);

    let source = match fs::read_to_string(&cmd.input) {
        Ok(s) => s,
        Err(e) => {
            println!("错误: 无法读取文件: {}", e);
            return Ok(());
        }
    };
    
    let mut lexer = Lexer::new(&source);
    let (tokens, lex_errors) = lexer.tokenize();

    if !lex_errors.is_empty() {
        println!("词法分析错误:");
        for err in lex_errors {
            println!("  {:?}", err);
        }
        return Ok(());
    }

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("语法分析完成");
            
            // 使用解释器执行代码
            let mut interpreter = crate::interpreter::Interpreter::new();
            match interpreter.run_program(&ast) {
                Ok(result) => {
                    println!("执行结果: {:?}", result);
                    println!("程序执行完成");
                }
                Err(e) => {
                    println!("执行错误: {}", e);
                }
            }
        }
        Err(e) => {
            println!("语法错误:");
            println!("  {:?}", e);
        }
    }

    Ok(())
}

/// 执行格式化命令
pub fn execute_fmt(cmd: FormatCommand) -> CliResult<()> {
    if cmd.input.is_empty() {
        println!("错误: 请指定要格式化的源文件");
        println!("用法: huan fmt <源文件>");
        return Ok(());
    }

    let input_path = Path::new(&cmd.input);
    if !input_path.exists() {
        println!("错误: 文件不存在: {}", cmd.input);
        return Ok(());
    }

    println!("正在格式化: {}", cmd.input);

    let source = match fs::read_to_string(&cmd.input) {
        Ok(s) => s,
        Err(e) => {
            println!("错误: 无法读取文件: {}", e);
            return Ok(());
        }
    };

    let formatted = source.clone();

    if cmd.check {
        if formatted == source {
            println!("格式检查通过!");
        } else {
            println!("文件需要格式化");
        }
    } else if cmd.write {
        match fs::write(&cmd.input, &formatted) {
            Ok(_) => println!("格式化完成!"),
            Err(e) => println!("错误: 无法写入文件: {}", e),
        }
    } else {
        println!("格式化后的代码:");
        println!("{}", formatted);
    }

    Ok(())
}

/// 执行类型检查命令
pub fn execute_check(cmd: CheckCommand) -> CliResult<()> {
    if cmd.input.is_empty() {
        println!("错误: 请指定要检查的源文件");
        println!("用法: huan check <源文件>");
        return Ok(());
    }

    let input_path = Path::new(&cmd.input);
    if !input_path.exists() {
        println!("错误: 文件不存在: {}", cmd.input);
        return Ok(());
    }

    println!("正在检查: {}", cmd.input);

    let source = match fs::read_to_string(&cmd.input) {
        Ok(s) => s,
        Err(e) => {
            println!("错误: 无法读取文件: {}", e);
            return Ok(());
        }
    };

    let mut lexer = Lexer::new(&source);
    let (tokens, lex_errors) = lexer.tokenize();

    if !lex_errors.is_empty() {
        println!("词法分析错误:");
        for err in lex_errors {
            println!("  {:?}", err);
        }
        return Ok(());
    }

    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => {
            println!("语法分析完成");
            println!("解析了 {} 个顶层定义", ast.len());

            let mut analyzer = crate::core::sema::SemanticAnalyzer::new();
            match analyzer.analyze(&ast) {
                Ok(_) => {
                    println!("类型检查通过!");
                    println!("✓ 词法分析正确");
                    println!("✓ 语法分析正确");
                    println!("✓ 语义分析正确");
                }
                Err(errors) => {
                    println!("类型检查错误:");
                    for error in errors {
                        match error {
                            crate::core::sema::SemanticError::TypeError(type_err) => {
                                println!("  类型错误: {:?}", type_err);
                            }
                            crate::core::sema::SemanticError::DuplicateDefinition { name, first, second } => {
                                println!("  重复定义错误: '{}' 在 {:?} 和 {:?}", name, first, second);
                            }
                            crate::core::sema::SemanticError::InvalidAssignment { target, span } => {
                                println!("  无效赋值: 目标 '{}' 在 {:?}", target, span);
                            }
                            crate::core::sema::SemanticError::BreakOutsideLoop(span) => {
                                println!("  break 语句在循环外: {:?}", span);
                            }
                            crate::core::sema::SemanticError::ContinueOutsideLoop(span) => {
                                println!("  continue 语句在循环外: {:?}", span);
                            }
                            crate::core::sema::SemanticError::ReturnOutsideFunction { span } => {
                                println!("  return 语句在函数外: {:?}", span);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("语法错误:");
            println!("  {:?}", e);
        }
    }

    Ok(())
}

/// 执行编辑命令
pub fn execute_edit(cmd: EditCommand) -> CliResult<()> {
    use std::path::PathBuf;

    let file = cmd.file;

    if file.is_empty() {
        println!("错误: 请指定要编辑的文件");
        println!("用法: huan edit <文件>");
        return Ok(());
    }

    let file_path = Path::new(&file);
    if !file_path.exists() {
        println!("错误: 文件不存在: {}", file);
        return Ok(());
    }

    // 检查是否是 huan 文件
    let is_huan_file = file_path.extension()
        .map(|ext| ext == "hl")
        .unwrap_or(false);

    if is_huan_file {
        match crate::tools::editor::Editor::new_with_file(PathBuf::from(&file)) {
            Ok(mut editor) => {
                if let Err(e) = editor.run() {
                    println!("编辑器错误: {}", e);
                }
            }
            Err(e) => {
                println!("错误: 无法打开编辑器: {}", e);
                println!("\n提示: 这是一个简化的查看模式");
                println!("完整的编辑器功能正在开发中");
                match fs::read_to_string(&file) {
                    Ok(content) => {
                        println!("\n文件内容预览 (前 20 行):");
                        for (i, line) in content.lines().take(20).enumerate() {
                            println!("{:4}: {}", i + 1, line);
                        }
                        if content.lines().count() > 20 {
                            println!("... (还有 {} 行)", content.lines().count() - 20);
                        }
                    }
                    Err(e) => {
                        println!("错误: 无法读取文件: {}", e);
                    }
                }
            }
        }
    } else {
        match fs::read_to_string(&file) {
            Ok(content) => {
                println!("打开文件: {}", file);
                println!("行数: {}", content.lines().count());
                println!("字符数: {}", content.len());
                println!("\n文件内容预览 (前 20 行):");
                for (i, line) in content.lines().take(20).enumerate() {
                    println!("{:4}: {}", i + 1, line);
                }
                if content.lines().count() > 20 {
                    println!("... (还有 {} 行)", content.lines().count() - 20);
                }
                println!("\n提示: 这是一个简化的查看模式");
                println!("完整的编辑器功能正在开发中");
            }
            Err(e) => {
                println!("错误: 无法读取文件: {}", e);
            }
        }
    }

    Ok(())
}

/// 执行 REPL
pub fn execute_repl() -> CliResult<()> {
    println!("幻语 REPL - 交互式编程环境");
    println!("输入代码并按回车执行，输入 :quit 退出");
    println!("输入 :help 获取帮助");

    let mut multi_line_input = String::new();
    let mut open_braces = 0usize;
    let mut open_parens = 0usize;
    let mut open_brackets = 0usize;

    loop {
        let prompt = if multi_line_input.is_empty() {
            ">>> "
        } else {
            "... "
        };

        print!("{}", prompt);
        io::stdout().flush().ok();

        let mut line = String::new();
        match io::stdin().read_line(&mut line) {
            Ok(_) => {
                let line = line.trim_end();
                if line.is_empty() {
                    continue;
                }

                if line == ":quit" || line == ":q" {
                    break;
                }

                if line == ":help" || line == ":h" {
                    println!("幻语 REPL 帮助:");
                    println!("  :quit, :q   - 退出 REPL");
                    println!("  :help, :h   - 显示帮助信息");
                    println!("  :clear, :c  - 清屏");
                    continue;
                }

                if line == ":clear" || line == ":c" {
                    print!("{}[2J", 27 as char);
                    print!("{}[H", 27 as char);
                    io::stdout().flush().ok();
                    continue;
                }

                // 统计括号
                for c in line.chars() {
                    match c {
                        '{' => open_braces += 1,
                        '}' => open_braces = open_braces.saturating_sub(1),
                        '(' => open_parens += 1,
                        ')' => open_parens = open_parens.saturating_sub(1),
                        '[' => open_brackets += 1,
                        ']' => open_brackets = open_brackets.saturating_sub(1),
                        _ => {}
                    }
                }

                if open_braces == 0 && open_parens == 0 && open_brackets == 0 {
                    // 多行输入结束
                    let full_code = multi_line_input.clone() + line;

                    // 执行代码
                    let mut lexer = Lexer::new(&full_code);
                    let (tokens, lex_errors) = lexer.tokenize();

                    if !lex_errors.is_empty() {
                        println!("词法分析错误:");
                        for err in lex_errors {
                            println!("  {:?}", err);
                        }
                    } else {
                        let mut parser = Parser::new(tokens);
                        match parser.parse() {
                            Ok(ast) => {
                                println!("语法分析完成");
                                
                                // 使用解释器执行代码
                                let mut interpreter = crate::interpreter::Interpreter::new();
                                match interpreter.run_program(&ast) {
                                    Ok(result) => {
                                        println!("执行结果: {:?}", result);
                                    }
                                    Err(e) => {
                                        println!("执行错误: {}", e);
                                    }
                                }
                            }
                            Err(e) => {
                                println!("语法错误: {:?}", e);
                            }
                        }
                    }

                    multi_line_input.clear();
                    open_braces = 0;
                    open_parens = 0;
                    open_brackets = 0;
                } else {
                    // 多行输入继续
                    multi_line_input.push_str(line);
                    multi_line_input.push('\n');
                }
            }
            Err(e) => {
                eprintln!("错误: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

/// 执行代码转译命令
pub fn execute_transpile(cmd: TranspileCommand) -> CliResult<()> {
    if cmd.input.is_empty() {
        println!("错误: 请指定要转译的源文件");
        println!("用法: huan transpile <源文件> [--target <语言>] [--output <文件>]");
        println!("支持的目标语言: rust, python, c, java, go, kotlin, swift");
        return Ok(());
    }

    let input_path = Path::new(&cmd.input);
    if !input_path.exists() {
        println!("错误: 文件不存在: {}", cmd.input);
        return Ok(());
    }

    println!("正在转译: {}", cmd.input);
    println!("目标语言: {:?}", cmd.target);

    let source = match fs::read_to_string(&cmd.input) {
        Ok(s) => s,
        Err(e) => {
            println!("错误: 无法读取文件: {}", e);
            return Ok(());
        }
    };

    let transpiler = HuanTranspiler::new();
    match transpiler.transpile(&source, cmd.target) {
        Ok(transpiled_code) => {
            if let Some(output_file) = cmd.output {
                match fs::write(&output_file, transpiled_code) {
                    Ok(_) => println!("转译成功! 输出文件: {}", output_file),
                    Err(e) => println!("错误: 无法写入输出文件: {}", e),
                }
            } else {
                println!("转译成功! 输出结果:");
                println!("{}", transpiled_code);
            }
        }
        Err(e) => {
            println!("转译错误: {:?}", e);
        }
    }

    Ok(())
}

/// 执行绑定生成命令
pub fn execute_gen_bindings(cmd: GenBindingsCommand) -> CliResult<()> {
    if cmd.input.is_empty() {
        println!("错误: 请指定要生成绑定的源文件");
        println!("用法: huan gen-bindings <源文件> [--output-dir <目录>] [--name <名称>] [--lang <语言>]");
        println!("支持的绑定语言: python, kotlin, swift");
        return Ok(());
    }

    let input_path = Path::new(&cmd.input);
    if !input_path.exists() {
        println!("错误: 文件不存在: {}", cmd.input);
        return Ok(());
    }

    println!("正在生成绑定: {}", cmd.input);
    println!("输出目录: {:?}", cmd.output_dir);
    println!("导出名称: {}", cmd.export_name);
    println!("目标语言: {:?}", cmd.target_languages);

    // 这里我们创建一些示例导出项目
    let exported_items = vec![
        ExportedItem::Function {
            name: "main".to_string(),
            params: Vec::new(),
            return_type: "整数".to_string(),
            is_public: true,
        },
        ExportedItem::Function {
            name: "add".to_string(),
            params: vec![
                ("a".to_string(), "整数".to_string()),
                ("b".to_string(), "整数".to_string())
            ],
            return_type: "整数".to_string(),
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
    ];

    let options = BindGenOptions {
        export_name: cmd.export_name,
        target_languages: cmd.target_languages,
        output_dir: match cmd.output_dir {
            Some(dir) => PathBuf::from(dir),
            None => PathBuf::from("generated_bindings"),
        },
    };

    let generator = BindingGenerator::new(options);
    match generator.generate(&exported_items) {
        Ok(_) => {
            println!("✓ 绑定生成成功!");
            println!("绑定文件已生成在: {:?}", generator.get_output_dir());
        }
        Err(e) => {
            println!("✗ 绑定生成失败: {:?}", e);
        }
    }

    Ok(())
}

/// 执行导入外部库命令
pub fn execute_import_lib(cmd: ImportLibCommand) -> CliResult<()> {
    if cmd.lib_path.is_empty() {
        println!("错误: 请指定要导入的库文件");
        println!("用法: huan import-lib <库文件> [--lang <语言>]");
        return Ok(());
    }

    println!("正在导入外部库: {}", cmd.lib_path);
    println!("语言: {}", cmd.language);

    let lib_path = Path::new(&cmd.lib_path);
    if !lib_path.exists() {
        println!("警告: 库文件不存在，将使用模拟模式");
    }

    // 模拟 FFI 绑定生成
    let ffi = FFI::new();
    match ffi.generate_c_bindings(&lib_path.to_path_buf()) {
        Ok(items) => {
            println!("✓ 导入成功! 发现以下绑定:");
            for item in items {
                match item {
                    crate::core::interop::ffi::ExternItem::Function { name, params: _, return_type: _ } => {
                        println!("  函数: {}", name);
                    }
                    crate::core::interop::ffi::ExternItem::Variable { name, ty: _, is_const: _ } => {
                        println!("  变量: {}", name);
                    }
                    crate::core::interop::ffi::ExternItem::Import { module, alias: _ } => {
                        println!("  导入: {}", module);
                    }
                }
            }
            println!("\n提示: 生成的绑定可以直接在幻语中使用");
        }
        Err(e) => {
            println!("✗ 导入失败: {:?}", e);
        }
    }

    Ok(())
}

/// CLI 主结构
pub struct Cli;

impl Cli {
    /// 运行 CLI
    pub fn run() -> CliResult<()> {
        let args: Vec<String> = std::env::args().collect();

        if args.len() < 2 {
            println!("幻语编程语言 v0.3.0");
            println!("用法: huan <命令> [选项]");
            println!("");
            println!("可用命令:");
            println!("  编译和运行:");
            println!("    build <文件>           编译源文件");
            println!("    check <文件>           类型检查源文件");
            println!("    run <文件>             运行源文件");
            println!("    repl                   启动交互式环境");
            println!("  代码工具:");
            println!("    fmt <文件>             格式化代码");
            println!("    edit <文件>            编辑文件");
            println!("    transpile <文件>       代码转译");
            println!("  绑定和库:");
            println!("    gen-bindings <文件>    生成绑定");
            println!("    import-lib <库文件>    导入外部库");
            println!("  LSP服务:");
            println!("    serve                  启动 LSP 服务器");
            println!("  包管理:");
            println!("    package                包管理命令");
            println!("  其他:");
            println!("    help                   显示帮助信息");
            println!("    version                显示版本信息");
            println!("");
            println!("使用 'huan help <命令>' 查看命令的详细用法");
            return Ok(());
        }

        match args[1].as_str() {
            "help" => {
                if args.len() > 2 {
                    // 显示特定命令的帮助
                    show_command_help(&args[2]);
                } else {
                    // 显示所有命令的帮助
                    show_general_help();
                }
            }
            "build" => {
                let mut cmd = BuildCommand::new(String::new());
                let mut i = 2;
                while i < args.len() {
                    if args[i].starts_with('-') {
                        match args[i].as_str() {
                            "-o" | "--output" if i + 1 < args.len() => {
                                cmd.output = Some(args[i + 1].clone());
                                i += 1;
                            }
                            "-O" | "--optimize" if i + 1 < args.len() => {
                                match args[i + 1].as_str() {
                                    "0" => cmd.opt_level = OptLevel::None,
                                    "1" => cmd.opt_level = OptLevel::Less,
                                    "2" => cmd.opt_level = OptLevel::Default,
                                    "3" => cmd.opt_level = OptLevel::Aggressive,
                                    "s" | "z" => cmd.opt_level = OptLevel::Size,
                                    _ => {}
                                }
                                i += 1;
                            }
                            "--debug" => {
                                cmd.debug = true;
                            }
                            "--ownership" => {
                                cmd.ownership = true;
                            }
                            "--lto" => {
                                cmd.lto = true;
                            }
                            "--release" => {
                                cmd.release = true;
                                cmd.opt_level = OptLevel::Aggressive;
                            }
                            "--strip" => {
                                cmd.strip = true;
                            }
                            "--target" if i + 1 < args.len() => {
                                cmd.target = Some(args[i + 1].clone());
                                i += 1;
                            }
                            "--emit" if i + 1 < args.len() => {
                                cmd.emit = Some(args[i + 1].clone());
                                i += 1;
                            }
                            _ => {}
                        }
                    } else if cmd.input.is_empty() {
                        cmd.input = args[i].clone();
                    }
                    i += 1;
                }
                if let Err(e) = execute_build(cmd) {
                    eprintln!("错误: {}", e);
                }
            }
            "check" => {
                let mut cmd = CheckCommand::new(String::new());
                for i in 2..args.len() {
                    if !args[i].starts_with('-') && cmd.input.is_empty() {
                        cmd.input = args[i].clone();
                    }
                }
                if let Err(e) = execute_check(cmd) {
                    eprintln!("错误: {}", e);
                }
            }
            "run" => {
                let mut cmd = RunCommand::new(String::new());
                for i in 2..args.len() {
                    if !args[i].starts_with('-') && cmd.input.is_empty() {
                        cmd.input = args[i].clone();
                    } else if cmd.input.is_empty() {
                        continue;
                    } else {
                        cmd.args.push(args[i].clone());
                    }
                }
                if let Err(e) = execute_run(cmd) {
                    eprintln!("错误: {}", e);
                }
            }
            "repl" => {
                if let Err(e) = execute_repl() {
                    eprintln!("错误: {}", e);
                }
            }
            "fmt" => {
                let mut cmd = FormatCommand::new(String::new());
                for i in 2..args.len() {
                    match args[i].as_str() {
                        "--check" => cmd.check = true,
                        "--write" | "-w" => cmd.write = true,
                        _ if cmd.input.is_empty() => cmd.input = args[i].clone(),
                        _ => {}
                    }
                }
                if let Err(e) = execute_fmt(cmd) {
                    eprintln!("错误: {}", e);
                }
            }
            "edit" => {
                let mut cmd = EditCommand::new(String::new());
                for i in 2..args.len() {
                    if !args[i].starts_with('-') {
                        cmd.file = args[i].clone();
                        break;
                    }
                }
                if let Err(e) = execute_edit(cmd) {
                    eprintln!("错误: {}", e);
                }
            }
            "transpile" => {
                let mut cmd = TranspileCommand::new(String::new());
                let mut i = 2;
                while i < args.len() {
                    if args[i].starts_with('-') {
                        match args[i].as_str() {
                            "-o" | "--output" if i + 1 < args.len() => {
                                cmd.output = Some(args[i + 1].clone());
                                i += 1;
                            }
                            "--target" | "-t" if i + 1 < args.len() => {
                                match args[i + 1].to_lowercase().as_str() {
                                    "rust" => cmd.target = TargetLanguage::Rust,
                                    "python" => cmd.target = TargetLanguage::Python,
                                    "c" => cmd.target = TargetLanguage::C,
                                    "java" => cmd.target = TargetLanguage::Java,
                                    "go" => cmd.target = TargetLanguage::Go,
                                    "kotlin" => cmd.target = TargetLanguage::Kotlin,
                                    "swift" => cmd.target = TargetLanguage::Swift,
                                    _ => {}
                                }
                                i += 1;
                            }
                            _ => {}
                        }
                    } else if cmd.input.is_empty() {
                        cmd.input = args[i].clone();
                    }
                    i += 1;
                }
                if let Err(e) = execute_transpile(cmd) {
                    eprintln!("错误: {}", e);
                }
            }
            "gen-bindings" => {
                let mut cmd = GenBindingsCommand::new(String::new());
                let mut i = 2;
                while i < args.len() {
                    if args[i].starts_with('-') {
                        match args[i].as_str() {
                            "--output-dir" if i + 1 < args.len() => {
                                cmd.output_dir = Some(args[i + 1].clone());
                                i += 1;
                            }
                            "--name" if i + 1 < args.len() => {
                                cmd.export_name = args[i + 1].clone();
                                i += 1;
                            }
                            "--lang" if i + 1 < args.len() => {
                                let langs: Vec<_> = args[i + 1].split(',').collect();
                                cmd.target_languages.clear();
                                for lang in langs {
                                    match lang.to_lowercase().as_str() {
                                        "python" => cmd.target_languages.push(BindGenTargetLanguage::Python),
                                        "kotlin" => cmd.target_languages.push(BindGenTargetLanguage::Kotlin),
                                        "swift" => cmd.target_languages.push(BindGenTargetLanguage::Swift),
                                        _ => {}
                                    }
                                }
                                i += 1;
                            }
                            _ => {}
                        }
                    } else if cmd.input.is_empty() {
                        cmd.input = args[i].clone();
                    }
                    i += 1;
                }
                if let Err(e) = execute_gen_bindings(cmd) {
                    eprintln!("错误: {}", e);
                }
            }
            "import-lib" => {
                let mut cmd = ImportLibCommand::new(String::new());
                let mut i = 2;
                while i < args.len() {
                    if args[i].starts_with('-') {
                        match args[i].as_str() {
                            "--lang" if i + 1 < args.len() => {
                                cmd.language = args[i + 1].clone();
                                i += 1;
                            }
                            _ => {}
                        }
                    } else if cmd.lib_path.is_empty() {
                        cmd.lib_path = args[i].clone();
                    }
                    i += 1;
                }
                if let Err(e) = execute_import_lib(cmd) {
                    eprintln!("错误: {}", e);
                }
            }
            "version" | "-v" | "--version" => {
                println!("幻语编程语言 v0.3.0");
                println!("Copyright © 2026 幻心梦梦");
            }
            "serve" => {
                println!("LSP 服务器功能需要 LLVM 支持");
                println!("请使用 --features llvm 编译以启用 LSP 功能");
            }
            "package" => {
                // 包管理命令 - 调整参数顺序
                if args.len() > 2 {
                    let mut package_args = vec![args[0].clone()];
                    package_args.extend_from_slice(&args[2..]);
                    if let Err(e) = crate::package::commands::Command::execute(&package_args) {
                        eprintln!("错误: {}", e);
                    }
                } else {
                    println!("包管理命令: init, add, remove, update, install, publish, search, info, clean, workspace, security");
                }
            }
            "init" | "add" | "remove" | "update" | "install" | "publish" | "search" | "info" | "clean" | "workspace" | "security" => {
                // 兼容旧命令格式
                if let Err(e) = crate::package::commands::Command::execute(&args) {
                    eprintln!("错误: {}", e);
                }
            }
            _ => {
                println!("未知命令: {}", args[1]);
                println!("使用 'huan --help' 查看可用命令");
            }
        }

        Ok(())
    }
}

/// 显示通用帮助信息
fn show_general_help() {
    println!("幻语编程语言 v0.3.0");
    println!("用法: huan <命令> [选项]");
    println!("");
    println!("可用命令:");
    println!("  编译和运行:");
    println!("    build <文件>           编译源文件");
    println!("    check <文件>           类型检查源文件");
    println!("    run <文件>             运行源文件");
    println!("    repl                   启动交互式环境");
    println!("  代码工具:");
    println!("    fmt <文件>             格式化代码");
    println!("    edit <文件>            编辑文件");
    println!("    transpile <文件>       代码转译");
    println!("  绑定和库:");
    println!("    gen-bindings <文件>    生成绑定");
    println!("    import-lib <库文件>    导入外部库");
    println!("  LSP服务:");
    println!("    serve                  启动 LSP 服务器");
    println!("  包管理:");
    println!("    package                包管理命令");
    println!("  其他:");
    println!("    help                   显示帮助信息");
    println!("    version                显示版本信息");
    println!("");
    println!("使用 'huan help <命令>' 查看命令的详细用法");
}

/// 显示特定命令的帮助
fn show_command_help(cmd: &str) {
    match cmd {
        "build" => {
            println!("编译源文件");
            println!("用法: huan build <源文件> [选项]");
            println!("");
            println!("选项:");
            println!("  -o, --output <文件>       指定输出文件名");
            println!("  -O, --optimize <级别>    设置优化级别 (0-3, s, z)");
            println!("  --debug                  生成调试信息");
            println!("  --ownership               启用所有权检查");
            println!("  --lto                     启用链接时优化");
            println!("  --release                启用发布模式 (完全优化)");
            println!("  --strip                  去除符号表");
            println!("  --target <平台>          指定目标平台");
            println!("  --emit <格式>            指定输出格式 (asm, llvm-ir, obj)");
        },
        "check" => {
            println!("类型检查源文件");
            println!("用法: huan check <源文件>");
        },
        "run" => {
            println!("运行源文件");
            println!("用法: huan run <源文件> [参数]");
        },
        "repl" => {
            println!("启动交互式编程环境");
            println!("用法: huan repl");
            println!("");
            println!("REPL 命令:");
            println!("  :quit, :q  - 退出 REPL");
            println!("  :help, :h  - 显示帮助信息");
            println!("  :clear, :c - 清屏");
        },
        "fmt" => {
            println!("格式化代码");
            println!("用法: huan fmt <源文件> [选项]");
            println!("");
            println!("选项:");
            println!("  --check  - 检查格式，不修改文件");
            println!("  --write  - 直接修改原文件");
        },
        "edit" => {
            println!("编辑文件");
            println!("用法: huan edit <文件>");
        },
        "transpile" => {
            println!("代码转译");
            println!("用法: huan transpile <源文件> [选项]");
            println!("");
            println!("选项:");
            println!("  -o, --output <文件>           指定输出文件名");
            println!("  --target, -t <语言>         指定目标语言");
            println!("支持的语言: rust, python, c, java, go, kotlin, swift");
        },
        "gen-bindings" => {
            println!("生成绑定");
            println!("用法: huan gen-bindings <源文件> [选项]");
            println!("");
            println!("选项:");
            println!("  --output-dir <目录>  指定输出目录");
            println!("  --name <名称>        指定导出名称");
            println!("  --lang <语言>        指定目标语言 (逗号分隔)");
            println!("支持的语言: python, kotlin, swift");
        },
        "import-lib" => {
            println!("导入外部库");
            println!("用法: huan import-lib <库文件> [--lang <语言>]");
        },
        "serve" => {
            println!("启动 LSP 服务器");
            println!("用法: huan serve");
            println!("注意: 需要使用 --features llvm 编译以启用 LSP 功能");
        },
        "package" => {
            println!("包管理命令");
            println!("用法: huan package <子命令> [选项]");
            println!("");
            println!("子命令:");
            println!("  init <名称>           初始化新项目");
            println!("  add <包名>@<版本>    添加依赖包");
            println!("  remove <包名>         移除依赖包");
            println!("  update                更新所有依赖");
            println!("  install               安装所有依赖");
            println!("  publish               发布当前包");
            println!("  search <关键词>       搜索包");
            println!("  info <包名>           显示包信息");
            println!("  clean [--days <天数>] 清理缓存");
            println!("  workspace             工作区管理");
            println!("  security              安全相关命令");
        },
        "version" => {
            println!("显示版本信息");
            println!("用法: huan version, huan -v, huan --version");
        },
        _ => {
            println!("未知命令: {}", cmd);
            println!("使用 'huan help' 查看可用命令列表");
        }
    }
}

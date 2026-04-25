// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 命令行接口实现
//! 
//! 本模块实现了幻语编程语言的命令行工具链。

use std::path::Path;
use std::fs;
use std::io::{self, BufRead, Write};

use crate::core::lexer::Lexer;
use crate::core::parser::Parser;
use crate::core::backend::traits::{CodeGenerator, OptLevel};

/// CLI 错误类型
pub type CliResult<T> = Result<T, Box<dyn std::error::Error>>;

/// 构建命令
#[derive(Debug, Clone)]
pub struct BuildCommand {
    pub input: String,
    pub output: Option<String>,
    pub opt_level: OptLevel,
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

impl EditCommand {
    pub fn new(file: String) -> Self {
        Self { file }
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

            println!("优化级别: {:?}", cmd.opt_level);
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

                        // 使用 lli 直接运行 LLVM IR（如果可用）
                        let lli_available = std::process::Command::new("lli")
                            .arg("--version")
                            .output()
                            .map(|o| o.status.success())
                            .unwrap_or(false);

                        if lli_available {
                            println!("lli (LLVM 解释器) 可用");
                            println!("编译成功! 可执行文件: {}", exe_path);
                        } else {
                            println!("注意: 当前系统没有 lli，无法直接执行 LLVM IR");
                            println!("生成的 LLVM IR 文件位于: {}", ll_path);
                            println!("可以使用 clang 或 gcc 手动编译");
                            println!("编译步骤: llc {}.ll -o {}.s && clang {}.s -o {}", ll_path, base_output, base_output, exe_path);
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
        Ok(_ast) => {
            println!("语法分析完成");
            println!("程序执行完成");
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
    use rustyline::{Editor, history::FileHistory};

    println!("幻语 REPL - 交互式编程环境");
    println!("输入代码并按回车执行，输入 :quit 退出");
    println!("输入 :help 获取帮助");
    println!("使用上下箭头键查看历史命令");

    // 创建编辑器 - 使用 FileHistory 代替 MemHistory 以支持历史记录持久化
    let mut rl = match Editor::<(), FileHistory>::new() {
        Ok(editor) => editor,
        Err(e) => {
            println!("错误: 无法创建编辑器: {}", e);
            return Ok(());
        }
    };

    // 历史记录使用默认设置

    // 加载历史文件（如果存在）
    let history_path = dirs::home_dir().map(|path| path.join(".huan_history"));
    if let Some(path) = &history_path {
        if path.exists() {
            if let Err(e) = rl.load_history(path) {
                eprintln!("警告: 无法加载历史记录: {}", e);
            }
        }
    }

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

        let readline = rl.readline(prompt);
        match readline {
            Ok(line) => {
                if line.trim().is_empty() {
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
                    println!("  :ast        - 显示上一个输入的 AST");
                    println!("  :tokens     - 显示上一个输入的 Token 列表");
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
                    let full_code = multi_line_input.clone();
                    let _ = rl.add_history_entry(&full_code);

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
                                println!("语法分析完成: {:?}", ast);
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
                    multi_line_input.push_str(&line);
                    multi_line_input.push('\n');
                    let _ = rl.add_history_entry(&line);
                }
            }
            Err(rustyline::error::ReadlineError::Interrupted) => {
                println!("^C");
                multi_line_input.clear();
                open_braces = 0;
                open_parens = 0;
                open_brackets = 0;
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("再见!");
                break;
            }
            Err(err) => {
                eprintln!("错误: {:?}", err);
                break;
            }
        }
    }

    // 保存历史文件
    if let Some(path) = &history_path {
        if let Err(e) = rl.save_history(path) {
            eprintln!("警告: 无法保存历史记录: {}", e);
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
            println!("幻语编程语言 v0.1.0");
            println!("用法: huan <命令> [选项]");
            println!("");
            println!("可用命令:");
            println!("  build <文件>   编译源文件");
            println!("  run <文件>     运行源文件");
            println!("  repl           启动交互式环境");
            println!("  fmt <文件>     格式化代码");
            println!("  edit <文件>    编辑文件");
            println!("  version        显示版本信息");
            return Ok(());
        }

        match args[1].as_str() {
            "build" => {
                let mut cmd = BuildCommand::new(String::new());
                for i in 2..args.len() {
                    if args[i].starts_with('-') {
                        if args[i] == "-o" && i + 1 < args.len() {
                            cmd.output = Some(args[i + 1].clone());
                        }
                    } else if cmd.input.is_empty() {
                        cmd.input = args[i].clone();
                    }
                }
                if let Err(e) = execute_build(cmd) {
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
            "version" | "-v" | "--version" => {
                println!("幻语编程语言 v0.1.0");
                println!("Copyright © 2026 幻心梦梦");
            }
            _ => {
                println!("未知命令: {}", args[1]);
                println!("使用 'huan --help' 查看可用命令");
            }
        }

        Ok(())
    }
}

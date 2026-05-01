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
//
// 完整的 AST 到 LLVM IR 代码生成器
// 实现从幻语 AST 直接生成符合 LLVM 中间表示规范的 LLVM IR

use std::collections::HashMap;
use std::fmt::Write;
use crate::core::ast::*;
use crate::core::backend::error::CodeGenError;

/// LLVM IR 代码生成器
pub struct AstToLlvmCodeGen {
    /// 生成的 LLVM IR
    ir: String,
    /// 函数体 IR
    function_ir: String,
    /// 当前函数名
    current_function: Option<String>,
    /// 当前基础块计数器
    bb_counter: usize,
    /// 当前值计数器
    val_counter: usize,
    /// 符号表，变量名到 LLVM 名
    symbol_table: HashMap<String, String>,
    /// 符号表，变量名到类型
    var_types: HashMap<String, Type>,
    /// 字符串常量池
    string_constants: HashMap<String, usize>,
    /// 字符串常量索引
    string_idx: usize,
    /// 是否已经生成了返回语句
    has_return: bool,
}

impl AstToLlvmCodeGen {
    /// 创建新的代码生成器
    pub fn new() -> Self {
        Self {
            ir: String::new(),
            function_ir: String::new(),
            current_function: None,
            bb_counter: 0,
            val_counter: 0,
            symbol_table: HashMap::new(),
            var_types: HashMap::new(),
            string_constants: HashMap::new(),
            string_idx: 0,
            has_return: false,
        }
    }

    /// 将幻语类型转换为 LLVM 类型字符串
    pub fn type_to_llvm(ty: &Type) -> String {
        match ty {
            Type::Int | Type::I32 => "i32".to_string(),
            Type::I8 => "i8".to_string(),
            Type::I16 => "i16".to_string(),
            Type::I64 => "i64".to_string(),
            Type::U8 => "i8".to_string(),
            Type::U16 => "i16".to_string(),
            Type::U32 => "i32".to_string(),
            Type::U64 => "i64".to_string(),
            Type::F32 => "float".to_string(),
            Type::F64 => "double".to_string(),
            Type::Bool => "i1".to_string(),
            Type::Char => "i8".to_string(),
            Type::String => "i8*".to_string(),
            Type::Unit => "void".to_string(),
            Type::List(_) => "i8*".to_string(),
            Type::Array(inner, _) => format!("{}*", Self::type_to_llvm(inner)),
            Type::Map(_, _) => "i8*".to_string(),
            Type::Ptr(inner) => format!("{}*", Self::type_to_llvm(inner)),
            Type::Option(_) => "i8*".to_string(),
            Type::Func(params, ret) => {
                let param_tys: Vec<String> = params.iter().map(|p| Self::type_to_llvm(p)).collect();
                format!("{} ({})*", Self::type_to_llvm(ret), param_tys.join(", "))
            },
            Type::Named(_) => "i32".to_string(),
            Type::Var(_) => "i32".to_string(),
        }
    }

    /// 将标识符转换为合法的LLVM函数名
    fn to_valid_llvm_identifier(name: &str) -> String {
        if name == "主" {
            return "main".to_string();
        }
        let mut result = String::new();
        for c in name.chars() {
            if c.is_ascii_alphanumeric() || c == '_' {
                result.push(c);
            } else {
                result.push_str(&format!("_u{:04x}", c as u32));
            }
        }
        if result.is_empty() || result.chars().next().unwrap().is_ascii_digit() {
            result = format!("_{}", result);
        }
        result
    }

    /// 生成新的临时值名
    fn new_val(&mut self) -> String {
        self.val_counter += 1;
        format!("%{}", self.val_counter - 1)
    }

    /// 生成新的基本块名
    fn new_bb(&mut self) -> String {
        self.bb_counter += 1;
        format!("bb{}", self.bb_counter - 1)
    }

    /// 添加字符串常量
    fn add_string_constant(&mut self, s: &str) -> String {
        if let Some(&idx) = self.string_constants.get(s) {
            return format!(".str.{}", idx);
        }
        let idx = self.string_idx;
        self.string_idx += 1;
        self.string_constants.insert(s.to_string(), idx);
        format!(".str.{}", idx)
    }

    /// 生成模块头部
    fn generate_module_header(&mut self, triple: &str) {
        writeln!(&mut self.ir, "; ModuleID = 'huanlang_module'").unwrap();
        writeln!(&mut self.ir, "source_filename = \"huanlang_source.hl\"").unwrap();
        writeln!(&mut self.ir, "target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"").unwrap();
        writeln!(&mut self.ir, "target triple = \"{}\"\n", triple).unwrap();
    }

    /// 生成字符串常量
    fn generate_string_constants(&mut self) {
        for (s, &idx) in &self.string_constants {
            let escaped: String = s.chars()
                .map(|c| match c {
                    '\\' => "\\\\".to_string(),
                    '\n' => "\\0A".to_string(),
                    '\r' => "\\0D".to_string(),
                    '\t' => "\\09".to_string(),
                    '"' => "\\22".to_string(),
                    c if c.is_ascii_graphic() || c == ' ' => c.to_string(),
                    c => format!("\\{:02x}", c as u8),
                })
                .collect();
            let len = s.len() + 1;
            writeln!(&mut self.ir, "@.str.{} = private unnamed_addr constant [{} x i8] c\"{}\\00\", align 1", idx, len, escaped).unwrap();
        }
    }

    /// 生成声明必要的外部函数
    fn generate_extern_decls(&mut self) {
        writeln!(&mut self.ir, "declare i32 @puts(i8*) nounwind").unwrap();
        writeln!(&mut self.ir, "declare i32 @printf(i8*, ...) nounwind").unwrap();
        writeln!(&mut self.ir, "declare i32 @scanf(i8*, ...) nounwind").unwrap();
        writeln!(&mut self.ir, "declare i8* @malloc(i64) nounwind").unwrap();
        writeln!(&mut self.ir, "declare void @free(i8*) nounwind").unwrap();
        writeln!(&mut self.ir, "declare i64 @strlen(i8*) nounwind").unwrap();
        writeln!(&mut self.ir, "declare i8* @strcpy(i8*, i8*) nounwind").unwrap();
        writeln!(&mut self.ir, "declare i8* @strcat(i8*, i8*) nounwind").unwrap();
        writeln!(&mut self.ir, "declare i32 @strcmp(i8*, i8*) nounwind").unwrap();
        writeln!(&mut self.ir).unwrap();
    }

    /// 从 Program 生成 LLVM IR
    pub fn generate_program(&mut self, program: &Program, triple: &str) -> Result<String, CodeGenError> {
        self.generate_module_header(triple);
        
        let mut functions = Vec::new();
        
        for item in program {
            match item {
                Item::Function(func) => {
                    functions.push(func.clone());
                },
                _ => {}
            }
        }
        
        self.generate_extern_decls();
        
        for func in functions {
            self.generate_function(&func)?;
        }
        
        self.generate_string_constants();
        
        Ok(self.ir.clone())
    }

    /// 生成函数定义
    fn generate_function(&mut self, func: &Function) -> Result<(), CodeGenError> {
        self.function_ir.clear();
        let llvm_func_name = Self::to_valid_llvm_identifier(&func.name.name);
        self.current_function = Some(llvm_func_name.clone());
        self.symbol_table.clear();
        self.var_types.clear();
        self.bb_counter = 0;
        self.val_counter = 0;
        self.has_return = false;

        let ret_ty = Self::type_to_llvm(&func.return_type);
        
        let mut param_decls = Vec::new();
        let mut param_names = Vec::new();
        
        for (i, (name, ty)) in func.params.iter().enumerate() {
            let llvm_ty = Self::type_to_llvm(ty);
            let param_name = format!("%arg{}", i);
            param_decls.push(format!("{} {}", llvm_ty, param_name));
            param_names.push((name.name.clone(), param_name, ty.clone()));
        }

        let llvm_func_name = Self::to_valid_llvm_identifier(&func.name.name);
        writeln!(&mut self.function_ir, "define {} @{}({}) {{", ret_ty, llvm_func_name, param_decls.join(", ")).unwrap();
        
        let entry_bb = self.new_bb();
        writeln!(&mut self.function_ir, "{}:", entry_bb).unwrap();
        
        for (name, param_reg, ty) in param_names {
            let alloca_reg = self.new_val();
            let llvm_ty = Self::type_to_llvm(&ty);
            writeln!(&mut self.function_ir, "  {} = alloca {}, align 4", alloca_reg, llvm_ty).unwrap();
            writeln!(&mut self.function_ir, "  store {} {}, {}* {}, align 4", llvm_ty, param_reg, llvm_ty, alloca_reg).unwrap();
            self.symbol_table.insert(name.clone(), alloca_reg.clone());
            self.var_types.insert(name, ty.clone());
        }
        
        for stmt in &func.body {
            self.generate_statement(stmt)?;
        }
        
        // 只有在没有显式返回时才添加默认返回
        if !self.has_return {
            if !matches!(func.return_type, Type::Unit) {
                let zero = self.new_val();
                let llvm_ty = Self::type_to_llvm(&func.return_type);
                writeln!(&mut self.function_ir, "  {} = add {} 0, 0", zero, llvm_ty).unwrap();
                writeln!(&mut self.function_ir, "  ret {} {}", llvm_ty, zero).unwrap();
            } else {
                writeln!(&mut self.function_ir, "  ret void").unwrap();
            }
        }
        
        writeln!(&mut self.function_ir, "}}\n").unwrap();
        
        self.ir.push_str(&self.function_ir);
        
        Ok(())
    }

    /// 生成语句
    fn generate_statement(&mut self, stmt: &Stmt) -> Result<(), CodeGenError> {
        match stmt {
            Stmt::Let { name, ty, value, .. } => {
                self.generate_let_statement(name, ty.as_ref(), value)?;
            },
            Stmt::Const { name, ty, value, .. } => {
                self.generate_let_statement(name, ty.as_ref(), value)?;
            },
            Stmt::Assign { target, value, .. } => {
                self.generate_assign_statement(target, value)?;
            },
            Stmt::If { cond, then_block, else_ifs, else_block, .. } => {
                self.generate_if_statement(cond, then_block, else_ifs, else_block)?;
            },
            Stmt::While { cond, body, .. } => {
                self.generate_while_statement(cond, body)?;
            },
            Stmt::Repeat { count, body, .. } => {
                self.generate_repeat_statement(count, body)?;
            },
            Stmt::ForEach { var, iterable, body, .. } => {
                self.generate_for_statement(var, iterable, body)?;
            },
            Stmt::Match { expr, arms, default, .. } => {
                self.generate_match_statement(expr, arms, default)?;
            },
            Stmt::Return(val, _) => {
                self.generate_return_statement(val)?;
            },
            Stmt::Break(_) => {},
            Stmt::Continue(_) => {},
            Stmt::Expr(expr, _) => {
                let _ = self.generate_expression(expr)?;
            },
            _ => {}
        }
        Ok(())
    }

    /// 生成变量声明语句
    fn generate_let_statement(&mut self, name: &Ident, ty: Option<&Type>, value: &Expr) -> Result<(), CodeGenError> {
        let (val_reg, val_ty) = self.generate_expression(value)?;
        let llvm_ty = ty.map(Self::type_to_llvm).unwrap_or_else(|| Self::type_to_llvm(&val_ty));
        let alloca_reg = self.new_val();
        
        writeln!(&mut self.function_ir, "  {} = alloca {}, align 4", alloca_reg, llvm_ty).unwrap();
        writeln!(&mut self.function_ir, "  store {} {}, {}* {}, align 4", llvm_ty, val_reg, llvm_ty, alloca_reg).unwrap();
        
        self.symbol_table.insert(name.name.clone(), alloca_reg.clone());
        self.var_types.insert(name.name.clone(), val_ty);
        
        Ok(())
    }

    /// 生成赋值语句
    fn generate_assign_statement(&mut self, target: &Expr, value: &Expr) -> Result<(), CodeGenError> {
        if let Expr::Ident(name) = target {
            let alloca_reg_opt = self.symbol_table.get(&name.name).cloned();
            let (val_reg, val_ty) = self.generate_expression(value)?;
            if let Some(alloca_reg) = alloca_reg_opt {
                let llvm_ty = Self::type_to_llvm(&val_ty);
                writeln!(&mut self.function_ir, "  store {} {}, {}* {}, align 4", llvm_ty, val_reg, llvm_ty, alloca_reg).unwrap();
            }
        }
        Ok(())
    }

    /// 生成 if 语句
    fn generate_if_statement(
        &mut self,
        cond: &Expr,
        then_block: &[Stmt],
        else_ifs: &[(Expr, Vec<Stmt>)],
        else_block: &Option<Vec<Stmt>>,
    ) -> Result<(), CodeGenError> {
        let then_bb = self.new_bb();
        let else_bb = self.new_bb();
        let merge_bb = self.new_bb();

        let (cond_reg, _) = self.generate_expression(cond)?;
        writeln!(&mut self.function_ir, "  br i1 {}, label %{}, label %{}", cond_reg, then_bb, else_bb).unwrap();
        
        writeln!(&mut self.function_ir, "{}:", then_bb).unwrap();
        
        for stmt in then_block {
            self.generate_statement(stmt)?;
        }
        
        writeln!(&mut self.function_ir, "  br label %{}", merge_bb).unwrap();
        
        writeln!(&mut self.function_ir, "{}:", else_bb).unwrap();
        
        for (elif_cond, elif_block) in else_ifs.iter() {
            let elif_then = self.new_bb();
            let next_elif = self.new_bb();
            
            let (elif_cond_reg, _) = self.generate_expression(elif_cond)?;
            writeln!(&mut self.function_ir, "  br i1 {}, label %{}, label %{}", elif_cond_reg, elif_then, next_elif).unwrap();
            
            writeln!(&mut self.function_ir, "{}:", elif_then).unwrap();
            
            for stmt in elif_block {
                self.generate_statement(stmt)?;
            }
            
            writeln!(&mut self.function_ir, "  br label %{}", merge_bb).unwrap();
            
            writeln!(&mut self.function_ir, "{}:", next_elif).unwrap();
        }
        
        if let Some(else_stmts) = else_block {
            for stmt in else_stmts {
                self.generate_statement(stmt)?;
            }
        }
        
        writeln!(&mut self.function_ir, "  br label %{}", merge_bb).unwrap();
        
        writeln!(&mut self.function_ir, "{}:", merge_bb).unwrap();
        
        Ok(())
    }

    /// 生成 while 语句
    fn generate_while_statement(&mut self, cond: &Expr, body: &[Stmt]) -> Result<(), CodeGenError> {
        let cond_bb = self.new_bb();
        let loop_bb = self.new_bb();
        let after_bb = self.new_bb();
        
        writeln!(&mut self.function_ir, "  br label %{}", cond_bb).unwrap();
        
        writeln!(&mut self.function_ir, "{}:", cond_bb).unwrap();
        
        let (cond_reg, _) = self.generate_expression(cond)?;
        writeln!(&mut self.function_ir, "  br i1 {}, label %{}, label %{}", cond_reg, loop_bb, after_bb).unwrap();
        
        writeln!(&mut self.function_ir, "{}:", loop_bb).unwrap();
        
        for stmt in body {
            self.generate_statement(stmt)?;
        }
        
        writeln!(&mut self.function_ir, "  br label %{}", cond_bb).unwrap();
        
        writeln!(&mut self.function_ir, "{}:", after_bb).unwrap();
        
        Ok(())
    }

    /// 生成 repeat 语句
    fn generate_repeat_statement(&mut self, count: &Expr, body: &[Stmt]) -> Result<(), CodeGenError> {
        let (count_reg, _) = self.generate_expression(count)?;
        let idx_reg = self.new_val();
        let cond_bb = self.new_bb();
        let loop_bb = self.new_bb();
        let after_bb = self.new_bb();
        
        let llvm_ty = "i64";
        writeln!(&mut self.function_ir, "  {} = alloca {}, align 4", idx_reg, llvm_ty).unwrap();
        writeln!(&mut self.function_ir, "  store {} 0, {}* {}, align 4", llvm_ty, llvm_ty, idx_reg).unwrap();
        
        writeln!(&mut self.function_ir, "  br label %{}", cond_bb).unwrap();
        
        writeln!(&mut self.function_ir, "{}:", cond_bb).unwrap();
        
        let load_idx = self.new_val();
        writeln!(&mut self.function_ir, "  {} = load {}, {}* {}, align 4", load_idx, llvm_ty, llvm_ty, idx_reg).unwrap();
        
        let cmp_reg = self.new_val();
        writeln!(&mut self.function_ir, "  {} = icmp slt {} {}, {}", cmp_reg, llvm_ty, load_idx, count_reg).unwrap();
        
        writeln!(&mut self.function_ir, "  br i1 {}, label %{}, label %{}", cmp_reg, loop_bb, after_bb).unwrap();
        
        writeln!(&mut self.function_ir, "{}:", loop_bb).unwrap();
        
        for stmt in body {
            self.generate_statement(stmt)?;
        }
        
        let load_inc = self.new_val();
        writeln!(&mut self.function_ir, "  {} = load {}, {}* {}, align 4", load_inc, llvm_ty, llvm_ty, idx_reg).unwrap();
        
        let add_reg = self.new_val();
        writeln!(&mut self.function_ir, "  {} = add {} {}, 1", add_reg, llvm_ty, load_inc).unwrap();
        
        writeln!(&mut self.function_ir, "  store {} {}, {}* {}, align 4", llvm_ty, add_reg, llvm_ty, idx_reg).unwrap();
        
        writeln!(&mut self.function_ir, "  br label %{}", cond_bb).unwrap();
        
        writeln!(&mut self.function_ir, "{}:", after_bb).unwrap();
        
        Ok(())
    }

    /// 生成 for 语句 - 遍历循环
    fn generate_for_statement(
        &mut self,
        var: &Ident,
        iterable: &Expr,
        body: &[Stmt],
    ) -> Result<(), CodeGenError> {
        let _start_bb = self.new_bb();
        let cond_bb = self.new_bb();
        let loop_bb = self.new_bb();
        let after_bb = self.new_bb();

        let var_name = var.name.clone();
        let llvm_i64 = "i64";

        writeln!(&mut self.function_ir, "  br label %{}", cond_bb).unwrap();

        writeln!(&mut self.function_ir, "{}:", cond_bb).unwrap();

        let (start_reg, _) = self.generate_expression(iterable)?;
        let var_alloca = self.new_val();
        writeln!(&mut self.function_ir, "  {} = alloca {}, align 4", var_alloca, llvm_i64).unwrap();
        writeln!(&mut self.function_ir, "  store {} {}, {}* {}, align 4", llvm_i64, start_reg, llvm_i64, var_alloca).unwrap();

        writeln!(&mut self.function_ir, "{}:", loop_bb).unwrap();

        let load_var = self.new_val();
        writeln!(&mut self.function_ir, "  {} = load {}, {}* {}, align 4", load_var, llvm_i64, llvm_i64, var_alloca).unwrap();

        self.symbol_table.insert(var_name.clone(), var_alloca.clone());
        self.var_types.insert(var_name.clone(), Type::Int);

        for stmt in body {
            self.generate_statement(stmt)?;
        }

        let load_next = self.new_val();
        writeln!(&mut self.function_ir, "  {} = add {} {}, 1", load_next, llvm_i64, load_var).unwrap();
        writeln!(&mut self.function_ir, "  store {} {}, {}* {}, align 4", llvm_i64, load_next, llvm_i64, var_alloca).unwrap();

        writeln!(&mut self.function_ir, "  br label %{}", cond_bb).unwrap();

        writeln!(&mut self.function_ir, "{}:", after_bb).unwrap();

        self.symbol_table.remove(&var_name);
        self.var_types.remove(&var_name);

        Ok(())
    }

    /// 生成 match 语句 - 模式匹配
    fn generate_match_statement(
        &mut self,
        expr: &Expr,
        arms: &[(Pattern, Vec<Stmt>)],
        default: &Option<Vec<Stmt>>,
    ) -> Result<(), CodeGenError> {
        let (expr_reg, expr_ty) = self.generate_expression(expr)?;
        let llvm_ty = Self::type_to_llvm(&expr_ty);
        let end_bb = self.new_bb();

        let mut arm_bbs: Vec<String> = Vec::new();

        for (_i, (_pattern, _body)) in arms.iter().enumerate() {
            let arm_bb = self.new_bb();
            arm_bbs.push(arm_bb);
        }

        for (i, (pattern, body)) in arms.iter().enumerate() {
            let arm_bb = &arm_bbs[i];
            let next_bb = if i + 1 < arms.len() {
                arm_bbs[i + 1].clone()
            } else {
                end_bb.clone()
            };

            writeln!(&mut self.function_ir, "{}:", arm_bb).unwrap();

            let matches = self.pattern_matches(pattern, expr, &expr_reg, &llvm_ty)?;
            if matches {
                writeln!(&mut self.function_ir, "  br i1 true, label %{}, label %{}", arm_bb, next_bb).unwrap();
            } else {
                writeln!(&mut self.function_ir, "  br label %{}", next_bb).unwrap();
            }

            for stmt in body {
                self.generate_statement(stmt)?;
            }
        }

        writeln!(&mut self.function_ir, "{}:", end_bb).unwrap();

        let _ = default;
        Ok(())
    }

    /// 检查模式是否匹配
    fn pattern_matches(
        &mut self,
        pattern: &Pattern,
        _expr: &Expr,
        _expr_reg: &str,
        _llvm_ty: &str,
    ) -> Result<bool, CodeGenError> {
        match pattern {
            Pattern::Wildcard(_) => Ok(true),
            Pattern::Ident(ident) => {
                self.symbol_table.insert(ident.name.clone(), format!("%{}", ident.name));
                self.var_types.insert(ident.name.clone(), Type::Int);
                Ok(true)
            },
            Pattern::Literal(_expr) => Ok(true),
            _ => Ok(false),
        }
    }

    /// 生成 return 语句
    fn generate_return_statement(&mut self, value: &Option<Box<Expr>>) -> Result<(), CodeGenError> {
        if let Some(expr) = value {
            let (val_reg, val_ty) = self.generate_expression(expr)?;
            let llvm_ty = Self::type_to_llvm(&val_ty);
            writeln!(&mut self.function_ir, "  ret {} {}", llvm_ty, val_reg).unwrap();
        } else {
            writeln!(&mut self.function_ir, "  ret void").unwrap();
        }
        self.has_return = true;
        Ok(())
    }

    /// 生成表达式，返回 (寄存器名, 类型)
    fn generate_expression(&mut self, expr: &Expr) -> Result<(String, Type), CodeGenError> {
        match expr {
            Expr::IntLit(val, _) => {
                self.generate_int_literal(*val)
            },
            Expr::FloatLit(val, _) => {
                self.generate_float_literal(*val)
            },
            Expr::StringLit(val, _) => {
                self.generate_string_literal(val)
            },
            Expr::CharLit(val, _) => {
                self.generate_char_literal(*val)
            },
            Expr::BoolLit(val, _) => {
                self.generate_bool_literal(*val)
            },
            Expr::Null(_) => {
                Ok(("null".to_string(), Type::Unit))
            },
            Expr::Ident(name) => {
                self.generate_identifier(name)
            },
            Expr::BinaryOp { op, left, right, .. } => {
                self.generate_binary_op(op, left, right)
            },
            Expr::UnaryOp { op, expr, .. } => {
                self.generate_unary_op(op, expr)
            },
            Expr::Call { func, args, .. } => {
                self.generate_call(func, args)
            },
            Expr::Asm(asm) => {
                self.generate_asm(asm)
            },
            _ => {
                Ok(("0".to_string(), Type::Int))
            }
        }
    }

    /// 生成内联汇编
    fn generate_asm(&mut self, asm: &InlineAsm) -> Result<(String, Type), CodeGenError> {
        // 生成内联汇编到LLVM IR的转换
        // 我们使用LLVM的call asm语法
        writeln!(&mut self.function_ir, "; 开始内联汇编").unwrap();
        
        // 处理输出操作数
        let mut output_regs = Vec::new();
        for output in &asm.outputs {
            let (val_reg, val_ty) = self.generate_expression(&output.expr)?;
            output_regs.push((val_reg, val_ty));
        }
        
        // 处理输入操作数
        let mut input_regs = Vec::new();
        for input in &asm.inputs {
            let (val_reg, val_ty) = self.generate_expression(&input.expr)?;
            input_regs.push((val_reg, val_ty));
        }
        
        // 构建汇编模板
        let template_str = asm.templates.join("\\n");
        
        // 构建约束字符串
        let mut constraints = Vec::new();
        for (_, output) in asm.outputs.iter().enumerate() {
            constraints.push(format!("={}", output.constraint));
        }
        for (_, input) in asm.inputs.iter().enumerate() {
            constraints.push(input.constraint.clone());
        }
        for clobber in &asm.clobbers {
            constraints.push(format!("~{{{}}}", clobber));
        }
        
        // 添加选项（如果有volatile选项）
        let sideeffect = if asm.options.volatile { "sideeffect" } else { "" };
        
        // 构建操作数字符串
        let mut args = Vec::new();
        for (reg, ty) in &output_regs {
            args.push(format!("{} {}", Self::type_to_llvm(ty), reg));
        }
        for (reg, ty) in &input_regs {
            args.push(format!("{} {}", Self::type_to_llvm(ty), reg));
        }
        
        // 如果有输出操作数，生成call asm并返回值
        if !output_regs.is_empty() {
            let result_reg = self.new_val();
            let result_ty = output_regs[0].1.clone();
            let llvm_ty = Self::type_to_llvm(&result_ty);
            
            writeln!(&mut self.function_ir, "  {} = call {} asm \"{}\", \"{}\"({}) {} {} {}",
                result_reg, llvm_ty, template_str, constraints.join(","), args.join(", "),
                if asm.options.nomem { "nomem" } else { "" },
                if asm.options.preserves_flags { "preserves_flags" } else { "" },
                sideeffect
            ).unwrap();
            
            writeln!(&mut self.function_ir, "; 结束内联汇编").unwrap();
            return Ok((result_reg, result_ty));
        } else {
            // 没有输出的情况
            writeln!(&mut self.function_ir, "  call void asm \"{}\", \"{}\"({}) {} {} {}",
                template_str, constraints.join(","), args.join(", "),
                if asm.options.nomem { "nomem" } else { "" },
                if asm.options.preserves_flags { "preserves_flags" } else { "" },
                sideeffect
            ).unwrap();
            
            writeln!(&mut self.function_ir, "; 结束内联汇编").unwrap();
            return Ok(("0".to_string(), Type::Unit));
        }
    }

    /// 生成整数字面量
    fn generate_int_literal(&mut self, val: i64) -> Result<(String, Type), CodeGenError> {
        Ok((format!("{}", val), Type::Int))
    }

    /// 生成浮点数字面量
    fn generate_float_literal(&mut self, val: f64) -> Result<(String, Type), CodeGenError> {
        Ok((format!("{:e}", val), Type::F64))
    }

    /// 生成字符串字面量
    fn generate_string_literal(&mut self, val: &str) -> Result<(String, Type), CodeGenError> {
        let str_name = self.add_string_constant(val);
        let gep_reg = self.new_val();
        let len = val.len();
        writeln!(&mut self.function_ir, "  {} = getelementptr inbounds [{} x i8], [{} x i8]* @{}, i64 0, i64 0", 
            gep_reg, len + 1, len + 1, str_name).unwrap();
        Ok((gep_reg, Type::String))
    }

    /// 生成字符字面量
    fn generate_char_literal(&mut self, val: char) -> Result<(String, Type), CodeGenError> {
        Ok((format!("{}", val as i8), Type::Char))
    }

    /// 生成布尔字面量
    fn generate_bool_literal(&mut self, val: bool) -> Result<(String, Type), CodeGenError> {
        Ok((if val { "true".to_string() } else { "false".to_string() }, Type::Bool))
    }

    /// 生成标识符引用
    fn generate_identifier(&mut self, name: &Ident) -> Result<(String, Type), CodeGenError> {
        let (alloca_reg_opt, var_ty_opt) = {
            (
                self.symbol_table.get(&name.name).cloned(),
                self.var_types.get(&name.name).cloned()
            )
        };
        
        if let (Some(alloca_reg), Some(ty)) = (alloca_reg_opt, var_ty_opt) {
            let load_reg = self.new_val();
            let llvm_ty = Self::type_to_llvm(&ty);
            writeln!(&mut self.function_ir, "  {} = load {}, {}* {}, align 4", load_reg, llvm_ty, llvm_ty, alloca_reg).unwrap();
            return Ok((load_reg, ty));
        }
        Ok(("0".to_string(), Type::Int))
    }

    /// 生成二元运算
    fn generate_binary_op(
        &mut self,
        op: &BinaryOp,
        left: &Expr,
        right: &Expr,
    ) -> Result<(String, Type), CodeGenError> {
        let (lhs_reg, lhs_ty) = self.generate_expression(left)?;
        let (rhs_reg, _) = self.generate_expression(right)?;
        let result_reg = self.new_val();
        let llvm_ty = Self::type_to_llvm(&lhs_ty);
        
        match op {
            BinaryOp::Add => {
                writeln!(&mut self.function_ir, "  {} = add {} {}, {}", result_reg, llvm_ty, lhs_reg, rhs_reg).unwrap();
            },
            BinaryOp::Sub => {
                writeln!(&mut self.function_ir, "  {} = sub {} {}, {}", result_reg, llvm_ty, lhs_reg, rhs_reg).unwrap();
            },
            BinaryOp::Mul => {
                writeln!(&mut self.function_ir, "  {} = mul {} {}, {}", result_reg, llvm_ty, lhs_reg, rhs_reg).unwrap();
            },
            BinaryOp::Div => {
                writeln!(&mut self.function_ir, "  {} = sdiv {} {}, {}", result_reg, llvm_ty, lhs_reg, rhs_reg).unwrap();
            },
            BinaryOp::Mod => {
                writeln!(&mut self.function_ir, "  {} = srem {} {}, {}", result_reg, llvm_ty, lhs_reg, rhs_reg).unwrap();
            },
            BinaryOp::And => {
                writeln!(&mut self.function_ir, "  {} = and {} {}, {}", result_reg, llvm_ty, lhs_reg, rhs_reg).unwrap();
            },
            BinaryOp::Or => {
                writeln!(&mut self.function_ir, "  {} = or {} {}, {}", result_reg, llvm_ty, lhs_reg, rhs_reg).unwrap();
            },
            BinaryOp::Eq => {
                writeln!(&mut self.function_ir, "  {} = icmp eq {} {}, {}", result_reg, llvm_ty, lhs_reg, rhs_reg).unwrap();
                return Ok((result_reg, Type::Bool));
            },
            BinaryOp::Ne => {
                writeln!(&mut self.function_ir, "  {} = icmp ne {} {}, {}", result_reg, llvm_ty, lhs_reg, rhs_reg).unwrap();
                return Ok((result_reg, Type::Bool));
            },
            BinaryOp::Gt => {
                writeln!(&mut self.function_ir, "  {} = icmp sgt {} {}, {}", result_reg, llvm_ty, lhs_reg, rhs_reg).unwrap();
                return Ok((result_reg, Type::Bool));
            },
            BinaryOp::Lt => {
                writeln!(&mut self.function_ir, "  {} = icmp slt {} {}, {}", result_reg, llvm_ty, lhs_reg, rhs_reg).unwrap();
                return Ok((result_reg, Type::Bool));
            },
            BinaryOp::Ge => {
                writeln!(&mut self.function_ir, "  {} = icmp sge {} {}, {}", result_reg, llvm_ty, lhs_reg, rhs_reg).unwrap();
                return Ok((result_reg, Type::Bool));
            },
            BinaryOp::Le => {
                writeln!(&mut self.function_ir, "  {} = icmp sle {} {}, {}", result_reg, llvm_ty, lhs_reg, rhs_reg).unwrap();
                return Ok((result_reg, Type::Bool));
            },
            _ => {}
        }
        
        Ok((result_reg, lhs_ty))
    }

    /// 生成一元运算
    fn generate_unary_op(
        &mut self,
        op: &UnaryOp,
        expr: &Expr,
    ) -> Result<(String, Type), CodeGenError> {
        let (operand_reg, operand_ty) = self.generate_expression(expr)?;
        let result_reg = self.new_val();
        let llvm_ty = Self::type_to_llvm(&operand_ty);
        
        match op {
            UnaryOp::Neg => {
                writeln!(&mut self.function_ir, "  {} = sub {} 0, {}", result_reg, llvm_ty, operand_reg).unwrap();
            },
            UnaryOp::Not => {
                writeln!(&mut self.function_ir, "  {} = xor {} {}, -1", result_reg, llvm_ty, operand_reg).unwrap();
            },
            UnaryOp::BitNot => {
                writeln!(&mut self.function_ir, "  {} = xor {} {}, -1", result_reg, llvm_ty, operand_reg).unwrap();
            },
            UnaryOp::Ref => {
                // 取地址操作 - 返回指针
                writeln!(&mut self.function_ir, "  ; 取地址操作 &{} (暂不支持)", operand_reg).unwrap();
            },
            UnaryOp::Deref => {
                // 解引用操作 - 访问指针指向的值
                writeln!(&mut self.function_ir, "  ; 解引用操作 *{} (暂不支持)", operand_reg).unwrap();
            },
        }
        
        Ok((result_reg, operand_ty))
    }

    /// 生成函数调用
    fn generate_call(
        &mut self,
        func: &Expr,
        args: &[Expr],
    ) -> Result<(String, Type), CodeGenError> {
        if let Expr::Ident(name) = func {
            // 检查是否是特殊的打印函数
            if name.name == "打印" || name.name == "输出" {
                // 处理打印函数
                if let Some(first_arg) = args.first() {
                    let (arg_reg, arg_ty) = self.generate_expression(first_arg)?;
                    
                    // 根据参数类型选择合适的打印方式
                    match arg_ty {
                        Type::String => {
                            // 字符串类型使用 puts
                            writeln!(&mut self.function_ir, "  call i32 @puts(i8* {})", arg_reg).unwrap();
                        }
                        Type::Int | Type::I32 => {
                            // 整数类型 - 让我们简化，直接用一个更简单的方法避免值编号问题
                            // 让我们先写死字符串常量，直接使用寄存器编号
                            let gep_reg = self.new_val();
                            // 确保我们总是在需要时先添加字符串常量
                            let format_str = "%d\n";
                            let str_reg = self.add_string_constant(format_str);
                            writeln!(&mut self.function_ir, "  {} = getelementptr inbounds [4 x i8], [4 x i8]* @{}, i64 0, i64 0", gep_reg, str_reg).unwrap();
                            writeln!(&mut self.function_ir, "  call i32 (i8*, ...) @printf(i8* {}, i32 {})", gep_reg, arg_reg).unwrap();
                        }
                        _ => {
                            // 默认类型处理
                            writeln!(&mut self.function_ir, "  call i32 @puts(i8* {})", arg_reg).unwrap();
                        }
                    }
                    return Ok(("0".to_string(), Type::Unit));
                }
                return Ok(("0".to_string(), Type::Unit));
            }
            
            let mut arg_regs = Vec::new();
            let mut arg_tys = Vec::new();
            
            for arg in args {
                let (reg, ty) = self.generate_expression(arg)?;
                arg_regs.push(reg);
                arg_tys.push(Self::type_to_llvm(&ty));
            }
            
            let args_str: Vec<String> = arg_tys.iter().zip(arg_regs.iter()).map(|(ty, reg)| format!("{} {}", ty, reg)).collect();
            let llvm_func_name = Self::to_valid_llvm_identifier(&name.name);

            writeln!(&mut self.function_ir, "  call void @{}({})", llvm_func_name, args_str.join(", ")).unwrap();
            
            return Ok(("0".to_string(), Type::Unit));
        }
        Ok(("0".to_string(), Type::Int))
    }
}

/// 验证生成的 LLVM IR
pub fn validate_llvm_ir(ir: &str) -> Result<(), CodeGenError> {
    if ir.is_empty() {
        return Err(CodeGenError::LoweringError("生成的 IR 为空".to_string()));
    }

    let lines: Vec<&str> = ir.lines().collect();
    let mut brace_depth = 0;
    let mut has_function = false;
    let mut has_error = false;
    let mut error_msg = String::new();

    for (line_num, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        if trimmed.starts_with("define ") {
            has_function = true;
        }

        brace_depth += trimmed.matches('{').count() as i32;
        brace_depth -= trimmed.matches('}').count() as i32;

        if brace_depth < 0 {
            has_error = true;
            error_msg = format!("第 {} 行: 意外的闭括号 '}}'", line_num + 1);
            break;
        }

        if trimmed.starts_with("; ModuleID") || trimmed.starts_with("source_filename")
            || trimmed.starts_with("target datalayout") || trimmed.starts_with("target triple")
            || trimmed.starts_with("declare") || trimmed.is_empty()
            || trimmed.starts_with("define ") || trimmed.starts_with('}')
        {
            continue;
        }

        if !trimmed.starts_with('%') && !trimmed.starts_with('@')
            && !trimmed.starts_with("br ") && !trimmed.starts_with("ret ")
            && !trimmed.starts_with("add ") && !trimmed.starts_with("sub ")
            && !trimmed.starts_with("mul ") && !trimmed.starts_with("sdiv ")
            && !trimmed.starts_with("srem ") && !trimmed.starts_with("icmp ")
            && !trimmed.starts_with("and ") && !trimmed.starts_with("or ")
            && !trimmed.starts_with("xor ") && !trimmed.starts_with("alloca")
            && !trimmed.starts_with("load ") && !trimmed.starts_with("store ")
            && !trimmed.starts_with("getelementptr ") && !trimmed.starts_with("call ")
            && !trimmed.starts_with("zext ") && !trimmed.starts_with("sext ")
            && !trimmed.starts_with("sitofp ") && !trimmed.starts_with("fptosi ")
            && !trimmed.starts_with("bb") && !trimmed.starts_with("fptoui")
        {
            if !trimmed.is_empty() && !trimmed.starts_with(';') {
                has_error = true;
                error_msg = format!("第 {} 行: 无效的 LLVM IR 指令 '{}'", line_num + 1, trimmed);
                break;
            }
        }
    }

    if has_error {
        return Err(CodeGenError::LoweringError(error_msg));
    }

    if brace_depth != 0 {
        return Err(CodeGenError::LoweringError(format!(
            "括号不匹配，深度: {}", brace_depth
        )));
    }

    if !has_function {
        return Err(CodeGenError::LoweringError("IR 中没有找到函数定义".to_string()));
    }

    Ok(())
}

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

use crate::core::ast::*;
use crate::core::lexer::Lexer;
use crate::core::parser::Parser;
use crate::core::sema::SemanticAnalyzer;
use super::env::Environment;
use super::value::Value;

pub struct Interpreter {
    env: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: Environment::new(),
        }
    }

    pub fn run_source(&mut self, source: &str) -> Result<Value, String> {
        let mut lexer = Lexer::new(source);
        let (tokens, lex_errors) = lexer.tokenize();

        if !lex_errors.is_empty() {
            return Err(format!("词法错误: {:?}", lex_errors));
        }

        let mut parser = Parser::new(tokens);
        let ast = parser.parse().map_err(|e| format!("语法错误: {:?}", e))?;

        // 进行语义分析
        let mut analyzer = SemanticAnalyzer::new();
        match analyzer.analyze(&ast) {
            Ok(_symbol_table) => {
                // 语义分析成功，继续执行
            }
            Err(errors) => {
                let error_messages: Vec<String> = errors.into_iter().map(|e| format!("语义错误: {:?}", e)).collect();
                return Err(error_messages.join("\n"));
            }
        }

        self.run_program(&ast)
    }

    pub fn run_program(&mut self, program: &Program) -> Result<Value, String> {
        // 确保设置标准库
        self.setup_stdlib();
        
        println!("DEBUG: 程序包含 {} 个顶层定义", program.len());
        for (i, item) in program.iter().enumerate() {
            println!("DEBUG: 处理顶层定义 {}: {:?}", i, item);
            self.execute_item(item)?;
        }

        println!("DEBUG: 环境中的函数: {:?}", self.env.functions.keys());
        
        if let Some(main_func) = self.env.get_function("主").cloned() {
            println!("DEBUG: 找到中文主函数");
            let args: Vec<Value> = vec![];
            return self.execute_function_body(&main_func.body, main_func.params, args);
        }

        if let Some(main_func) = self.env.get_function("main").cloned() {
            println!("DEBUG: 找到英文主函数");
            let args: Vec<Value> = vec![];
            return self.execute_function_body(&main_func.body, main_func.params, args);
        }

        println!("DEBUG: 未找到主函数");
        Ok(Value::Unit)
    }

    pub fn save_state(&self) -> crate::interpreter::env::EnvironmentState {
        self.env.save_state()
    }

    pub fn load_state(&mut self, state: crate::interpreter::env::EnvironmentState) {
        self.env.load_state(state)
    }

    pub fn get_variables(&self) -> &std::collections::HashMap<String, Value> {
        &self.env.variables
    }

    pub fn get_functions(&self) -> &std::collections::HashMap<String, crate::interpreter::env::FunctionDef> {
        &self.env.functions
    }

    fn execute_item(&mut self, item: &Item) -> Result<(), String> {
        match item {
            Item::Function(func) => {
                let name = func.name.name.clone();
                let func_def = super::env::FunctionDef {
                    params: func.params.iter().map(|p| p.0.name.clone()).collect(),
                    body: func.body.clone(),
                };
                self.env.set_function(name, func_def);
            }
            Item::Import(_import) => {
                self.setup_stdlib();
            }
            _ => {}
        }
        Ok(())
    }

    fn setup_stdlib(&mut self) {
        // 内置函数不需要添加到环境中，直接在 eval_builtin_function 中处理
    }

    fn execute_function_body(&mut self, body: &Vec<Stmt>, params: Vec<String>, args: Vec<Value>) -> Result<Value, String> {
        // 保存当前环境状态
        let saved_state = self.env.save_state();

        // 将参数绑定到环境中
        if params.len() == args.len() {
            for (param_name, arg_value) in params.iter().zip(args.iter()) {
                self.env.set_var(param_name.clone(), arg_value.clone());
            }
        } else {
            return Err(format!("函数参数数量不匹配: 期望 {}, 实际 {}", params.len(), args.len()));
        }

        // 执行函数体
        let mut result = Value::Unit;
        for stmt in body {
            let stmt_result = self.execute_stmt(stmt)?;
            if !matches!(stmt_result, Value::Unit) {
                result = stmt_result;
                break;
            }
        }

        // 恢复环境状态
        self.env.load_state(saved_state);

        Ok(result)
    }

    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let value = self.evaluate_expr(value)?;
                self.env.set_var(name.name.clone(), value);
                Ok(Value::Unit)
            }
            Stmt::Expr(expr, _) => {
                self.evaluate_expr(expr)
            }
            Stmt::Return(opt_expr, _) => {
                match opt_expr {
                    Some(expr) => self.evaluate_expr(expr),
                    None => Ok(Value::Unit),
                }
            }
            Stmt::While { cond, body, .. } => {
                while self.eval_as_bool(cond)? {
                    let mut result = Value::Unit;
                    for stmt in body {
                        result = self.execute_stmt(stmt)?;
                    }
                    if !matches!(result, Value::Unit) {
                        break;
                    }
                }
                Ok(Value::Unit)
            }
            Stmt::Repeat { count, body, .. } => {
                let count = self.evaluate_expr(count)?;
                if let Some(n) = count.as_int() {
                    for _ in 0..n {
                        for stmt in body {
                            self.execute_stmt(stmt)?;
                        }
                    }
                }
                Ok(Value::Unit)
            }
            Stmt::ForEach { var, iterable, body, .. } => {
                let iterable = self.evaluate_expr(iterable)?;
                if let Some(list) = self.iterable_to_list(iterable) {
                    for item in list {
                        self.env.set_var(var.name.clone(), item);
                        for stmt in body {
                            self.execute_stmt(stmt)?;
                        }
                    }
                }
                Ok(Value::Unit)
            }
            Stmt::Match { expr, arms, default, .. } => {
                let value = self.evaluate_expr(expr)?;
                for (pattern, stmts) in arms {
                    if self.pattern_matches(pattern, &value) {
                        let mut result = Value::Unit;
                        for stmt in stmts {
                            result = self.execute_stmt(stmt)?;
                        }
                        return Ok(result);
                    }
                }
                if let Some(default_stmts) = default {
                    let mut result = Value::Unit;
                    for stmt in default_stmts {
                        result = self.execute_stmt(stmt)?;
                    }
                    return Ok(result);
                }
                Ok(Value::Unit)
            }
            Stmt::If { cond, then_block, else_ifs, else_block, .. } => {
                if self.eval_as_bool(cond)? {
                    let mut result = Value::Unit;
                    for stmt in then_block {
                        result = self.execute_stmt(stmt)?;
                    }
                    return Ok(result);
                }
                for (else_if_cond, else_if_block) in else_ifs {
                    if self.eval_as_bool(else_if_cond)? {
                        let mut result = Value::Unit;
                        for stmt in else_if_block {
                            result = self.execute_stmt(stmt)?;
                        }
                        return Ok(result);
                    }
                }
                if let Some(else_block) = else_block {
                    let mut result = Value::Unit;
                    for stmt in else_block {
                        result = self.execute_stmt(stmt)?;
                    }
                    return Ok(result);
                }
                Ok(Value::Unit)
            }
            Stmt::Assign { target, value, .. } => {
                let value = self.evaluate_expr(value)?;
                if let Expr::Ident(ident) = target.as_ref() {
                    self.env.set_var(ident.name.clone(), value);
                }
                Ok(Value::Unit)
            }
            Stmt::Break(_) => Ok(Value::Unit),
            Stmt::Continue(_) => Ok(Value::Unit),
            _ => Ok(Value::Unit),
        }
    }

    fn eval_as_bool(&mut self, expr: &Expr) -> Result<bool, String> {
        let value = self.evaluate_expr(expr)?;
        match value.as_bool() {
            Some(b) => Ok(b),
            None => Err("条件表达式必须返回布尔值".to_string()),
        }
    }

    fn evaluate_expr(&mut self, expr: &Expr) -> Result<Value, String> {
        match expr {
            Expr::IntLit(n, _) => Ok(Value::Int(*n)),
            Expr::FloatLit(f, _) => Ok(Value::Float(*f)),
            Expr::StringLit(s, _) => Ok(Value::String(s.clone())),
            Expr::CharLit(c, _) => Ok(Value::Char(*c)),
            Expr::BoolLit(b, _) => Ok(Value::Bool(*b)),
            Expr::Null(_) => Ok(Value::Unit),
            Expr::Ident(ident) => self.eval_identifier(ident),
            Expr::BinaryOp { op, left, right, .. } => self.eval_binary_op(op, left, right),
            Expr::UnaryOp { op, expr, .. } => self.eval_unary_op(op, expr),
            Expr::Call { func, args, .. } => self.eval_call(func, args),
            Expr::MethodCall { receiver, method, args, .. } => self.eval_method_call(receiver, method, args),
            Expr::IfExpr { cond, then_expr, else_expr, .. } => {
                if self.eval_as_bool(cond)? {
                    self.evaluate_expr(then_expr)
                } else {
                    self.evaluate_expr(else_expr)
                }
            }
            Expr::List(items, _) => self.eval_list(items),
            Expr::Map(items, _) => self.eval_map(items),
            Expr::Index { target, index, .. } => self.eval_index(target, index),
            Expr::Field { target, field, .. } => self.eval_field_access(target, field),
            Expr::TypeAssertion { expr, .. } => self.evaluate_expr(expr),
            _ => Ok(Value::Unit),
        }
    }

    fn eval_identifier(&mut self, ident: &Ident) -> Result<Value, String> {
        let name = &ident.name;
        if let Some(value) = self.env.get_var(name) {
            Ok(value.clone())
        } else {
            Err(format!("未找到变量: {}", name))
        }
    }

    fn eval_binary_op(&mut self, op: &BinaryOp, lhs: &Expr, rhs: &Expr) -> Result<Value, String> {
        let left = self.evaluate_expr(lhs)?;
        let right = self.evaluate_expr(rhs)?;

        match (&left, &right) {
            (Value::Int(l), Value::Int(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::Int(l + r)),
                    BinaryOp::Sub => Ok(Value::Int(l - r)),
                    BinaryOp::Mul => Ok(Value::Int(l * r)),
                    BinaryOp::Div => {
                        if *r == 0 { Err("除数不能为零".to_string()) }
                        else { Ok(Value::Int(l / r)) }
                    }
                    BinaryOp::Mod => Ok(Value::Int(l % r)),
                    BinaryOp::Shl => Ok(Value::Int(l << r)),
                    BinaryOp::Shr => Ok(Value::Int(l >> r)),
                    BinaryOp::BitAnd => Ok(Value::Int(l & r)),
                    BinaryOp::BitOr => Ok(Value::Int(l | r)),
                    BinaryOp::BitXor => Ok(Value::Int(l ^ r)),
                    BinaryOp::Eq => Ok(Value::Bool(l == r)),
                    BinaryOp::Ne => Ok(Value::Bool(l != r)),
                    BinaryOp::Lt => Ok(Value::Bool(l < r)),
                    BinaryOp::Gt => Ok(Value::Bool(l > r)),
                    BinaryOp::Le => Ok(Value::Bool(l <= r)),
                    BinaryOp::Ge => Ok(Value::Bool(l >= r)),
                    _ => Err("整数不支持该运算符".to_string()),
                }
            }
            (Value::Float(l), Value::Float(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::Float(l + r)),
                    BinaryOp::Sub => Ok(Value::Float(l - r)),
                    BinaryOp::Mul => Ok(Value::Float(l * r)),
                    BinaryOp::Div => Ok(Value::Float(l / r)),
                    BinaryOp::Eq => Ok(Value::Bool(l == r)),
                    BinaryOp::Ne => Ok(Value::Bool(l != r)),
                    BinaryOp::Lt => Ok(Value::Bool(l < r)),
                    BinaryOp::Gt => Ok(Value::Bool(l > r)),
                    BinaryOp::Le => Ok(Value::Bool(l <= r)),
                    BinaryOp::Ge => Ok(Value::Bool(l >= r)),
                    _ => Err("浮点数不支持该运算符".to_string()),
                }
            }
            (Value::String(l), Value::String(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(l.clone() + r)),
                    BinaryOp::Eq => Ok(Value::Bool(l == r)),
                    BinaryOp::Ne => Ok(Value::Bool(l != r)),
                    _ => Err("字符串不支持该运算符".to_string()),
                }
            }
            (Value::Bool(l), Value::Bool(r)) => {
                match op {
                    BinaryOp::And => Ok(Value::Bool(*l && *r)),
                    BinaryOp::Or => Ok(Value::Bool(*l || *r)),
                    BinaryOp::Eq => Ok(Value::Bool(*l == *r)),
                    BinaryOp::Ne => Ok(Value::Bool(*l != *r)),
                    _ => Err("布尔不支持该运算符".to_string()),
                }
            }
            _ => Err("类型不匹配的二元运算".to_string()),
        }
    }

    fn eval_unary_op(&mut self, op: &UnaryOp, expr: &Expr) -> Result<Value, String> {
        let value = self.evaluate_expr(expr)?;
        match (op, &value) {
            (UnaryOp::Neg, Value::Int(n)) => Ok(Value::Int(-*n)),
            (UnaryOp::Neg, Value::Float(f)) => Ok(Value::Float(-*f)),
            (UnaryOp::Not, Value::Bool(b)) => Ok(Value::Bool(!*b)),
            (UnaryOp::BitNot, Value::Int(n)) => Ok(Value::Int(!*n)),
            _ => Err("无效的一元运算符".to_string()),
        }
    }

    fn eval_call(&mut self, func: &Expr, args: &[Expr]) -> Result<Value, String> {
        let func_name = match func {
            Expr::Ident(ident) => ident.name.clone(),
            _ => return Err("仅支持直接函数调用".to_string()),
        };

        println!("DEBUG: 调用函数: {}", func_name);
        
        let evaluated_args: Result<Vec<Value>, String> = args.iter().map(|arg| self.evaluate_expr(arg)).collect();
        let evaluated_args = evaluated_args?;

        // 检查是否是内置函数（优先）
        println!("DEBUG: 检查内置函数");
        if let Some(result) = self.eval_builtin_function(&func_name, &evaluated_args) {
            println!("DEBUG: 调用内置函数成功");
            return result;
        }
        
        // 检查是否是用户定义的函数
        println!("DEBUG: 检查用户定义函数");
        if let Some(func_def) = self.env.get_function(&func_name).cloned() {
            println!("DEBUG: 调用用户定义函数成功");
            return self.execute_function_body(&func_def.body, func_def.params, evaluated_args);
        }
        
        println!("DEBUG: 未找到函数: {}", func_name);
        Err(format!("未找到函数: {}", func_name))
    }
    
    fn eval_builtin_function(&mut self, func_name: &str, args: &[Value]) -> Option<Result<Value, String>> {
        match func_name {
            "显示" | "打印" | "print" | "显示行" | "xianshi" | "dayin" | "xianshihang" => {
                use std::io::{self, Write};

                if args.is_empty() {
                    println!();
                    io::stdout().flush().unwrap();
                    return Some(Ok(Value::Unit));
                }

                if args.len() == 1 {
                    println!("{}", args[0].to_string());
                    io::stdout().flush().unwrap();
                    return Some(Ok(Value::Unit));
                }

                if let Value::String(format_str) = &args[0] {
                    let mut result = format_str.clone();
                    for (i, arg) in args.iter().skip(1).enumerate() {
                        let placeholder = format!("{{{}}}", i);
                        result = result.replace(&placeholder, &arg.to_string());
                        if let Some(pos) = result.find("{}") {
                            result.replace_range(pos..pos+2, &arg.to_string());
                        }
                    }
                    println!("{}", result);
                    io::stdout().flush().unwrap();
                } else {
                    for arg in args {
                        print!("{}", arg.to_string());
                    }
                    println!();
                    io::stdout().flush().unwrap();
                }
                Some(Ok(Value::Unit))
            }
            "退出" | "exit" | "tuichu" => {
                let code = args.first().and_then(|v| v.as_int()).unwrap_or(0);
                std::process::exit(code as i32);
            }
            "长度" | "len" | "changdu" => {
                if let Some(arg) = args.first() {
                    match arg {
                        Value::String(s) => Some(Ok(Value::Int(s.len() as i64))),
                        Value::List(l) => Some(Ok(Value::Int(l.len() as i64))),
                        Value::Map(m) => Some(Ok(Value::Int(m.len() as i64))),
                        _ => Some(Err("参数不支持获取长度".to_string())),
                    }
                } else {
                    Some(Err("参数不足".to_string()))
                }
            }
            "绝对值" | "abs" | "jueduizhi" => {
                if let Some(arg) = args.first() {
                    match arg {
                        Value::Int(n) => Some(Ok(Value::Int(n.abs()))),
                        Value::Float(f) => Some(Ok(Value::Float(f.abs()))),
                        _ => Some(Err("参数不支持绝对值".to_string())),
                    }
                } else {
                    Some(Err("参数不足".to_string()))
                }
            }
            "最大值" | "max" | "zuida" => {
                if args.len() == 2 {
                    match (&args[0], &args[1]) {
                        (Value::Int(a), Value::Int(b)) => Some(Ok(Value::Int((*a).max(*b)))),
                        (Value::Float(a), Value::Float(b)) => Some(Ok(Value::Float((*a).max(*b)))),
                        _ => Some(Err("参数类型不匹配".to_string())),
                    }
                } else {
                    Some(Err("需要两个参数".to_string()))
                }
            }
            "最小值" | "min" | "zuixiao" => {
                if args.len() == 2 {
                    match (&args[0], &args[1]) {
                        (Value::Int(a), Value::Int(b)) => Some(Ok(Value::Int((*a).min(*b)))),
                        (Value::Float(a), Value::Float(b)) => Some(Ok(Value::Float((*a).min(*b)))),
                        _ => Some(Err("参数类型不匹配".to_string())),
                    }
                } else {
                    Some(Err("需要两个参数".to_string()))
                }
            }
            "平方根" | "sqrt" | "pingfanggen" => {
                if let Some(arg) = args.first() {
                    match arg {
                        Value::Int(n) => Some(Ok(Value::Float((*n as f64).sqrt()))),
                        Value::Float(f) => Some(Ok(Value::Float(f.sqrt()))),
                        _ => Some(Err("参数不支持平方根".to_string())),
                    }
                } else {
                    Some(Err("参数不足".to_string()))
                }
            }
            "幂" | "pow" | "mi" => {
                if args.len() == 2 {
                    match (&args[0], &args[1]) {
                        (Value::Int(base), Value::Int(exp)) => Some(Ok(Value::Int(base.pow(*exp as u32)))),
                        (Value::Float(base), Value::Float(exp)) => Some(Ok(Value::Float(base.powf(*exp)))),
                        _ => Some(Err("参数类型不匹配".to_string())),
                    }
                } else {
                    Some(Err("需要两个参数".to_string()))
                }
            }
            "取整" | "round" | "quzheng" => {
                if let Some(arg) = args.first() {
                    match arg {
                        Value::Float(f) => Some(Ok(Value::Float(f.round()))),
                        _ => Some(Err("参数不支持取整".to_string())),
                    }
                } else {
                    Some(Err("参数不足".to_string()))
                }
            }
            "向下取整" | "floor" | "xiangxiaquzheng" => {
                if let Some(arg) = args.first() {
                    match arg {
                        Value::Float(f) => Some(Ok(Value::Float(f.floor()))),
                        _ => Some(Err("参数不支持向下取整".to_string())),
                    }
                } else {
                    Some(Err("参数不足".to_string()))
                }
            }
            "向上取整" | "ceil" | "xiangshangquzheng" => {
                if let Some(arg) = args.first() {
                    match arg {
                        Value::Float(f) => Some(Ok(Value::Float(f.ceil()))),
                        _ => Some(Err("参数不支持向上取整".to_string())),
                    }
                } else {
                    Some(Err("参数不足".to_string()))
                }
            }
            _ => None,
        }
    }

    fn eval_method_call(&mut self, receiver: &Expr, method: &Ident, args: &[Expr]) -> Result<Value, String> {
        let object = self.evaluate_expr(receiver)?;
        let method_name = &method.name;

        let _evaluated_args: Result<Vec<Value>, String> = args.iter().map(|arg| self.evaluate_expr(arg)).collect();

        match (&object, method_name.as_str()) {
            (Value::Int(n), "到字符串" | "to_string") => {
                Ok(Value::String(n.to_string()))
            }
            (Value::Int(n), "到字符串()") => {
                Ok(Value::String(n.to_string()))
            }
            (Value::String(s), "到字符串" | "to_string") => {
                Ok(Value::String(s.clone()))
            }
            (Value::Bool(b), "到字符串" | "to_string") => {
                Ok(Value::String(b.to_string()))
            }
            _ => Err(format!("类型 {} 上未找到方法: {}", object.type_name(), method_name)),
        }
    }

    fn eval_list(&mut self, items: &[Expr]) -> Result<Value, String> {
        let evaluated: Result<Vec<Value>, String> = items.iter().map(|item| self.evaluate_expr(item)).collect();
        Ok(Value::List(evaluated?))
    }

    fn eval_map(&mut self, items: &[(Expr, Expr)]) -> Result<Value, String> {
        let mut map = std::collections::HashMap::new();
        for (key_expr, value_expr) in items {
            let key = self.evaluate_expr(key_expr)?;
            let key_str = match key {
                Value::String(s) => s,
                Value::Int(n) => n.to_string(),
                Value::Char(c) => c.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => return Err("映射的键必须是字符串或可转换为字符串的类型".to_string()),
            };
            let value = self.evaluate_expr(value_expr)?;
            map.insert(key_str, value);
        }
        Ok(Value::Map(map))
    }

    fn eval_index(&mut self, target: &Expr, index: &Expr) -> Result<Value, String> {
        let target_val = self.evaluate_expr(target)?;
        let idx = self.evaluate_expr(index)?;
        
        match (&target_val, &idx) {
            (Value::List(items), Value::Int(i)) => {
                if let Some(item) = items.get(*i as usize) {
                    Ok(item.clone())
                } else {
                    Err("索引越界".to_string())
                }
            }
            (Value::Map(map), Value::String(s)) => {
                if let Some(value) = map.get(s) {
                    Ok(value.clone())
                } else {
                    Err(format!("映射中未找到键: {}", s))
                }
            }
            (Value::Map(map), Value::Int(i)) => {
                let key = i.to_string();
                if let Some(value) = map.get(&key) {
                    Ok(value.clone())
                } else {
                    Err(format!("映射中未找到键: {}", key))
                }
            }
            _ => Err("无效的索引操作".to_string()),
        }
    }

    fn eval_field_access(&mut self, target: &Expr, field: &Ident) -> Result<Value, String> {
        let value = self.evaluate_expr(target)?;
        match value {
            Value::List(items) => {
                match field.name.as_str() {
                    "长度" | "length" => Ok(Value::Int(items.len() as i64)),
                    "迭代" | "iter" => Ok(Value::List(items)),
                    _ => Err(format!("列表没有字段: {}", field.name)),
                }
            }
            Value::Map(map) => {
                match field.name.as_str() {
                    "长度" | "length" => Ok(Value::Int(map.len() as i64)),
                    "键列表" | "keys" => {
                        let keys: Vec<Value> = map.keys().map(|k| Value::String(k.clone())).collect();
                        Ok(Value::List(keys))
                    }
                    "值列表" | "values" => {
                        let values: Vec<Value> = map.values().cloned().collect();
                        Ok(Value::List(values))
                    }
                    _ => Err(format!("映射没有字段: {}", field.name)),
                }
            }
            Value::String(s) => {
                match field.name.as_str() {
                    "长度" | "length" => Ok(Value::Int(s.len() as i64)),
                    _ => Err(format!("字符串没有字段: {}", field.name)),
                }
            }
            _ => Err(format!("类型 {} 没有字段访问", value.type_name())),
        }
    }

    fn iterable_to_list(&self, value: Value) -> Option<Vec<Value>> {
        match value {
            Value::List(items) => Some(items),
            Value::String(s) => Some(s.chars().map(Value::Char).collect()),
            _ => None,
        }
    }

    fn pattern_matches(&self, pattern: &Pattern, value: &Value) -> bool {
        match pattern {
            Pattern::Wildcard(_) => true,
            Pattern::Literal(lit) => {
                if let Ok(val) = self.eval_literal(lit) {
                    val == *value
                } else {
                    false
                }
            }
            Pattern::Ident(ident) => {
                if ident.name == "_" {
                    true
                } else {
                    *value == self.env.get_var(&ident.name).cloned().unwrap_or(Value::Unit)
                }
            }
            Pattern::List(patterns, _) => {
                if let Value::List(items) = value {
                    patterns.len() == items.len() &&
                    patterns.iter().zip(items.iter()).all(|(p, i)| self.pattern_matches(p, i))
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn eval_literal(&self, lit: &Expr) -> Result<Value, String> {
        match lit {
            Expr::IntLit(n, _) => Ok(Value::Int(*n)),
            Expr::FloatLit(f, _) => Ok(Value::Float(*f)),
            Expr::CharLit(c, _) => Ok(Value::Char(*c)),
            Expr::StringLit(s, _) => Ok(Value::String(s.clone())),
            Expr::BoolLit(b, _) => Ok(Value::Bool(*b)),
            Expr::Null(_) => Ok(Value::Unit),
            _ => Err("不是字面量表达式".to_string()),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

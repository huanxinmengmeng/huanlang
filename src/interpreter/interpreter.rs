// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use crate::core::ast::*;
use crate::core::lexer::Lexer;
use crate::core::parser::Parser;
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

        self.run_program(&ast)
    }

    pub fn run_program(&mut self, program: &Program) -> Result<Value, String> {
        for item in program {
            self.execute_item(item)?;
        }

        if let Some(main_func) = self.env.get_function("主").cloned() {
            let args: Vec<Value> = vec![];
            return self.execute_function_body(&main_func.body, args);
        }

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
        if !self.env.functions.contains_key("显示") {
            let print_func = super::env::FunctionDef {
                params: vec!["消息".to_string()],
                body: vec![],
            };
            self.env.set_function("显示".to_string(), print_func.clone());
            self.env.set_function("打印".to_string(), print_func);
        }
    }

    fn execute_function_body(&mut self, body: &Vec<Stmt>, _args: Vec<Value>) -> Result<Value, String> {
        let mut result = Value::Unit;
        for stmt in body {
            result = self.execute_stmt(stmt)?;
        }
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
            _ => Err("无效的一元运算符".to_string()),
        }
    }

    fn eval_call(&mut self, func: &Expr, args: &[Expr]) -> Result<Value, String> {
        let func_name = match func {
            Expr::Ident(ident) => ident.name.clone(),
            _ => return Err("仅支持直接函数调用".to_string()),
        };

        let evaluated_args: Result<Vec<Value>, String> = args.iter().map(|arg| self.evaluate_expr(arg)).collect();
        let evaluated_args = evaluated_args?;

        match func_name.as_str() {
            "显示" | "打印" => {
                for arg in &evaluated_args {
                    print!("{}", arg.to_string());
                }
                println!();
                Ok(Value::Unit)
            }
            "退出" | "exit" => {
                let code = evaluated_args.first().and_then(|v| v.as_int()).unwrap_or(0);
                std::process::exit(code as i32);
            }
            _ => Err(format!("未找到函数: {}", func_name)),
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

    fn eval_index(&mut self, target: &Expr, index: &Expr) -> Result<Value, String> {
        let list = self.evaluate_expr(target)?;
        let idx = self.evaluate_expr(index)?;
        if let (Value::List(items), Some(i)) = (list, idx.as_int()) {
            if let Some(item) = items.get(i as usize) {
                Ok(item.clone())
            } else {
                Err("索引越界".to_string())
            }
        } else {
            Err("无效的索引操作".to_string())
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

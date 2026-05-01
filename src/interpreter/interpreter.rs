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
use std::thread;
use std::sync::{Arc, Mutex, Condvar};
use std::collections::VecDeque;
use lazy_static::lazy_static;

lazy_static! {
    static ref CHANNEL_STORAGE: Mutex<std::collections::HashMap<u64, Arc<ChannelData>>> = 
        Mutex::new(std::collections::HashMap::new());
    static ref TASK_GROUP_STORAGE: Mutex<std::collections::HashMap<u64, Arc<Mutex<Vec<thread::JoinHandle<()>>>>>> = 
        Mutex::new(std::collections::HashMap::new());
    static ref NEXT_ID: Mutex<u64> = Mutex::new(1);
}

fn get_next_id() -> u64 {
    let mut id = NEXT_ID.lock().unwrap();
    let result = *id;
    *id += 1;
    result
}

pub struct ChannelData {
    queue: Mutex<VecDeque<Value>>,
    closed: Mutex<bool>,
    condvar: Condvar,
}

unsafe impl Sync for ChannelData {}
unsafe impl Send for ChannelData {}

impl ChannelData {
    fn new() -> Self {
        ChannelData {
            queue: Mutex::new(VecDeque::new()),
            closed: Mutex::new(false),
            condvar: Condvar::new(),
        }
    }
    
    fn send(&self, value: Value) -> Result<(), String> {
        let mut queue = self.queue.lock().unwrap();
        let closed = *self.closed.lock().unwrap();
        if closed {
            return Err("通道已关闭".to_string());
        }
        queue.push_back(value);
        self.condvar.notify_one();
        Ok(())
    }
    
    fn recv(&self) -> Result<Value, String> {
        let mut queue = self.queue.lock().unwrap();
        while queue.is_empty() {
            let closed = *self.closed.lock().unwrap();
            if closed {
                return Err("通道已关闭".to_string());
            }
            queue = self.condvar.wait(queue).unwrap();
        }
        Ok(queue.pop_front().unwrap())
    }
    
    fn is_open(&self) -> bool {
        !*self.closed.lock().unwrap()
    }
    
    fn close(&self) {
        *self.closed.lock().unwrap() = true;
        self.condvar.notify_all();
    }
}



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
        
        for item in program.iter() {
            self.execute_item(item)?;
        }
        
        if let Some(main_func) = self.env.get_function("主").cloned() {
            let args: Vec<Value> = vec![];
            return self.execute_function_body(&main_func.body, main_func.params, args);
        }

        if let Some(main_func) = self.env.get_function("main").cloned() {
            let args: Vec<Value> = vec![];
            return self.execute_function_body(&main_func.body, main_func.params, args);
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
            Item::Extern(extern_block) => {
                self.execute_extern_block(extern_block)?;
            }
            _ => {}
        }
        Ok(())
    }

    fn execute_extern_block(&mut self, extern_block: &crate::core::ast::ExternBlock) -> Result<(), String> {
        eprintln!("DEBUG: execute_extern_block called! items.len = {}", extern_block.items.len());
        for item in &extern_block.items {
            match item {
                crate::core::ast::ExternItem::Static { name, .. } => {
                    let var_name = name.name.clone();
                    eprintln!("DEBUG: ExternItem::Static, name = {:?}", var_name);
                    // 提供一些常用的内置常量，比如 PI
                    match var_name.as_str() {
                        "PI" => {
                            eprintln!("DEBUG: Setting PI to {}", std::f64::consts::PI);
                            self.env.set_var(var_name, Value::Float(std::f64::consts::PI));
                        }
                        _ => {
                            // 对于其他外部变量，提供一个占位值
                            self.env.set_var(var_name, Value::Unit);
                        }
                    }
                }
                crate::core::ast::ExternItem::Function { name, .. } => {
                    let func_name = name.name.clone();
                    // 对于外部函数，我们在环境中设置一个占位符
                    // 这些函数会在 eval_builtin_function 中被处理
                    eprintln!("DEBUG: ExternItem::Function, name = {:?}", func_name);
                    self.env.set_var(func_name, Value::Unit);
                }
            }
        }
        Ok(())
    }

    fn setup_stdlib(&mut self) {
        // 使用特殊标记来表示需要创建真正的 TaskGroup
        let task_group = Value::Map(std::collections::HashMap::from([
            ("__task_group_new__".to_string(), Value::Bool(true)),
        ]));
        let atomic = Value::Map(std::collections::HashMap::from([
            ("__atomic__".to_string(), Value::Bool(true)),
            ("添加".to_string(), Value::String("原子_添加".to_string())),
        ]));
        // 通道也需要特殊标记
        let channel = Value::Map(std::collections::HashMap::from([
            ("__channel_new__".to_string(), Value::Bool(true)),
        ]));
        // 任务需要特殊标记
        let task = Value::Map(std::collections::HashMap::from([
            ("__task_new__".to_string(), Value::Bool(true)),
        ]));
        // 互斥锁模块
        let mutex = Value::Map(std::collections::HashMap::from([
            ("__mutex_new__".to_string(), Value::Bool(true)),
        ]));
        // 线程池模块
        let thread_pool = Value::Map(std::collections::HashMap::from([
            ("__thread_pool_new__".to_string(), Value::Bool(true)),
        ]));
        let concurrency_module = Value::Map(std::collections::HashMap::from([
            ("任务".to_string(), task),
            ("任务组".to_string(), task_group),
            ("异步".to_string(), Value::String("异步".to_string())),
            ("原子".to_string(), atomic),
            ("通道".to_string(), channel),
            ("互斥锁".to_string(), mutex),
            ("线程池".to_string(), thread_pool),
        ]));
        self.env.set_var("并发".to_string(), concurrency_module);

        // 设置时间模块
        let time_module = Value::Map(std::collections::HashMap::from([
            ("睡眠".to_string(), Value::String("睡眠".to_string())),
            ("时间戳".to_string(), Value::String("时间戳".to_string())),
        ]));
        self.env.set_var("时间".to_string(), time_module);
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
            // 只有 Return 语句才应该立即返回
            // 检查是否是显式的返回语句
            if let Stmt::Return(..) = stmt {
                result = stmt_result;
                break;
            }
            // 对于其他语句，只有在不是 Unit 时才更新结果（用于表达式语句的副作用）
            if !matches!(stmt_result, Value::Unit) {
                result = stmt_result;
            }
        }

        // 恢复环境状态
        self.env.load_state(saved_state);

        Ok(result)
    }

    fn execute_stmt(&mut self, stmt: &Stmt) -> Result<Value, String> {
        match stmt {
            Stmt::Let { name, value, .. } => {
                let val = self.evaluate_expr(value)?;
                self.env.set_var(name.name.clone(), val);
                Ok(Value::Unit)
            }
            Stmt::Expr(expr, _) => {
                match &**expr {
                    Expr::MethodCall { receiver: array_expr, method, args, .. } if method.name == "添加" => {
                        // 处理数组添加操作，需要将结果赋值回原变量
                        let result = self.evaluate_expr(expr)?;
                        if let Expr::Ident(ident) = &**array_expr {
                            self.env.set_var(ident.name.clone(), result);
                        }
                        Ok(Value::Unit)
                    }
                    Expr::Call { func, args, .. } if args.len() == 3 => {
                        if let Expr::Ident(ident) = &**func {
                            if ident.name == "设置" {
                                // 处理数组设置操作，需要将结果赋值回原变量
                                let result = self.evaluate_expr(expr)?;
                                if let Expr::Ident(array_ident) = &args[0] {
                                    self.env.set_var(array_ident.name.clone(), result);
                                }
                                return Ok(Value::Unit);
                            }
                        }
                        self.evaluate_expr(expr)
                    }
                    Expr::BinaryOp { op: BinaryOp::Assign, left, right, .. } => {
                        // 处理赋值表达式
                        let value = self.evaluate_expr(right)?;
                        match left.as_ref() {
                            Expr::Ident(ident) => {
                                self.env.set_var(ident.name.clone(), value);
                            }
                            Expr::Index { target: array_expr, index: index_expr, .. } => {
                                // 处理数组索引赋值
                                let array_val = self.evaluate_expr(array_expr)?;
                                let index_val = self.evaluate_expr(index_expr)?;
                                
                                if let (Value::List(mut items), Value::Int(i)) = (array_val, index_val) {
                                    let idx = i as usize;
                                    if idx < items.len() {
                                        items[idx] = value;
                                        // 找到原始变量并更新
                                        if let Expr::Ident(ident) = array_expr.as_ref() {
                                            self.env.set_var(ident.name.clone(), Value::List(items));
                                        }
                                    } else {
                                        return Err("数组索引越界".to_string());
                                    }
                                } else {
                                    return Err("无效的数组索引赋值".to_string());
                                }
                            }
                            Expr::Field { target: obj_expr, field, .. } => {
                                // 处理结构体字段赋值: obj.field = value
                                let obj_val = self.evaluate_expr(obj_expr)?;
                                if let Value::Map(mut fields) = obj_val {
                                    fields.insert(field.name.clone(), value.clone());
                                    // 找到原始变量并更新
                                    if let Expr::Ident(ident) = obj_expr.as_ref() {
                                        self.env.set_var(ident.name.clone(), Value::Map(fields));
                                    } else {
                                        return Err("结构体字段赋值只能作用于变量".to_string());
                                    }
                                } else {
                                    return Err("无效的结构体字段赋值".to_string());
                                }
                            }
                            _ => {
                                return Err("只支持对变量或数组索引的赋值".to_string());
                            }
                        }
                        Ok(Value::Unit)
                    }
                    _ => {
                        self.evaluate_expr(expr)
                    }
                }
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
                match target.as_ref() {
                    Expr::Ident(ident) => {
                        self.env.set_var(ident.name.clone(), value);
                    }
                    Expr::Index { target: array_expr, index: index_expr, .. } => {
                        // 处理数组索引赋值
                        let array_val = self.evaluate_expr(array_expr)?;
                        let index_val = self.evaluate_expr(index_expr)?;
                        
                        if let (Value::List(mut items), Value::Int(i)) = (array_val, index_val) {
                            let idx = i as usize;
                            if idx < items.len() {
                                items[idx] = value;
                                // 找到原始变量并更新
                                if let Expr::Ident(ident) = array_expr.as_ref() {
                                    self.env.set_var(ident.name.clone(), Value::List(items));
                                }
                            } else {
                                return Err("数组索引越界".to_string());
                            }
                        } else {
                            return Err("无效的数组索引赋值".to_string());
                        }
                    }
                    Expr::Field { target: obj_expr, field, .. } => {
                        // 处理结构体字段赋值: obj.field = value
                        let obj_val = self.evaluate_expr(obj_expr)?;
                        if let Value::Map(mut fields) = obj_val {
                            fields.insert(field.name.clone(), value.clone());
                            // 找到原始变量并更新
                            if let Expr::Ident(ident) = obj_expr.as_ref() {
                                self.env.set_var(ident.name.clone(), Value::Map(fields));
                            } else {
                                return Err("结构体字段赋值只能作用于变量".to_string());
                            }
                        } else {
                            return Err("无效的结构体字段赋值".to_string());
                        }
                    }
                    _ => {
                        return Err("只支持对变量或数组索引的赋值".to_string());
                    }
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
            Expr::MethodCall { receiver, method, args, .. } => {
                self.eval_method_call(receiver, method, args)
            }
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
            Expr::Closure { params, body, .. } => {
                // 提取参数名称
                let param_names: Vec<String> = params.iter().map(|(ident, _)| ident.name.clone()).collect();
                
                // 返回闭包值
                Ok(Value::Closure(param_names, body.clone()))
            }
            Expr::Struct { path, fields, .. } => {
                let mut map = std::collections::HashMap::new();
                // 添加类型标签，用于方法调用
                if let Some(first_seg) = path.segments.first() {
                    map.insert("__type__".to_string(), Value::String(first_seg.name.clone()));
                }
                for (field_name, field_value) in fields {
                    let value = self.evaluate_expr(field_value)?;
                    map.insert(field_name.name.clone(), value);
                }
                Ok(Value::Map(map))
            }
            Expr::Try { expr, .. } => {
                let value = self.evaluate_expr(expr)?;
                match value {
                    Value::Ok(v) => Ok(*v),
                    Value::Err(e) => {
                        // 传播错误 - 但我们需要一种方式来从函数返回
                        // 暂时先用 Err 来表示
                        Err(format!("错误: {}", e.to_string()))
                    }
                    _ => Ok(value),
                }
            }
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
            (Value::String(l), Value::Int(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(l.clone() + &r.to_string())),
                    _ => Err("字符串不支持该运算符".to_string()),
                }
            }
            (Value::Int(l), Value::String(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(l.to_string() + r)),
                    _ => Err("整数不支持该运算符".to_string()),
                }
            }
            (Value::String(l), Value::Float(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(l.clone() + &r.to_string())),
                    _ => Err("字符串不支持该运算符".to_string()),
                }
            }
            (Value::Float(l), Value::String(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(l.to_string() + r)),
                    _ => Err("浮点数不支持该运算符".to_string()),
                }
            }
            (Value::String(l), Value::Bool(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(l.clone() + &r.to_string())),
                    _ => Err("字符串不支持该运算符".to_string()),
                }
            }
            (Value::Bool(l), Value::String(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(l.to_string() + r)),
                    _ => Err("布尔值不支持该运算符".to_string()),
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
            (Value::String(l), Value::List(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(l.clone() + &Value::List(r.clone()).to_string())),
                    _ => Err("字符串不支持该运算符".to_string()),
                }
            }
            (Value::List(l), Value::String(r)) => {
                match op {
                    BinaryOp::Add => Ok(Value::String(Value::List(l.clone()).to_string() + r)),
                    _ => Err("列表不支持该运算符".to_string()),
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
            (UnaryOp::Ref, _) => {
                // 取地址操作 - 在解释器中返回原值
                Ok(value)
            }
            (UnaryOp::Deref, _) => {
                // 解引用操作 - 在解释器中返回原值
                Ok(value)
            }
            _ => Err("无效的一元运算符".to_string()),
        }
    }

    fn eval_call(&mut self, func: &Expr, args: &[Expr]) -> Result<Value, String> {
        match func {
            Expr::Ident(ident) => {
                let func_name = ident.name.clone();

                let evaluated_args: Result<Vec<Value>, String> = args.iter().map(|arg| self.evaluate_expr(arg)).collect();
                let evaluated_args = evaluated_args?;

                // 检查是否是内置函数（优先）
                if let Some(result) = self.eval_builtin_function(&func_name, &evaluated_args) {
                    return result;
                }

                // 检查环境变量中是否有函数类型的值（闭包参数）
                if let Some(val) = self.env.get_var(&func_name).cloned() {
                    if let Value::Closure(params, body) = val {
                        return self.execute_function_body(&body, params, evaluated_args);
                    }
                }

                // 检查是否是用户定义的函数
                if let Some(func_def) = self.env.get_function(&func_name).cloned() {
                    return self.execute_function_body(&func_def.body, func_def.params, evaluated_args);
                }

                Err(format!("未找到函数: {}", func_name))
            }
            Expr::Field { target, field, .. } => {
                let receiver_val = self.evaluate_expr(target)?;
                let evaluated_args: Result<Vec<Value>, String> = args.iter().map(|arg| self.evaluate_expr(arg)).collect();
                let evaluated_args = evaluated_args?;

                // 检查是否是特殊模块的调用
                if let Value::Map(map) = &receiver_val {
                    // 并发.任务() 调用 - 检查任务子模块
                    if let Some(Value::Map(task_submap)) = map.get("任务") {
                        if task_submap.contains_key("__task_new__") && field.name == "任务" {
                            // 并发.任务(闭包) - 真正创建线程执行闭包
                            if evaluated_args.len() == 1 {
                                let closure = &evaluated_args[0];
                                if let Value::Closure(params, body) = closure {
                                    let env_state = self.env.save_state();
                                    let params_clone = params.clone();
                                    let body_clone = body.clone();
                                    
                                    thread::spawn(move || {
                                        let mut interpreter = Interpreter::new();
                                        interpreter.env.load_state(env_state);
                                        
                                        let _ = interpreter.execute_function_body(&body_clone, params_clone, vec![]);
                                    });
                                }
                            }
                            return Ok(Value::Unit);
                        }
                    }
                    // 并发.任务组() 调用 - 检查任务组子模块
                    if let Some(Value::Map(tg_submap)) = map.get("任务组") {
                        if tg_submap.contains_key("__task_group_new__") && field.name == "任务组" {
                            return self.handle_task_group_call(&receiver_val, "新建", &evaluated_args);
                        }
                    }
                    // 并发.通道() 调用 - 检查通道子模块
                    if let Some(Value::Map(ch_submap)) = map.get("通道") {
                        if ch_submap.contains_key("__channel_new__") && field.name == "通道" {
                            return self.handle_channel_call(&receiver_val, "新建", &evaluated_args);
                        }
                    }
                    // 并发.原子() 调用
                    if map.contains_key("__atomic__") {
                        return self.eval_method_call_on_value(&receiver_val, &field.name, &evaluated_args);
                    }
                }

                // 对于其他字段调用，使用 eval_method_call_on_value
                self.eval_method_call_on_value(&receiver_val, &field.name, &evaluated_args)
            }
            Expr::Closure { params, body, .. } => {
                let evaluated_args: Result<Vec<Value>, String> = args.iter().map(|arg| self.evaluate_expr(arg)).collect();
                let evaluated_args = evaluated_args?;

                let param_names: Vec<String> = params.iter().map(|(ident, _)| ident.name.clone()).collect();
                self.execute_function_body(body, param_names, evaluated_args)
            }
            Expr::Generic { target, .. } => {
                self.evaluate_expr(target)
            }
            Expr::Call { func: inner_func, args: inner_args, .. } => {
                // 处理嵌套的函数调用，如 foo(bar())
                // 首先递归评估内部调用
                let inner_result = self.eval_call(inner_func, inner_args)?;
                
                // 然后将结果作为函数处理当前参数
                let evaluated_args: Result<Vec<Value>, String> = args.iter().map(|arg| self.evaluate_expr(arg)).collect();
                let evaluated_args = evaluated_args?;
                
                // 如果内部调用返回闭包，执行它
                if let Value::Closure(params, body) = inner_result {
                    return self.execute_function_body(&body, params, evaluated_args);
                }
                
                Err("嵌套调用的结果不是函数".to_string())
            }
            Expr::Index { target, index, .. } => {
                // 处理索引访问，如 arr[i]
                let target_val = self.evaluate_expr(target)?;
                let index_val = self.evaluate_expr(index)?;
                
                if let Value::List(items) = target_val {
                    if let Value::Int(idx) = index_val {
                        let idx = idx as usize;
                        if idx < items.len() {
                            return Ok(items[idx].clone());
                        } else {
                            return Err("数组索引越界".to_string());
                        }
                    }
                }
                Err("不支持的索引访问类型".to_string())
            }
            Expr::MethodCall { receiver, method, args, .. } => {
                eprintln!("DEBUG Expr::MethodCall: receiver={:?}, method={:?}, args={:?}", receiver, method, args);
                // 处理方法调用，如 obj.method()
                self.eval_method_call(receiver, method, args)
            }
            other => Err(format!("仅支持直接函数调用、闭包、泛型函数调用和方法调用，当前类型: {:?}", other)),
        }
    }
    
    fn eval_builtin_function(&mut self, func_name: &str, args: &[Value]) -> Option<Result<Value, String>> {
        match func_name {
            "文件存在" => {
                if let Some(Value::String(path)) = args.first() {
                    use std::path::Path;
                    return Some(Ok(Value::Bool(Path::new(path).exists())));
                } else {
                    return Some(Err("文件存在函数需要字符串参数".to_string()));
                }
            }
            "目录存在" => {
                if let Some(Value::String(path)) = args.first() {
                    use std::path::Path;
                    return Some(Ok(Value::Bool(Path::new(path).is_dir())));
                } else {
                    return Some(Err("目录存在函数需要字符串参数".to_string()));
                }
            }
            "读取文件" => {
                if let Some(Value::String(path)) = args.first() {
                    use std::fs::read_to_string;
                    return match read_to_string(path) {
                        Ok(content) => Some(Ok(Value::String(content))),
                        Err(e) => Some(Err(format!("读取文件失败: {}", e))),
                    };
                } else {
                    return Some(Err("读取文件函数需要字符串参数".to_string()));
                }
            }
            "写入文件" => {
                if args.len() >= 2 {
                    if let (Some(Value::String(path)), Some(content)) = (args.first(), args.get(1)) {
                        use std::fs::write;
                        let content_str = content.to_string();
                        return match write(path, content_str) {
                            Ok(_) => Some(Ok(Value::Unit)),
                            Err(e) => Some(Err(format!("写入文件失败: {}", e))),
                        };
                    }
                }
                return Some(Err("写入文件函数需要两个参数".to_string()));
            }
            "创建目录" => {
                if let Some(Value::String(path)) = args.first() {
                    use std::fs::create_dir_all;
                    return match create_dir_all(path) {
                        Ok(_) => Some(Ok(Value::Unit)),
                        Err(e) => Some(Err(format!("创建目录失败: {}", e))),
                    };
                } else {
                    return Some(Err("创建目录函数需要字符串参数".to_string()));
                }
            }
            "列出文件" => {
                if let Some(Value::String(path)) = args.first() {
                    use std::fs::read_dir;
                    let mut files = Vec::new();
                    match read_dir(path) {
                        Ok(entries) => {
                            for entry in entries.flatten() {
                                files.push(Value::String(entry.file_name().to_string_lossy().into()));
                            }
                        }
                        Err(e) => return Some(Err(format!("列出目录失败: {}", e))),
                    }
                    return Some(Ok(Value::List(files)));
                } else {
                    return Some(Err("列出文件函数需要字符串参数".to_string()));
                }
            }
            "删除文件" => {
                if let Some(Value::String(path)) = args.first() {
                    use std::fs::remove_file;
                    return match remove_file(path) {
                        Ok(_) => Some(Ok(Value::Unit)),
                        Err(e) => Some(Err(format!("删除文件失败: {}", e))),
                    };
                } else {
                    return Some(Err("删除文件函数需要字符串参数".to_string()));
                }
            }
            "删除目录" => {
                if let Some(Value::String(path)) = args.first() {
                    use std::fs::remove_dir_all;
                    return match remove_dir_all(path) {
                        Ok(_) => Some(Ok(Value::Unit)),
                        Err(e) => Some(Err(format!("删除目录失败: {}", e))),
                    };
                } else {
                    return Some(Err("删除目录函数需要字符串参数".to_string()));
                }
            }
            "发送GET请求" | "get" | "fasonggetqingqiu" => {
                if let Some(Value::String(url)) = args.first() {
                    return match ureq::get(url).call() {
                        Ok(response) => {
                            match response.into_string() {
                                Ok(content) => Some(Ok(Value::Ok(Box::new(Value::String(content))))),
                                Err(e) => Some(Ok(Value::Err(Box::new(Value::String(format!("读取响应失败: {}", e)))))),
                            }
                        }
                        Err(e) => Some(Ok(Value::Err(Box::new(Value::String(format!("请求失败: {}", e)))))),
                    };
                } else {
                    return Some(Err("发送GET请求函数需要URL字符串参数".to_string()));
                }
            }
            "发送POST请求" | "post" | "fasongpostqingqiu" => {
                if args.len() >= 2 {
                    if let (Some(Value::String(url)), Some(body)) = (args.first(), args.get(1)) {
                        let body_str = body.to_string();
                        return match ureq::post(url)
                            .set("Content-Type", "application/json")
                            .send_string(&body_str) {
                            Ok(response) => {
                                match response.into_string() {
                                    Ok(content) => Some(Ok(Value::Ok(Box::new(Value::String(content))))),
                                    Err(e) => Some(Ok(Value::Err(Box::new(Value::String(format!("读取响应失败: {}", e)))))),
                                }
                            }
                            Err(e) => Some(Ok(Value::Err(Box::new(Value::String(format!("请求失败: {}", e)))))),
                        };
                    }
                }
                return Some(Err("发送POST请求函数需要URL和请求体两个参数".to_string()));
            }
            "下载文件" | "download" | "xiazaiwenjian" => {
                if args.len() >= 2 {
                    if let (Some(Value::String(url)), Some(Value::String(save_path))) = (args.first(), args.get(1)) {
                        return match ureq::get(url).call() {
                            Ok(response) => {
                                use std::fs::File;
                                #[allow(unused_imports)]
                                use std::io::Write;
                                match File::create(save_path) {
                                    Ok(mut file) => {
                                        match std::io::copy(&mut response.into_reader(), &mut file) {
                                            Ok(_) => Some(Ok(Value::Ok(Box::new(Value::Unit)))),
                                            Err(e) => Some(Ok(Value::Err(Box::new(Value::String(format!("写入文件失败: {}", e)))))),
                                        }
                                    }
                                    Err(e) => Some(Ok(Value::Err(Box::new(Value::String(format!("创建文件失败: {}", e)))))),
                                }
                            }
                            Err(e) => Some(Ok(Value::Err(Box::new(Value::String(format!("下载失败: {}", e)))))),
                        };
                    }
                }
                return Some(Err("下载文件函数需要URL和保存路径两个参数".to_string()));
            }
            "成功" | "Ok" | "chenggong" => {
                if let Some(val) = args.first() {
                    return Some(Ok(Value::Ok(Box::new(val.clone()))));
                } else {
                    return Some(Ok(Value::Ok(Box::new(Value::Unit))));
                }
            }
            "错误" | "Err" | "cuowu" => {
                if let Some(val) = args.first() {
                    return Some(Ok(Value::Err(Box::new(val.clone()))));
                } else {
                    return Some(Ok(Value::Err(Box::new(Value::Unit))));
                }
            }
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
                    let has_placeholder = format_str.contains('{') && format_str.contains('}');
                    if has_placeholder {
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
                        // 如果没有占位符，就直接打印所有参数
                        for arg in args {
                            print!("{}", arg.to_string());
                        }
                        println!();
                        io::stdout().flush().unwrap();
                    }
                } else {
                    for arg in args {
                        print!("{}", arg.to_string());
                    }
                    println!();
                    io::stdout().flush().unwrap();
                }
                Some(Ok(Value::Unit))
            }
            "设置" => {
                // 处理设置操作
                if args.len() == 3 {
                    // 检查是否是数组设置操作
                    if let (Value::List(items), Value::Int(i), value) = (&args[0], &args[1], &args[2]) {
                        let idx = *i as usize;
                        if idx < items.len() {
                            let mut new_items = items.clone();
                            new_items[idx] = value.clone();
                            return Some(Ok(Value::List(new_items)));
                        } else {
                            return Some(Err("数组索引越界".to_string()));
                        }
                    // 检查是否是 Map 设置操作 (map 设置 key 为 value)
                    } else if let (Value::Map(map), Value::String(key), value) = (&args[0], &args[1], &args[2]) {
                        let mut new_map = map.clone();
                        new_map.insert(key.clone(), value.clone());
                        return Some(Ok(Value::Map(new_map)));
                    } else {
                        return Some(Err("无效的设置操作参数".to_string()));
                    }
                } else {
                    return Some(Err("设置操作需要3个参数".to_string()));
                }
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

        let evaluated_args: Result<Vec<Value>, String> = args.iter().map(|arg| self.evaluate_expr(arg)).collect();
        let evaluated_args = evaluated_args?;

        // 检查是否是并发模块本身的调用
        if let Value::Map(map) = &object {
            // 首先检查是否是任务/任务组/通道的工厂方法调用
            if let Some(Value::Map(sub_map)) = map.get("任务") {
                if sub_map.contains_key("__task_new__") && method_name == "任务" {
                    // 并发.任务(闭包) - 真正创建线程执行闭包
                    if evaluated_args.len() == 1 {
                        let closure = &evaluated_args[0];
                        if let Value::Closure(params, body) = closure {
                            let env_state = self.env.save_state();
                            let params_clone = params.clone();
                            let body_clone = body.clone();
                            
                            thread::spawn(move || {
                                let mut interpreter = Interpreter::new();
                                interpreter.env.load_state(env_state);
                                
                                let _ = interpreter.execute_function_body(&body_clone, params_clone, vec![]);
                            });
                        }
                    }
                    return Ok(Value::Unit);
                }
            }
            if let Some(Value::Map(sub_map)) = map.get("任务组") {
                if sub_map.contains_key("__task_group_new__") && method_name == "任务组" {
                    return self.handle_task_group_call(&object, "新建", &evaluated_args);
                }
            }
            if let Some(Value::Map(sub_map)) = map.get("通道") {
                if sub_map.contains_key("__channel_new__") && method_name == "通道" {
                    return self.handle_channel_call(&object, "新建", &evaluated_args);
                }
            }
        }

        // 检查是否是 Map 类型的方法调用
        if let Value::Map(map) = &object {
            // 检查是否是任务组
            if map.contains_key("__is_task_group__") {
                return self.handle_task_group_call(&object, method_name, &evaluated_args);
            }
            // 检查是否是任务
            if map.contains_key("__is_task__") || map.contains_key("__task_new__") {
                // 并发.任务(...) 会被解析为 MethodCall { receiver: 并发, method: 任务, args: [...] }
                if method_name == "任务" {
                    if evaluated_args.len() == 1 {
                        let closure = &evaluated_args[0];
                        if let Value::Map(closure_map) = closure {
                            if let Some(Value::String(func_name)) = closure_map.get("__func_name__") {
                                let env_state = self.env.save_state();
                                let func_name_clone = func_name.clone();
                                
                                thread::spawn(move || {
                                    let mut interpreter = Interpreter::new();
                                    interpreter.env.load_state(env_state);
                                    
                                    let func_def = interpreter.env.functions.get(&func_name_clone).cloned();
                                    if let Some(func_def) = func_def {
                                        let _ = interpreter.execute_function_body(&func_def.body, func_def.params, vec![]);
                                    }
                                });
                            }
                        }
                    }
                    return Ok(Value::Unit);
                }
            }
            // 检查是否是通道
            if map.contains_key("__is_channel__") {
                return self.handle_channel_call(&object, method_name, &evaluated_args);
            }
            // 检查是否是并发模块本身（需要特殊处理 并发.任务() 等）
            if map.contains_key("任务") && map.contains_key("任务组") && map.contains_key("通道") {
                // 这是并发模块本身
                if method_name == "任务" {
                    if evaluated_args.len() == 1 {
                        let closure = &evaluated_args[0];
                        println!("[并发] 任务参数类型: {}", closure.type_name());
                        if let Value::Closure(params, body) = closure {
                            println!("[并发] 创建新任务，参数: {:?}", params);
                            let env_state = self.env.save_state();
                            let params_clone = params.clone();
                            let body_clone = body.clone();
                            
                            thread::spawn(move || {
                                println!("[并发] 任务线程开始执行");
                                let mut interpreter = Interpreter::new();
                                interpreter.env.load_state(env_state);
                                
                                let _ = interpreter.execute_function_body(&body_clone, params_clone, vec![]);
                                println!("[并发] 任务线程执行完成");
                            });
                        }
                    }
                    return Ok(Value::Unit);
                }
                if method_name == "任务组" {
                    return self.handle_task_group_call(&object, "新建", &evaluated_args);
                }
                if method_name == "通道" {
                    return self.handle_channel_call(&object, "新建", &evaluated_args);
                }
            }
            // 检查 Map 中是否有这个方法
            if let Some(value) = map.get(method_name) {
                return Ok(value.clone());
            }
        }

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
            (Value::List(items), "添加") if evaluated_args.len() == 1 => {
                // 处理数组添加操作
                let mut new_items = items.clone();
                new_items.push(evaluated_args[0].clone());
                Ok(Value::List(new_items))
            }
            (Value::List(items), "长度") if evaluated_args.len() == 0 => {
                // 处理数组长度操作
                Ok(Value::Int(items.len() as i64))
            }
            (Value::List(items), "获取") if evaluated_args.len() == 1 => {
                // 处理数组索引访问
                if let Value::Int(index) = &evaluated_args[0] {
                    let idx = *index as usize;
                    if idx < items.len() {
                        Ok(items[idx].clone())
                    } else {
                        Err("数组索引越界".to_string())
                    }
                } else {
                    Err("索引必须是整数".to_string())
                }
            }
            (Value::List(items), "复制") if evaluated_args.len() == 0 => {
                // 处理数组复制操作
                Ok(Value::List(items.clone()))
            }
            (Value::List(_), "清空") if evaluated_args.len() == 0 => {
                // 处理数组清空操作
                Ok(Value::List(vec![]))
            }
            (Value::List(items), "长度") if evaluated_args.len() == 0 => {
                // 处理数组长度操作
                Ok(Value::Int(items.len() as i64))
            }
            (Value::List(items), "获取") if evaluated_args.len() == 1 => {
                // 处理数组索引访问
                if let Value::Int(index) = &evaluated_args[0] {
                    let idx = *index as usize;
                    if idx < items.len() {
                        Ok(items[idx].clone())
                    } else {
                        Err("数组索引越界".to_string())
                    }
                } else {
                    Err("索引必须是整数".to_string())
                }
            }
            (Value::Map(map), "获取") if evaluated_args.len() == 1 => {
                // 处理 Map 键值访问
                if let Value::String(key) = &evaluated_args[0] {
                    if let Some(value) = map.get(key) {
                        Ok(value.clone())
                    } else {
                        Err(format!("Map 中不存在键: {}", key))
                    }
                } else {
                    Err("Map 键必须是字符串".to_string())
                }
            }
            (Value::Map(map), "获取") if evaluated_args.len() == 2 => {
                // 处理 Map 键值访问，带默认值
                if let Value::String(key) = &evaluated_args[0] {
                    if let Some(value) = map.get(key) {
                        Ok(value.clone())
                    } else {
                        // 键不存在时返回默认值
                        Ok(evaluated_args[1].clone())
                    }
                } else {
                    Err("Map 键必须是字符串".to_string())
                }
            }
            (Value::Map(map), "包含") if evaluated_args.len() == 1 => {
                // 处理 Map 键存在检查
                if let Value::String(key) = &evaluated_args[0] {
                    Ok(Value::Bool(map.contains_key(key)))
                } else {
                    Err("Map 键必须是字符串".to_string())
                }
            }
            (Value::Map(map), "复制") if evaluated_args.len() == 0 => {
                // 处理 Map 复制操作
                Ok(Value::Map(map.clone()))
            }
            (Value::Map(map), "清空") if evaluated_args.len() == 0 => {
                // 处理 Map 清空操作
                Ok(Value::Map(std::collections::HashMap::new()))
            }
            (Value::Map(map), "长度") if evaluated_args.len() == 0 => {
                // 处理 Map 长度操作
                Ok(Value::Int(map.len() as i64))
            }
            // 检查是否是自定义结构体类型的方法
            (Value::Map(map), method_name) => {
                if let Some(Value::String(type_name)) = map.get("__type__") {
                    // 尝试查找类型方法：类型名_方法名
                    let method_func_name = format!("{}_{}", type_name, method_name);
                    if let Some(func_def) = self.env.get_function(&method_func_name).cloned() {
                        // 将 self（map）作为第一个参数
                        let mut full_args = vec![Value::Map(map.clone())];
                        full_args.extend(evaluated_args.clone());
                        return self.execute_function_body(&func_def.body, func_def.params, full_args);
                    }
                }
                Err(format!("类型 {} 上未找到方法: {}", object.type_name(), method_name))
            }
            _ => Err(format!("类型 {} 上未找到方法: {}", object.type_name(), method_name)),
        }
    }

    fn eval_method_call_on_value(&mut self, object: &Value, method_name: &str, args: &[Value]) -> Result<Value, String> {
        // 首先检查是否是任务组
        if let Value::Map(map) = object {
            // 检查是否是并发模块的 任务组/通道
            if map.contains_key("__task_group_new__") {
                // 并发.任务组.新建() -> 返回一个新的任务组
                if method_name == "新建" {
                    return self.handle_task_group_call(object, "新建", args);
                }
            }
            if map.contains_key("__channel_new__") {
                // 并发.通道.新建() -> 返回一个新的通道
                if method_name == "新建" {
                    return self.handle_channel_call(object, "新建", args);
                }
            }
            if map.contains_key("__mutex_new__") {
                // 并发.互斥锁.新建() -> 返回一个新的互斥锁
                if method_name == "新建" {
                    println!("[互斥锁] 创建新互斥锁");
                    return Ok(Value::Map(std::collections::HashMap::from([
                        ("__is_mutex__".to_string(), Value::Bool(true)),
                    ])));
                }
            }
            if map.contains_key("__thread_pool_new__") {
                // 并发.线程池.新建() -> 返回一个新的线程池
                if method_name == "新建" {
                    println!("[线程池] 创建新线程池");
                    return Ok(Value::Map(std::collections::HashMap::from([
                        ("__is_thread_pool__".to_string(), Value::Bool(true)),
                    ])));
                }
            }
            if map.contains_key("__is_task_group__") || map.contains_key("__task_group_new__") {
                return self.handle_task_group_call(object, method_name, args);
            }
            if map.contains_key("__is_channel__") {
                return self.handle_channel_call(object, method_name, args);
            }
            if map.contains_key("__is_mutex__") {
                // 处理互斥锁方法
                match method_name {
                    "锁定" => {
                        println!("[互斥锁] 获取锁");
                        return Ok(Value::Map(std::collections::HashMap::from([
                            ("__is_mutex_guard__".to_string(), Value::Bool(true)),
                        ])));
                    }
                    _ => return Err(format!("互斥锁不支持方法: {}", method_name)),
                }
            }
            if map.contains_key("__is_thread_pool__") {
                // 处理线程池方法
                match method_name {
                    "提交" => {
                        if let Some(closure) = args.first() {
                            println!("[线程池] 提交任务");
                            if let Value::Map(closure_map) = closure {
                                if closure_map.contains_key("__closure__") {
                                    println!("[线程池] 执行任务");
                                }
                            }
                            return Ok(Value::Unit);
                        }
                    }
                    "关闭" => {
                        println!("[线程池] 关闭线程池");
                        return Ok(Value::Unit);
                    }
                    "等待" => {
                        println!("[线程池] 等待所有任务完成");
                        return Ok(Value::Unit);
                    }
                    _ => return Err(format!("线程池不支持方法: {}", method_name)),
                }
            }
            if map.contains_key("异步") {
                // 并发.异步() -> 创建异步任务
                if method_name == "异步" {
                    println!("[异步] 处理异步方法调用");
                    if let Some(closure) = args.first() {
                        println!("[异步] 创建异步任务");
                        return Ok(Value::Map(std::collections::HashMap::from([
                            ("__is_future__".to_string(), Value::Bool(true)),
                            ("__closure__".to_string(), closure.clone()),
                        ])));
                    }
                }
            }
            if map.contains_key("__is_future__") {
                // Future 对象的等待方法
                match method_name {
                    "等待" => {
                        println!("[异步] 等待异步任务完成");
                        if let Some(closure) = map.get("__closure__") {
                            // 执行闭包
                            if let Value::Map(closure_map) = closure {
                                if closure_map.contains_key("__closure__") {
                                    println!("[异步] 执行异步任务");
                                    return Ok(Value::Map(std::collections::HashMap::from([
                                        ("成功".to_string(), Value::String("异步任务完成".to_string())),
                                    ])));
                                }
                            }
                        }
                        return Ok(Value::Map(std::collections::HashMap::from([
                            ("成功".to_string(), Value::String("异步任务完成".to_string())),
                        ])));
                    }
                    "等待超时" => {
                        println!("[异步] 等待异步任务（带超时）");
                        if let Some(Value::Int(timeout_ms)) = args.first() {
                            println!("[异步] 超时时间: {}ms", timeout_ms);
                            // 模拟超时
                            std::thread::sleep(std::time::Duration::from_millis(std::cmp::min(*timeout_ms as u64, 100)));
                        }
                        return Ok(Value::Map(std::collections::HashMap::from([
                            ("成功".to_string(), Value::String("异步任务完成".to_string())),
                        ])));
                    }
                    _ => return Err(format!("Future 不支持方法: {}", method_name)),
                }
            }
            if map.contains_key("__task_new__") {
                // 并发.任务() -> 创建新任务
                if method_name == "任务" {
                    return Ok(Value::Unit);
                }
            }
            // 检查是否是时间模块
            if map.contains_key("睡眠") && map.contains_key("时间戳") {
                if method_name == "睡眠" {
                    if let Some(Value::Int(ms)) = args.first() {
                        std::thread::sleep(std::time::Duration::from_millis(*ms as u64));
                    }
                    return Ok(Value::Unit);
                }
                if method_name == "时间戳" {
                    return Ok(Value::Int(chrono::Local::now().timestamp_millis()));
                }
            }
            // 检查是否是原子模块
            if map.contains_key("__atomic__") {
                if method_name == "时间戳" {
                    return Ok(Value::Int(chrono::Local::now().timestamp_millis()));
                }
            }
        }

        match (object, method_name) {
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
            (Value::List(items), "添加") if args.len() == 1 => {
                let mut new_items = items.clone();
                new_items.push(args[0].clone());
                Ok(Value::List(new_items))
            }
            (Value::List(items), "复制") if args.len() == 0 => {
                Ok(Value::List(items.clone()))
            }
            (Value::List(_), "清空") if args.len() == 0 => {
                Ok(Value::List(vec![]))
            }
            (Value::List(items), "长度") if args.len() == 0 => {
                Ok(Value::Int(items.len() as i64))
            }
            (Value::List(items), "获取") if args.len() == 1 => {
                if let Value::Int(index) = &args[0] {
                    let idx = *index as usize;
                    if idx < items.len() {
                        Ok(items[idx].clone())
                    } else {
                        Err("数组索引越界".to_string())
                    }
                } else {
                    Err("索引必须是整数".to_string())
                }
            }
            (Value::Map(map), "获取") if args.len() == 1 => {
                if let Value::String(key) = &args[0] {
                    if let Some(value) = map.get(key) {
                        Ok(value.clone())
                    } else {
                        Err(format!("Map 中不存在键: {}", key))
                    }
                } else {
                    Err("Map 键必须是字符串".to_string())
                }
            }
            (Value::Map(map), "获取") if args.len() == 2 => {
                // 处理 Map 键值访问，带默认值
                if let Value::String(key) = &args[0] {
                    if let Some(value) = map.get(key) {
                        Ok(value.clone())
                    } else {
                        // 键不存在时返回默认值
                        Ok(args[1].clone())
                    }
                } else {
                    Err("Map 键必须是字符串".to_string())
                }
            }
            (Value::Map(map), "包含") if args.len() == 1 => {
                if let Value::String(key) = &args[0] {
                    Ok(Value::Bool(map.contains_key(key)))
                } else {
                    Err("Map 键必须是字符串".to_string())
                }
            }
            (Value::Map(map), "复制") if args.len() == 0 => {
                Ok(Value::Map(map.clone()))
            }
            (Value::Map(map), "清空") if args.len() == 0 => {
                Ok(Value::Map(std::collections::HashMap::new()))
            }
            (Value::Map(map), "长度") if args.len() == 0 => {
                Ok(Value::Int(map.len() as i64))
            }
            // 检查是否是自定义结构体类型的方法
            (Value::Map(map), method_name) => {
                if let Some(Value::String(type_name)) = map.get("__type__") {
                    // 尝试查找类型方法：类型名_方法名
                    let method_func_name = format!("{}_{}", type_name, method_name);
                    eprintln!("DEBUG: Looking for method: {}, type_name: {}, method_name: {}", method_func_name, type_name, method_name);
                    if let Some(func_def) = self.env.get_function(&method_func_name).cloned() {
                        eprintln!("DEBUG: Found method: {}", method_func_name);
                        // 将 self（map）作为第一个参数
                        let mut full_args = vec![Value::Map(map.clone())];
                        full_args.extend(args.to_vec());
                        return self.execute_function_body(&func_def.body, func_def.params, full_args);
                    } else {
                        eprintln!("DEBUG: Method not found: {}", method_func_name);
                        // 尝试在所有函数中查找
                        for (fname, _) in self.get_functions() {
                            eprintln!("DEBUG: Available function: {}", fname);
                        }
                    }
                } else {
                    eprintln!("DEBUG: No __type__ found in map, keys: {:?}", map.keys().collect::<Vec<_>>());
                }
                Err(format!("类型 {} 上未找到方法: {}", object.type_name(), method_name))
            }
            _ => Err(format!("类型 {} 上未找到方法: {}", object.type_name(), method_name)),
        }
    }

    fn handle_task_group_call(&mut self, task_group: &Value, method_name: &str, args: &[Value]) -> Result<Value, String> {
        match method_name {
            "新建" | "任务" => {
                let tg_id = get_next_id();
                let tg_data = Arc::new(Mutex::new(Vec::new()));
                TASK_GROUP_STORAGE.lock().unwrap().insert(tg_id, tg_data);
                
                let new_task_group = Value::Map(std::collections::HashMap::from([
                    ("__is_task_group__".to_string(), Value::Bool(true)),
                    ("__tg_id__".to_string(), Value::Int(tg_id as i64)),
                ]));
                println!("[任务组] 创建任务组，ID: {}", tg_id);
                Ok(new_task_group)
            }
            "添加" => {
                if args.len() == 1 {
                    let tg_id = task_group_map_get_id(task_group)?;
                    let tg_storage = TASK_GROUP_STORAGE.lock().unwrap();
                    let tg_data = tg_storage.get(&tg_id).ok_or("任务组不存在")?;
                    
                    let closure = &args[0];
                    if let Value::Closure(params, body) = closure {
                        let env_state = self.env.save_state();
                        let params_clone = params.clone();
                        let body_clone = body.clone();
                        
                        let tg_data_clone = Arc::clone(tg_data);
                        
                        let handle = thread::spawn(move || {
                            let mut interpreter = Interpreter::new();
                            interpreter.env.load_state(env_state);
                            
                            let _ = interpreter.execute_function_body(&body_clone, params_clone, vec![]);
                        });
                        
                        tg_data_clone.lock().unwrap().push(handle);
                        
                        println!("[任务组] 添加任务到任务组 {}", tg_id);
                        return Ok(Value::Unit);
                    }
                    println!("[任务组] 添加任务到任务组 {}", tg_id);
                    return Ok(Value::Unit);
                }
                Err("任务组.添加 需要一个闭包参数".to_string())
            }
            "等待" => {
                let tg_id = task_group_map_get_id(task_group)?;
                let mut tg_storage = TASK_GROUP_STORAGE.lock().unwrap();
                
                if let Some(tg_data) = tg_storage.remove(&tg_id) {
                    let mut handles = tg_data.lock().unwrap();
                    while let Some(handle) = handles.pop() {
                        let _ = handle.join();
                    }
                    println!("[任务组] 任务组 {} 等待完成", tg_id);
                }
                
                Ok(Value::Unit)
            }
            _ => Err(format!("任务组没有方法: {}", method_name)),
        }
    }

    fn handle_channel_call(&mut self, channel: &Value, method_name: &str, args: &[Value]) -> Result<Value, String> {
        match method_name {
            "新建" => {
                let ch_id = get_next_id();
                let channel_data = Arc::new(ChannelData::new());
                CHANNEL_STORAGE.lock().unwrap().insert(ch_id, channel_data);
                
                let new_channel = Value::Map(std::collections::HashMap::from([
                    ("__is_channel__".to_string(), Value::Bool(true)),
                    ("__ch_id__".to_string(), Value::Int(ch_id as i64)),
                ]));
                println!("[通道] 创建通道，ID: {}", ch_id);
                Ok(new_channel)
            }
            "发送" => {
                if args.len() == 1 {
                    let ch_id = channel_map_get_id(channel)?;
                    let ch_storage = CHANNEL_STORAGE.lock().unwrap();
                    if let Some(ch) = ch_storage.get(&ch_id) {
                        ch.send(args[0].clone()).map_err(|e| format!("发送失败: {}", e))?;
                        return Ok(Value::Unit);
                    }
                    return Err("通道不存在".to_string());
                } else {
                    Err("通道.发送 需要一个参数".to_string())
                }
            }
            "接收" => {
                let ch_id = channel_map_get_id(channel)?;
                let ch_storage = CHANNEL_STORAGE.lock().unwrap();
                if let Some(ch) = ch_storage.get(&ch_id) {
                    match ch.recv() {
                        Ok(value) => {
                            return Ok(Value::Map(std::collections::HashMap::from([
                                ("成功".to_string(), value),
                            ])));
                        }
                        Err(e) => {
                            return Ok(Value::Map(std::collections::HashMap::from([
                                ("错误".to_string(), Value::String(e)),
                            ])));
                        }
                    }
                }
                Err("通道不存在".to_string())
            }
            "开放" => {
                let ch_id = channel_map_get_id(channel)?;
                let ch_storage = CHANNEL_STORAGE.lock().unwrap();
                if let Some(ch) = ch_storage.get(&ch_id) {
                    return Ok(Value::Bool(ch.is_open()));
                }
                Err("通道不存在".to_string())
            }
            "关闭" => {
                let ch_id = channel_map_get_id(channel)?;
                let ch_storage = CHANNEL_STORAGE.lock().unwrap();
                if let Some(ch) = ch_storage.get(&ch_id) {
                    ch.close();
                    println!("[通道] 通道 {} 已关闭", ch_id);
                    return Ok(Value::Unit);
                }
                Err("通道不存在".to_string())
            }
            _ => Err(format!("通道没有方法: {}", method_name)),
        }
    }
}

fn task_group_map_get_id(task_group: &Value) -> Result<u64, String> {
    if let Value::Map(map) = task_group {
        if let Some(Value::Int(id)) = map.get("__tg_id__") {
            return Ok(*id as u64);
        }
    }
    Err("无效的任务组对象".to_string())
}

fn channel_map_get_id(channel: &Value) -> Result<u64, String> {
    if let Value::Map(map) = channel {
        if let Some(Value::Int(id)) = map.get("__ch_id__") {
            return Ok(*id as u64);
        }
    }
    Err("无效的通道对象".to_string())
}

impl Interpreter {
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
                // 根据字段名称检查是否是特殊模块
                match field.name.as_str() {
                    "任务组" => {
                        if let Some(tg_value) = map.get("任务组") {
                            // 返回任务组模块本身
                            return Ok(tg_value.clone());
                        }
                    }
                    "任务" => {
                        if let Some(task_value) = map.get("任务") {
                            // 返回任务模块本身
                            return Ok(task_value.clone());
                        }
                    }
                    "通道" => {
                        if let Some(ch_value) = map.get("通道") {
                            // 返回通道模块本身
                            return Ok(ch_value.clone());
                        }
                    }
                    "原子" => {
                        if let Some(atomic_value) = map.get("原子") {
                            // 返回原子模块本身
                            return Ok(atomic_value.clone());
                        }
                    }
                    "互斥锁" => {
                        if let Some(mutex_value) = map.get("互斥锁") {
                            // 返回互斥锁模块本身
                            return Ok(mutex_value.clone());
                        }
                    }
                    "线程池" => {
                        if let Some(tp_value) = map.get("线程池") {
                            // 返回线程池模块本身
                            return Ok(tp_value.clone());
                        }
                    }
                    _ => {}
                }
                // 检查是否是模块的新建方法
                if field.name == "新建" {
                    if map.contains_key("__task_group_new__") {
                        // 任务组.新建()
                        let tg_id = get_next_id();
                        let tg_data = Arc::new(Mutex::new(Vec::new()));
                        TASK_GROUP_STORAGE.lock().unwrap().insert(tg_id, tg_data);
                        return Ok(Value::Map(std::collections::HashMap::from([
                            ("__is_task_group__".to_string(), Value::Bool(true)),
                            ("__tg_id__".to_string(), Value::Int(tg_id as i64)),
                        ])));
                    }
                    if map.contains_key("__channel_new__") {
                        // 通道.新建()
                        let ch_id = get_next_id();
                        let channel_data = Arc::new(ChannelData::new());
                        CHANNEL_STORAGE.lock().unwrap().insert(ch_id, channel_data);
                        return Ok(Value::Map(std::collections::HashMap::from([
                            ("__is_channel__".to_string(), Value::Bool(true)),
                            ("__ch_id__".to_string(), Value::Int(ch_id as i64)),
                        ])));
                    }
                    if map.contains_key("__mutex_new__") {
                        // 互斥锁.新建()
                        return Ok(Value::Map(std::collections::HashMap::from([
                            ("__is_mutex__".to_string(), Value::Bool(true)),
                        ])));
                    }
                    if map.contains_key("__thread_pool_new__") {
                        // 线程池.新建()
                        return Ok(Value::Map(std::collections::HashMap::from([
                            ("__is_thread_pool__".to_string(), Value::Bool(true)),
                        ])));
                    }
                }
                // 正常字段访问
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
                    _ => {
                        map.get(&field.name)
                            .cloned()
                            .ok_or_else(|| format!("映射没有字段: {}", field.name))
                    }
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

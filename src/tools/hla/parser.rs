
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

use std::collections::HashMap;
use crate::core::ast::Program;
use crate::core::lexer::token::SourceSpan;
use crate::tools::hla::types::*;
use crate::tools::hla::error::*;

/// HLA 解析器
pub struct HlaParser {
    _variables: HashMap<String, crate::core::ast::Type>,
    labels: HashMap<String, usize>,
    current_line: usize,
    errors: Vec<HlaParseError>,
}

impl Default for HlaParser {
    fn default() -> Self {
        Self::new()
    }
}

impl HlaParser {
    /// 创建新的解析器
    pub fn new() -> Self {
        Self {
            _variables: HashMap::new(),
            labels: HashMap::new(),
            current_line: 0,
            errors: Vec::new(),
        }
    }

    /// 解析 .hla 源码，返回 AST
    pub fn parse(&mut self, source: &str) -> ParseResult<Program> {

        let mut operations = Vec::new();

        for (line_num, line) in source.lines().enumerate() {
            self.current_line = line_num + 1;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            if trimmed.starts_with("#!") {
                continue;
            }

            if trimmed.starts_with('#') {
                continue;
            }

            match self.parse_operation(trimmed) {
                Ok(op) => {
                    if let Some(label) = &op.label {
                        if self.labels.contains_key(label) {
                            self.errors.push(HlaParseError::DuplicateLabel {
                                label: label.clone(),
                                line: self.current_line,
                            });
                        } else {
                            self.labels.insert(label.clone(), operations.len());
                        }
                    }
                    operations.push(op);
                }
                Err(e) => {
                    self.errors.push(e);
                }
            }
        }

        if !self.errors.is_empty() {
            return Err(std::mem::take(&mut self.errors));
        }

        let items = self.convert_operations_to_ast(&operations)?;
        Ok(items)
    }

    /// 解析操作行
    fn parse_operation(&mut self, line: &str) -> Result<HlaOperation, HlaParseError> {
        let (label, rest) = self.parse_label(line);
        let parts: Vec<&str> = rest.split_whitespace().collect();

        if parts.is_empty() {
            return Err(HlaParseError::InvalidOperand {
                expected: "至少一个操作码".to_string(),
                found: "空".to_string(),
                line: self.current_line,
            });
        }

        let opcode = Opcode::from_str(parts[0]);

        if opcode == Opcode::Unknown {
            return Err(HlaParseError::UnknownOpcode {
                opcode: parts[0].to_string(),
                line: self.current_line,
            });
        }

        let operands: Vec<String> = parts[1..].iter().map(|s| s.to_string()).collect();

        Ok(HlaOperation {
            label,
            opcode,
            operands,
            line_number: self.current_line,
        })
    }

    /// 解析标签
    fn parse_label<'a>(&self, line: &'a str) -> (Option<String>, &'a str) {
        if let Some(space_pos) = line.find(' ') {
            let first_part = &line[..space_pos];
            if first_part.starts_with('L') && first_part[1..].parse::<usize>().is_ok() {
                return (Some(first_part.to_string()), line[space_pos + 1..].trim());
            }
        }

        (None, line)
    }

    /// 解析类型
    pub fn parse_type(&self, s: &str) -> crate::core::ast::Type {
        match s {
            "整数" => crate::core::ast::Type::Int,
            "整数8" => crate::core::ast::Type::I8,
            "整数16" => crate::core::ast::Type::I16,
            "整数32" => crate::core::ast::Type::I32,
            "整数64" => crate::core::ast::Type::I64,
            "无符号8" => crate::core::ast::Type::U8,
            "无符号16" => crate::core::ast::Type::U16,
            "无符号32" => crate::core::ast::Type::U32,
            "无符号64" => crate::core::ast::Type::U64,
            "浮点32" => crate::core::ast::Type::F32,
            "浮点64" => crate::core::ast::Type::F64,
            "布尔" => crate::core::ast::Type::Bool,
            "字符" => crate::core::ast::Type::Char,
            "字符串" => crate::core::ast::Type::String,
            "单元" => crate::core::ast::Type::Unit,
            s if s.starts_with("列表[") => {
                let inner = &s[3..s.len()-1].trim();
                crate::core::ast::Type::List(Box::new(self.parse_type(inner)))
            }
            s if s.starts_with("指针[") => {
                let inner = &s[3..s.len()-1].trim();
                crate::core::ast::Type::Ptr(Box::new(self.parse_type(inner)))
            }
            s if s.starts_with("可选[") => {
                let inner = &s[3..s.len()-1].trim();
                crate::core::ast::Type::Option(Box::new(self.parse_type(inner)))
            }
            _ => {
                let ident = crate::core::ast::Ident::dummy(s);
                let path = crate::core::ast::Path::from_ident(ident);
                crate::core::ast::Type::Named(path)
            }
        }
    }

    /// 解析值为表达式
    pub fn parse_value(&self, s: &str) -> crate::core::ast::Expr {
        if s == "真" {
            return crate::core::ast::Expr::BoolLit(true, SourceSpan::default());
        } else if s == "假" {
            return crate::core::ast::Expr::BoolLit(false, SourceSpan::default());
        } else if s == "空" {
            return crate::core::ast::Expr::Null(SourceSpan::default());
        } else if s.starts_with('"') && s.ends_with('"') {
            let content = &s[1..s.len()-1];
            return crate::core::ast::Expr::StringLit(content.to_string(), SourceSpan::default());
        } else if s.starts_with('\'') && s.len() >= 3 && s.ends_with('\'') {
            let c = s.chars().nth(1).unwrap_or('?');
            return crate::core::ast::Expr::CharLit(c, SourceSpan::default());
        } else if let Ok(n) = s.parse::<i64>() {
            return crate::core::ast::Expr::IntLit(n, SourceSpan::default());
        } else if let Ok(n) = s.parse::<f64>() {
            return crate::core::ast::Expr::FloatLit(n, SourceSpan::default());
        }

        crate::core::ast::Expr::Ident(crate::core::ast::Ident::dummy(s))
    }

    /// 转换操作到 AST
    fn convert_operations_to_ast(&self, operations: &[HlaOperation]) -> ParseResult<Program> {
        let mut program = Vec::new();
        let mut current_function: Option<crate::core::ast::Function> = None;

        for op in operations {
            match op.opcode {
                Opcode::Hanshu => {
                    if let Some(func) = current_function.take() {
                        program.push(crate::core::ast::Item::Function(func));
                    }
                    current_function = Some(self.parse_hanshu(op).map_err(|e| vec![e])?);
                }
                Opcode::Fanhui => {
                    let stmt = self.parse_fanhui(op);
                    if let Some(func) = &mut current_function {
                        func.body.push(stmt);
                    }
                }
                Opcode::Jieshu => {
                    if let Some(func) = current_function.take() {
                        program.push(crate::core::ast::Item::Function(func));
                    }
                }
                _ => {
                    if let Some(stmt) = self.parse_statement(op) {
                        if let Some(func) = &mut current_function {
                            func.body.push(stmt);
                        }
                    }
                }
            }
        }

        if let Some(func) = current_function {
            program.push(crate::core::ast::Item::Function(func));
        }

        Ok(program)
    }

    /// 解析函数定义
    fn parse_hanshu(&self, op: &HlaOperation) -> Result<crate::core::ast::Function, HlaParseError> {
        if op.operands.is_empty() {
            return Err(HlaParseError::InvalidOperand {
                expected: "HANSHU 至少需要一个操作数".to_string(),
                found: op.operands.join(" "),
                line: op.line_number,
            });
        }

        let name = crate::core::ast::Ident::dummy(&op.operands[0]);
        let mut params = Vec::new();
        let mut return_type = crate::core::ast::Type::Unit;

        for operand in op.operands.iter().skip(1) {
            if operand.starts_with("返回:") {
                return_type = self.parse_type(&operand[3..]);
            } else if let Some(colon_pos) = operand.find(':') {
                let param_name = &operand[..colon_pos];
                let param_type = self.parse_type(&operand[colon_pos + 1..]);
                params.push((crate::core::ast::Ident::dummy(param_name), param_type));
            }
        }

        Ok(crate::core::ast::Function {
            public: false,
            is_async: false,
            name,
            generics: Vec::new(),
            params,
            return_type,
            where_clause: Vec::new(),
            preconditions: Vec::new(),
            postconditions: Vec::new(),
            body: Vec::new(),
            span: SourceSpan::default(),
        })
    }

    /// 解析返回语句
    fn parse_fanhui(&self, op: &HlaOperation) -> crate::core::ast::Stmt {
        let value = op.operands.first().map(|s| Box::new(self.parse_value(s)));
        crate::core::ast::Stmt::Return(value, SourceSpan::default())
    }

    /// 解析语句
    fn parse_statement(&self, op: &HlaOperation) -> Option<crate::core::ast::Stmt> {
        match op.opcode {
            Opcode::Ling => {
                if op.operands.len() >= 3 {
                    let name = crate::core::ast::Ident::dummy(&op.operands[0]);
                    let ty = Some(self.parse_type(&op.operands[1]));
                    let value = Box::new(self.parse_value(&op.operands[2]));
                    Some(crate::core::ast::Stmt::Let {
                        name,
                        ty,
                        value,
                        span: SourceSpan::default(),
                    })
                } else {
                    None
                }
            }
            Opcode::Ding => {
                if op.operands.len() >= 3 {
                    let name = crate::core::ast::Ident::dummy(&op.operands[0]);
                    let ty = Some(self.parse_type(&op.operands[1]));
                    let value = Box::new(self.parse_value(&op.operands[2]));
                    Some(crate::core::ast::Stmt::Const {
                        name,
                        ty,
                        value,
                        span: SourceSpan::default(),
                    })
                } else {
                    None
                }
            }
            Opcode::Jia | Opcode::Jian | Opcode::Cheng | Opcode::Chu | Opcode::Quyu |
            Opcode::Dayu | Opcode::Xiaoyu | Opcode::Dengyu => {
                self.parse_binary_operation(op)
            }
            Opcode::Diaoyong => {
                self.parse_function_call(op)
            }
            _ => None,
        }
    }

    /// 解析二元操作
    fn parse_binary_operation(&self, op: &HlaOperation) -> Option<crate::core::ast::Stmt> {
        if op.operands.len() < 3 {
            return None;
        }

        let dest_name = crate::core::ast::Ident::dummy(&op.operands[0]);
        let left = Box::new(self.parse_value(&op.operands[1]));
        let right = Box::new(self.parse_value(&op.operands[2]));

        let binary_op = match op.opcode {
            Opcode::Jia => crate::core::ast::BinaryOp::Add,
            Opcode::Jian => crate::core::ast::BinaryOp::Sub,
            Opcode::Cheng => crate::core::ast::BinaryOp::Mul,
            Opcode::Chu => crate::core::ast::BinaryOp::Div,
            Opcode::Quyu => crate::core::ast::BinaryOp::Mod,
            Opcode::Dayu => crate::core::ast::BinaryOp::Gt,
            Opcode::Xiaoyu => crate::core::ast::BinaryOp::Lt,
            Opcode::Dengyu => crate::core::ast::BinaryOp::Eq,
            _ => return None,
        };

        let expr = crate::core::ast::Expr::BinaryOp {
            op: binary_op,
            left,
            right,
            span: SourceSpan::default(),
        };

        Some(crate::core::ast::Stmt::Let {
            name: dest_name,
            ty: None,
            value: Box::new(expr),
            span: SourceSpan::default(),
        })
    }

    /// 解析函数调用
    fn parse_function_call(&self, op: &HlaOperation) -> Option<crate::core::ast::Stmt> {
        if op.operands.len() < 2 {
            return None;
        }

        let dest_name = crate::core::ast::Ident::dummy(&op.operands[0]);
        let func_name = &op.operands[1];
        let args: Vec<_> = op.operands.iter().skip(2).map(|s| self.parse_value(s)).collect();

        let func_expr = Box::new(crate::core::ast::Expr::Ident(crate::core::ast::Ident::dummy(func_name)));

        let call_expr = crate::core::ast::Expr::Call {
            func: func_expr,
            args,
            span: SourceSpan::default(),
        };

        Some(crate::core::ast::Stmt::Let {
            name: dest_name,
            ty: None,
            value: Box::new(call_expr),
            span: SourceSpan::default(),
        })
    }

    /// 从文件解析
    pub fn parse_file(path: &std::path::Path) -> ParseResult<Program> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| vec![HlaParseError::IoError(e.to_string())])?;
        let mut parser = Self::new();
        parser.parse(&content)
    }
}

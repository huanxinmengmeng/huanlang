
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

use crate::tools::hla::types::*;
use crate::tools::hla::error::*;

/// HLA 序列化器
pub struct HlaSerializer {
    label_counter: usize,
    temp_counter: usize,
    output: String,
}

impl Default for HlaSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl HlaSerializer {
    /// 创建新的序列化器
    pub fn new() -> Self {
        Self {
            label_counter: 1,
            temp_counter: 1,
            output: String::new(),
        }
    }

    /// 序列化 AST 到 HLA 字符串
    pub fn serialize(&mut self, program: &Program) -> SerializeResult<String> {
        self.output.clear();
        self.emit_metadata();

        for item in program {
            self.serialize_item(item)?;
        }

        Ok(self.output.clone())
    }

    /// 序列化到文件
    pub fn serialize_to_file(&mut self, program: &Program, path: &std::path::Path) -> SerializeResult<()> {
        let content = self.serialize(program)?;
        std::fs::write(path, content)
            .map_err(|e| HlaSerializeError::IoError(e.to_string()))?;
        Ok(())
    }

    /// 发射元数据
    fn emit_metadata(&mut self) {
        self.output.push_str("#! 版本 = \"1.2\"\n");
        self.output.push_str("#! 来源 = \"huan-compiler\"\n");
        self.output.push_str("#! 关键词风格 = \"中文\"\n");
        self.output.push_str("#! 编码 = \"UTF-8\"\n");
        self.output.push('\n');
    }

    /// 获取下一个标签
    fn next_label(&mut self) -> String {
        let label = format!("L{:03}", self.label_counter);
        self.label_counter += 1;
        label
    }

    /// 获取下一个临时变量名
    fn fresh_temp(&mut self) -> String {
        let temp = format!("t{}", self.temp_counter);
        self.temp_counter += 1;
        temp
    }

    /// 发射一行操作
    fn emit_line(&mut self, label: Option<&str>, opcode: Opcode, operands: &[String]) {
        if let Some(lbl) = label {
            self.output.push_str(lbl);
            self.output.push(' ');
        }
        self.output.push_str(opcode.to_str());
        for op in operands {
            self.output.push(' ');
            self.output.push_str(op);
        }
        self.output.push('\n');
    }

    /// 序列化项目
    fn serialize_item(&mut self, item: &Item) -> SerializeResult<()> {
        match item {
            Item::Function(func) => self.serialize_function(func),
            Item::Struct(s) => self.serialize_struct(s),
            Item::Global(g) => self.serialize_global(g),
            _ => {}
        }
        Ok(())
    }

    /// 序列化函数
    fn serialize_function(&mut self, func: &Function) {
        let label = self.next_label();
        let mut operands = vec![func.name.name.clone()];

        for (name, ty) in &func.params {
            let ty_str = self.type_to_string(ty);
            operands.push(format!("{}:{}", name.name, ty_str));
        }

        let ret_str = self.type_to_string(&func.return_type);
        operands.push(format!("返回:{}", ret_str));

        self.emit_line(Some(&label), Opcode::Hanshu, &operands);

        for stmt in &func.body {
            self.serialize_stmt(stmt);
        }

        let end_label = self.next_label();
        self.emit_line(Some(&end_label), Opcode::Jieshu, &[]);
    }

    /// 序列化语句
    fn serialize_stmt(&mut self, stmt: &Stmt) {
        let label = self.next_label();

        match stmt {
            Stmt::Let { name, ty, value, .. } => {
                let ty_str = ty.as_ref().map(|t| self.type_to_string(t)).unwrap_or("".to_string());
                let val_str = self.expr_to_string(value);
                self.emit_line(Some(&label), Opcode::Ling, &[name.name.clone(), ty_str, val_str]);
            }

            Stmt::Const { name, ty, value, .. } => {
                let ty_str = ty.as_ref().map(|t| self.type_to_string(t)).unwrap_or("".to_string());
                let val_str = self.expr_to_string(value);
                self.emit_line(Some(&label), Opcode::Ding, &[name.name.clone(), ty_str, val_str]);
            }

            Stmt::Return(Some(expr), ..) => {
                self.emit_line(Some(&label), Opcode::Fanhui, &[self.expr_to_string(expr)]);
            }

            Stmt::Return(None, ..) => {
                self.emit_line(Some(&label), Opcode::Fanhui, &[]);
            }

            Stmt::Expr(expr, ..) => {
                self.serialize_expr_stmt(expr, Some(label));
            }

            Stmt::If { .. } => {
                self.emit_line(Some(&label), Opcode::Jieshu, &[]);
            }

            _ => {}
        }
    }

    /// 序列化表达式语句
    fn serialize_expr_stmt(&mut self, expr: &Expr, label: Option<String>) {
        let label_str = label.unwrap_or_else(|| {
            self.next_label()
        });

        match expr {
            Expr::BinaryOp { op, left, right, .. } => {
                let opcode = self.binary_op_to_opcode(op);
                let dest = self.fresh_temp();
                let left_str = self.expr_to_string(left);
                let right_str = self.expr_to_string(right);

                if let Some(opcode) = opcode {
                    self.emit_line(Some(&label_str), opcode, &[dest, left_str, right_str]);
                }
            }

            Expr::Call { func, args, .. } => {
                if let Expr::Ident(ident) = func.as_ref() {
                    let dest = self.fresh_temp();
                    let mut operands = vec![dest, ident.name.clone()];
                    operands.extend(args.iter().map(|a| self.expr_to_string(a)));
                    self.emit_line(Some(&label_str), Opcode::Diaoyong, &operands);
                }
            }

            _ => {}
        }
    }

    /// 表达式转字符串
    fn expr_to_string(&self, expr: &Expr) -> String {
        match expr {
            Expr::IntLit(n, ..) => n.to_string(),
            Expr::FloatLit(n, ..) => n.to_string(),
            Expr::StringLit(s, ..) => format!("\"{}\"", s),
            Expr::CharLit(c, ..) => format!("'{}'", c),
            Expr::BoolLit(b, ..) => if *b { "真".to_string() } else { "假".to_string() },
            Expr::Null(..) => "空".to_string(),
            Expr::Ident(ident) => ident.name.clone(),
            _ => "?".to_string(),
        }
    }

    /// 类型转字符串
    fn type_to_string(&self, ty: &Type) -> String {
        match ty {
            Type::Int => "整数".to_string(),
            Type::I8 => "整数8".to_string(),
            Type::I16 => "整数16".to_string(),
            Type::I32 => "整数32".to_string(),
            Type::I64 => "整数64".to_string(),
            Type::U8 => "无符号8".to_string(),
            Type::U16 => "无符号16".to_string(),
            Type::U32 => "无符号32".to_string(),
            Type::U64 => "无符号64".to_string(),
            Type::F32 => "浮点32".to_string(),
            Type::F64 => "浮点64".to_string(),
            Type::Bool => "布尔".to_string(),
            Type::Char => "字符".to_string(),
            Type::String => "字符串".to_string(),
            Type::Unit => "单元".to_string(),
            Type::List(inner) => format!("列表[{}]", self.type_to_string(inner)),
            Type::Map(key, val) => format!("字典[{},{}]", self.type_to_string(key), self.type_to_string(val)),
            Type::Ptr(inner) => format!("指针[{}]", self.type_to_string(inner)),
            Type::Option(inner) => format!("可选[{}]", self.type_to_string(inner)),
            Type::Named(path) => {
                path.segments.iter()
                    .map(|s| s.name.clone())
                    .collect::<Vec<_>>()
                    .join("::")
            }
            _ => "?".to_string(),
        }
    }

    /// 二元运算符转操作码
    fn binary_op_to_opcode(&self, op: &BinaryOp) -> Option<Opcode> {
        match op {
            BinaryOp::Add => Some(Opcode::Jia),
            BinaryOp::Sub => Some(Opcode::Jian),
            BinaryOp::Mul => Some(Opcode::Cheng),
            BinaryOp::Div => Some(Opcode::Chu),
            BinaryOp::Mod => Some(Opcode::Quyu),
            BinaryOp::Gt => Some(Opcode::Dayu),
            BinaryOp::Lt => Some(Opcode::Xiaoyu),
            BinaryOp::Eq => Some(Opcode::Dengyu),
            _ => None,
        }
    }

    /// 序列化结构体
    fn serialize_struct(&mut self, _s: &Struct) {
    }

    /// 序列化全局变量
    fn serialize_global(&mut self, _g: &Global) {
    }
}

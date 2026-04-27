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

pub struct CodeGen {
    output: String,
    indent: usize,
}

impl CodeGen {
    pub fn new() -> Self {
        Self {
            output: String::new(),
            indent: 0,
        }
    }

    fn generate_indent(&mut self) {
        self.output.push_str(&" ".repeat(self.indent * 4));
    }

    pub fn generate(&mut self, program: &Program) -> String {
        self.output.clear();
        for item in program {
            self.generate_item(item);
        }
        self.output.clone()
    }

    fn generate_item(&mut self, item: &Item) {
        match item {
            Item::Function(func) => self.generate_function(func),
            Item::Struct(struct_def) => self.generate_struct(struct_def),
            Item::Trait(_) => {}
            Item::Impl(_) => {}
            Item::Module(module_def) => self.generate_module(module_def),
            Item::Import(_) => {}
            Item::Global(global_var) => self.generate_global(global_var),
            Item::TypeAlias(type_alias) => self.generate_type_alias(type_alias),
            Item::Extern(_) => {}
            Item::Peripheral(peripheral) => self.generate_peripheral(peripheral),
            Item::MemoryLayout(layout) => self.generate_memory_layout(layout),
        }
    }

    fn generate_function(&mut self, func: &Function) {
        let visibility = if func.public { "pub " } else { "" };
        self.output.push_str(&format!("{}fn {}(", visibility, func.name.name));
        for (i, param) in func.params.iter().enumerate() {
            if i > 0 {
                self.output.push_str(", ");
            }
            self.output.push_str(&format!("{}: ", param.0.name));
            self.generate_type(&param.1);
        }
        self.output.push_str(")");
        self.output.push_str(" -> ");
        self.generate_type(&func.return_type);
        self.output.push_str(" {\n");
        self.indent += 1;
        self.generate_block(&func.body);
        self.indent -= 1;
        self.output.push_str("}\n\n");
    }

    fn generate_struct(&mut self, struct_def: &Struct) {
        let visibility = if struct_def.public { "pub " } else { "" };
        self.output.push_str(&format!("{}struct {} {{\n", visibility, struct_def.name.name));
        self.indent += 1;
        for field in &struct_def.fields {
            self.output.push_str(&format!("{}: ", field.0.name));
            self.generate_type(&field.1);
            self.output.push_str(",\n");
        }
        self.indent -= 1;
        self.output.push_str("}\n\n");
    }

    fn generate_module(&mut self, module_def: &Module) {
        let visibility = if module_def.public { "pub " } else { "" };
        self.output.push_str(&format!("{}module {} {{\n", visibility, module_def.name.name));
        self.indent += 1;
        for item in &module_def.items {
            self.generate_item(item);
        }
        self.indent -= 1;
        self.output.push_str("}\n\n");
    }

    fn generate_global(&mut self, global_var: &Global) {
        let visibility = "pub ";
        let keyword = if !global_var.mutable { "const" } else { "let" };
        self.output.push_str(&format!("{}{} {}", visibility, keyword, global_var.name.name));
        if let Some(var_type) = &global_var.ty {
            self.output.push_str(": ");
            self.generate_type(var_type);
        }
        self.output.push_str(" = ");
        self.generate_expr(&global_var.value);
        self.output.push_str(";\n");
    }

    fn generate_type_alias(&mut self, type_alias: &TypeAlias) {
        let visibility = if type_alias.public { "pub " } else { "" };
        self.output.push_str(&format!("{}type {} = ", visibility, type_alias.name.name));
        self.generate_type(&type_alias.ty);
        self.output.push_str(";\n");
    }

    fn generate_peripheral(&mut self, peripheral: &PeripheralDef) {
        self.output.push_str(&format!("peripheral {} @ 0x{:X} {{\n", 
            peripheral.name.name, peripheral.base_addr));
        for reg in &peripheral.registers {
            let access_str = match reg.access {
                RegisterAccess::ReadOnly => "readonly",
                RegisterAccess::WriteOnly => "writeonly",
                RegisterAccess::ReadWrite => "readwrite",
            };
            self.output.push_str(&format!("  {} @ 0x{:X}: ", reg.name.name, reg.offset));
            self.generate_type(&reg.ty);
            self.output.push_str(&format!(" ({})\n", access_str));
        }
        self.output.push_str("}\n");
    }

    fn generate_memory_layout(&mut self, layout: &MemoryLayout) {
        self.output.push_str(&format!("memory_layout {} {{\n", layout.name.name));
        for region in &layout.regions {
            self.output.push_str(&format!("  {} @ 0x{:X} size 0x{:X} (", 
                region.name, region.start, region.size));
            let attrs: Vec<&str> = region.attributes.iter().map(|a| match a {
                MemoryAttr::Readable => "readable",
                MemoryAttr::Writable => "writable",
                MemoryAttr::Executable => "executable",
            }).collect();
            self.output.push_str(&attrs.join(", "));
            self.output.push_str(")\n");
        }
        for segment in &layout.segments {
            self.output.push_str(&format!("  .{} in {} align {}\n", 
                segment.name, segment.region, segment.alignment));
        }
        self.output.push_str("}\n");
    }

    fn generate_block(&mut self, block: &Vec<Stmt>) {
        for stmt in block {
            self.generate_stmt(stmt);
        }
    }

    fn generate_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, ty, value, .. } => {
                self.generate_indent();
                self.output.push_str("let ");
                self.output.push_str(&name.name);
                if let Some(ty) = ty {
                    self.output.push_str(": ");
                    self.generate_type(ty);
                }
                self.output.push_str(" = ");
                self.generate_expr(value);
                self.output.push_str(";
");
            }
            Stmt::Expr(expr, _) => {
                self.generate_indent();
                self.generate_expr(expr);
                self.output.push_str(";
");
            }
            Stmt::Return(expr, _) => {
                self.generate_indent();
                self.output.push_str("return ");
                if let Some(e) = expr {
                    self.generate_expr(e);
                }
                self.output.push_str(";
");
            }
            Stmt::While { cond, body, .. } => {
                self.generate_indent();
                self.output.push_str("while ");
                self.generate_expr(cond);
                self.output.push_str(" {\n");
                self.indent += 1;
                self.generate_block(body);
                self.indent -= 1;
                self.generate_indent();
                self.output.push_str("}\n");
            }
            Stmt::Repeat { count, body, .. } => {
                self.generate_indent();
                self.output.push_str("repeat ");
                self.generate_expr(count);
                self.output.push_str(" {\n");
                self.indent += 1;
                self.generate_block(body);
                self.indent -= 1;
                self.generate_indent();
                self.output.push_str("}\n");
            }
            Stmt::ForEach { var, iterable, body, .. } => {
                self.generate_indent();
                self.output.push_str(&format!("for {} in ", var.name));
                self.generate_expr(iterable);
                self.output.push_str(" {\n");
                self.indent += 1;
                self.generate_block(body);
                self.indent -= 1;
                self.generate_indent();
                self.output.push_str("}\n");
            }
            Stmt::Match { expr, arms, default, .. } => {
                self.generate_indent();
                self.output.push_str("match ");
                self.generate_expr(expr);
                self.output.push_str(" {\n");
                self.indent += 1;
                for (pattern, body) in arms {
                    self.generate_indent();
                    self.generate_pattern(pattern);
                    self.output.push_str(" => {\n");
                    self.indent += 1;
                    self.generate_block(body);
                    self.indent -= 1;
                    self.generate_indent();
                    self.output.push_str("}\n");
                }
                if let Some(default) = default {
                    self.generate_indent();
                    self.output.push_str("_ => {\n");
                    self.indent += 1;
                    self.generate_block(default);
                    self.indent -= 1;
                    self.generate_indent();
                    self.output.push_str("}\n");
                }
                self.indent -= 1;
                self.generate_indent();
                self.output.push_str("}\n");
            }
            Stmt::Break(_) => {
                self.generate_indent();
                self.output.push_str("break;\n");
            },
            Stmt::Continue(_) => {
                self.generate_indent();
                self.output.push_str("continue;\n");
            },
            Stmt::Const { name, ty, value, .. } => {
                self.generate_indent();
                self.output.push_str("const ");
                self.output.push_str(&name.name);
                if let Some(ty) = ty {
                    self.output.push_str(": ");
                    self.generate_type(ty);
                }
                self.output.push_str(" = ");
                self.generate_expr(value);
                self.output.push_str(";\n");
            },
            Stmt::Assign { target, value, .. } => {
                self.generate_indent();
                self.generate_expr(target);
                self.output.push_str(" = ");
                self.generate_expr(value);
                self.output.push_str(";\n");
            },
            Stmt::If { cond, then_block, else_ifs, else_block, .. } => {
                self.generate_indent();
                self.output.push_str("if ");
                self.generate_expr(cond);
                self.output.push_str(" {\n");
                self.indent += 1;
                self.generate_block(then_block);
                self.indent -= 1;
                self.generate_indent();
                self.output.push_str("}");
                for (elif_cond, elif_block) in else_ifs {
                    self.output.push_str(" else if ");
                    self.generate_expr(elif_cond);
                    self.output.push_str(" {\n");
                    self.indent += 1;
                    self.generate_block(elif_block);
                    self.indent -= 1;
                    self.generate_indent();
                    self.output.push_str("}");
                }
                if let Some(else_block) = else_block {
                    self.output.push_str(" else {\n");
                    self.indent += 1;
                    self.generate_block(else_block);
                    self.indent -= 1;
                    self.generate_indent();
                    self.output.push_str("}");
                }
                self.output.push_str("\n");
            },
            Stmt::Asm(asm, _) => {
                self.generate_indent();
                self.output.push_str("asm {\n");
                self.indent += 1;
                for template in &asm.templates {
                    self.generate_indent();
                    self.output.push_str(&format!("{}\n", template));
                }
                self.indent -= 1;
                self.generate_indent();
                self.output.push_str("}\n");
            },
        }
    }

    fn generate_pattern(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Wildcard(_) => self.output.push_str("_"),
            Pattern::Literal(lit) => self.generate_literal(lit),
            Pattern::Ident(ident) => self.output.push_str(&ident.name),
            Pattern::Struct { path, fields, .. } => {
                self.output.push_str(&path.segments.last().unwrap().name);
                self.output.push_str(" {");
                for (i, (field_name, field_pattern)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.output.push_str(&field_name.name);
                    self.output.push_str(": ");
                    self.generate_pattern(field_pattern);
                }
                self.output.push_str("}");
            }

            Pattern::Or(p1, p2, _) => {
                self.generate_pattern(p1);
                self.output.push_str(" | ");
                self.generate_pattern(p2);
            }
            Pattern::List(patterns, _) => {
                self.output.push_str("[");
                for (i, pattern) in patterns.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.generate_pattern(pattern);
                }
                self.output.push_str("]");
            }
        }
    }

    fn generate_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::IntLit(n, _) => self.output.push_str(&n.to_string()),
            Expr::FloatLit(n, _) => self.output.push_str(&n.to_string()),
            Expr::StringLit(s, _) => self.output.push_str(&format!("\"{}\"", s)),
            Expr::CharLit(c, _) => self.output.push_str(&format!("'{}'", c)),
            Expr::BoolLit(b, _) => self.output.push_str(if *b { "true" } else { "false" }),
            Expr::Null(_) => self.output.push_str("null"),
            Expr::Ident(ident) => self.output.push_str(&ident.name),
            Expr::BinaryOp { op, left, right, .. } => {
                self.output.push_str("(");
                self.generate_expr(left);
                self.output.push_str(&format!(" {} ", self.binary_op_str(op)));
                self.generate_expr(right);
                self.output.push_str(")");
            }
            Expr::UnaryOp { op, expr, .. } => {
                self.output.push_str(&format!("{}{}", self.unary_op_str(op), ""));
                self.generate_expr(expr);
            }
            Expr::Call { func, args, .. } => {
                self.generate_expr(func);
                self.output.push_str("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.generate_expr(arg);
                }
                self.output.push_str(")");
            }
            Expr::Index { target, index, .. } => {
                self.generate_expr(target);
                self.output.push_str("[");
                self.generate_expr(index);
                self.output.push_str("]");
            }
            Expr::Field { target, field, .. } => {
                self.generate_expr(target);
                self.output.push_str(".");
                self.output.push_str(&field.name);
            }
            Expr::MethodCall { receiver, method, args, .. } => {
                self.generate_expr(receiver);
                self.output.push_str(".");
                self.output.push_str(&method.name);
                self.output.push_str("(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.generate_expr(arg);
                }
                self.output.push_str(")");
            }
            Expr::IfExpr { cond, then_expr, else_expr, .. } => {
                self.output.push_str("if ");
                self.generate_expr(cond);
                self.output.push_str(" {");
                self.indent += 1;
                self.generate_expr(then_expr);
                self.indent -= 1;
                self.output.push_str("} else {");
                self.indent += 1;
                self.generate_expr(else_expr);
                self.indent -= 1;
                self.output.push_str("}");
            }
            Expr::Match { expr, arms, default, .. } => {
                self.output.push_str("match ");
                self.generate_expr(expr);
                self.output.push_str(" {\n");
                self.indent += 1;
                for (pattern, body) in arms {
                    self.generate_indent();
                    self.generate_pattern(pattern);
                    self.output.push_str(" => {");
                    self.indent += 1;
                    self.generate_expr(body);
                    self.indent -= 1;
                    self.generate_indent();
                    self.output.push_str("}\n");
                }
                if let Some(default) = default {
                    self.generate_indent();
                    self.output.push_str("_ => {");
                    self.indent += 1;
                    self.generate_expr(default);
                    self.indent -= 1;
                    self.generate_indent();
                    self.output.push_str("}\n");
                }
                self.indent -= 1;
                self.generate_indent();
                self.output.push_str("}");
            }
            Expr::List(exprs, _) => {
                self.output.push_str("[");
                for (i, expr) in exprs.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.generate_expr(expr);
                }
                self.output.push_str("]");
            }
            Expr::Map(pairs, _) => {
                self.output.push_str("{");
                for (i, (key, value)) in pairs.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.generate_expr(key);
                    self.output.push_str(": ");
                    self.generate_expr(value);
                }
                self.output.push_str("}");
            }
            Expr::Closure { params, body, .. } => {
                self.output.push_str("|");
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.output.push_str(&param.0.name);
                }
                self.output.push_str("| {");
                self.indent += 1;
                self.generate_block(body);
                self.indent -= 1;
                self.generate_indent();
                self.output.push_str("}");
            }
            Expr::Asm(_) => self.output.push_str("asm {}"),
            Expr::TypeAssertion { expr, ty, .. } => {
                self.generate_expr(expr);
                self.output.push_str(" as ");
                self.generate_type(ty);
            }
            Expr::Struct { path, fields, .. } => {
                self.output.push_str(&path.segments.last().unwrap().name);
                self.output.push_str(" {");
                for (i, (field_name, field_value)) in fields.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.output.push_str(&field_name.name);
                    self.output.push_str(": ");
                    self.generate_expr(field_value);
                }
                self.output.push_str("}");
            }
            Expr::Try { expr, .. } => {
                self.output.push_str("try ");
                self.generate_expr(expr);
            }
        }
    }

    fn generate_type(&mut self, type_: &Type) {
        match type_ {
            Type::Int => self.output.push_str("int"),
            Type::I8 => self.output.push_str("i8"),
            Type::I16 => self.output.push_str("i16"),
            Type::I32 => self.output.push_str("i32"),
            Type::I64 => self.output.push_str("i64"),
            Type::U8 => self.output.push_str("u8"),
            Type::U16 => self.output.push_str("u16"),
            Type::U32 => self.output.push_str("u32"),
            Type::U64 => self.output.push_str("u64"),
            Type::F32 => self.output.push_str("f32"),
            Type::F64 => self.output.push_str("f64"),
            Type::Bool => self.output.push_str("bool"),
            Type::Char => self.output.push_str("char"),
            Type::String => self.output.push_str("string"),
            Type::Unit => self.output.push_str("unit"),
            Type::List(inner) => {
                self.output.push_str("list<");
                self.generate_type(inner);
                self.output.push_str(">");
            }
            Type::Array(inner, size) => {
                self.output.push_str("array<");
                self.generate_type(inner);
                self.output.push_str(", ");
                self.output.push_str(&format!("{:?}", size));
                self.output.push_str(">");
            }
            Type::Map(key, value) => {
                self.output.push_str("map<");
                self.generate_type(key);
                self.output.push_str(", ");
                self.generate_type(value);
                self.output.push_str(">");
            }
            Type::Ptr(inner) => {
                self.output.push_str("ptr<");
                self.generate_type(inner);
                self.output.push_str(">");
            }
            Type::Option(inner) => {
                self.output.push_str("option<");
                self.generate_type(inner);
                self.output.push_str(">");
            }
            Type::Named(ident) => {
                let segments: Vec<String> = ident.segments.iter().map(|seg| seg.name.clone()).collect();
                self.output.push_str(&segments.join("::"));
            },
            Type::Func(params, return_type) => {
                self.output.push_str("func<");
                for (i, param) in params.iter().enumerate() {
                    if i > 0 {
                        self.output.push_str(", ");
                    }
                    self.generate_type(param);
                }
                self.output.push_str(", ");
                self.generate_type(return_type);
                self.output.push_str(">" );
            },
            Type::Var(id) => {
                self.output.push_str(&format!("var{}", id));
            },
        }
    }

    fn binary_op_str(&self, op: &BinaryOp) -> &'static str {
        match op {
            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",
            BinaryOp::And => "&&",
            BinaryOp::Or => "||",
            BinaryOp::Eq => "==",
            BinaryOp::Ne => "!=",
            BinaryOp::Lt => "<",
            BinaryOp::Gt => ">",
            BinaryOp::Le => "<=",
            BinaryOp::Ge => ">=",
            BinaryOp::Shl => "<<",
            BinaryOp::Shr => ">>",
            BinaryOp::BitAnd => "&",
            BinaryOp::BitOr => "|",
            BinaryOp::BitXor => "^",
            BinaryOp::Assign => "=",
            BinaryOp::AddAssign => "+",
            BinaryOp::SubAssign => "-",
            BinaryOp::MulAssign => "*",
            BinaryOp::DivAssign => "/",
            BinaryOp::ModAssign => "%",
            BinaryOp::ShlAssign => "<<",
            BinaryOp::ShrAssign => ">>",
            BinaryOp::BitAndAssign => "&",
            BinaryOp::BitOrAssign => "|",
            BinaryOp::BitXorAssign => "^",
        }
    }

    fn unary_op_str(&self, op: &UnaryOp) -> &'static str {
        match op {
            UnaryOp::Neg => "-",
            UnaryOp::Not => "!",
            UnaryOp::BitNot => "~",
        }
    }

    fn generate_literal(&mut self, lit: &Expr) {
        match lit {
            Expr::IntLit(n, _) => self.output.push_str(&n.to_string()),
            Expr::FloatLit(n, _) => self.output.push_str(&n.to_string()),
            Expr::CharLit(c, _) => self.output.push_str(&format!("'{}'", c)),
            Expr::StringLit(s, _) => self.output.push_str(&format!("\"{}\"", s)),
            Expr::BoolLit(b, _) => self.output.push_str(if *b { "true" } else { "false" }),
            Expr::Null(_) => self.output.push_str("null"),
            _ => self.output.push_str("null"),
        }
    }
}

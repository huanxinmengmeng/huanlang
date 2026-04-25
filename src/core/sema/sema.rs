// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::collections::HashMap;
use crate::core::ast::*;
use crate::core::lexer::token::SourceSpan;

// ==================== 符号定义 ====================

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub kind: SymbolKind,
    pub ty: Type,
    pub mutable: bool,
    pub visibility: Visibility,
    pub span: SourceSpan,
    pub scope_depth: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Constant,
    Function,
    Struct,
    Trait,
    Type,
    Module,
    GenericParam,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Visibility {
    Private,
    Public,
    Exported(String),
}

// ==================== 作用域 ====================

#[derive(Debug, Clone)]
pub struct Scope {
    symbols: HashMap<String, Symbol>,
    parent: Option<usize>,
    depth: usize,
    pub associated_node: ScopeNode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScopeNode {
    Module,
    Function,
    Block,
    Trait,
    Impl,
}

impl Scope {
    pub fn new(parent: Option<usize>, depth: usize, node: ScopeNode) -> Self {
        Self {
            symbols: HashMap::new(),
            parent,
            depth,
            associated_node: node,
        }
    }
}

// ==================== 符号表 ====================

#[derive(Debug, Clone)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    current_scope: usize,
    module_path: Vec<String>,
    imports: HashMap<String, String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let global = Scope::new(None, 0, ScopeNode::Module);
        Self {
            scopes: vec![global],
            current_scope: 0,
            module_path: Vec::new(),
            imports: HashMap::new(),
        }
    }

    pub fn enter_scope(&mut self, node: ScopeNode) {
        let depth = self.scopes[self.current_scope].depth + 1;
        let scope = Scope::new(Some(self.current_scope), depth, node);
        self.scopes.push(scope);
        self.current_scope = self.scopes.len() - 1;
    }

    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.scopes[self.current_scope].parent {
            self.current_scope = parent;
        }
    }

    pub fn define(
        &mut self,
        name: String,
        kind: SymbolKind,
        ty: Type,
        mutable: bool,
        visibility: Visibility,
        span: SourceSpan,
    ) -> Result<(), SemanticError> {
        let scope = &mut self.scopes[self.current_scope];
        if scope.symbols.contains_key(&name) {
            let existing = scope.symbols.get(&name).unwrap();
            return Err(SemanticError::DuplicateDefinition {
                name,
                first: existing.span,
                second: span,
            });
        }
        let depth = scope.depth;
        scope.symbols.insert(name.clone(), Symbol {
            name,
            kind,
            ty,
            mutable,
            visibility,
            span,
            scope_depth: depth,
        });
        Ok(())
    }

    pub fn resolve(&self, name: &str) -> Option<&Symbol> {
        if let Some(full_name) = self.imports.get(name) {
            return self.resolve_fully_qualified(full_name);
        }

        let mut scope_idx = self.current_scope;
        loop {
            let scope = &self.scopes[scope_idx];
            if let Some(sym) = scope.symbols.get(name) {
                return Some(sym);
            }
            if let Some(parent) = scope.parent {
                scope_idx = parent;
            } else {
                break;
            }
        }
        None
    }

    pub fn resolve_mut(&mut self, name: &str) -> Option<&mut Symbol> {
        let mut scope_idx = self.current_scope;
        let mut found_scope_idx: Option<usize> = None;
        
        // 先找到包含符号的作用域索引
        loop {
            if let Some(scope) = self.scopes.get(scope_idx) {
                if scope.symbols.contains_key(name) {
                    found_scope_idx = Some(scope_idx);
                    break;
                } else if let Some(parent) = scope.parent {
                    scope_idx = parent;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        // 然后在循环外进行可变借用
        if let Some(idx) = found_scope_idx {
            self.scopes[idx].symbols.get_mut(name)
        } else {
            None
        }
    }

    pub fn resolve_fully_qualified(&self, _path: &str) -> Option<&Symbol> {
        None
    }

    pub fn current_depth(&self) -> usize {
        self.scopes[self.current_scope].depth
    }

    pub fn add_import(&mut self, alias: String, full_path: String) {
        self.imports.insert(alias, full_path);
    }

    pub fn enter_module(&mut self, name: String) {
        self.module_path.push(name);
        self.enter_scope(ScopeNode::Module);
    }

    pub fn exit_module(&mut self) {
        self.module_path.pop();
        self.exit_scope();
    }

    pub fn current_module_prefix(&self) -> String {
        self.module_path.join("::")
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== 语义错误类型 ====================

#[derive(Debug, Clone, PartialEq)]
pub enum SemanticError {
    TypeError(TypeError),
    DuplicateDefinition {
        name: String,
        first: SourceSpan,
        second: SourceSpan,
    },
    InvalidAssignment {
        target: String,
        span: SourceSpan,
    },
    BreakOutsideLoop(SourceSpan),
    ContinueOutsideLoop(SourceSpan),
    ReturnOutsideFunction {
        span: SourceSpan,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeError {
    Mismatch {
        expected: Type,
        found: Type,
        span: SourceSpan,
    },
    UnboundVariable {
        name: String,
        span: SourceSpan,
    },
    UndefinedType {
        name: String,
        span: SourceSpan,
    },
    UndefinedField {
        ty: Type,
        field: String,
        span: SourceSpan,
    },
    UndefinedMethod {
        ty: Type,
        method: String,
        span: SourceSpan,
    },
    Cyclic {
        span: SourceSpan,
    },
    NotCallable {
        ty: Type,
        span: SourceSpan,
    },
    ArgCountMismatch {
        expected: usize,
        found: usize,
        span: SourceSpan,
    },
    InvalidOperation {
        op: String,
        left: Type,
        right: Type,
        span: SourceSpan,
    },
    InvalidUnaryOperation {
        op: String,
        ty: Type,
        span: SourceSpan,
    },
    NotIndexable {
        ty: Type,
        span: SourceSpan,
    },
    NotIterable {
        ty: Type,
        span: SourceSpan,
    },
    InvalidTry {
        ty: Type,
        span: SourceSpan,
    },
    InvalidCast {
        from: Type,
        to: Type,
    },
    PatternMismatch {
        pattern: String,
        ty: Type,
        span: SourceSpan,
    },
    NoField {
        ty: Type,
        span: SourceSpan,
    },
}

impl TypeError {
    pub fn with_span(self, span: SourceSpan) -> Self {
        match self {
            TypeError::Mismatch { expected, found, .. } => TypeError::Mismatch { expected, found, span },
            TypeError::UnboundVariable { name, .. } => TypeError::UnboundVariable { name, span },
            TypeError::Cyclic { .. } => TypeError::Cyclic { span },
            _ => self,
        }
    }
}

// ==================== 类型推导器 ====================

pub struct TypeInfer {
    next_var: usize,
    substitutions: HashMap<usize, Type>,
    _constraints: Vec<(Type, Type)>,
    env: Vec<HashMap<String, Type>>,
    symbol_table: SymbolTable,
    current_return_type: Option<Type>,
}

impl TypeInfer {
    pub fn new(symbol_table: SymbolTable) -> Self {
        Self {
            next_var: 0,
            substitutions: HashMap::new(),
            _constraints: Vec::new(),
            env: vec![HashMap::new()],
            symbol_table,
            current_return_type: None,
        }
    }

    fn fresh_var(&mut self) -> Type {
        let var = self.next_var;
        self.next_var += 1;
        Type::Var(var)
    }

    fn _fresh_vars(&mut self, count: usize) -> Vec<Type> {
        (0..count).map(|_| self.fresh_var()).collect()
    }

    fn apply_subst(&self, ty: &Type) -> Type {
        match ty {
            Type::Var(v) => {
                if let Some(t) = self.substitutions.get(v) {
                    self.apply_subst(t)
                } else {
                    ty.clone()
                }
            }
            Type::List(inner) => Type::List(Box::new(self.apply_subst(inner))),
            Type::Array(inner, size) => Type::Array(Box::new(self.apply_subst(inner)), size.clone()),
            Type::Map(k, v) => Type::Map(
                Box::new(self.apply_subst(k)),
                Box::new(self.apply_subst(v)),
            ),
            Type::Ptr(inner) => Type::Ptr(Box::new(self.apply_subst(inner))),
            Type::Option(inner) => Type::Option(Box::new(self.apply_subst(inner))),
            Type::Func(params, ret) => Type::Func(
                params.iter().map(|t| self.apply_subst(t)).collect(),
                Box::new(self.apply_subst(ret)),
            ),
            _ => ty.clone(),
        }
    }

    fn occurs(&self, var: usize, ty: &Type) -> bool {
        match ty {
            Type::Var(v) => *v == var,
            Type::List(inner) => self.occurs(var, inner),
            Type::Array(inner, _) => self.occurs(var, inner),
            Type::Map(k, v) => self.occurs(var, k) || self.occurs(var, v),
            Type::Ptr(inner) => self.occurs(var, inner),
            Type::Option(inner) => self.occurs(var, inner),
            Type::Func(params, ret) => {
                params.iter().any(|t| self.occurs(var, t)) || self.occurs(var, ret)
            }
            _ => false,
        }
    }

    fn unify(&mut self, t1: Type, t2: Type) -> Result<(), TypeError> {
        let t1 = self.apply_subst(&t1);
        let t2 = self.apply_subst(&t2);

        match (t1, t2) {
            (Type::Var(v1), Type::Var(v2)) if v1 == v2 => Ok(()),
            (Type::Var(v), t) => {
                if self.occurs(v, &t) {
                    Err(TypeError::Cyclic { span: SourceSpan::dummy() })
                } else {
                    self.substitutions.insert(v, t);
                    Ok(())
                }
            }
            (t, Type::Var(v)) => {
                if self.occurs(v, &t) {
                    Err(TypeError::Cyclic { span: SourceSpan::dummy() })
                } else {
                    self.substitutions.insert(v, t);
                    Ok(())
                }
            }
            (Type::Int, Type::Int) => Ok(()),
            (Type::I8, Type::I8) => Ok(()),
            (Type::I16, Type::I16) => Ok(()),
            (Type::I32, Type::I32) => Ok(()),
            (Type::I64, Type::I64) => Ok(()),
            (Type::U8, Type::U8) => Ok(()),
            (Type::U16, Type::U16) => Ok(()),
            (Type::U32, Type::U32) => Ok(()),
            (Type::U64, Type::U64) => Ok(()),
            (Type::F32, Type::F32) => Ok(()),
            (Type::F64, Type::F64) => Ok(()),
            (Type::Bool, Type::Bool) => Ok(()),
            (Type::Char, Type::Char) => Ok(()),
            (Type::String, Type::String) => Ok(()),
            (Type::Unit, Type::Unit) => Ok(()),
            (Type::List(a), Type::List(b)) => self.unify(*a, *b),
            (Type::Map(k1, v1), Type::Map(k2, v2)) => {
                self.unify(*k1, *k2)?;
                self.unify(*v1, *v2)
            }
            (Type::Ptr(a), Type::Ptr(b)) => self.unify(*a, *b),
            (Type::Option(a), Type::Option(b)) => self.unify(*a, *b),
            (Type::Func(p1, r1), Type::Func(p2, r2)) => {
                if p1.len() != p2.len() {
                    return Err(TypeError::Mismatch {
                        expected: Type::Func(p1, r1),
                        found: Type::Func(p2, r2),
                        span: SourceSpan::dummy(),
                    });
                }
                for (a, b) in p1.into_iter().zip(p2) {
                    self.unify(a, b)?;
                }
                self.unify(*r1, *r2)
            }
            (a, b) => Err(TypeError::Mismatch {
                expected: a,
                found: b,
                span: SourceSpan::dummy(),
            }),
        }
    }

    pub fn infer_expr(&mut self, expr: &Expr) -> Result<Type, TypeError> {
        let _span = expr.span();
        match expr {
            Expr::IntLit(_, _) => Ok(Type::Int),
            Expr::FloatLit(_, _) => Ok(Type::F64),
            Expr::StringLit(_, _) => Ok(Type::String),
            Expr::CharLit(_, _) => Ok(Type::Char),
            Expr::BoolLit(_, _) => Ok(Type::Bool),
            Expr::Null(_) => Ok(self.fresh_var()),

            Expr::Ident(ident) => {
                self.lookup_type(&ident.name)
                    .cloned()
                    .ok_or_else(|| TypeError::UnboundVariable {
                        name: ident.name.clone(),
                        span: ident.span,
                    })
            }

            Expr::BinaryOp { op, left, right, span } => {
                let t_left = self.infer_expr(left)?;
                let t_right = self.infer_expr(right)?;

                match op {
                    BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
                        self.unify(t_left.clone(), t_right.clone()).map_err(|e| e.with_span(*span))?;
                        Ok(t_left)
                    }
                    BinaryOp::And | BinaryOp::Or => {
                        self.unify(t_left, Type::Bool).map_err(|e| e.with_span(*span))?;
                        self.unify(t_right, Type::Bool).map_err(|e| e.with_span(*span))?;
                        Ok(Type::Bool)
                    }
                    BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Gt | BinaryOp::Lt | BinaryOp::Ge | BinaryOp::Le => {
                        self.unify(t_left, t_right).map_err(|e| e.with_span(*span))?;
                        Ok(Type::Bool)
                    }
                    BinaryOp::Shl | BinaryOp::Shr | BinaryOp::BitAnd | BinaryOp::BitOr | BinaryOp::BitXor => {
                        if !t_left.is_integer() || !t_right.is_integer() {
                            return Err(TypeError::InvalidOperation {
                                op: format!("{:?}", op),
                                left: t_left,
                                right: t_right,
                                span: *span,
                            });
                        }
                        self.unify(t_left.clone(), t_right).map_err(|e| e.with_span(*span))?;
                        Ok(t_left)
                    }
                    BinaryOp::Assign | BinaryOp::AddAssign | BinaryOp::SubAssign
                    | BinaryOp::MulAssign | BinaryOp::DivAssign | BinaryOp::ModAssign
                    | BinaryOp::ShlAssign | BinaryOp::ShrAssign | BinaryOp::BitAndAssign
                    | BinaryOp::BitOrAssign | BinaryOp::BitXorAssign => {
                        self.unify(t_left, t_right).map_err(|e| e.with_span(*span))?;
                        Ok(Type::Unit)
                    }
                }
            }

            Expr::UnaryOp { op, expr, span } => {
                let t_expr = self.infer_expr(expr)?;
                match op {
                    UnaryOp::Not => {
                        self.unify(t_expr, Type::Bool).map_err(|e| e.with_span(*span))?;
                        Ok(Type::Bool)
                    }
                    UnaryOp::Neg => {
                        if !t_expr.is_numeric() {
                            return Err(TypeError::InvalidUnaryOperation {
                                op: "负号".to_string(),
                                ty: t_expr,
                                span: *span,
                            });
                        }
                        Ok(t_expr)
                    }
                    UnaryOp::BitNot => {
                        if !t_expr.is_integer() {
                            return Err(TypeError::InvalidUnaryOperation {
                                op: "按位非".to_string(),
                                ty: t_expr,
                                span: *span,
                            });
                        }
                        Ok(t_expr)
                    }
                }
            }

            Expr::Call { func, args, span } => {
                let t_func = self.infer_expr(func)?;
                match t_func {
                    Type::Func(param_tys, ret_ty) => {
                        if param_tys.len() != args.len() {
                            return Err(TypeError::ArgCountMismatch {
                                expected: param_tys.len(),
                                found: args.len(),
                                span: *span,
                            });
                        }
                        for (arg, expected) in args.iter().zip(param_tys.iter()) {
                            let t_arg = self.infer_expr(arg)?;
                            self.unify(t_arg, expected.clone()).map_err(|e| e.with_span(arg.span()))?;
                        }
                        Ok(*ret_ty)
                    }
                    _ => Err(TypeError::NotCallable {
                        ty: t_func,
                        span: func.span(),
                    }),
                }
            }

            Expr::List(elements, _) => {
                if elements.is_empty() {
                    Ok(Type::List(Box::new(self.fresh_var())))
                } else {
                    let elem_ty = self.infer_expr(&elements[0])?;
                    for elem in &elements[1..] {
                        let t = self.infer_expr(elem)?;
                        self.unify(elem_ty.clone(), t).map_err(|e| e.with_span(elem.span()))?;
                    }
                    Ok(Type::List(Box::new(elem_ty)))
                }
            }

            Expr::Map(pairs, _) => {
                if pairs.is_empty() {
                    Ok(Type::Map(Box::new(self.fresh_var()), Box::new(self.fresh_var())))
                } else {
                    let key_ty = self.infer_expr(&pairs[0].0)?;
                    let val_ty = self.infer_expr(&pairs[0].1)?;
                    for (k, v) in &pairs[1..] {
                        let tk = self.infer_expr(k)?;
                        let tv = self.infer_expr(v)?;
                        self.unify(key_ty.clone(), tk).map_err(|e| e.with_span(k.span()))?;
                        self.unify(val_ty.clone(), tv).map_err(|e| e.with_span(v.span()))?;
                    }
                    Ok(Type::Map(Box::new(key_ty), Box::new(val_ty)))
                }
            }

            Expr::IfExpr { cond, then_expr, else_expr, span } => {
                let t_cond = self.infer_expr(cond)?;
                self.unify(t_cond, Type::Bool).map_err(|e| e.with_span(cond.span()))?;

                let t_then = self.infer_expr(then_expr)?;
                let t_else = self.infer_expr(else_expr)?;

                self.unify(t_then.clone(), t_else).map_err(|e| e.with_span(*span))?;
                Ok(t_then)
            }

            Expr::Asm(_asm) => {
                Ok(Type::Unit)
            }

            Expr::Field { target, field, span } => {
                let t_target = self.infer_expr(target)?;
                let field_name = &field.name;
                if let Type::Named(_) = &t_target {
                    return Err(TypeError::UndefinedField {
                        ty: t_target,
                        field: field_name.clone(),
                        span: *span,
                    });
                }
                Ok(self.fresh_var())
            }

            Expr::Index { target, index, span } => {
                let t_target = self.infer_expr(target)?;
                let t_index = self.infer_expr(index)?;
                self.unify(t_index, Type::Int).map_err(|e| e.with_span(*span))?;
                match t_target {
                    Type::List(inner) => Ok(*inner),
                    Type::Array(inner, _) => Ok(*inner),
                    Type::Map(k, v) => Ok(*v),
                    _ => Err(TypeError::NotIndexable {
                        ty: t_target,
                        span: *span,
                    }),
                }
            }

            Expr::MethodCall { receiver, method, args, span } => {
                let t_receiver = self.infer_expr(receiver)?;
                let method_name = &method.name;
                return Err(TypeError::UndefinedMethod {
                    ty: t_receiver,
                    method: method_name.clone(),
                    span: *span,
                });
            }

            Expr::Struct { path, fields, span } => {
                let _path = path;
                let _fields = fields;
                Ok(self.fresh_var())
            }

            Expr::Closure { params, return_type, body, span } => {
                let _params = params;
                let _return_type = return_type;
                let _body = body;
                let _span = span;
                Ok(self.fresh_var())
            }

            Expr::Match { expr, arms, default, span } => {
                let t_expr = self.infer_expr(expr)?;
                if arms.is_empty() {
                    if let Some(def) = default {
                        return self.infer_expr(def);
                    }
                    return Ok(self.fresh_var());
                }
                let first_arm_ty = self.fresh_var();
                for (pattern, arm_expr) in arms {
                    let _pattern = pattern;
                    let t_arm = self.infer_expr(arm_expr)?;
                    self.unify(first_arm_ty.clone(), t_arm).map_err(|e| e.with_span(*span))?;
                }
                if let Some(def) = default {
                    let t_def = self.infer_expr(def)?;
                    self.unify(first_arm_ty.clone(), t_def).map_err(|e| e.with_span(*span))?;
                }
                Ok(first_arm_ty)
            }

            Expr::Try { expr, span } => {
                let t_expr = self.infer_expr(expr)?;
                match t_expr {
                    Type::Option(inner) => Ok(*inner),
                    _ => Err(TypeError::InvalidTry {
                        ty: t_expr,
                        span: *span,
                    }),
                }
            }

            Expr::TypeAssertion { expr, ty, span } => {
                let _t_expr = self.infer_expr(expr)?;
                let _ty = ty;
                let _span = span;
                Ok(self.fresh_var())
            }

            _ => Ok(self.fresh_var()),
        }
    }

    fn lookup_type(&self, name: &str) -> Option<&Type> {
        for scope in self.env.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty);
            }
        }
        None
    }

    pub fn infer_stmt(&mut self, stmt: &Stmt) -> Result<(), TypeError> {
        match stmt {
            Stmt::Let { name, ty, value, span } => {
                let t_value = self.infer_expr(value)?;
                let var_type = if let Some(anno) = ty {
                    self.unify(t_value.clone(), anno.clone()).map_err(|e| e.with_span(*span))?;
                    anno.clone()
                } else {
                    t_value
                };
                self.env.last_mut().unwrap().insert(name.name.clone(), var_type);
                Ok(())
            }

            Stmt::Const { name, ty, value, span } => {
                let t_value = self.infer_expr(value)?;
                let var_type = if let Some(anno) = ty {
                    self.unify(t_value.clone(), anno.clone()).map_err(|e| e.with_span(*span))?;
                    anno.clone()
                } else {
                    t_value
                };
                self.env.last_mut().unwrap().insert(name.name.clone(), var_type);
                Ok(())
            }

            Stmt::Assign { target, value, span } => {
                let t_target = self.infer_expr(target)?;
                let t_value = self.infer_expr(value)?;
                self.unify(t_target, t_value).map_err(|e| e.with_span(*span))?;
                Ok(())
            }

            Stmt::If { cond, then_block, else_ifs, else_block, span } => {
                let t_cond = self.infer_expr(cond)?;
                self.unify(t_cond, Type::Bool).map_err(|e| e.with_span(cond.span()))?;

                for stmt in then_block {
                    self.infer_stmt(stmt)?;
                }

                for (elif_cond, elif_block) in else_ifs {
                    let t_elif_cond = self.infer_expr(elif_cond)?;
                    self.unify(t_elif_cond, Type::Bool).map_err(|e| e.with_span(elif_cond.span()))?;
                    for stmt in elif_block {
                        self.infer_stmt(stmt)?;
                    }
                }

                if let Some(else_block) = else_block {
                    for stmt in else_block {
                        self.infer_stmt(stmt)?;
                    }
                }

                Ok(())
            }

            Stmt::While { cond, body, span } => {
                let t_cond = self.infer_expr(cond)?;
                self.unify(t_cond, Type::Bool).map_err(|e| e.with_span(*span))?;
                for stmt in body {
                    self.infer_stmt(stmt)?;
                }
                Ok(())
            }

            Stmt::Repeat { count, body, span } => {
                let t_count = self.infer_expr(count)?;
                self.unify(t_count, Type::Int).map_err(|e| e.with_span(*span))?;
                for stmt in body {
                    self.infer_stmt(stmt)?;
                }
                Ok(())
            }

            Stmt::ForEach { var, iterable, body, span } => {
                let t_iter = self.infer_expr(iterable)?;
                match &t_iter {
                    Type::List(inner) => {
                        self.env.last_mut().unwrap().insert(var.name.clone(), *inner.clone());
                    }
                    Type::Array(inner, _) => {
                        self.env.last_mut().unwrap().insert(var.name.clone(), *inner.clone());
                    }
                    _ => {
                        return Err(TypeError::NotIterable {
                            ty: t_iter,
                            span: *span,
                        });
                    }
                }
                for stmt in body {
                    self.infer_stmt(stmt)?;
                }
                Ok(())
            }

            Stmt::Match { expr, arms, default, span } => {
                let t_expr = self.infer_expr(expr)?;
                for (pattern, arm_stmts) in arms {
                    let _pattern = pattern;
                    for stmt in arm_stmts {
                        self.infer_stmt(stmt)?;
                    }
                }
                if let Some(default_stmts) = default {
                    for stmt in default_stmts {
                        self.infer_stmt(stmt)?;
                    }
                }
                Ok(())
            }

            Stmt::Return(expr, span) => {
                if let Some(expected) = self.current_return_type.clone() {
                    if let Some(e) = expr {
                        let t_expr = self.infer_expr(e)?;
                        self.unify(t_expr, expected).map_err(|e| e.with_span(*span))?;
                    } else {
                        self.unify(Type::Unit, expected).map_err(|e| e.with_span(*span))?;
                    }
                } else {
                    return Err(TypeError::Mismatch {
                        expected: Type::Unit,
                        found: Type::Unit,
                        span: *span,
                    });
                }
                Ok(())
            }

            Stmt::Break(span) => {
                Ok(())
            }

            Stmt::Continue(span) => {
                Ok(())
            }

            Stmt::Expr(expr, _) => {
                self.infer_expr(expr)?;
                Ok(())
            }

            Stmt::Asm(_, span) => {
                Ok(())
            }
        }
    }

    pub fn infer_program(&mut self, program: &Program) -> Result<(), Vec<TypeError>> {
        let mut errors = Vec::new();

        for item in program {
            if let Err(e) = self.collect_top_level(item) {
                errors.push(e);
            }
        }

        for item in program {
            if let Err(e) = self.check_item(item) {
                errors.push(e);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn collect_top_level(&mut self, item: &Item) -> Result<(), TypeError> {
        match item {
            Item::Function(func) => {
                let param_types = func.params.iter().map(|(_, t)| t.clone()).collect();
                let ty = Type::Func(param_types, Box::new(func.return_type.clone()));
                self.symbol_table.define(
                    func.name.name.clone(),
                    SymbolKind::Function,
                    ty,
                    false,
                    if func.public { Visibility::Public } else { Visibility::Private },
                    func.span,
                ).map_err(|e| {
                    match e {
                        SemanticError::DuplicateDefinition { name: _, first: _, second } => {
                            TypeError::Mismatch {
                                expected: Type::Unit,
                                found: Type::Unit,
                                span: second,
                            }
                        }
                        _ => TypeError::Mismatch {
                            expected: Type::Unit,
                            found: Type::Unit,
                            span: SourceSpan::dummy(),
                        }
                    }
                })?;
                Ok(())
            }
            Item::Struct(s) => {
                self.symbol_table.define(
                    s.name.name.clone(),
                    SymbolKind::Struct,
                    Type::Unit,
                    false,
                    if s.public { Visibility::Public } else { Visibility::Private },
                    s.span,
                ).map_err(|e| {
                    match e {
                        SemanticError::DuplicateDefinition { name: _, first: _, second } => {
                            TypeError::Mismatch {
                                expected: Type::Unit,
                                found: Type::Unit,
                                span: second,
                            }
                        }
                        _ => TypeError::Mismatch {
                            expected: Type::Unit,
                            found: Type::Unit,
                            span: SourceSpan::dummy(),
                        }
                    }
                })?;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn check_item(&mut self, item: &Item) -> Result<(), TypeError> {
        match item {
            Item::Function(func) => self.check_function(func),
            Item::Global(g) => {
                let t_value = self.infer_expr(&g.value)?;
                if let Some(anno) = &g.ty {
                    self.unify(t_value, anno.clone()).map_err(|e| e.with_span(g.span))?;
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn check_function(&mut self, func: &Function) -> Result<(), TypeError> {
        self.env.push(HashMap::new());
        self.current_return_type = Some(func.return_type.clone());

        for (name, ty) in &func.params {
            self.env.last_mut().unwrap().insert(name.name.clone(), ty.clone());
        }

        for stmt in &func.body {
            self.infer_stmt(stmt)?;
        }

        self.env.pop();
        self.current_return_type = None;
        Ok(())
    }
}

// ==================== 语义分析驱动 ====================

pub struct SemanticAnalyzer {
    symbol_table: SymbolTable,
    type_infer: TypeInfer,
    errors: Vec<SemanticError>,
}

impl SemanticAnalyzer {
    pub fn new() -> Self {
        let symbol_table = SymbolTable::new();
        let type_infer = TypeInfer::new(symbol_table.clone());
        Self {
            symbol_table,
            type_infer,
            errors: Vec::new(),
        }
    }

    pub fn analyze(&mut self, program: &Program) -> Result<SymbolTable, Vec<SemanticError>> {
        if let Err(errs) = self.type_infer.infer_program(program) {
            self.errors.extend(errs.into_iter().map(|e| SemanticError::TypeError(e)));
        }

        if self.errors.is_empty() {
            Ok(self.symbol_table.clone())
        } else {
            Err(std::mem::take(&mut self.errors))
        }
    }
}

impl Default for SemanticAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_symbol_table_new() {
        let table = SymbolTable::new();
        assert_eq!(table.current_scope, 0);
        assert_eq!(table.scopes.len(), 1);
    }

    #[test]
    fn test_scope_enter_exit() {
        let mut table = SymbolTable::new();
        let original_depth = table.current_depth();
        table.enter_scope(ScopeNode::Block);
        assert_eq!(table.current_depth(), original_depth + 1);
        table.exit_scope();
        assert_eq!(table.current_depth(), original_depth);
    }

    #[test]
    fn test_semantic_analyzer_new() {
        let analyzer = SemanticAnalyzer::new();
        assert!(analyzer.errors.is_empty());
    }

    #[test]
    fn test_type_error_with_span() {
        let err = TypeError::UnboundVariable {
            name: "test".to_string(),
            span: SourceSpan::dummy(),
        };
        let new_span = SourceSpan::dummy();
        let new_err = err.with_span(new_span);
        assert!(matches!(new_err, TypeError::UnboundVariable { .. }));
    }
}

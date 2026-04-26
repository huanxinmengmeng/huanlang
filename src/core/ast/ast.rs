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

use crate::core::lexer::token::SourceSpan;

// ==========================================
// 5.2 通用节点定义
// ==========================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: String,
    pub span: SourceSpan,
}

impl Ident {
    pub fn new(name: String, span: SourceSpan) -> Self {
        Self { name, span }
    }

    pub fn dummy(name: &str) -> Self {
        Self {
            name: name.to_string(),
            span: SourceSpan::dummy(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub segments: Vec<Ident>,
    pub generics: Option<Vec<Type>>,
    pub span: SourceSpan,
}

impl Path {
    pub fn from_ident(ident: Ident) -> Self {
        Self {
            segments: vec![ident.clone()],
            generics: None,
            span: ident.span,
        }
    }
}

// ==========================================
// 5.3 类型节点
// ==========================================

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // 基本数值类型
    Int,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,

    // 其他基本类型
    Bool,
    Char,
    String,
    Unit,

    // 复合类型
    List(Box<Type>),
    Array(Box<Type>, Box<Expr>),
    Map(Box<Type>, Box<Type>),
    Ptr(Box<Type>),
    Option(Box<Type>),

    // 函数类型
    Func(Vec<Type>, Box<Type>),

    // 自定义类型
    Named(Path),

    // 内部使用
    Var(usize),
}

impl Type {
    pub fn span(&self) -> SourceSpan {
        match self {
            Type::Named(path) => path.span,
            _ => SourceSpan::dummy(),
        }
    }

    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            Type::Int
                | Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
                | Type::F32
                | Type::F64
        )
    }

    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            Type::Int
                | Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
        )
    }
}

// ==========================================
// 5.4 表达式节点
// ==========================================

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // 字面量
    IntLit(i64, SourceSpan),
    FloatLit(f64, SourceSpan),
    StringLit(String, SourceSpan),
    CharLit(char, SourceSpan),
    BoolLit(bool, SourceSpan),
    Null(SourceSpan),

    // 标识符
    Ident(Ident),

    // 二元运算
    BinaryOp {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
        span: SourceSpan,
    },

    // 一元运算
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
        span: SourceSpan,
    },

    // 函数调用
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
        span: SourceSpan,
    },

    // 方法调用
    MethodCall {
        receiver: Box<Expr>,
        method: Ident,
        args: Vec<Expr>,
        span: SourceSpan,
    },

    // 字段访问
    Field {
        target: Box<Expr>,
        field: Ident,
        span: SourceSpan,
    },

    // 索引访问
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
        span: SourceSpan,
    },

    // 复合字面量
    List(Vec<Expr>, SourceSpan),
    Map(Vec<(Expr, Expr)>, SourceSpan),
    Struct {
        path: Path,
        fields: Vec<(Ident, Expr)>,
        span: SourceSpan,
    },

    // 闭包
    Closure {
        params: Vec<(Ident, Option<Type>)>,
        return_type: Option<Type>,
        body: Vec<Stmt>,
        span: SourceSpan,
    },

    // 条件表达式
    IfExpr {
        cond: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
        span: SourceSpan,
    },

    // 模式匹配表达式
    Match {
        expr: Box<Expr>,
        arms: Vec<(Pattern, Expr)>,
        default: Option<Box<Expr>>,
        span: SourceSpan,
    },

    // 可选解包
    Try {
        expr: Box<Expr>,
        span: SourceSpan,
    },

    // 类型断言
    TypeAssertion {
        expr: Box<Expr>,
        ty: Type,
        span: SourceSpan,
    },

    // 内联汇编
    Asm(InlineAsm),
}

impl Expr {
    pub fn span(&self) -> SourceSpan {
        match self {
            Expr::IntLit(_, span) => *span,
            Expr::FloatLit(_, span) => *span,
            Expr::StringLit(_, span) => *span,
            Expr::CharLit(_, span) => *span,
            Expr::BoolLit(_, span) => *span,
            Expr::Null(span) => *span,
            Expr::Ident(ident) => ident.span,
            Expr::BinaryOp { span, .. } => *span,
            Expr::UnaryOp { span, .. } => *span,
            Expr::Call { span, .. } => *span,
            Expr::MethodCall { span, .. } => *span,
            Expr::Field { span, .. } => *span,
            Expr::Index { span, .. } => *span,
            Expr::List(_, span) => *span,
            Expr::Map(_, span) => *span,
            Expr::Struct { span, .. } => *span,
            Expr::Closure { span, .. } => *span,
            Expr::IfExpr { span, .. } => *span,
            Expr::Match { span, .. } => *span,
            Expr::Try { span, .. } => *span,
            Expr::TypeAssertion { span, .. } => *span,
            Expr::Asm(asm) => asm.span,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Eq,
    Ne,
    Gt,
    Lt,
    Ge,
    Le,
    Shl,
    Shr,
    BitAnd,
    BitOr,
    BitXor,
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
    ShlAssign,
    ShrAssign,
    BitAndAssign,
    BitOrAssign,
    BitXorAssign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,
    Neg,
    BitNot,
}

// ==========================================
// 5.5 语句节点
// ==========================================

#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // 变量声明
    Let {
        name: Ident,
        ty: Option<Type>,
        value: Box<Expr>,
        span: SourceSpan,
    },

    // 常量声明
    Const {
        name: Ident,
        ty: Option<Type>,
        value: Box<Expr>,
        span: SourceSpan,
    },

    // 赋值
    Assign {
        target: Box<Expr>,
        value: Box<Expr>,
        span: SourceSpan,
    },

    // 条件语句
    If {
        cond: Box<Expr>,
        then_block: Vec<Stmt>,
        else_ifs: Vec<(Expr, Vec<Stmt>)>,
        else_block: Option<Vec<Stmt>>,
        span: SourceSpan,
    },

    // 模式匹配语句
    Match {
        expr: Box<Expr>,
        arms: Vec<(Pattern, Vec<Stmt>)>,
        default: Option<Vec<Stmt>>,
        span: SourceSpan,
    },

    // 固定次数循环
    Repeat {
        count: Box<Expr>,
        body: Vec<Stmt>,
        span: SourceSpan,
    },

    // 条件循环
    While {
        cond: Box<Expr>,
        body: Vec<Stmt>,
        span: SourceSpan,
    },

    // 遍历循环
    ForEach {
        var: Ident,
        iterable: Box<Expr>,
        body: Vec<Stmt>,
        span: SourceSpan,
    },

    // 返回语句
    Return(Option<Box<Expr>>, SourceSpan),

    // 控制流
    Break(SourceSpan),
    Continue(SourceSpan),

    // 内联汇编语句
    Asm(InlineAsm, SourceSpan),

    // 表达式语句
    Expr(Box<Expr>, SourceSpan),
}

impl Stmt {
    pub fn span(&self) -> SourceSpan {
        match self {
            Stmt::Let { span, .. } => *span,
            Stmt::Const { span, .. } => *span,
            Stmt::Assign { span, .. } => *span,
            Stmt::If { span, .. } => *span,
            Stmt::Match { span, .. } => *span,
            Stmt::Repeat { span, .. } => *span,
            Stmt::While { span, .. } => *span,
            Stmt::ForEach { span, .. } => *span,
            Stmt::Return(_, span) => *span,
            Stmt::Break(span) => *span,
            Stmt::Continue(span) => *span,
            Stmt::Asm(_, span) => *span,
            Stmt::Expr(_, span) => *span,
        }
    }
}

// ==========================================
// 5.6 模式匹配节点
// ==========================================

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    // 通配符
    Wildcard(SourceSpan),

    // 标识符绑定
    Ident(Ident),

    // 字面量模式
    Literal(Expr),

    // 结构体模式
    Struct {
        path: Path,
        fields: Vec<(Ident, Pattern)>,
        span: SourceSpan,
    },

    // 列表模式
    List(Vec<Pattern>, SourceSpan),

    // 或模式
    Or(Box<Pattern>, Box<Pattern>, SourceSpan),
}

impl Pattern {
    pub fn span(&self) -> SourceSpan {
        match self {
            Pattern::Wildcard(span) => *span,
            Pattern::Ident(ident) => ident.span,
            Pattern::Literal(expr) => expr.span(),
            Pattern::Struct { span, .. } => *span,
            Pattern::List(_, span) => *span,
            Pattern::Or(_, _, span) => *span,
        }
    }
}

// ==========================================
// 5.7 内联汇编节点
// ==========================================

#[derive(Debug, Clone, PartialEq)]
pub struct InlineAsm {
    pub templates: Vec<String>,
    pub outputs: Vec<AsmOperand>,
    pub inputs: Vec<AsmOperand>,
    pub clobbers: Vec<String>,
    pub options: AsmOptions,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AsmOperand {
    pub name: Option<Ident>,
    pub constraint: String,
    pub expr: Expr,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AsmOptions {
    pub volatile: bool,
    pub pure: bool,
    pub nomem: bool,
    pub preserves_flags: bool,
    pub noreturn: bool,
    pub alignstack: bool,
    pub intel_syntax: bool,
}

// ==========================================
// 5.8 顶层项节点
// ==========================================

#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    Function(Function),
    Struct(Struct),
    Trait(Trait),
    Impl(Impl),
    Module(Module),
    Import(Import),
    Extern(ExternBlock),
    TypeAlias(TypeAlias),
    Global(Global),
}

impl Item {
    pub fn span(&self) -> SourceSpan {
        match self {
            Item::Function(f) => f.span,
            Item::Struct(s) => s.span,
            Item::Trait(t) => t.span,
            Item::Impl(i) => i.span,
            Item::Module(m) => m.span,
            Item::Import(i) => i.span,
            Item::Extern(e) => e.span,
            Item::TypeAlias(t) => t.span,
            Item::Global(g) => g.span,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub public: bool,
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub params: Vec<(Ident, Type)>,
    pub return_type: Type,
    pub where_clause: Vec<WherePredicate>,
    pub preconditions: Vec<Expr>,
    pub postconditions: Vec<Expr>,
    pub body: Vec<Stmt>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub public: bool,
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub where_clause: Vec<WherePredicate>,
    pub fields: Vec<(Ident, Type, Option<Expr>)>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Trait {
    pub public: bool,
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub super_traits: Vec<Path>,
    pub methods: Vec<TraitMethod>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitMethod {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub params: Vec<(Ident, Type)>,
    pub return_type: Type,
    pub where_clause: Vec<WherePredicate>,
    pub default_body: Option<Vec<Stmt>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Impl {
    pub generics: Vec<GenericParam>,
    pub trait_name: Option<Path>,
    pub target_type: Type,
    pub where_clause: Vec<WherePredicate>,
    pub methods: Vec<Function>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub public: bool,
    pub name: Ident,
    pub items: Vec<Item>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    pub items: Option<Vec<Ident>>,
    pub path: String,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExternBlock {
    pub abi: ExternAbi,
    pub items: Vec<ExternItem>,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExternAbi {
    C,
    System,
    Rust,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExternItem {
    Function {
        name: Ident,
        generics: Vec<GenericParam>,
        params: Vec<(Ident, Type)>,
        return_type: Type,
        variadic: bool,
    },
    Static {
        name: Ident,
        ty: Type,
        mutable: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeAlias {
    pub public: bool,
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub ty: Type,
    pub span: SourceSpan,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Global {
    pub mutable: bool,
    pub name: Ident,
    pub ty: Option<Type>,
    pub value: Box<Expr>,
    pub span: SourceSpan,
}

// ==========================================
// 5.9 泛型与约束
// ==========================================

#[derive(Debug, Clone, PartialEq)]
pub struct GenericParam {
    pub name: Ident,
    pub bounds: Vec<Path>,
    pub default: Option<Type>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WherePredicate {
    TypeBound {
        ty: Type,
        bounds: Vec<Path>,
    },
    LifetimeBound {
        lifetime: Ident,
        bounds: Vec<Ident>,
    },
    Equate {
        lhs: Type,
        rhs: Type,
    },
}

// ==========================================
// 5.10 程序根节点
// ==========================================

pub type Program = Vec<Item>;

// ==========================================
// 5.11 AST 工具函数 - 访问者模式
// ==========================================

pub trait AstVisitor<T> {
    fn visit_program(&mut self, program: &Program) -> T;
    fn visit_item(&mut self, item: &Item) -> T;
    fn visit_stmt(&mut self, stmt: &Stmt) -> T;
    fn visit_expr(&mut self, expr: &Expr) -> T;
    fn visit_type(&mut self, ty: &Type) -> T;
    fn visit_pattern(&mut self, pattern: &Pattern) -> T;
}

pub trait AstVisitorMut {
    fn visit_item(&mut self, item: &mut Item);
    fn visit_stmt(&mut self, stmt: &mut Stmt);
    fn visit_expr(&mut self, expr: &mut Expr);
    fn visit_type(&mut self, ty: &mut Type);
    fn visit_pattern(&mut self, pattern: &mut Pattern);
}

// 从 AST 中收集所有标识符
pub fn collect_idents(program: &Program) -> Vec<Ident> {
    struct IdentCollector(Vec<Ident>);

    impl AstVisitor<()> for IdentCollector {
        fn visit_program(&mut self, program: &Program) {
            for item in program {
                self.visit_item(item);
            }
        }

        fn visit_item(&mut self, _item: &Item) {
        }

        fn visit_stmt(&mut self, _stmt: &Stmt) {
        }

        fn visit_expr(&mut self, expr: &Expr) {
            if let Expr::Ident(ident) = expr {
                self.0.push(ident.clone());
            }
            match expr {
                Expr::BinaryOp { left, right, .. } => {
                    self.visit_expr(left);
                    self.visit_expr(right);
                }
                Expr::UnaryOp { expr, .. } => {
                    self.visit_expr(expr);
                }
                Expr::Call { func, args, .. } => {
                    self.visit_expr(func);
                    for arg in args {
                        self.visit_expr(arg);
                    }
                }
                Expr::MethodCall {
                    receiver, args, ..
                } => {
                    self.visit_expr(receiver);
                    for arg in args {
                        self.visit_expr(arg);
                    }
                }
                Expr::Field { target, .. } => {
                    self.visit_expr(target);
                }
                Expr::Index { target, index, .. } => {
                    self.visit_expr(target);
                    self.visit_expr(index);
                }
                Expr::List(elements, _) => {
                    for element in elements {
                        self.visit_expr(element);
                    }
                }
                Expr::Map(pairs, _) => {
                    for (key, value) in pairs {
                        self.visit_expr(key);
                        self.visit_expr(value);
                    }
                }
                Expr::Struct { fields, .. } => {
                    for (_, expr) in fields {
                        self.visit_expr(expr);
                    }
                }
                Expr::Closure { body, .. } => {
                    for stmt in body {
                        self.visit_stmt(stmt);
                    }
                }
                Expr::IfExpr {
                    cond,
                    then_expr,
                    else_expr,
                    ..
                } => {
                    self.visit_expr(cond);
                    self.visit_expr(then_expr);
                    self.visit_expr(else_expr);
                }
                Expr::Match {
                    expr, arms, default, ..
                } => {
                    self.visit_expr(expr);
                    for (_, e) in arms {
                        self.visit_expr(e);
                    }
                    if let Some(d) = default {
                        self.visit_expr(d);
                    }
                }
                Expr::Try { expr, .. } => {
                    self.visit_expr(expr);
                }
                Expr::TypeAssertion { expr, .. } => {
                    self.visit_expr(expr);
                }
                _ => {}
            }
        }

        fn visit_type(&mut self, _ty: &Type) {
        }

        fn visit_pattern(&mut self, pattern: &Pattern) {
            match pattern {
                Pattern::Ident(ident) => {
                    self.0.push(ident.clone());
                }
                Pattern::Struct { fields, .. } => {
                    for (_, p) in fields {
                        self.visit_pattern(p);
                    }
                }
                Pattern::List(patterns, _) => {
                    for p in patterns {
                        self.visit_pattern(p);
                    }
                }
                Pattern::Or(p1, p2, _) => {
                    self.visit_pattern(p1);
                    self.visit_pattern(p2);
                }
                Pattern::Literal(e) => {
                    self.visit_expr(e);
                }
                _ => {}
            }
        }
    }

    let mut collector = IdentCollector(Vec::new());
    collector.visit_program(program);
    collector.0
}

// ==========================================
// 单元测试
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ident_new() {
        let span = SourceSpan::dummy();
        let ident = Ident::new("test".to_string(), span);
        assert_eq!(ident.name, "test");
    }

    #[test]
    fn test_ident_dummy() {
        let ident = Ident::dummy("test");
        assert_eq!(ident.name, "test");
    }

    #[test]
    fn test_path_from_ident() {
        let ident = Ident::dummy("test");
        let path = Path::from_ident(ident.clone());
        assert_eq!(path.segments.len(), 1);
        assert_eq!(path.segments[0].name, "test");
    }

    #[test]
    fn test_type_numeric() {
        assert!(Type::Int.is_numeric());
        assert!(Type::I32.is_numeric());
        assert!(Type::F64.is_numeric());
        assert!(!Type::String.is_numeric());
    }

    #[test]
    fn test_type_integer() {
        assert!(Type::Int.is_integer());
        assert!(Type::I32.is_integer());
        assert!(!Type::F64.is_integer());
    }

    #[test]
    fn test_expr_span() {
        let span = SourceSpan::dummy();
        let expr = Expr::IntLit(42, span);
        assert_eq!(expr.span(), span);
    }

    #[test]
    fn test_program_empty() {
        let program: Program = vec![];
        assert_eq!(program.len(), 0);
    }

    #[test]
    fn test_binary_op_eq() {
        let op1 = BinaryOp::Add;
        let op2 = BinaryOp::Add;
        assert_eq!(op1, op2);
    }

    #[test]
    fn test_unary_op_eq() {
        let op1 = UnaryOp::Not;
        let op2 = UnaryOp::Not;
        assert_eq!(op1, op2);
    }

    #[test]
    fn test_collect_idents_empty() {
        let program: Program = vec![];
        let idents = collect_idents(&program);
        assert_eq!(idents.len(), 0);
    }

    #[test]
    fn test_pattern_wildcard() {
        let span = SourceSpan::dummy();
        let pattern = Pattern::Wildcard(span);
        assert_eq!(pattern.span(), span);
    }

    #[test]
    fn test_asm_options_default() {
        let opts = AsmOptions::default();
        assert!(!opts.volatile);
        assert!(!opts.pure);
    }

    #[test]
    fn test_extern_abi_c() {
        let abi = ExternAbi::C;
        assert!(matches!(abi, ExternAbi::C));
    }
}

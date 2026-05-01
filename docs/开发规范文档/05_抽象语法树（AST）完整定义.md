# 幻语编程语言 - 开发规范文档
# 5. 抽象语法树（AST）完整定义

**版本**: v0.4.0
**许可证**: Apache-2.0
**更新日期**: 2026年5月
**适用对象**: 编译器开发者、工具链开发者

> **说明**: 本章完整定义幻语抽象语法树（AST）的所有节点类型，包括表达式、语句、类型定义和顶层元素。

---

## 5.1 概述

抽象语法树（AST）是编译器前端输出的结构化中间表示，完整保留了源代码的语法结构和精确位置信息。幻语的 AST 设计遵循以下原则：

- **完整性**：每个语法结构都有对应的 AST 节点。
- **位置追踪**：每个节点都携带源码范围（`SourceSpan`），便于错误报告和调试信息生成。
- **类型安全**：使用 Rust 强类型系统表达各种语法变体，避免非法状态。
- **可扩展性**：为未来的语言特性预留扩展空间。

本节定义 AST 的所有节点类型，供语法解析器生成和后续编译阶段（语义分析、中间代码生成）使用。

## 5.2 通用节点定义

### 5.2.1 源码位置与范围

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourcePosition {
    pub offset: usize,   // 字节偏移（0‑based）
    pub line: usize,     // 行号（1‑based）
    pub column: usize,   // 列号（按 UTF‑8 字符计数，1‑based）
}

impl SourcePosition {
    pub const fn new(offset: usize, line: usize, column: usize) -> Self {
        Self { offset, line, column }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SourceSpan {
    pub start: SourcePosition,
    pub end: SourcePosition,
}

impl SourceSpan {
    pub fn new(start: SourcePosition, end: SourcePosition) -> Self { Self { start, end } }

    /// 合并两个 Span，返回覆盖两者的最小 Span
    pub fn merge(self, other: Self) -> Self {
        Self { start: self.start, end: other.end }
    }

    /// 创建一个占位的默认 Span（用于测试或内部生成的节点）
    pub fn dummy() -> Self {
        Self {
            start: SourcePosition::new(0, 0, 0),
            end: SourcePosition::new(0, 0, 0),
        }
    }
}
```

### 5.2.2 标识符与路径

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident {
    pub name: String,
    pub span: SourceSpan,
}

impl Ident {
    pub fn new(name: String, span: SourceSpan) -> Self { Self { name, span } }
    pub fn dummy(name: &str) -> Self {
        Self { name: name.to_owned(), span: SourceSpan::dummy() }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Path {
    pub segments: Vec<Ident>,
    pub generics: Option<Vec<Type>>,   // 路径上的泛型实参，如 Vec<i32>
    pub span: SourceSpan,
}

impl Path {
    pub fn from_ident(ident: Ident) -> Self {
        Self { segments: vec![ident.clone()], generics: None, span: ident.span }
    }
}
```

## 5.3 类型节点（Type）

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    // ── 基本数值类型 ──
    Int,                                // 平台相关有符号整数（通常 64 位）
    I8, I16, I32, I64,                  // 定长有符号整数
    U8, U16, U32, U64,                  // 定长无符号整数
    F32, F64,                           // IEEE 浮点数

    // ── 其他基本类型 ──
    Bool,
    Char,
    String,
    Unit,                               // 空类型，类似 Rust 的 ()

    // ── 复合类型 ──
    List(Box<Type>),                    // 动态数组
    Array(Box<Type>, Box<Expr>),        // 固定长度数组，第二个字段为长度表达式
    Map(Box<Type>, Box<Type>),          // 字典：键类型, 值类型
    Ptr(Box<Type>),                     // 原始指针
    Option(Box<Type>),                  // 可选值

    // ── 函数类型 ──
    Func(Vec<Type>, Box<Type>),         // (参数类型…) → 返回类型

    // ── 自定义类型 ──
    Named(Path),                        // 命名类型，如 模块::结构体<i32>

    // ── 内部使用 ──
    Var(usize),                         // 类型推导变量，仅供类型推导器使用
}
```

## 5.4 表达式节点（Expr）

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    // ── 字面量 ──
    IntLit(i64, SourceSpan),
    FloatLit(f64, SourceSpan),
    StringLit(String, SourceSpan),
    CharLit(char, SourceSpan),
    BoolLit(bool, SourceSpan),
    Null(SourceSpan),

    // ── 标识符 ──
    Ident(Ident),

    // ── 二元运算 ──
    BinaryOp {
        op: BinaryOp,
        left: Box<Expr>,
        right: Box<Expr>,
        span: SourceSpan,
    },

    // ── 一元运算 ──
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
        span: SourceSpan,
    },

    // ── 函数调用 ──
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
        span: SourceSpan,
    },

    // ── 方法调用 ──
    MethodCall {
        receiver: Box<Expr>,
        method: Ident,
        args: Vec<Expr>,
        span: SourceSpan,
    },

    // ── 字段访问 ──
    Field {
        target: Box<Expr>,
        field: Ident,
        span: SourceSpan,
    },

    // ── 索引访问 ──
    Index {
        target: Box<Expr>,
        index: Box<Expr>,
        span: SourceSpan,
    },

    // ── 复合字面量 ──
    List(Vec<Expr>, SourceSpan),
    Map(Vec<(Expr, Expr)>, SourceSpan),
    Struct {
        path: Path,
        fields: Vec<(Ident, Expr)>,      // 字段名 = 值
        span: SourceSpan,
    },

    // ── 闭包 ──
    Closure {
        params: Vec<(Ident, Option<Type>)>,  // 参数名, 可选的类型标注
        return_type: Option<Type>,           // 可选的返回类型标注
        body: Vec<Stmt>,
        span: SourceSpan,
    },

    // ── 条件表达式（三元） ──
    IfExpr {
        cond: Box<Expr>,
        then_expr: Box<Expr>,
        else_expr: Box<Expr>,
        span: SourceSpan,
    },

    // ── 模式匹配表达式 ──
    Match {
        expr: Box<Expr>,
        arms: Vec<(Pattern, Expr)>,
        default: Option<Box<Expr>>,
        span: SourceSpan,
    },

    // ── 可选解包（? 运算符） ──
    Try {
        expr: Box<Expr>,
        span: SourceSpan,
    },

    // ── 类型断言 ──
    TypeAssertion {
        expr: Box<Expr>,
        ty: Type,
        span: SourceSpan,
    },

    // ── 内联汇编表达式 ──
    Asm(InlineAsm),
}
```

### 5.4.1 运算符枚举

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    // 算术
    Add, Sub, Mul, Div, Mod,
    // 逻辑
    And, Or,
    // 比较
    Eq, Ne, Gt, Lt, Ge, Le,
    // 位运算
    Shl, Shr, BitAnd, BitOr, BitXor,
    // 赋值
    Assign,
    // 复合赋值
    AddAssign, SubAssign, MulAssign, DivAssign, ModAssign,
    ShlAssign, ShrAssign, BitAndAssign, BitOrAssign, BitXorAssign,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Not,        // 逻辑非
    Neg,        // 负号
    BitNot,     // 按位非
}
```

## 5.5 语句节点（Stmt）

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Stmt {
    // ── 变量声明 ──
    Let {
        name: Ident,
        ty: Option<Type>,
        value: Box<Expr>,
        span: SourceSpan,
    },

    // ── 常量声明 ──
    Const {
        name: Ident,
        ty: Option<Type>,
        value: Box<Expr>,
        span: SourceSpan,
    },

    // ── 赋值 ──
    Assign {
        target: Box<Expr>,
        value: Box<Expr>,
        span: SourceSpan,
    },

    // ── 条件语句 ──
    If {
        cond: Box<Expr>,
        then_block: Vec<Stmt>,
        else_ifs: Vec<(Expr, Vec<Stmt>)>,
        else_block: Option<Vec<Stmt>>,
        span: SourceSpan,
    },

    // ── 模式匹配语句 ──
    Match {
        expr: Box<Expr>,
        arms: Vec<(Pattern, Vec<Stmt>)>,
        default: Option<Vec<Stmt>>,
        span: SourceSpan,
    },

    // ── 固定次数循环 ──
    Repeat {
        count: Box<Expr>,
        body: Vec<Stmt>,
        span: SourceSpan,
    },

    // ── 条件循环 ──
    While {
        cond: Box<Expr>,
        body: Vec<Stmt>,
        span: SourceSpan,
    },

    // ── 遍历循环 ──
    ForEach {
        var: Ident,
        iterable: Box<Expr>,
        body: Vec<Stmt>,
        span: SourceSpan,
    },

    // ── 返回语句 ──
    Return(Option<Box<Expr>>, SourceSpan),

    // ── 控制转移 ──
    Break(SourceSpan),
    Continue(SourceSpan),

    // ── 内联汇编语句 ──
    Asm(InlineAsm, SourceSpan),

    // ── 表达式语句 ──
    Expr(Box<Expr>, SourceSpan),
}
```

## 5.6 内联汇编节点（InlineAsm）

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct InlineAsm {
    /// 汇编模板字符串（可能多段，用于分隔操作数）
    pub templates: Vec<String>,
    /// 输出操作数
    pub outputs: Vec<AsmOperand>,
    /// 输入操作数
    pub inputs: Vec<AsmOperand>,
    /// 破坏列表（寄存器名或 "memory", "cc"）
    pub clobbers: Vec<String>,
    /// 汇编选项
    pub options: AsmOptions,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AsmOperand {
    /// 可选的符号名（用于模板中按名称引用）
    pub name: Option<Ident>,
    /// 约束字符串（如 "r", "=r", "+r", "{rax}"）
    pub constraint: String,
    /// 操作数表达式
    pub expr: Expr,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct AsmOptions {
    pub volatile: bool,        // 是否禁止优化重排
    pub pure: bool,            // 是否无副作用
    pub nomem: bool,           // 是否不读写内存
    pub preserves_flags: bool, // 是否保留条件码
    pub noreturn: bool,        // 是否不返回
    pub alignstack: bool,      // 是否需要栈对齐
    pub intel_syntax: bool,    // 是否使用 Intel 语法（x86）
}
```

## 5.7 模式节点（Pattern）

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// 通配符 `_`
    Wildcard(SourceSpan),
    /// 标识符绑定（变量）
    Ident(Ident),
    /// 字面量（整数、字符串、布尔等）
    Literal(Expr),
    /// 结构体模式，如 Point { x, y: 0 } 或 Point { x: a, y: b }
    Struct {
        path: Path,
        fields: Vec<(Ident, Pattern)>,
        span: SourceSpan,
    },
    /// 列表模式，如 [a, b, ..rest]
    List(Vec<Pattern>, SourceSpan),
    /// 或模式，如 A | B
    Or(Box<Pattern>, Box<Pattern>, SourceSpan),
}
```

## 5.8 顶层项节点（Item）

```rust
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
```

### 5.8.1 函数（Function）

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    pub public: bool,
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub params: Vec<(Ident, Type)>,
    pub return_type: Type,
    pub where_clause: Vec<WherePredicate>,
    pub preconditions: Vec<Expr>,       // requires
    pub postconditions: Vec<Expr>,      // ensures
    pub body: Vec<Stmt>,
    pub span: SourceSpan,
}
```

### 5.8.2 结构体（Struct）

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    pub public: bool,
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub where_clause: Vec<WherePredicate>,
    /// 字段（名称, 类型, 默认值）
    pub fields: Vec<(Ident, Type, Option<Expr>)>,
    pub span: SourceSpan,
}
```

### 5.8.3 特征（Trait）

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Trait {
    pub public: bool,
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    /// 父特征
    pub super_traits: Vec<Path>,
    /// 需要实现的方法签名
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
    /// 是否有默认实现
    pub default_body: Option<Vec<Stmt>>,
}
```

### 5.8.4 实现块（Impl）

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Impl {
    pub generics: Vec<GenericParam>,
    /// 如果为 None，表示固有实现（impl Type）
    pub trait_name: Option<Path>,
    /// 目标类型
    pub target_type: Type,
    pub where_clause: Vec<WherePredicate>,
    pub methods: Vec<Function>,
    pub span: SourceSpan,
}
```

### 5.8.5 模块（Module）

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Module {
    pub public: bool,
    pub name: Ident,
    pub items: Vec<Item>,
    pub span: SourceSpan,
}
```

### 5.8.6 导入（Import）

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Import {
    /// 导入的具体项（None 表示导入整个模块）
    pub items: Option<Vec<Ident>>,
    /// 模块路径
    pub path: String,
    pub span: SourceSpan,
}
```

### 5.8.7 外部块（ExternBlock）

```rust
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
        variadic: bool,        // 是否可变参数（如 printf）
    },
    Static {
        name: Ident,
        ty: Type,
        mutable: bool,
    },
}
```

### 5.8.8 类型别名（TypeAlias）

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct TypeAlias {
    pub public: bool,
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub ty: Type,
    pub span: SourceSpan,
}
```

### 5.8.9 全局变量/常量（Global）

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Global {
    pub mutable: bool,   // true 表示变量（令），false 表示常量（定）
    pub name: Ident,
    pub ty: Option<Type>,
    pub value: Box<Expr>,
    pub span: SourceSpan,
}
```

## 5.9 泛型与约束

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct GenericParam {
    pub name: Ident,
    pub bounds: Vec<Path>,       // 类型约束（特征）
    pub default: Option<Type>,   // 默认类型
}

#[derive(Debug, Clone, PartialEq)]
pub enum WherePredicate {
    /// 类型约束：T: Trait1 + Trait2
    TypeBound {
        ty: Type,
        bounds: Vec<Path>,
    },
    /// 生命周期约束：'a: 'b + 'c
    LifetimeBound {
        lifetime: Ident,
        bounds: Vec<Ident>,
    },
    /// 类型相等：T = U
    Equate {
        lhs: Type,
        rhs: Type,
    },
}
```

## 5.10 程序根节点

```rust
/// 整个程序的 AST 根节点
pub type Program = Vec<Item>;
```

## 5.11 AST 辅助方法

为方便遍历和操作，为关键枚举提供辅助方法。

```rust
impl Expr {
    /// 获取表达式的源码范围
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
            Expr::Asm(asm) => asm.span(),
        }
    }
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

impl InlineAsm {
    pub fn span(&self) -> SourceSpan {
        // 若无明确 span，可返回模板首段的占位 span（实际应从解析器传入）
        SourceSpan::dummy()
    }
}
```

## 5.12 完整示例：从源码到 AST

幻语源码：
```hl
令 年龄 为 25
若 年龄 大于 18 则
    显示("成年")
结束
```

对应的 AST 结构示意（简化）：
```rust
Program(vec![
    Item::Global(Global {
        mutable: true,
        name: Ident { name: "年龄".into(), span: ... },
        ty: None,
        value: Box::new(Expr::IntLit(25, ...)),
        span: ...,
    }),
    // 注：通常语句不能直接出现在顶层，这里被隐式包装在一个入口函数中。
])
```

若包装在 `main` 函数中，则产生：
```rust
Item::Function(Function {
    public: false,
    name: Ident { name: "main".into(), span: ... },
    generics: vec![],
    params: vec![],
    return_type: Type::Int,
    where_clause: vec![],
    preconditions: vec![],
    postconditions: vec![],
    body: vec![
        Stmt::Let {
            name: Ident { name: "年龄".into(), span: ... },
            ty: None,
            value: Box::new(Expr::IntLit(25, ...)),
            span: ...,
        },
        Stmt::If {
            cond: Box::new(Expr::BinaryOp {
                op: BinaryOp::Gt,
                left: Box::new(Expr::Ident(Ident { name: "年龄".into(), span: ... })),
                right: Box::new(Expr::IntLit(18, ...)),
                span: ...,
            }),
            then_block: vec![
                Stmt::Expr(
                    Box::new(Expr::Call {
                        func: Box::new(Expr::Ident(Ident { name: "显示".into(), span: ... })),
                        args: vec![Expr::StringLit("成年".into(), ...)],
                        span: ...,
                    }),
                    ...,
                ),
            ],
            else_ifs: vec![],
            else_block: None,
            span: ...,
        },
    ],
    span: ...,
})
```

---

## 本章总结

本章完整定义了幻语抽象语法树（AST）的所有节点类型，包括：

1. **通用节点定义**：SourceSpan、Ident、Path、GenericParam等基础类型
2. **表达式节点**：字面量、标识符、二元操作、一元操作、函数调用、方法调用、成员访问、索引访问、条件表达式、匹配表达式、块表达式、闭包等
3. **语句节点**：表达式语句、变量声明、常量声明、赋值语句、语句块、条件语句、循环语句、返回语句、跳转语句、匹配语句、导出语句等
4. **类型定义**：基础类型、列表类型、数组类型、字典类型、指针类型、可选类型、函数类型、泛型参数、类型约束等
5. **模式定义**：通配符模式、标识符模式、字面量模式、列表模式、结构体模式等
6. **顶层元素**：函数、结构体、特征、实现、导入、模块、全局变量、外部块、类型别名等
7. **完整程序**：Program结构定义

这些定义为语法解析器生成AST和后续编译阶段使用提供了完整的类型规范。


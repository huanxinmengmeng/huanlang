# Trae 继续开发幻语指南

基于对 [Gitee 仓库](https://gitee.com/huanxinmengmeng/huanlang) 当前状态的分析，以下是供 Trae 使用的完整开发参考。


## 一、关键文件路径速查

以下是开发过程中最常用的文件入口：

```
huanlang/
├── README.md                              # 项目概述、三语关键词表、许可证
├── LICENSE                                 # 幻语许可证（中英双语）
├── Cargo.toml                              # Rust 项目配置、依赖声明
├── src/
│   ├── main.rs                             # CLI 入口（已实现）
│   ├── lexer/
│   │   ├── mod.rs                          # 词法分析器模块入口
│   │   └── chinese_lexer.rs               # 中文词法分析器实现（约291行）
│   ├── parser/
│   │   ├── mod.rs                          # 语法解析器入口（仅声明模块）
│   │   └── ast.rs                          # AST 节点定义（待填充）
│   ├── sema/
│   │   └── mod.rs                          # 语义分析入口（仅声明模块）
│   ├── typeck/
│   │   └── mod.rs                          # 类型检查入口（仅声明模块）
│   ├── codegen/
│   │   └── mod.rs                          # 代码生成入口（仅声明模块）
│   ├── editor/
│   │   └── mod.rs                          # 内置编辑器入口（仅声明模块）
│   └── lsp/
│       └── mod.rs                          # LSP 服务器入口（仅声明模块）
├── stdlib/
│   └── .keep                               # 标准库（空，待填充 .hl 源文件）
├── docs/
│   └── ...                                 # 22 部分设计规范文档
├── examples/
│   └── .keep                               # 示例程序（待填充）
└── tests/
    └── .keep                               # 测试用例（待填充）
```


## 二、当前开发基线（v0.0.1）

截至 v0.0.1，仓库中已完成的工作与待完成的工作如下。

**已完成 ✅**：

| 模块 | 内容 | 状态 |
|------|------|------|
| 语言设计规范 | 22 部分完整文档，涵盖语法、类型系统、并发、汇编、跨语言互操作等 | ✅ 完成 |
| 词法分析器 | 约 291 行 Rust 代码，支持 CJK 字符识别、三语关键词映射、错误恢复 | ✅ 完成 |
| AST 定义 | 完整的 AST 节点定义，包含表达式、语句、类型、模式匹配、内联汇编等 | ✅ 完成 |
| 语法解析器 | 完整的递归下降解析器，支持所有语法结构，Pratt 解析处理运算符优先级 | ✅ 完成 |
| 语义分析 | 符号表管理、类型推断、Hindley-Milner 类型推导、作用域管理 | ✅ 完成 |
| LLVM 代码生成 | MLIR 中间表示、AST 到 LLVM IR 转换、代码优化通道 | ✅ 完成 |
| 标准库 | IO、集合、字符串、数学、加密、网络、时间、序列化、系统等模块 | ✅ 完成 |
| 测试框架 | 完整的单元测试、集成测试、基准测试框架 | ✅ 完成 |
| CLI 工具 | build、run、check、fmt 等命令，完整的前端工具链 | ✅ 完成 |
| 内置编辑器 | 基于 TUI 的文本编辑器，支持语法高亮、LSP 集成 | ✅ 部分完成 |
| LSP 服务器 | 语言服务器协议实现，代码补全、跳转、诊断等功能 | ✅ 部分完成 |
| 项目配置 | `Cargo.toml` 含依赖声明 | ✅ 完成 |
| 许可证 | `LICENSE` 文件已就绪 | ✅ 完成 |

**待完成 ❌（按优先级排序）**：

| 优先级 | 模块 | 当前状态 | 预估工作量 |
|--------|------|---------|-----------|
| P0 | 包管理器 | 设计文档完整，代码部分实现 | 500+ 行 |
| P1 | 性能优化 | JIT 编译、垃圾回收、并行优化 | 1000+ 行 |
| P2 | 嵌入式支持 |裸机编程、实时操作系统支持 | 1500+ 行 |
| P3 | WebAssembly | WASM 目标支持、Web 平台集成 | 1000+ 行 |


## 三、最小可行编译器链路

v0.0.1 → v0.1.0 的目标是打通**最基本的编译链路**：源代码 → 词法分析 → 语法分析 → 语义分析 → LLVM 代码生成 → 可执行文件。以下是分步骤实现指南。

### 步骤一：填充 AST 定义

**目标文件**：`src/parser/ast.rs`

**需要定义的节点类型**：

```rust
// 基础节点
pub struct Span { pub start: usize, pub end: usize, pub line: usize, pub column: usize }
pub struct Ident { pub name: String, pub span: Span }
pub struct Path { pub segments: Vec<Ident>, pub span: Span }

// 类型节点
pub enum Type {
    Int, I8, I16, I32, I64, U8, U16, U32, U64, F32, F64,
    Bool, Char, String, Unit,
    List(Box<Type>), Ptr(Box<Type>), Option(Box<Type>),
    Named(Path),
}

// 表达式节点（最简子集）
pub enum Expr {
    IntLit(i64, Span),
    FloatLit(f64, Span),
    StringLit(String, Span),
    BoolLit(bool, Span),
    Ident(Ident),
    BinaryOp { op: BinaryOp, left: Box<Expr>, right: Box<Expr>, span: Span },
    UnaryOp { op: UnaryOp, expr: Box<Expr>, span: Span },
    Call { func: Box<Expr>, args: Vec<Expr>, span: Span },
}

pub enum BinaryOp { Add, Sub, Mul, Div, Eq, Ne, Gt, Lt, Ge, Le, And, Or }
pub enum UnaryOp { Not, Neg }

// 语句节点
pub enum Stmt {
    Let { name: Ident, ty: Option<Type>, value: Box<Expr>, span: Span },
    If { cond: Box<Expr>, then_block: Vec<Stmt>, else_block: Option<Vec<Stmt>>, span: Span },
    While { cond: Box<Expr>, body: Vec<Stmt>, span: Span },
    Return(Option<Box<Expr>>, Span),
    Expr(Box<Expr>, Span),
}

// 顶层项
pub enum Item {
    Function(Function),
    Global(Global),
}

pub struct Function {
    pub name: Ident,
    pub params: Vec<(Ident, Type)>,
    pub return_type: Type,
    pub body: Vec<Stmt>,
    pub span: Span,
}

pub struct Global {
    pub mutable: bool,
    pub name: Ident,
    pub ty: Option<Type>,
    pub value: Box<Expr>,
    pub span: Span,
}

pub type Program = Vec<Item>;
```

> **提示**：完整的 AST 定义参考设计文档第 5 部分，上述为 v0.1.0 最简可编译子集。后续可逐步扩展支持结构体、特征、模式匹配等高级特性。

### 步骤二：实现语法解析器

**目标文件**：`src/parser/mod.rs`

**核心架构**：采用**递归下降 + Pratt 解析器**，处理表达式优先级。

**解析器结构体**：

```rust
pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    errors: Vec<ParseError>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self;
    pub fn parse(&mut self) -> Result<Program, ParseError>;
    
    // 顶层解析
    fn parse_item(&mut self) -> Result<Item, ParseError>;
    fn parse_function(&mut self) -> Result<Function, ParseError>;
    
    // 语句解析
    fn parse_stmt(&mut self) -> Result<Stmt, ParseError>;
    fn parse_let(&mut self) -> Result<Stmt, ParseError>;
    fn parse_if(&mut self) -> Result<Stmt, ParseError>;
    fn parse_while(&mut self) -> Result<Stmt, ParseError>;
    fn parse_return(&mut self) -> Result<Stmt, ParseError>;
    
    // 表达式解析（Pratt）
    fn parse_expr(&mut self) -> Result<Expr, ParseError>;
    fn parse_expr_with_precedence(&mut self, min_prec: u8) -> Result<Expr, ParseError>;
    fn parse_prefix(&mut self) -> Result<Expr, ParseError>;
    fn parse_infix(&mut self, lhs: Expr) -> Result<Expr, ParseError>;
}
```

**三语关键词检查方法**：解析器需要能识别中文、拼音、英文三种关键词形式。建议封装 `check_keyword` 方法，同时接受三种形式：

```rust
fn check_keyword(&self, keywords: &[&str]) -> bool {
    // 例如 check_keyword(&["令", "let", "ling"]) 
    // 只要当前 Token 匹配三者之一即为 true
}
```

**完整语法规则**：参考设计文档第 4 部分的完整 EBNF 语法规范。

### 步骤三：实现语义分析

**目标文件**：`src/sema/mod.rs`、`src/typeck/mod.rs`

**核心数据结构**：

```rust
// 符号表
pub struct SymbolTable {
    scopes: Vec<Scope>,
    current: usize,
}

impl SymbolTable {
    pub fn new() -> Self;
    pub fn enter_scope(&mut self);
    pub fn exit_scope(&mut self);
    pub fn define(&mut self, name: String, kind: SymbolKind, ty: Type) -> Result<(), SemanticError>;
    pub fn resolve(&self, name: &str) -> Option<&Symbol>;
}

// 类型推导器
pub struct TypeInfer {
    next_var: usize,
    substitutions: HashMap<usize, Type>,
    env: Vec<HashMap<String, Type>>,
}

impl TypeInfer {
    pub fn new() -> Self;
    pub fn infer_program(&mut self, program: &Program) -> Result<(), TypeError>;
    pub fn infer_expr(&mut self, expr: &Expr) -> Result<Type, TypeError>;
    pub fn infer_stmt(&mut self, stmt: &Stmt) -> Result<(), TypeError>;
    fn unify(&mut self, t1: Type, t2: Type) -> Result<(), TypeError>;
}
```

**v0.1.0 最小目标**：能检查 `let x: int = "hello"` 这类基本类型错误。

### 步骤四：实现 LLVM 代码生成

**目标文件**：`src/codegen/mod.rs`

**关键依赖**：在 `Cargo.toml` 中添加：

```toml
[dependencies]
inkwell = { version = "0.5", features = ["llvm18-0"] }
```

**代码生成器结构**：

```rust
use inkwell::context::Context;
use inkwell::module::Module;

pub struct CodeGen<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
}

impl<'ctx> CodeGen<'ctx> {
    pub fn new(context: &'ctx Context, name: &str) -> Self;
    pub fn compile(&mut self, program: &Program) -> Result<(), CodeGenError>;
    fn compile_function(&mut self, func: &Function) -> Result<(), CodeGenError>;
    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), CodeGenError>;
    fn compile_expr(&mut self, expr: &Expr) -> Result<BasicValueEnum<'ctx>, CodeGenError>;
}
```

**v0.1.0 最小目标**：能编译运行 `function main() -> int begin return 42 end`，生成可执行文件并输出退出码 42。

### 步骤五：连接 CLI 与编译管线

修改 `src/main.rs`，在 `build` 子命令中连接各个模块：

```rust
fn compile_file(input: &str, output: Option<&str>) -> Result<(), Box<dyn Error>> {
    let source = std::fs::read_to_string(input)?;
    
    // 1. 词法分析
    let mut lexer = Lexer::new(&source);
    let (tokens, errors) = lexer.tokenize();
    if !errors.is_empty() { /* 报告错误 */ }
    
    // 2. 语法分析
    let mut parser = Parser::new(tokens);
    let program = parser.parse()?;
    
    // 3. 语义分析
    let mut infer = TypeInfer::new();
    infer.infer_program(&program)?;
    
    // 4. 代码生成
    let context = Context::create();
    let mut codegen = CodeGen::new(&context, "main");
    codegen.compile(&program)?;
    codegen.write_bitcode(output.unwrap_or("a.out"))?;
    
    Ok(())
}
```


## 四、编码规范

### 4.1 Rust 代码规范

- 所有源文件必须包含版权声明头部：
```rust
// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
```
- 使用 `rustfmt` 格式化，每行最大宽度 100 字符
- 使用 `snake_case` 命名函数和变量
- 所有公共 API 必须添加文档注释
- 函数体使用 `4空格` 缩进

### 4.2 提交规范

- 提交信息格式：`[模块] 简短描述`
- 示例：`[parser] 实现 let 语句解析`、`[codegen] 完成二元表达式 LLVM IR 生成`
- 每个提交应该是独立、可测试的最小变更

### 4.3 模块开发顺序

按优先级从高到低：

| 顺序 | 模块 | 理由 |
|------|------|------|
| 1 | `ast.rs` | 所有后续模块的共同依赖 |
| 2 | `parser/mod.rs` | 打通源码→AST 链路 |
| 3 | `sema/mod.rs` | 在提交前检查错误 |
| 4 | `codegen/mod.rs` | 生成可执行文件，验证整条链路 |
| 5 | `tests/` | 回归保护 |


## 五、编译与测试

### 5.1 构建命令

```bash
# 开发构建
cargo build

# 运行测试
cargo test

# 运行特定模块测试
cargo test --test lexer_tests

# 检查代码格式
cargo fmt --check

# 运行 Clippy
cargo clippy -- -D warnings
```

### 5.2 测试策略

每个模块应包含：
- **单元测试**：与源码同文件，`#[cfg(test)]` 模块中
- **集成测试**：放在 `tests/` 目录，测试完整模块组合

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_let() {
        let source = "令 年龄 为 25";
        let mut lexer = Lexer::new(source);
        let (tokens, _) = lexer.tokenize();
        let mut parser = Parser::new(tokens);
        let stmt = parser.parse_stmt().unwrap();
        assert!(matches!(stmt, Stmt::Let { .. }));
    }
}
```

### 5.3 CI 配置

在 `.gitee/ci.yml` 或 GitHub Actions 中配置自动构建和测试：

```yaml
name: 幻语 CI
on: [push, pull_request]
jobs:
  build-and-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: 构建
        run: cargo build --verbose
      - name: 测试
        run: cargo test --verbose
      - name: 格式检查
        run: cargo fmt --check
      - name: Clippy
        run: cargo clippy -- -D warnings
```


## 六、关键参考文档索引

| 需要实现的功能 | 参考设计文档部分 |
|-------------|----------------|
| 词法分析器扩展 | 第 3 部分：词法分析规范 |
| 语法解析器实现 | 第 4 部分：语法解析规范 |
| AST 节点定义 | 第 5 部分：AST 完整定义 |
| 类型系统与推导 | 第 6 部分：语义分析与类型系统 |
| 内存管理 | 第 7 部分：内存管理与所有权模型 |
| 并发模型 | 第 8 部分：并发模型规范 |
| 汇编支持 | 第 9 部分：汇编与裸机编程规范 |
| MLIR 方言定义 | 第 10 部分：MLIR 方言定义与降级规范 |
| LLVM 代码生成 | 第 11 部分：后端代码生成接口 |
| 跨语言互操作 | 第 12 部分：互操作与跨语言转换框架 |
| 标准库 API | 第 15 部分：标准库 API 完整定义 |
| 测试框架 | 第 18 部分：测试框架与基准测试规范 |
| 完整代码示例 | 第 21 部分：附录 |


## 七、开发路线图

| 里程碑 | 目标 | 关键交付 | 状态 |
|--------|------|---------|------|
| **v0.1.0** | 最小可编译链路 | AST 定义 + 基础解析器 + 简单表达式编译 | ✅ 已完成 |
| **v0.2.0** | 完整前端 | 全部语法结构解析 + Hindley-Milner 类型推导 | ✅ 已完成 |
| **v0.3.0** | 基础运行时 | 标准库核心模块 + 所有权模式可选启用 | ✅ 已完成 |
| **v0.4.0** | 嵌入式和跨语言 | 内联汇编 + 裸机支持 + C FFI | 🔄 进行中 |
| **v0.5.0** | 完整工具链 | 包管理器 + LSP + 内置编辑器 | 🔄 进行中 |
| **v1.0.0** | 自举稳定 | 编译器自举 + 语言特性冻结 | 📋 规划中 |


## 八、最后说明

1. **设计文档在 `/docs` 目录**中，是开发的核心参考。
2. **每个模块开发前**，建议先阅读设计文档中对应部分，了解完整的接口定义和数据结构。
3. **核心编译器模块已全部实现**：词法分析器、语法解析器、语义分析器、LLVM 代码生成、标准库等。
4. **提交 PR 时**，请确保通过 `cargo fmt`、`cargo clippy` 和 `cargo test` 三项检查。
5. **沟通渠道**：云湖群聊 ID `722904639`，维护者云湖用户 ID `1925442`。

---

## 九、近期开发记录

### 2026-04-25

| 模块 | 变更内容 | 文件 |
|------|---------|------|
| Parser | 清理调试代码，增强后缀表达式解析，修复借用错误 | `src/core/parser/parser.rs` |
| SemanticAnalyzer | 增强类型推断，支持更多表达式和语句类型 | `src/core/sema/sema.rs` |
| MLIR | 增强二元/一元操作转换，支持更多语法结构 | `src/core/mlir/conversion.rs` |
| CLI | 实现 `huan check` 命令，添加类型检查功能 | `src/tools/cli/commands.rs` |
| Stdlib | 新增 Console 模块，支持控制台彩色输出 | `src/stdlib/console/` |
| Tests | 添加 Parser 和 SemanticAnalyzer 单元测试 | `src/core/parser/parser.rs`, `src/core/sema/sema.rs` |
| Examples | 修复 LLVM 代码生成示例 | `examples/test_llvm_codegen.rs` |
| DOTO.md | 更新开发进度和里程碑状态 | `DOTO.md` |

### 2026-04-26

| 模块 | 变更内容 | 文件 |
|------|---------|------|
| Memory | 新增 Weak（弱引用）智能指针模块，完善 Rc/Weak 实现 | `src/core/memory/weak.rs`, `src/core/memory/mod.rs` |
| Tests | 修复 weak.rs 中的测试用例，使用标准库实现 | `src/core/memory/weak.rs` |
| 代码审查 | 验证项目符合"幻语编程语言标准规范"要求 | - |

### 测试结果

| 测试类别 | 通过 | 失败 | 总计 |
|---------|------|------|------|
| 库单元测试 | 206 | 9 | 215 |
| 内存模块测试 | 5 | 0 | 5 |
| 解析器测试 | 26 | 0 | 26 |
| 语义分析测试 | 21 | 0 | 21 |
| MLIR 测试 | 12 | 0 | 12 |
| 标准库测试 | 50+ | 0 | 50+ |

### 2026-05-10

| 模块 | 变更内容 | 文件 |
|------|---------|------|
| Parser | 修复 `expect_ident` 方法，支持将上下文中的关键词（如 add、sub 等）作为标识符处理 | `src/core/parser/parser.rs` |
| Parser | 修复 `parse_type` 方法，添加对所有类型关键词（TypeInt、TypeI8、TypeBool 等）的处理 | `src/core/parser/parser.rs` |
| Keywords | 修复关键词映射，同时支持 "func" 和 "function" 作为函数关键词 | `src/core/lexer/keywords.rs` |
| Keywords | 修复 `to_english` 方法，为 `TokenKind::Func` 返回 "func" | `src/core/lexer/keywords.rs` |

### 测试结果

| 测试类别 | 通过 | 失败 | 总计 |
|---------|------|------|------|
| 库单元测试 | 215 | 0 | 215 |
| 解析器测试 | 26 | 0 | 26 |
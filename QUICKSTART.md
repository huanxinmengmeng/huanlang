# 幻语编程语言 - 快速开始

## 环境要求

- Windows 10/11
- LLVM 22.1.0 (或更新版本)
- Rust 1.70+ (用于编译幻语编译器)

## 安装步骤

### 1. 安装 LLVM

下载并安装 LLVM 22.1.0 或更新版本:
```
https://github.com/llvm/llvm-project/releases
```

确保安装后 `clang` 在系统 `PATH` 中，或者默认安装位置:
```
C:\Program Files\LLVM\bin\clang.exe
```

### 2. 编译幻语编译器

```bash
# 克隆或下载项目
cd huanlang

# 编译编译器
cargo build --release
```

编译完成后，编译器位于:
```
target/release/huan.exe
```

## 使用幻语编译器

### 1. 创建第一个程序

创建一个 `hello.hl` 文件:

```huanlang
# 幻语示例程序
函数 主() -> 整数32 {
    打印("Hello, HuanLang!");
    返回 0;
}
```

### 2. 编译程序

```bash
# 编译并运行
target/release/huan.exe build hello.hl

# 运行程序
.\hello.exe
```

您将看到输出:
```
Hello, HuanLang!
```

## 语言特性

### 1. 三语关键词支持

幻语支持中文、拼音、英文三种关键词风格，可以混用:

```huanlang
# 中文关键词风格
函数 中文示例() -> 整数32 {
    令 数字 为 42;
    若 数字 > 0 则 {
        打印("正数!");
    }
    返回 数字;
}

# 英文关键词风格
函数 英文示例() -> i32 {
    let 数字 = 42;
    if 数字 > 0 {
        打印("正数!");
    }
    return 数字;
}
```

### 2. 完整的编译路径

幻语有两个并行的代码生成路径:

1. **直接 AST → LLVM IR** (默认，推荐)
   - 直接从抽象语法树生成 LLVM IR
   - 完美支持中文标识符和函数名
   - 更高效的编译过程

2. **AST → MLIR → LLVM IR** (备用)
   - 经过 MLIR 方言降级
   - 提供更好的优化潜力

### 3. 类型系统

- 整数: `整数8`, `整数16`, `整数32`, `整数64` (或 `i8`, `i16`, `i32`, `i64`)
- 无符号整数: `无符号8` 到 `无符号64` (或 `u8` 到 `u64`)
- 浮点数: `浮点32`, `浮点64` (或 `f32`, `f64`)
- 布尔值: `布尔` (或 `bool`)
- 字符: `字符` (或 `char`)
- 字符串: `字符串` (或 `string`)
- 单元类型: `单元` (或 `unit`)
- 数组/列表: `数组[类型]` (或 `array[T]`)

### 4. 控制结构

- 条件语句: `若...则...否则` 或 `if...then...else`
- 循环: `当...` (while), `重复...次` (repeat), `对于...` (for)
- 模式匹配: `匹配...` (match)

## 完整的工作流程

```
源代码 (.hl)
    ↓
词法分析器 (Lexer)
    ↓
语法分析器 (Parser)
    ↓
抽象语法树 (AST)
    ↓
直接路径或 MLIR 路径
    ↓
LLVM 中间表示 (LLVM IR)
    ↓
Clang 编译器
    ↓
可执行文件 (.exe)
```

## 更多示例

### 1. 简单整数运算

```huanlang
函数 主() -> 整数32 {
    令 a 为 10;
    令 b 为 5;
    打印("a + b = ");
    返回 a + b;
}
```

### 2. 函数调用

```huanlang
函数 计算(a: 整数32, b: 整数32) -> 整数32 {
    返回 a * b;
}

函数 主() -> 整数32 {
    返回 计算(10, 20);
}
```

## 测试

运行项目的完整测试套件:

```bash
# 运行所有测试
cargo test

# 运行 MLIR 相关测试
cargo test --lib mlir
```

## 项目结构

```
huanlang/
├── src/
│   ├── core/
│   │   ├── lexer/          # 词法分析器
│   │   ├── parser/         # 语法分析器
│   │   ├── mlir/           # MLIR 后端
│   │   └── backend/llvm/   # 直接 LLVM 后端
│   └── tools/cli/          # 命令行工具
├── dialects/               # C++ MLIR 方言定义
├── tests/                  # 测试
├── examples/               # 示例程序
└── docs/                   # 文档
```

## 后续步骤

- 查看完整的开发规范文档: `docs/开发规范文档/`
- 尝试更多的示例程序: `examples/`
- 查看更多的测试用例: `tests/`

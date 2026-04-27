# 幻语 MLIR 方言降级流程

## 概述

本文档描述了幻语（HuanLang）MLIR方言的完整降级流程，从高级的`huan`方言降级到标准的`scf`、`arith`、`func`方言，最终到LLVM方言和LLVM IR。

## 方言定义

### Huan 方言 (huan)

`huan`方言定义了幻语语言的高级操作：

- **变量操作**: `ling` (变量声明), `ding` (常量声明), `shewei` (赋值)
- **控制流**: `ruo` (if条件), `chongfu` (循环), `dang` (while), `duiyu` (for)
- **函数操作**: `hanshu` (函数定义), `fanhui` (返回), `diaoyong` (调用)
- **算术操作**: `jia` (add), `jian` (sub), `cheng` (mul), `chu` (div)
- **比较操作**: `dengyu` (eq), `buyudengyu` (ne), `xiaoyu` (lt), `dayu` (gt)
- **逻辑操作**: `qie` (and), `huo` (or), `fei` (not)
- **数据结构**: `liebiao` (列表), `zidian` (字典)

## 降级管线

### 降级流程

```
  Huan 方言
      ↓
  LowerHuanToSCF (控制流降级)
      ↓
  LowerHuanToArith (算术操作降级)
      ↓
  LowerHuanToFunc (函数操作降级)
      ↓
  标准方言 (scf + arith + func)
      ↓
  LowerHuanToLLVM (最终降级到LLVM方言)
      ↓
  LLVM 方言
      ↓
  LLVM IR
```

## 降级Pass详解

### 1. LowerHuanToSCF

把Huan方言的控制流操作转换为SCF方言：

| Huan 操作 | SCF 操作 |
|---------|---------|
| `ruo` (if) | `scf.if` |
| `dang` (while) | `scf.while` |
| `chongfu` (repeat) | `scf.for` |

### 2. LowerHuanToArith

把Huan方言的算术操作转换为Arith方言：

| Huan 操作 | Arith 操作 |
|---------|-----------|
| `jia` | `arith.addi` / `arith.addf` |
| `jian` | `arith.subi` / `arith.subf` |
| `cheng` | `arith.muli` / `arith.mulf` |
| `chu` | `arith.divi` / `arith.divf` |
| `dayu` | `arith.cmpgt` |
| `xiaoyu` | `arith.cmplt` |
| `dengyu` | `arith.cmpeq` |
| `qie` | `arith.andi` |
| `huo` | `arith.ori` |
| `fei` | `arith.xori` |

### 3. LowerHuanToFunc

把Huan方言的函数操作转换为Func方言：

| Huan 操作 | Func 操作 |
|---------|---------|
| `hanshu` (函数定义) | `func.func` |
| `diaoyong` (函数调用) | `func.call` |
| `fanhui` (返回语句) | `func.return` |

### 4. LowerHuanToLLVM

最终转换到LLVM方言，使用标准的MLIR到LLVM的转换Pass。

## 文件结构

```
dialects/
├── CMakeLists.txt
├── include/huan/
│   ├── HuanDialect.h          # 方言定义头文件
│   ├── HuanOps.td             # 操作定义 (TableGen)
│   └── HuanTypes.td           # 类型定义 (TableGen)
└── lib/
    ├── HuanDialect.cpp        # 方言实现
    ├── HuanOps.cpp            # 操作实现
    ├── HuanTypes.cpp          # 类型实现
    ├── LowerHuanToSCF.cpp     # 降级到SCF
    ├── LowerHuanToArith.cpp   # 降级到Arith
    ├── LowerHuanToFunc.cpp    # 降级到Func
    └── LowerHuanToLLVM.cpp    # 降级到LLVM
```

## Rust 端实现

在`src/core/mlir/`目录下还有完整的Rust端实现：

- `dialect.rs` - Huan方言定义
- `types.rs` - 类型系统
- `ops.rs` - 操作定义
- `passes.rs` - 降级Pass实现
- `conversion.rs` - AST到MLIR转换
- `lowering.rs` - LLVM类型转换

## 测试

### 运行测试

```bash
# 运行所有MLIR相关的测试
cargo test --lib mlir
```

### 测试覆盖

- ✅ 方言定义和名称检查
- ✅ 类型系统测试
- ✅ 基本操作测试
- ✅ 算术操作测试
- ✅ Pass管线测试
- ✅ 类型转换测试
- ✅ 完整流程测试
- ✅ 变量和标识符定义测试
- ✅ 列表和字符串类型测试

### 测试结果

```
test result: ok. 18 passed; 0 failed; 0 ignored
```

## 编译MLIR方言

### 前置要求

- CMake 3.20+
- C++17 编译器
- MLIR 15.0+ (包含LLVM)

### 编译步骤

```bash
cd dialects
mkdir build && cd build
cmake ..
make
```

## 使用mlir-opt（可选）

如果你已经编译了Huan方言库，可以使用`mlir-opt`来测试降级管线：

```bash
# 运行完整的降级管线
mlir-opt test.mlir \
    --lower-huan-to-scf \
    --lower-huan-to-arith \
    --lower-huan-to-func \
    --lower-huan-to-llvm \
    -o test.llvm.mlir
```

## 与编译器集成

幻语编译器有两个并行的代码生成路径：

1. **直接AST→LLVM IR**（推荐，已完整实现，支持中文函数名）
2. **AST→MLIR→LLVM IR**（本文档描述的路径）

两个路径最终都会生成LLVM IR，然后使用clang编译成可执行文件。

## 参考资料

- [MLIR 文档](https://mlir.llvm.org/)
- [TableGen 语言参考](https://llvm.org/docs/TableGen/)
- [MLIR 方言开发指南](https://mlir.llvm.org/docs/Tutorials/)

## 贡献

欢迎提交PR来完善和优化MLIR方言降级流程！

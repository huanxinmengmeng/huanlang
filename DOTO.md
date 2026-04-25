# 幻语从源码构建、编译到运行的完整方案

## 一、概览

幻语编译工具链本身是用 Rust 编写的，最终用户的 `.hl` 源码经由工具链编译为原生可执行文件。整个流程分为两大步骤：

1. **构建幻语编译器本身**：从 Gitee 仓库拉取源码，编译生成 `huan` 命令行工具。
2. **使用幻语编译器编译运行用户程序**：用 `huan` 命令将 `.hl` 源码编译为目标平台的可执行文件或固件。


## 二、环境准备

### 2.1 基础依赖

| 依赖 | 最低版本 | 说明 |
|------|---------|------|
| Rust 工具链 | 1.75+ | 编译器本体及 Cargo 包管理器 |
| LLVM | 18.0+ | 后端代码生成（含 `llvm-config`） |
| MLIR | 18.0+ | 中间表示优化框架（通常随 LLVM 发行） |
| CMake | 3.20+ | MLIR C++ 组件的构建系统 |
| Ninja | 1.10+ | 构建加速（可选，推荐） |
| Git | 2.0+ | 版本控制 |

**安装示例（Ubuntu/Debian）**：

```bash
# Rust 工具链
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# LLVM 18（含 MLIR）
wget https://apt.llvm.org/llvm.sh
chmod +x llvm.sh
sudo ./llvm.sh 18
sudo apt install libllvm18 llvm-18 llvm-18-dev llvm-18-runtime mlir-18-tools

# CMake 和 Ninja
sudo apt install cmake ninja-build
```

### 2.2 交叉编译附加依赖

针对嵌入式目标（ARM Cortex-M、RISC-V 等），需额外安装对应的 GCC 工具链和调试器：

```bash
# ARM Cortex-M
sudo apt install gcc-arm-none-eabi binutils-arm-none-eabi gdb-multiarch

# RISC-V
sudo apt install gcc-riscv64-unknown-elf

# AVR
sudo apt install gcc-avr avr-libc
```

### 2.3 环境变量

```bash
export LLVM_SYS_180_PREFIX=/usr/lib/llvm-18
export HUAN_PATH=$HOME/.huan
export PATH=$HUAN_PATH/bin:$PATH
```


## 三、获取并构建幻语编译器

### 3.1 克隆仓库

```bash
git clone https://gitee.com/huanxinmengmeng/huanlang.git
cd huanlang
```

### 3.2 构建 MLIR 方言组件

幻语的 MLIR 方言（`huan` Dialect）用 C++ 实现，需要先用 CMake 构建：

```bash
cd dialects
mkdir build && cd build

cmake .. \
  -G Ninja \
  -DCMAKE_BUILD_TYPE=Release \
  -DMLIR_DIR=/usr/lib/llvm-18/lib/cmake/mlir \
  -DLLVM_EXTERNAL_LIT=/usr/lib/llvm-18/build/utils/lit/lit.py

ninja
cd ../..
```

构建产物包括：
- `dialects/build/lib/libHuanDialect.a`：幻语方言静态库
- `dialects/build/lib/libLowerHuanToLLVM.a`：降级 Pass 静态库

### 3.3 构建 Rust 编译器

```bash
# 开发构建（快速迭代）
cargo build

# 发布构建（优化性能）
cargo build --release
```

编译完成后，可执行文件位于 `target/debug/huan`（或 `target/release/huan`）。

### 3.4 安装到系统

```bash
cargo install --path .
```

或者手动复制：

```bash
cp target/release/huan $HOME/.huan/bin/
```


## 四、构建运行时库

幻语运行时库（`libhuanrt`）提供列表、字符串、字典、输出函数和分配器等基础能力的 C 语言实现。

### 4.1 编译运行时库

```bash
cd runtime
mkdir build && cd build

cmake .. -DCMAKE_BUILD_TYPE=Release
make -j$(nproc)

cd ../..
```

构建产物：
- `runtime/build/libhuanrt.a`（静态库）
- `runtime/build/libhuanrt.so`（动态库，Linux）
- `runtime/build/huanrt.lib`（Windows）

### 4.2 安装运行时库

```bash
cp runtime/build/libhuanrt.a $HOME/.huan/lib/
cp runtime/ming_runtime.h $HOME/.huan/include/
```


## 五、构建标准库

幻语标准库本身用幻语编写，需要用已构建好的编译器来编译。在 CI 或正式构建流程中，这一步在编译器构建完成后自动执行。

### 5.1 标准库源码结构

标准库位于 `stdlib/` 目录，按模块组织：

```
stdlib/
├── 核心.hl
├── 数学.hl
├── 集合.hl
├── 字符串.hl
├── 文件系统.hl
├── 网络.hl
├── 并发.hl
├── 序列化.hl
├── 加密.hl
├── 系统.hl
└── 随机.hl
```

### 5.2 编译标准库

```bash
# 假设 huan 已在 PATH 中
cd stdlib

# 编译所有标准库模块为静态库
huan build 核心.hl --emit lib --output ../target/huanrt/libcore.a
huan build 数学.hl --emit lib --output ../target/huanrt/libmath.a
huan build 集合.hl --emit lib --output ../target/huanrt/libcollections.a
# ... 其余模块同理

# 或者使用批量构建命令（如已实现）
huan build --workspace --emit lib
```

标准库编译产物默认输出到 `$HUAN_PATH/lib/`，供后续用户程序链接时自动查找。


## 六、编写并编译用户程序

### 6.1 最简单的程序

创建文件 `hello.hl`：

```hl
函数 主() 返回 整数
开始
    显示("你好，世界！")
    返回 0
结束
```

### 6.2 编译并运行

```bash
# 编译并直接运行
huan run hello.hl

# 或者分步操作
huan build hello.hl -o hello       # 编译为可执行文件
./hello                             # 运行
```

**内部流程**（编译器自动完成）：

```
hello.hl
  → [词法分析器] → Token 流
  → [语法解析器] → AST
  → [语义分析器] → 类型标注的 AST
  → [MLIR 生成器] → huan 方言 MLIR
  → [降级 Pass] → scf/arith/func/llvm 方言 MLIR
  → [LLVM 后端] → 目标文件 (.o)
  → [链接器] → 可执行文件 (hello)
```

### 6.3 编译选项

```bash
# 优化级别
huan build hello.hl -O 0          # 无优化，调试用
huan build hello.hl -O 2          # 标准优化（默认）
huan build hello.hl -O 3          # 激进优化
huan build hello.hl -O s          # 尺寸优化

# 查看中间产物
huan build hello.hl --emit llvm-ir -o hello.ll   # 输出 LLVM IR
huan build hello.hl --emit asm -o hello.s         # 输出汇编代码

# 生成调试信息
huan build hello.hl --debug

# 启用所有权检查
huan build hello.hl --ownership

# 链接时优化
huan build hello.hl --release --lto
```

### 6.4 多文件项目

项目结构：

```
myproject/
├── 幻语包.toml
├── 源码/
│   ├── 主程序.hl
│   ├── 模型.hl
│   └── 工具.hl
└── 测试/
    └── 全部测试.hl
```

`幻语包.toml`：

```toml
[package]
name = "myproject"
version = "0.1.0"

[[bin]]
name = "myproject"
path = "源码/主程序.hl"

[dependencies]
网络 = "0.3"
```

构建命令：

```bash
cd myproject

# 构建
huan package build

# 运行
huan package run

# 测试
huan package test
```


## 七、裸机/嵌入式构建方案

### 7.1 目标三元组

| 平台 | 三元组 | 说明 |
|------|--------|------|
| Cortex-M3 | `thumbv7m-none-eabi` | STM32F1 系列 |
| Cortex-M4（硬浮点） | `thumbv7em-none-eabihf` | STM32F4 系列 |
| Cortex-M0 | `thumbv6m-none-eabi` | STM32F0 系列 |
| RISC-V 32 | `riscv32imac-unknown-none-elf` | GD32V 系列 |
| RISC-V 64 | `riscv64gc-unknown-none-elf` | — |
| AVR | `avr-unknown-none` | ATmega 系列 |
| ESP32 | `xtensa-esp32-none-elf` | ESP32 系列 |

### 7.2 添加目标工具链

```bash
# 安装 Rust 目标
rustup target add thumbv7em-none-eabihf
rustup target add riscv32imac-unknown-none-elf
```

### 7.3 裸机项目结构

```
blinky/
├── 幻语包.toml
├── 链接脚本.hld             # 链接器脚本
├── 启动代码.has m           # 汇编启动代码
├── 主程序.hl                # 主程序
└── 目标配置.toml            # 芯片配置
```

`目标配置.toml`：

```toml
[目标]
架构 = "cortex-m4"
芯片 = "STM32F407VG"
频率 = 168_000_000

[内存]
闪存起始 = "0x08000000"
闪存大小 = "1M"
内存起始 = "0x20000000"
内存大小 = "128K"
```

`链接脚本.hld`：

```hl
内存布局:
    闪存: 起始 0x08000000, 长度 64K, 属性(可读, 可执行)
    内存: 起始 0x20000000, 长度 20K, 属性(可读, 可写, 可执行)

段定义:
    .向量表: 放入 闪存, 对齐 256
    .文本: 放入 闪存
    .数据: 放入 内存, 加载地址在 闪存
    .bss: 放入 内存, 清零
结束
```

### 7.4 编译裸机程序

```bash
# 汇编启动代码
huan asm 启动代码.has m --target thumbv7em-none-eabihf -o 启动代码.o

# 编译主程序
huan build 主程序.hl \
  --target thumbv7em-none-eabihf \
  --link-script 链接脚本.hld \
  --optimize z \
  --ownership \
  --output 固件.elf

# 生成烧录文件
huan convert 固件.elf --format bin --output 固件.bin
huan convert 固件.elf --format hex --output 固件.hex
```

### 7.5 烧录与调试

```bash
# 烧录
huan flash 固件.bin --chip STM32F407VG --interface swd

# 调试
huan debug 固件.elf --interface swd --remote :3333
```

### 7.6 完整裸机示例（STM32F103 LED 闪烁）

```hl
@不标准
@目标("thumbv7m-none-eabi")

外设 GPIOA 基址 0x40010800:
    寄存器 CRL  偏移 0x00, 类型 无符号32
    寄存器 BSRR 偏移 0x10, 类型 无符号32
结束

外设 RCC 基址 0x40021000:
    寄存器 APB2ENR 偏移 0x18, 类型 无符号32
结束

@导出 "C"
函数 主()
开始
    令 RCC.APB2ENR.位(2) 设为 真
    令 GPIOA.CRL.位域(0..4) 设为 0b0011

    当 真 循环
        GPIOA.BSRR 设为 0x00000001
        重复 500000 次
            汇编!("nop")
        结束
        GPIOA.BSRR 设为 0x00010000
        重复 500000 次
            汇编!("nop")
        结束
    结束
结束
```


## 八、JIT 执行引擎

幻语支持通过 LLVM JIT 直接执行代码，无需生成文件。

### 8.1 REPL 交互执行

```bash
huan repl
>>> 令 甲 = 10
>>> 令 乙 = 20
>>> 甲 + 乙
30
>>> 退出
```

### 8.2 JIT 执行单文件

```bash
huan run 脚本.hl
```

`huan run` 的内部流程：
1. 解析源文件生成 AST。
2. 语义检查通过后，生成 LLVM IR。
3. 通过 LLVM JIT 引擎立即执行。
4. 执行完毕后释放所有 JIT 资源。

这使得幻语适用于脚本式开发和快速原型验证。


## 九、快速参考卡片

### 9.1 日常开发命令

```bash
huan run 程序.hl                  # 一键运行
huan build 程序.hl                # 编译
huan check 程序.hl                # 仅检查（不生成代码）
huan fmt 程序.hl                  # 格式化
huan test                         # 运行测试
huan repl                         # 启动 REPL
huan edit 程序.hl                 # 内置编辑器
```

### 9.2 发布构建命令

```bash
huan build 程序.hl --release --lto --strip
huan build 程序.hl --target wasm32-unknown-unknown --release
```

### 9.3 跨语言转换命令

```bash
huan transpile 程序.hl --to rust -o 程序.rs
huan transpile 程序.hl --to c -o 程序.c
huan transpile 程序.hl --to python -o 程序.py
```


## 十、常见问题排查

| 问题 | 可能原因 | 解决方式 |
|------|---------|---------|
| `llvm-config not found` | LLVM 未安装或版本不对 | 检查 `llvm-config-18 --version` |
| `mlir/IR/BuiltinTypes.h not found` | MLIR 开发包缺失 | 安装 `mlir-18-dev` 或 `mlir-18-tools` |
| 链接时找不到 `huanrt` | 运行时库未构建 | 进入 `runtime/` 目录执行 `cmake .. && make` |
| `error: could not compile` | Rust 版本过低 | `rustup update` |
| 交叉编译找不到 GCC | ARM/RISC-V 工具链未安装 | 安装对应 `gcc-arm-none-eabi` 等 |
| `todo!()` panic | 对应编译器功能尚未实现 | 当前版本部分功能仍在开发中 |

## 十一、开发进度与计划

### 已完成的任务

1. **构建 MLIR 方言组件（C++实现）**
   - 实现了 `huan` 方言的完整定义
   - 实现了到 SCF、Arith、Func 和 LLVM 方言的降级 Pass
   - 提供了完整的 MLIR 操作和类型支持

2. **构建运行时库 libhuanrt**
   - 实现了内存分配器、字符串、列表、映射等核心模块
   - 提供了 IO 和控制台功能
   - 支持跨平台编译

3. **编译标准库（用已构建的编译器）**
   - 验证了编译器能够成功编译标准库模块
   - 确保标准库的完整性和可用性

4. **编写完整的测试套件**
   - 编写了 MLIR 方言测试
   - 编写了运行时库测试
   - 提供了测试运行脚本

5. **完善 JIT 执行引擎和 REPL 功能**
   - 实现了基于解释器的 JIT 执行
   - 完善了 REPL 交互式环境
   - 支持多行输入和错误处理

6. **实现跨语言转换功能**
   - 支持从幻语转译到 Rust、Python、C 等语言
   - 支持从其他语言导入到幻语
   - 提供了文件转译功能

7. **完善命令行工具的编译选项支持**
   - 添加了 debug、ownership、lto、release、strip 等选项
   - 支持 target 和 emit 选项
   - 提供了更灵活的编译配置

8. **完善裸机/嵌入式构建支持**
   - 添加了 Cortex-M、RISC-V、AVR 和 ESP32 等嵌入式目标
   - 支持交叉编译和固件生成

### 后续计划

1. **完善 MLIR 方言的实现**
   - 增强对复杂语法结构的支持
   - 优化降级 Pass 的性能

2. **改进运行时库**
   - 添加更多标准库功能
   - 优化内存管理和性能

3. **增强跨语言转换能力**
   - 支持更多目标语言
   - 提高转换的准确性和效率

4. **完善嵌入式支持**
   - 添加更多嵌入式目标平台
   - 提供完整的嵌入式开发工具链

5. **编写更完整的文档**
   - 完善用户开发文档
   - 添加 API 参考文档

6. **性能优化**
   - 优化编译器性能
   - 提高代码生成质量

7. **测试与验证**
   - 增加更多测试用例
   - 确保系统的稳定性和可靠性

## 十二、总结

整个幻语的构建和运行流程可以概括为以下路线：

1. **环境**：Rust + LLVM/MLIR + CMake + (交叉编译工具链)
2. **构建编译器**：CMake 构建方言 → Cargo 构建 Rust 编译器
3. **构建运行时**：CMake 构建 `libhuanrt`
4. **编译标准库**：用 `huan` 编译 `stdlib/*.hl`
5. **用户开发**：`huan run`（JIT）或 `huan build`（静态编译）
6. **嵌入式**：加 `--target` 参数，汇编启动代码，生成固件并烧录
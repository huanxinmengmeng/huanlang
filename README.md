# 幻语编程语言 (HuanLang)

[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-v0.3.0-green.svg)](Cargo.toml)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/huanxinmengmeng/huanlang)

幻语是一种支持中文、拼音、英文三种语言关键词的现代化编程语言，专为中文母语者设计，让编程变得更加自然和亲切。

## 特性

### 🌍 多语言支持
- **中文关键词**：函数、如果、当、重复、结束等
- **英文关键词**：function, if, while, repeat, end等
- **拼音关键词**：han_shu, ru_guo, dang, chong_fu, jie_shu等
- **自由切换**：可以在同一文件中混合使用任意语言的关键词

### 🚀 现代编译器
- 完整的词法分析器
- 强大的语法解析器
- 语义分析和类型检查
- MLIR中间表示
- LLVM代码生成器
- WASM后端支持

### 📦 包管理系统
- 简洁的依赖管理
- 版本控制和解析
- 本地缓存
- 注册表集成
- 工作区支持

### 🔧 完整工具链
- 编译器（build）
- 解释器（run）
- 类型检查器（check）
- 交互式REPL
- 代码格式化器
- LSP服务器
- 包管理器

### 💡 语言特性
- 强类型系统
- 函数式编程
- 面向对象编程（支持）
- 并发编程（支持）
- 内存安全（支持）
- 跨语言互操作

## 快速开始

### 安装

从源代码构建：
```bash
git clone https://github.com/huanxinmengmeng/huanlang.git
cd huanlang
cargo build --release
```

### 你的第一个程序

创建`hello.hl`：
```
函数 主() -> 整数 {
    打印("你好，幻语世界！");
    返回 0;
}
```

运行程序：
```bash
huan run hello.hl
```

## 语法示例

### 三种语言的关键词

**中文版本：**
```
函数 加法(a: 整数, b: 整数) -> 整数 {
    返回 a + b;
}
```

**英文版本：**
```
function add(a: int, b: int) -> int {
    return a + b;
}
```

**拼音版本：**
```
han_shu jia_fa(a: zheng_shu, b: zheng_shu) -> zheng_shu {
    fan_hui a + b;
}
```

### 完整示例

```
函数 阶乘(n: 整数) -> 整数 {
    如果 n <= 1 那么
        返回 1;
    否则
        返回 n * 阶乘(n - 1);
    结束
}

函数 主() -> 整数 {
    打印("斐波那契数列演示！");
    
    变量 i = 0;
    当 i < 10 时 {
        打印("阶乘(" + i + ") = " + 阶乘(i));
        i = i + 1;
    }
    
    返回 0;
}
```

## 命令行使用

```bash
# 编译程序
huan build my_program.hl

# 运行程序
huan run my_program.hl

# 类型检查
huan check my_program.hl

# 启动REPL
huan repl

# 格式化代码
huan fmt my_program.hl

# 包管理
huan package init my-project
huan package add 网络@0.3
huan package install

# 查看帮助
huan help
huan help build
```

## 项目结构

```
huanlang/
├── src/
│   ├── core/              # 核心编译系统
│   │   ├── lexer/        # 词法分析
│   │   ├── parser/       # 语法解析
│   │   ├── sema/         # 语义分析
│   │   ├── typeck/       # 类型检查
│   │   ├── mlir/         # MLIR中间表示
│   │   ├── backend/      # 代码生成后端
│   │   ├── interop/      # 跨语言互操作
│   │   ├── memory/       # 内存管理
│   │   ├── concurrent/   # 并发支持
│   │   └── performance/  # 性能子系统
│   ├── stdlib/           # 标准库
│   ├── package/          # 包管理器
│   ├── tools/            # 工具链
│   │   ├── cli/          # 命令行界面
│   │   ├── editor/       # 编辑器
│   │   └── hla/          # 高级语言
│   └── lsp/              # LSP服务器
├── examples/             # 示例程序
├── docs/                 # 文档
└── dialects/            # MLIR方言
```

## 示例程序

查看 [examples/](examples/) 目录获取完整的示例程序：

- [hello_world.hl](examples/hello_world.hl) - 简单的Hello World
- [variables.hl](examples/variables.hl) - 变量和类型
- [functions.hl](examples/functions.hl) - 函数使用
- [control_flow.hl](examples/control_flow.hl) - 控制流
- [collections.hl](examples/collections.hl) - 集合操作
- [fibonacci.hl](examples/fibonacci.hl) - 斐波那契数列

## 文档

- [开发规范文档](docs/开发规范文档/) - 编译器实现细节
- [用户开发使用文档](docs/用户开发使用文档/) - 用户指南
- [包管理器使用指南](docs/幻语包管理器使用指南.md) - 包管理

## 开发

### 构建项目

```bash
# 开发构建
cargo build

# 运行测试
cargo test

# 代码检查
cargo clippy

# 格式化
cargo fmt

# 发布构建
cargo build --release
```

### 贡献指南

1. Fork本项目
2. 创建你的特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交你的更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启Pull Request

## 路线图

### v0.1.0 - 核心链路稳定 ✅
- ✅ 基础词法分析器
- ✅ 基础语法解析器
- ✅ 基础语义分析
- ✅ LLVM代码生成基础

### v0.2.0 - 工具链建设 ✅
- ✅ 包管理器核心功能
- ✅ 测试框架
- ✅ 项目模板
- ✅ 基础标准库

### v0.3.0 - 生态扩展 ✅
- ✅ LSP服务器
- ✅ 跨语言互操作
- ✅ 性能子系统
- ✅ 完整的包管理功能

### v0.4.0 - 语言完善
- [ ] 完整的类型系统
- [ ] 所有权系统
- [ ] 错误处理
- [ ] 模式匹配

### v1.0.0 - 生产就绪
- [ ] 完整的标准库
- [ ] 性能优化
- [ ] 稳定的API
- [ ] 生产级工具链

## 社区

- [Gitee仓库](https://gitee.com/huanxinmengmeng/huanlang)
- [Issues](https://gitee.com/huanxinmengmeng/huanlang/issues)
- [Pull Requests](https://gitee.com/huanxinmengmeng/huanlang/pulls)

## 许可证

本项目采用 Apache License 2.0 许可证 - 详见 [LICENSE](LICENSE) 文件。

## 致谢

- 感谢所有为幻语项目贡献代码的开发者
- 感谢所有使用和支持幻语的用户
- 感谢开源社区提供的优秀工具和库

---

**幻语 - 让编程更自然！** 🚀

**HuanLang - Making programming more natural!** 🌟

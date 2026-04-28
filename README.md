# 幻语编程语言 (HuanLang)

[![Version](https://img.shields.io/badge/version-v0.3.0-green.svg)](Cargo.toml)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![Build Status](https://img.shields.io/badge/build-passing-brightgreen.svg)]()

**幻语**是一种支持**中文、拼音、英文**三种语言关键词的现代化编程语言，专为中文母语者设计，让编程变得更加自然和亲切。

## ✨ 核心特性

### 🌍 多语言关键词系统
幻语最大的特色是支持三种语言的关键词，可以在同一文件中自由混合使用：

```
// 中文版本
函数 主() -> 整数 {
    如果 x > 0 那么
        打印("正数")
    结束
}

// 英文版本
function main() -> int {
    if x > 0 {
        print("positive")
    }
}

// 拼音版本
han_shu zhu() -> zheng_shu {
    ru_guo x > 0 na_me
        da_yin("zheng shu")
    jie_shu
}
```

### 🚀 现代编译器架构
- **词法分析器**：支持Unicode、CJK字符、拼音标识符
- **语法解析器**：强大的递归下降解析器
- **语义分析**：完整的类型推导和检查
- **MLIR中间表示**：现代化的编译器基础设施
- **LLVM后端**：高性能代码生成
- **WASM支持**：WebAssembly编译目标

### 📦 完整生态系统
- **包管理器**：简洁的依赖管理、工作区支持
- **标准库**：IO、网络、并发、数学、数据处理
- **工具链**：编译器、解释器、REPL、格式化器、LSP
- **文档完善**：详尽的文档和丰富的示例

## 📚 学习资源

### 🏃 快速开始

**安装：**
```bash
git clone https://gitee.com/huanxinmengmeng/huanlang.git
cd huanlang
cargo build --release
```

**编写第一个程序：**
```huan
// hello.hl
函数 主() -> 整数 {
    打印("你好，幻语世界！");
    返回 0;
}
```

**运行：**
```bash
huan run hello.hl
```

### 📖 示例程序（13个）

**基础示例：**
- [hello_world.hl](examples/hello_world.hl) - 入门第一个程序
- [variables.hl](examples/variables.hl) - 变量和数据类型
- [functions.hl](examples/functions.hl) - 函数定义和使用
- [control_flow.hl](examples/control_flow.hl) - 循环和条件判断
- [collections.hl](examples/collections.hl) - 列表和映射
- [fibonacci.hl](examples/fibonacci.hl) - 递归函数

**进阶示例：**
- [oop.hl](examples/oop.hl) - 面向对象编程
- [algorithms.hl](examples/algorithms.hl) - 排序算法
- [error_handling.hl](examples/error_handling.hl) - 错误处理

**实用示例：**
- [file_processing.hl](examples/file_processing.hl) - 文件处理
- [network_example.hl](examples/network_example.hl) - 网络请求
- [concurrency_example.hl](examples/concurrency_example.hl) - 并发编程
- [data_processing.hl](examples/data_processing.hl) - 数据处理

### 📖 完整文档

- [用户指南](docs/用户开发使用文档/) - 语言基础教程
- [标准库文档](docs/标准库文档.md) - 标准库参考
- [包管理器指南](docs/幻语包管理器使用指南.md) - 包管理
- [开发规范](docs/开发规范文档/) - 开发者文档

## 🛠️ 命令行工具

```bash
# 编译和运行
huan build my_program.hl       # 编译
huan run my_program.hl         # 运行
huan check my_program.hl      # 类型检查

# 开发工具
huan repl                      # 交互式REPL
huan fmt my_program.hl        # 代码格式化
huan serve                    # LSP服务器

# 包管理
huan package init my-project   # 初始化项目
huan package add 网络@0.3      # 添加依赖
huan package install           # 安装依赖
```

## 🏗️ 项目结构

```
huanlang/
├── src/
│   ├── core/              # 核心编译系统
│   │   ├── lexer/        # 词法分析器
│   │   ├── parser/       # 语法解析器
│   │   ├── sema/         # 语义分析
│   │   ├── typeck/       # 类型检查
│   │   ├── mlir/         # MLIR中间表示
│   │   └── backend/      # 代码生成后端
│   ├── stdlib/           # 标准库
│   ├── package/          # 包管理器
│   ├── tools/           # 工具链
│   └── lsp/              # LSP服务器
├── examples/              # 示例程序（13个）
├── docs/                 # 文档
└── dialects/             # MLIR方言
```

## 📊 技术指标

| 指标 | 数值 |
|------|------|
| 代码行数 | 50,000+ |
| 测试用例 | 375 |
| 示例程序 | 13 |
| 核心模块 | 20+ |
| 标准库模块 | 8 |
| 文档页数 | 50+ |

## 🎯 发展路线图

### ✅ 已完成
- **v0.1.0** - 核心链路稳定
- **v0.2.0** - 工具链建设
- **v0.3.0** - 生态扩展

### ⏳ 进行中
- **v0.4.0** - 语言完善
  - 完整的类型系统
  - 所有权系统
  - 模式匹配

### 📅 计划中
- **v1.0.0** - 生产就绪
  - 完整的标准库
  - 性能优化
  - 稳定的API

## 🤝 贡献指南

欢迎贡献代码！请遵循以下步骤：

1. Fork本项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add AmazingFeature'`)
4. 推送分支 (`git push origin feature/AmazingFeature`)
5. 创建Pull Request

## 📄 许可证

本项目采用 Apache License 2.0 许可证。

## 🙏 致谢

感谢所有为幻语项目贡献代码和提出建议的开发者！

---

## 🚀 立即开始

```bash
# 克隆项目
git clone https://gitee.com/huanxinmengmeng/huanlang.git

# 构建
cd huanlang
cargo build --release

# 运行示例
./target/release/huan run examples/hello_world.hl
```

**幻语 - 让编程更自然！** 🌟

**HuanLang - Making programming more natural!** 🚀

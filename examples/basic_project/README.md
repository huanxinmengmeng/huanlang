# 基础项目示例

这是一个标准的幻语项目模板，展示了如何创建和组织幻语项目。

## 项目结构

```
basic-project/
├── 幻语包.toml        # 项目配置文件
├── README.md          # 项目说明
├── .gitignore         # Git 忽略文件
├── src/
│   ├── main.hl       # 主程序入口
│   └── lib.hl        # 库入口
├── examples/          # 示例程序
│   └── hello.hl
├── tests/            # 测试文件
│   └── test_basic.hl
└── benches/          # 基准测试
    └── benchmark.hl
```

## 功能

- **库功能**：提供 `问候()` 函数和 `人员` 结构体
- **主程序**：演示如何使用库功能
- **示例**：展示库的基本用法
- **测试**：验证库功能的正确性
- **基准测试**：测试性能

## 依赖

- `网络` - 版本 0.3
- `序列化` - 版本 1.0 (可选)

## 构建和运行

```bash
# 构建项目
huan build

# 运行主程序
huan run

# 运行示例
huan run --example hello

# 运行测试
huan test

# 运行基准测试
huan bench
```

## 特性

- `default` - 默认特性，包含 std
- `std` - 标准库支持
- `json` - JSON 序列化支持

## 许可证

MIT 许可证

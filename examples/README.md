<div align="center">
  <img src="../images/logo.png" alt="幻语编程语言" width="150" height="150">

  # 幻语编程语言示例
</div>

本目录包含了使用幻语编程语言的示例程序。所有示例都可以通过 `huan run` 命令直接运行。

## 示例列表（27个）

### 基础示例 (basic/) - 5个

| 文件 | 说明 |
|:-----|:-----|
| [你好世界.hl](basic/你好世界.hl) | 最简单的Hello World程序 |
| [变量示例.hl](basic/变量示例.hl) | 变量和类型示例 |
| [控制流示例.hl](basic/控制流示例.hl) | 循环和条件判断 |
| [基础语法全览.hl](basic/基础语法全览.hl) | 完整基础语法 |
| [控制流全览.hl](basic/控制流全览.hl) | 循环结构全览 |

### 函数示例 (functions/) - 2个

| 文件 | 说明 |
|:-----|:-----|
| [函数示例.hl](functions/函数示例.hl) | 函数定义、调用和递归 |
| [函数与递归全览.hl](functions/函数与递归全览.hl) | 函数和递归的使用 |

### 数据结构示例 (data_structures/) - 4个

| 文件 | 说明 |
|:-----|:-----|
| [集合示例.hl](data_structures/集合示例.hl) | 列表和映射的使用 |
| [结构体示例.hl](data_structures/结构体示例.hl) | 结构体和方法 |
| [算法示例.hl](data_structures/算法示例.hl) | 排序算法 |
| [数据结构全览.hl](data_structures/数据结构全览.hl) | 完整数据结构 |

### 错误处理示例 (error_handling/) - 1个

| 文件 | 说明 |
|:-----|:-----|
| [错误处理示例.hl](error_handling/错误处理示例.hl) | 错误类型和结果类型 |

### 并发示例 (concurrency/) - 1个

| 文件 | 说明 |
|:-----|:-----|
| [并发示例.hl](concurrency/并发示例.hl) | 多线程操作、任务组、通道 |

### 汇编与裸机编程示例 (assembly/) - 2个

| 文件 | 说明 |
|:-----|:-----|
| [汇编演示.hl](assembly/汇编演示.hl) | 内联汇编基础 |
| [裸机编程演示.hl](assembly/裸机编程演示.hl) | 外设定义、内存布局 |

### 文件IO示例 (file_io/) - 1个

| 文件 | 说明 |
|:-----|:-----|
| [文件处理示例.hl](file_io/文件处理示例.hl) | 文件读写操作 |

### 网络示例 (network/) - 1个

| 文件 | 说明 |
|:-----|:-----|
| [网络请求示例.hl](network/网络请求示例.hl) | HTTP请求 |

### 模块导入示例 (modules/) - 2个

| 文件 | 说明 |
|:-----|:-----|
| [模块导入示例.hl](modules/模块导入示例.hl) | 模块导入基础 |
| [选择性导入示例.hl](modules/选择性导入示例.hl) | 选择性导入 |

### 跨语言互操作示例 (interop/) - 1个

| 文件 | 说明 |
|:-----|:-----|
| [跨语言互操作示例.hl](interop/跨语言互操作示例.hl) | FFI、外部函数 |

### 面向对象示例 (oop/) - 3个

| 文件 | 说明 |
|:-----|:-----|
| [面向对象示例.hl](oop/面向对象示例.hl) | 结构体继承、接口实现 |
| [方法调用示例.hl](oop/方法调用示例.hl) | 方法定义和调用 |
| [排序算法示例.hl](oop/排序算法示例.hl) | 冒泡排序、选择排序 |

### 高级特性示例 (advanced/) - 1个

| 文件 | 说明 |
|:-----|:-----|
| [高级特性全览.hl](advanced/高级特性全览.hl) | 高级特性 |

### 性能优化示例 (performance/) - 1个

| 文件 | 说明 |
|:-----|:-----|
| [性能优化示例.hl](performance/性能优化示例.hl) | 性能优化 |

### 综合示例 (comprehensive/) - 2个

| 文件 | 说明 |
|:-----|:-----|
| [数组测试示例.hl](comprehensive/数组测试示例.hl) | 数组操作 |
| [数据处理示例.hl](comprehensive/数据处理示例.hl) | 数据转换和聚合 |

### 根目录综合示例

| 文件 | 说明 |
|:-----|:-----|
| [综合示例.hl](综合示例.hl) | 综合示例 |

### 项目示例

| 目录 | 说明 |
|:-----|:-----|
| [todo_app/](todo_app/) | Todo应用完整项目 |
| [basic_project/](basic_project/) | 基础项目结构 |
| [workspace_example/](workspace_example/) | 工作区多包项目 |

## 快速运行

```bash
# 运行基础示例
huan run examples/basic/你好世界.hl
huan run examples/basic/变量示例.hl
huan run examples/functions/函数示例.hl

# 运行进阶示例
huan run examples/data_structures/结构体示例.hl
huan run examples/data_structures/算法示例.hl
huan run examples/error_handling/错误处理示例.hl

# 运行实用示例
huan run examples/file_io/文件处理示例.hl
huan run examples/network/网络请求示例.hl
huan run examples/concurrency/并发示例.hl
```

## 学习路径

### 第一阶段：基础
1. [basic/你好世界.hl](basic/你好世界.hl) - 程序入口
2. [basic/变量示例.hl](basic/变量示例.hl) - 数据类型
3. [functions/函数示例.hl](functions/函数示例.hl) - 函数
4. [basic/控制流示例.hl](basic/控制流示例.hl) - 控制流
5. [data_structures/集合示例.hl](data_structures/集合示例.hl) - 集合

### 第二阶段：进阶
6. [functions/函数与递归全览.hl](functions/函数与递归全览.hl) - 递归
7. [data_structures/结构体示例.hl](data_structures/结构体示例.hl) - 面向对象
8. [data_structures/算法示例.hl](data_structures/算法示例.hl) - 算法
9. [error_handling/错误处理示例.hl](error_handling/错误处理示例.hl) - 错误处理

### 第三阶段：实用
10. [file_io/文件处理示例.hl](file_io/文件处理示例.hl) - 文件IO
11. [network/网络请求示例.hl](network/网络请求示例.hl) - 网络
12. [concurrency/并发示例.hl](concurrency/并发示例.hl) - 并发
13. [comprehensive/数据处理示例.hl](comprehensive/数据处理示例.hl) - 数据处理

### 第四阶段：高级
14. [assembly/汇编演示.hl](assembly/汇编演示.hl) - 汇编集成
15. [interop/跨语言互操作示例.hl](interop/跨语言互操作示例.hl) - 跨语言互操作
16. [advanced/高级特性全览.hl](advanced/高级特性全览.hl) - 高级特性
17. [todo_app/](todo_app/) - 项目实战

## 目录结构

```
examples/
├── basic/                    # 基础示例 (5个)
│   ├── 你好世界.hl
│   ├── 变量示例.hl
│   ├── 基础语法全览.hl
│   ├── 控制流全览.hl
│   └── 控制流示例.hl
├── functions/                # 函数示例 (2个)
│   ├── 函数示例.hl
│   └── 函数与递归全览.hl
├── data_structures/         # 数据结构示例 (4个)
│   ├── 数据结构全览.hl
│   ├── 算法示例.hl
│   ├── 结构体示例.hl
│   └── 集合示例.hl
├── error_handling/          # 错误处理示例 (1个)
│   └── 错误处理示例.hl
├── concurrency/             # 并发示例 (1个)
│   └── 并发示例.hl
├── assembly/               # 汇编与裸机示例 (2个)
│   ├── 汇编演示.hl
│   └── 裸机编程演示.hl
├── file_io/                # 文件IO示例 (1个)
│   └── 文件处理示例.hl
├── network/                # 网络示例 (1个)
│   └── 网络请求示例.hl
├── modules/                # 模块导入示例 (2个)
│   ├── 模块导入示例.hl
│   └── 选择性导入示例.hl
├── interop/               # 跨语言互操作示例 (1个)
│   └── 跨语言互操作示例.hl
├── oop/                   # 面向对象示例 (3个)
│   ├── 面向对象示例.hl
│   ├── 方法调用示例.hl
│   └── 排序算法示例.hl
├── advanced/              # 高级特性示例 (1个)
│   └── 高级特性全览.hl
├── performance/           # 性能优化示例 (1个)
│   └── 性能优化示例.hl
├── comprehensive/         # 综合示例 (2个)
│   ├── 数组测试示例.hl
│   └── 数据处理示例.hl
├── basic_project/         # 基础项目
├── workspace_example/     # 工作区示例
├── todo_app/             # Todo应用项目
└── 综合示例.hl            # 根目录综合示例
```

**总计：27个示例程序**

## 贡献

欢迎提交新的示例程序！

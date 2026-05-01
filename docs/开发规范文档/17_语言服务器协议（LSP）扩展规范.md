# 幻语编程语言 - 开发规范文档
# 17. 语言服务器协议（LSP）扩展规范

**版本**: v0.4.0
**许可证**: Apache-2.0
**更新日期**: 2026年5月
**适用对象**: IDE开发者、LSP集成工程师

> **说明**: 本章详细描述幻语语言服务器协议（LSP）的扩展规范，包括代码补全、跳转定义、重构和AI辅助功能。

---

## 17.1 概述

幻语语言服务器（Huan Language Server）实现了标准的语言服务器协议（LSP），为编辑器（如 VSCode、Vim、Emacs）提供智能代码辅助功能。此外，幻语扩展了 LSP 协议，增加了 AI 代码生成、代码解释、HLA 格式转换等自定义方法。

**核心功能**：
- **语法高亮语义标记**：提供比纯语法高亮更精确的语义着色
- **智能补全**：基于类型推导和上下文的代码补全
- **实时诊断**：语法错误、类型错误、所有权错误的即时反馈
- **跳转定义/引用**：符号导航
- **悬停提示**：显示类型信息和文档
- **重命名符号**：项目级安全重命名
- **代码格式化**：统一代码风格，支持关键词风格转换
- **AI 辅助**：自然语言生成代码、代码解释

## 17.2 服务器初始化

### 17.2.1 启动方式

语言服务器通过 `huan serve` 命令启动：

```bash
huan serve [--stdio | --socket <端口>]
```

- `--stdio`：通过标准输入输出与客户端通信（默认）
- `--socket`：通过 TCP 套接字监听指定端口

### 17.2.2 Initialize 请求与响应

服务器在接收到 `initialize` 请求时，返回其能力集。

**请求示例**：
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "processId": 12345,
    "rootUri": "file:///path/to/project",
    "capabilities": {
      "textDocument": {
        "completion": { "completionItem": { "snippetSupport": true } }
      }
    }
  }
}
```

**响应示例**：
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "result": {
    "serverInfo": {
      "name": "幻语 LSP",
      "version": "1.2.0"
    },
    "capabilities": {
      "textDocumentSync": {
        "openClose": true,
        "change": 2,
        "save": { "includeText": true }
      },
      "completionProvider": {
        "triggerCharacters": ["令", "let", " ", ".", ":", "(", "{"],
        "resolveProvider": true
      },
      "hoverProvider": true,
      "definitionProvider": true,
      "referencesProvider": true,
      "documentHighlightProvider": true,
      "documentSymbolProvider": true,
      "workspaceSymbolProvider": true,
      "codeActionProvider": {
        "codeActionKinds": ["quickfix", "refactor", "source"]
      },
      "codeLensProvider": {
        "resolveProvider": true
      },
      "documentFormattingProvider": true,
      "documentRangeFormattingProvider": true,
      "renameProvider": {
        "prepareProvider": true
      },
      "signatureHelpProvider": {
        "triggerCharacters": ["(", ","]
      },
      "semanticTokensProvider": {
        "legend": {
          "tokenTypes": [
            "keyword", "type", "function", "variable", "parameter",
            "string", "number", "comment", "operator", "property"
          ],
          "tokenModifiers": ["declaration", "definition", "readonly", "static"]
        },
        "full": { "delta": true },
        "range": true
      },
      "workspace": {
        "workspaceFolders": { "supported": true, "changeNotifications": true }
      }
    }
  }
}
```

## 17.3 标准 LSP 功能实现细节

### 17.3.1 文本同步

服务器维护每个打开文档的状态，包括内容、版本号和语法树。

```rust
pub struct Document {
    pub uri: String,
    pub version: i32,
    pub content: Rope,
    pub ast: Option<Program>,
    pub symbols: Vec<DocumentSymbol>,
    pub diagnostics: Vec<Diagnostic>,
    pub last_modified: SystemTime,
}
```

**事件处理**：
- `textDocument/didOpen`：创建文档状态，触发全量解析和诊断
- `textDocument/didChange`：增量更新内容，触发增量解析和诊断
- `textDocument/didSave`：保存文档，触发完整编译检查
- `textDocument/didClose`：释放文档资源

### 17.3.2 补全

补全提供以下类型的建议：
- **关键词补全**：`令`、`若`、`函数` 等
- **类型补全**：`整数`、`字符串`、`列表` 等
- **变量/函数补全**：基于当前作用域的符号
- **方法补全**：基于接收者类型的方法列表
- **字段补全**：结构体字段
- **代码片段**：如 `函数` 模板、`若` 模板

**补全项结构**：
```rust
pub struct CompletionItem {
    pub label: String,
    pub kind: CompletionItemKind,
    pub detail: Option<String>,
    pub documentation: Option<Documentation>,
    pub insert_text: Option<String>,
    pub insert_text_format: InsertTextFormat,
    pub sort_text: Option<String>,
    pub filter_text: Option<String>,
    // 幻语扩展：三语同义词
    pub huan_synonyms: Option<HuanSynonyms>,
}

pub struct HuanSynonyms {
    pub chinese: String,
    pub pinyin: String,
    pub english: String,
}
```

**补全逻辑**：
1. 解析光标位置的上下文（表达式、语句、类型位置）
2. 从符号表中收集可见符号
3. 根据上下文过滤（如类型位置只显示类型）
4. 计算排序分数（局部变量 > 全局 > 关键词）
5. 返回补全列表

**示例响应**：
```json
{
  "label": "令",
  "kind": 14,
  "detail": "变量声明",
  "documentation": "声明一个新的可变变量。\n\n语法: 令 变量名 [类型 类型] 为 表达式",
  "insertText": "令 ${1:变量名} 为 ${2:值}",
  "insertTextFormat": 2,
  "huanSynonyms": {
    "chinese": "令",
    "pinyin": "ling",
    "english": "let"
  }
}
```

### 17.3.3 悬停提示

显示光标下符号的类型信息和文档注释。

**信息提取流程**：
1. 解析光标位置的标识符
2. 在符号表中查找定义
3. 格式化类型信息
4. 提取文档注释
5. 对于函数，显示参数类型和返回类型

**示例响应**：
```json
{
  "contents": {
    "kind": "markdown",
    "value": "```幻语\n函数 斐波那契(次数: 整数) -> 整数\n```\n\n---\n计算斐波那契数列的第 n 项。\n\n**参数**:\n- `次数`: 要计算的项数（从 0 开始）\n\n**返回**: 第 n 项的值"
  },
  "range": {
    "start": { "line": 10, "character": 4 },
    "end": { "line": 10, "character": 12 }
  }
}
```

### 17.3.4 跳转定义

支持跳转到变量、函数、类型、模块的定义位置。

**查找逻辑**：
1. 解析光标位置的标识符
2. 在符号表中查找定义位置
3. 如果定义在其他文件，返回文件 URI 和位置
4. 对于导入的符号，解析导入路径并定位

**示例响应**：
```json
[
  {
    "uri": "file:///path/to/lib.hl",
    "range": {
      "start": { "line": 42, "character": 0 },
      "end": { "line": 42, "character": 6 }
    }
  }
]
```

### 17.3.5 引用查找

查找符号在工作区中的所有引用。

**查找流程**：
1. 遍历工作区所有文件
2. 使用符号表索引快速定位可能的位置
3. 对每个位置进行精确的语义验证
4. 返回所有匹配的位置

### 17.3.6 重命名

安全地重命名符号，更新所有引用。

**重命名流程**：
1. 验证新名称合法性（不与关键词冲突，不重复定义）
2. 查找所有引用位置
3. 对每个位置生成文本编辑
4. 返回 `WorkspaceEdit`

### 17.3.7 代码格式化

格式化整个文档或选定范围。

**格式化规则**：
- 统一缩进（默认 4 空格）
- 运算符两侧添加空格
- 逗号后添加空格
- 统一关键词风格（根据配置）
- 合理的换行

### 17.3.8 代码操作

提供快速修复和重构操作。

| 操作 | 触发条件 | 说明 |
|------|---------|------|
| 导入未定义符号 | 未定义变量错误 | 自动添加 `导入` 语句 |
| 提取函数 | 选中代码块 | 将选中代码提取为新函数 |
| 内联变量 | 变量只使用一次 | 用值替换变量引用 |
| 添加缺失的 `结束` | 未闭合块错误 | 自动补全 `结束` |
| 移除未使用的导入 | 导入未使用 | 删除无用的 `导入` 语句 |
| 实现缺失方法 | 特征未完全实现 | 生成方法骨架 |

## 17.4 语义标记

提供比语法高亮更精确的着色信息。

**标记类型映射**：
| 幻语语义 | LSP TokenType |
|----------|---------------|
| 关键词 | `keyword` |
| 类型名 | `type` |
| 函数定义 | `function` |
| 变量 | `variable` |
| 参数 | `parameter` |
| 字符串字面量 | `string` |
| 数字字面量 | `number` |
| 注释 | `comment` |
| 运算符 | `operator` |
| 字段 | `property` |

**增量更新**：仅发送变更区域的数据，减少网络传输。

## 17.5 诊断

实时报告代码错误和警告。

### 17.5.1 诊断类型

| 严重性 | 来源 | 说明 |
|--------|------|------|
| Error | 语法/类型检查 | 编译将失败的错误 |
| Warning | 静态分析 | 潜在问题（如未使用变量） |
| Information | 风格检查 | 代码风格建议 |
| Hint | 优化建议 | 可选的优化提示 |

### 17.5.2 诊断生成策略

- **增量诊断**：只重新分析修改的部分，提高响应速度
- **延迟诊断**：用户停止输入后延迟 300ms 再触发诊断，避免过度计算
- **优先级队列**：当前可见文件的诊断优先

### 17.5.3 诊断代码与快速修复

每个诊断可关联一个诊断代码，用于快速修复：

| 代码 | 说明 | 快速修复 |
|------|------|---------|
| `E001` | 未定义变量 | 建议导入或创建变量 |
| `E002` | 类型不匹配 | 建议类型转换 |
| `E003` | 未闭合块 | 补全 `结束` |
| `W001` | 未使用变量 | 移除或添加下划线前缀 |
| `W002` | 不可变变量 | 建议改为 `定` |

## 17.6 自定义扩展方法

幻语 LSP 扩展了标准协议，提供 AI 辅助功能。

### 17.6.1 `huan/generateCode` - AI 代码生成

**请求**：
```typescript
interface HuanGenerateCodeParams {
  description: string;           // 自然语言描述
  context?: {                    // 上下文信息
    uri: string;
    position: Position;
  };
  style?: 'chinese' | 'pinyin' | 'english';  // 关键词风格
}
```

**响应**：
```typescript
interface HuanGenerateCodeResult {
  code: string;                  // 生成的代码
  explanation?: string;          // 解释说明
  alternatives?: string[];       // 备选方案
}
```

### 17.6.2 `huan/explainCode` - 代码解释

**请求**：
```typescript
interface HuanExplainCodeParams {
  code: string;                  // 要解释的代码
  language?: 'zh' | 'en';        // 输出语言
}
```

**响应**：
```typescript
interface HuanExplainCodeResult {
  explanation: string;           // 解释内容
  complexity?: '简单' | '中等' | '复杂';
  suggestions?: string[];        // 改进建议
}
```

### 17.6.3 `huan/convertToHla` - 转换为 HLA 格式

**请求**：
```typescript
interface HuanConvertToHlaParams {
  textDocument: TextDocumentIdentifier;
}
```

**响应**：
```typescript
interface HuanConvertToHlaResult {
  hla: string;                   // HLA 格式内容
```

### 17.6.4 `huan/changeKeywordStyle` - 关键词风格转换

**请求**：
```typescript
interface HuanChangeKeywordStyleParams {
  textDocument: TextDocumentIdentifier;
  style: 'chinese' | 'pinyin' | 'english';
}
```

**响应**：
```typescript
interface HuanChangeKeywordStyleResult {
  newText: string;               // 转换后的文本
}
```

### 17.6.5 `huan/compilerProgress` - 编译进度通知（服务器 → 客户端）

```typescript
interface HuanCompilerProgressNotification {
  method: 'huan/compilerProgress';
  params: {
    stage: 'parsing' | 'typecheck' | 'codegen' | 'linking';
    progress: number;            // 0-100
    message?: string;
  };
}
```

## 17.7 服务器架构

### 17.7.1 模块结构

```
lsp_server/
├── main.rs                 # 入口
├── server.rs               # LSP 协议处理主循环
├── handlers/               # 请求处理器
│   ├── initialize.rs
│   ├── completion.rs
│   ├── hover.rs
│   ├── definition.rs
│   └── ...
├── analysis/               # 语义分析
│   ├── document.rs         # 文档状态管理
│   ├── symbol_table.rs     # 符号表索引
│   ├── type_checker.rs     # 增量类型检查
│   └── diagnostics.rs      # 诊断生成
├── ai/                     # AI 功能
│   ├── generator.rs        # 代码生成
│   └── explainer.rs        # 代码解释
└── utils/                  # 工具函数
```

### 17.7.2 并发模型

- **主线程**：处理 LSP 消息，维护文档状态
- **分析线程池**：执行耗时的语法分析、类型检查
- **诊断队列**：序列化诊断请求，避免竞争

### 17.7.3 工作区索引

为加速跨文件引用查找，服务器维护工作区符号索引：

```rust
pub struct WorkspaceIndex {
    /// 符号名 → 定义位置列表
    symbols: HashMap<String, Vec<SymbolLocation>>,
    /// 文件 → 符号列表
    files: HashMap<String, Vec<Symbol>>,
    /// 文件依赖图
    dependencies: DiGraph<FileId, ()>,
}

impl WorkspaceIndex {
    pub fn update_file(&mut self, uri: &str, ast: &Program);
    pub fn find_definition(&self, name: &str, context: &Context) -> Option<Location>;
    pub fn find_references(&self, name: &str, context: &Context) -> Vec<Location>;
}
```

## 17.8 编辑器集成指南

### 17.8.1 VSCode 扩展配置

```json
{
  "name": "vscode-huan",
  "displayName": "幻语",
  "version": "1.2.0",
  "engines": { "vscode": "^1.85.0" },
  "activationEvents": ["onLanguage:huan"],
  "main": "./out/extension.js",
  "contributes": {
    "languages": [{
      "id": "huan",
      "aliases": ["幻语", "HuanLang"],
      "extensions": [".hl", ".hla", ".hasm"],
      "configuration": "./language-configuration.json"
    }],
    "grammars": [{
      "language": "huan",
      "scopeName": "source.huan",
      "path": "./syntaxes/huan.tmLanguage.json"
    }],
    "commands": [
      { "command": "huan.generateCode", "title": "幻语: 生成代码" },
      { "command": "huan.explainCode", "title": "幻语: 解释代码" },
      { "command": "huan.convertToAef", "title": "幻语: 转换为 AEF" }
    ],
    "keybindings": [
      { "command": "huan.generateCode", "key": "ctrl+shift+g", "when": "editorLangId == huan" }
    ],
    "configuration": {
      "title": "幻语",
      "properties": {
        "huan.lsp.path": {
          "type": "string",
          "default": "huan",
          "description": "幻语 LSP 可执行文件路径"
        },
        "huan.keywordStyle": {
          "type": "string",
          "enum": ["chinese", "pinyin", "english", "mixed"],
          "default": "mixed",
          "description": "关键词显示风格"
        }
      }
    }
  }
}
```

### 17.8.2 Vim/Neovim 配置

```vim
" 使用 vim-lsp
augroup huan_lsp
  autocmd!
  autocmd User lsp_setup call lsp#register_server({
        \ 'name': 'huan',
        \ 'cmd': {server_info->['huan', 'serve']},
        \ 'allowlist': ['huan'],
        \ })
augroup END
```

## 17.9 测试用例

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_completion_keywords() {
        let server = TestServer::new();
        let items = server.completion("", Position::new(0, 0)).unwrap();
        assert!(items.iter().any(|i| i.label == "令"));
        assert!(items.iter().any(|i| i.label == "若"));
    }

    #[test]
    fn test_hover_type_info() {
        let server = TestServer::new();
        server.open_document("test.hl", "令 年龄 为 25\n年龄");
        let hover = server.hover("test.hl", Position::new(1, 0)).unwrap();
        assert!(hover.contents.contains("整数"));
    }

    #[test]
    fn test_diagnostics() {
        let server = TestServer::new();
        server.open_document("test.hl", "令 年龄 为 \"二十五\"\n返回 年龄");
        let diags = server.diagnostics("test.hl");
        assert!(!diags.is_empty());
    }
}
```

---

## 本章总结

本章详细描述了幻语语言服务器协议（LSP）扩展规范的完整设计，包括：

1. **LSP概述**：标准LSP协议实现和幻语扩展的AI功能
2. **服务器初始化**：InitializeResult定义和服务器能力
3. **文本文档同步**：文本文档内容同步的完整支持
4. **代码补全**：基于类型推导的智能补全和关键词补全
5. **悬停信息**：类型信息和文档的显示
6. **跳转定义**：符号导航和引用查找
7. **诊断信息**：语法错误、类型错误和所有权错误的实时反馈
8. **代码操作**：代码修复和重构的支持
9. **AI扩展**：代码生成、代码解释、AEF转换等AI功能
10. **VSCode集成**：完整的插件配置和扩展API

这些规范为幻语的IDE和编辑器支持提供了完整的技术指导。


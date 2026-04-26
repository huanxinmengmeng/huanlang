// Copyright © 2026 幻心梦梦 (huanxinmengmeng)
// Licensed under the Apache License, Version 2.0 (the "License");
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! 悬停请求处理器

use crate::lsp::{Position, Range};

/// 悬停内容
#[derive(Debug, Clone)]
pub struct HoverContents {
    /// 内容类型
    pub kind: HoverContentsKind,
    /// 内容值
    pub value: String,
}

/// 悬停内容类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HoverContentsKind {
    PlainText,
    Markdown,
}

impl Default for HoverContentsKind {
    fn default() -> Self {
        HoverContentsKind::Markdown
    }
}

/// 悬停结果
#[derive(Debug, Clone)]
pub struct Hover {
    /// 悬停内容
    pub contents: HoverContents,
    /// 范围（可选）
    pub range: Option<Range>,
}

impl Hover {
    /// 创建新的悬停
    pub fn new(contents: HoverContents) -> Self {
        Hover {
            contents,
            range: None,
        }
    }

    /// 设置范围
    pub fn with_range(mut self, range: Range) -> Self {
        self.range = Some(range);
        self
    }
}

/// 悬停请求处理器
pub struct HoverHandler;

impl HoverHandler {
    /// 处理悬停请求
    pub fn handle(uri: &str, position: Position, word: &str) -> Option<Hover> {
        // 这里应该从符号表或AST中查找符号信息
        // 简化实现：根据词语返回示例信息
        
        if word.is_empty() {
            return None;
        }
        
        // 示例：基于关键词返回悬停信息
        let info = match word {
            "函数" => Some(HoverInfo {
                type_info: "关键字".to_string(),
                signature: "函数 函数名(参数: 类型) 返回 类型".to_string(),
                documentation: "声明并定义一个函数。\n\n函数用于封装可重用的代码逻辑。".to_string(),
            }),
            "令" => Some(HoverInfo {
                type_info: "关键字".to_string(),
                signature: "令 变量名 [: 类型] 为 初始值".to_string(),
                documentation: "声明一个可变变量。\n\n变量可以在后续代码中被修改。".to_string(),
            }),
            "定" => Some(HoverInfo {
                type_info: "关键字".to_string(),
                signature: "定 常量名 [: 类型] 为 初始值".to_string(),
                documentation: "声明一个不可变常量。\n\n常量一旦声明就不能被修改。".to_string(),
            }),
            "整数" => Some(HoverInfo {
                type_info: "类型".to_string(),
                signature: "整数".to_string(),
                documentation: "整数类型。\n\n表示整数值，如 42、-17、0。".to_string(),
            }),
            "字符串" => Some(HoverInfo {
                type_info: "类型".to_string(),
                signature: "字符串".to_string(),
                documentation: "字符串类型。\n\n表示文本数据，如 \"Hello, World!\"。".to_string(),
            }),
            "若" => Some(HoverInfo {
                type_info: "关键字".to_string(),
                signature: "若 条件 那么\n    // 代码块\n结束".to_string(),
                documentation: "条件语句。\n\n根据条件是否为真来决定是否执行代码块。".to_string(),
            }),
            _ => None,
        };
        
        info.map(|i| i.to_hover())
    }
}

/// 悬停信息
struct HoverInfo {
    type_info: String,
    signature: String,
    documentation: String,
}

impl HoverInfo {
    /// 转换为悬停结果
    fn to_hover(self) -> Hover {
        let mut markdown = String::new();
        markdown.push_str(&format!("```huan\n{}\n```\n\n---\n\n", self.signature));
        markdown.push_str(&format!("**类型**: {}\n\n", self.type_info));
        markdown.push_str(&self.documentation);
        
        Hover::new(HoverContents {
            kind: HoverContentsKind::Markdown,
            value: markdown,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hover_creation() {
        let hover = Hover::new(HoverContents {
            kind: HoverContentsKind::Markdown,
            value: "# Test".to_string(),
        }).with_range(Range::new(
            Position::new(0, 0),
            Position::new(0, 4),
        ));
        
        assert!(hover.range.is_some());
    }

    #[test]
    fn test_keyword_hover() {
        let hover = HoverHandler::handle("file:///test.hl", Position::new(0, 0), "函数");
        assert!(hover.is_some());
        
        let hover = hover.unwrap();
        assert!(hover.contents.value.contains("函数"));
    }

    #[test]
    fn test_type_hover() {
        let hover = HoverHandler::handle("file:///test.hl", Position::new(0, 0), "整数");
        assert!(hover.is_some());
    }

    #[test]
    fn test_unknown_word_hover() {
        let hover = HoverHandler::handle("file:///test.hl", Position::new(0, 0), "未知词");
        assert!(hover.is_none());
    }

    #[test]
    fn test_empty_word_hover() {
        let hover = HoverHandler::handle("file:///test.hl", Position::new(0, 0), "");
        assert!(hover.is_none());
    }
}

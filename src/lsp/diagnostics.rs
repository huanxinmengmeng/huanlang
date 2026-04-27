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

//! 诊断模块
//!
//! 本模块负责生成和管理代码诊断信息，包括：
//! - 语法错误
//! - 类型错误
//! - 所有权错误
//! - 静态分析警告

use crate::lsp::{Range, Location};

/// 诊断严重程度
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticSeverity {
    /// 错误（导致编译失败）
    Error = 1,
    /// 警告（潜在问题）
    Warning = 2,
    /// 信息（风格建议）
    Information = 3,
    /// 提示（优化建议）
    Hint = 4,
}

impl Default for DiagnosticSeverity {
    fn default() -> Self {
        DiagnosticSeverity::Error
    }
}

/// 诊断标记
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiagnosticTag {
    /// 不建议使用
    Deprecated,
    /// 不可达代码
    Unnecessary,
    /// 未使用的代码
    Unused,
}

impl Default for DiagnosticTag {
    fn default() -> Self {
        DiagnosticTag::Unused
    }
}

/// 诊断代码
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiagnosticCode {
    /// 代码值
    pub value: String,
    /// 代码的可读描述
    pub description: Option<String>,
}

impl DiagnosticCode {
    /// 创建新的诊断代码
    pub fn new(value: String) -> Self {
        DiagnosticCode {
            value,
            description: None,
        }
    }

    /// 设置描述
    pub fn with_description(mut self, desc: String) -> Self {
        self.description = Some(desc);
        self
    }
}

/// 诊断相关代码
#[derive(Debug, Clone)]
pub struct DiagnosticRelatedInformation {
    /// 位置
    pub location: Location,
    /// 消息
    pub message: String,
}

impl DiagnosticRelatedInformation {
    /// 创建新的相关诊断信息
    pub fn new(location: Location, message: String) -> Self {
        DiagnosticRelatedInformation { location, message }
    }
}

/// 诊断结构
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// 诊断范围
    pub range: Range,
    /// 严重程度
    pub severity: DiagnosticSeverity,
    /// 诊断代码
    pub code: Option<DiagnosticCode>,
    /// 消息
    pub message: String,
    /// 来源
    pub source: Option<String>,
    /// 相关诊断信息
    pub related_information: Vec<DiagnosticRelatedInformation>,
    /// 诊断标签
    pub tags: Vec<DiagnosticTag>,
}

impl Diagnostic {
    /// 创建新的诊断
    pub fn new(range: Range, severity: DiagnosticSeverity, message: String) -> Self {
        Diagnostic {
            range,
            severity,
            code: None,
            message,
            source: Some("幻语 LSP".to_string()),
            related_information: Vec::new(),
            tags: Vec::new(),
        }
    }

    /// 设置诊断代码
    pub fn with_code(mut self, code: DiagnosticCode) -> Self {
        self.code = Some(code);
        self
    }

    /// 添加相关诊断信息
    pub fn with_related_info(mut self, info: DiagnosticRelatedInformation) -> Self {
        self.related_information.push(info);
        self
    }

    /// 添加标签
    pub fn with_tag(mut self, tag: DiagnosticTag) -> Self {
        self.tags.push(tag);
        self
    }

    /// 检查是否为错误
    pub fn is_error(&self) -> bool {
        self.severity == DiagnosticSeverity::Error
    }

    /// 检查是否为警告
    pub fn is_warning(&self) -> bool {
        self.severity == DiagnosticSeverity::Warning
    }
}

/// 诊断生成器
#[derive(Debug, Clone)]
pub struct DiagnosticGenerator {
    /// 诊断列表
    diagnostics: Vec<Diagnostic>,
    /// 源文件 URI
    _uri: String,
}

impl DiagnosticGenerator {
    /// 创建新的诊断生成器
    pub fn new(uri: String) -> Self {
        DiagnosticGenerator {
            diagnostics: Vec::new(),
            _uri: uri,
        }
    }

    /// 报告错误
    pub fn error(&mut self, range: Range, code: &str, message: &str) -> &mut Diagnostic {
        let diagnostic = Diagnostic::new(
            range,
            DiagnosticSeverity::Error,
            message.to_string(),
        ).with_code(DiagnosticCode::new(code.to_string()));
        
        self.diagnostics.push(diagnostic);
        self.diagnostics.last_mut().unwrap()
    }

    /// 报告警告
    pub fn warning(&mut self, range: Range, code: &str, message: &str) -> &mut Diagnostic {
        let diagnostic = Diagnostic::new(
            range,
            DiagnosticSeverity::Warning,
            message.to_string(),
        ).with_code(DiagnosticCode::new(code.to_string()));
        
        self.diagnostics.push(diagnostic);
        self.diagnostics.last_mut().unwrap()
    }

    /// 报告信息
    pub fn information(&mut self, range: Range, code: &str, message: &str) -> &mut Diagnostic {
        let diagnostic = Diagnostic::new(
            range,
            DiagnosticSeverity::Information,
            message.to_string(),
        ).with_code(DiagnosticCode::new(code.to_string()));
        
        self.diagnostics.push(diagnostic);
        self.diagnostics.last_mut().unwrap()
    }

    /// 报告提示
    pub fn hint(&mut self, range: Range, code: &str, message: &str) -> &mut Diagnostic {
        let diagnostic = Diagnostic::new(
            range,
            DiagnosticSeverity::Hint,
            message.to_string(),
        ).with_code(DiagnosticCode::new(code.to_string()));
        
        self.diagnostics.push(diagnostic);
        self.diagnostics.last_mut().unwrap()
    }

    /// 获取所有诊断
    pub fn into_diagnostics(self) -> Vec<Diagnostic> {
        self.diagnostics
    }

    /// 获取诊断引用
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// 清空诊断
    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    /// 获取错误数量
    pub fn error_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.is_error()).count()
    }

    /// 获取警告数量
    pub fn warning_count(&self) -> usize {
        self.diagnostics.iter().filter(|d| d.is_warning()).count()
    }
}

/// 预定义的诊断代码
pub mod codes {
    /// 未定义变量
    pub const E001_UNDEFINED_VARIABLE: &str = "E001";
    /// 类型不匹配
    pub const E002_TYPE_MISMATCH: &str = "E002";
    /// 未闭合块
    pub const E003_UNCLOSED_BLOCK: &str = "E003";
    /// 重复定义
    pub const E004_DUPLICATE_DEFINITION: &str = "E004";
    /// 无效语法
    pub const E005_INVALID_SYNTAX: &str = "E005";
    
    /// 未使用变量
    pub const W001_UNUSED_VARIABLE: &str = "W001";
    /// 建议使用不可变变量
    pub const W002_USE_IMMUTABLE: &str = "W002";
    /// 可能的所有权问题
    pub const W003_OWNERSHIP_ISSUE: &str = "W003";
    
    /// 信息：代码风格建议
    pub const I001_STYLE_SUGGESTION: &str = "I001";
    /// 提示：可以简化
    pub const H001_SIMPLIFICATION: &str = "H001";
}

// 常见的错误诊断生成辅助函数

/// 生成"未定义变量"诊断
pub fn undefined_variable(_uri: &str, range: Range, name: &str) -> Diagnostic {
    Diagnostic::new(
        range,
        DiagnosticSeverity::Error,
        format!("未定义的变量：{}", name),
    ).with_code(
        DiagnosticCode::new(codes::E001_UNDEFINED_VARIABLE.to_string())
            .with_description("尝试使用未声明的变量。".to_string())
    )
}

/// 生成"类型不匹配"诊断
pub fn type_mismatch(_uri: &str, range: Range, expected: &str, actual: &str) -> Diagnostic {
    Diagnostic::new(
        range,
        DiagnosticSeverity::Error,
        format!("类型不匹配：期望 {}，实际 {}", expected, actual),
    ).with_code(
        DiagnosticCode::new(codes::E002_TYPE_MISMATCH.to_string())
            .with_description("赋值或使用的类型与声明不匹配。".to_string())
    )
}

/// 生成"未闭合块"诊断
pub fn unclosed_block(_uri: &str, range: Range) -> Diagnostic {
    Diagnostic::new(
        range,
        DiagnosticSeverity::Error,
        "未闭合的代码块，缺少 '结束' 关键字".to_string(),
    ).with_code(
        DiagnosticCode::new(codes::E003_UNCLOSED_BLOCK.to_string())
            .with_description("代码块没有正确关闭。".to_string())
    )
}

/// 生成"未使用变量"诊断
pub fn unused_variable(_uri: &str, range: Range, name: &str) -> Diagnostic {
    Diagnostic::new(
        range,
        DiagnosticSeverity::Warning,
        format!("未使用的变量：{}", name),
    ).with_code(
        DiagnosticCode::new(codes::W001_UNUSED_VARIABLE.to_string())
            .with_description("声明了但未使用的变量。".to_string())
    ).with_tag(DiagnosticTag::Unused)
}

/// 生成"建议使用不可变变量"诊断
pub fn suggest_immutable(_uri: &str, range: Range, name: &str) -> Diagnostic {
    Diagnostic::new(
        range,
        DiagnosticSeverity::Warning,
        format!("变量 '{}' 声明后未被修改，建议使用 '定' 声明", name),
    ).with_code(
        DiagnosticCode::new(codes::W002_USE_IMMUTABLE.to_string())
            .with_description("对于不可变的变量，使用 '定' 可以提高性能和安全性。".to_string())
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lsp::Position;

    #[test]
    fn test_diagnostic_creation() {
        let range = Range::new(
            Position::new(0, 0),
            Position::new(0, 5),
        );
        let diag = Diagnostic::new(
            range.clone(),
            DiagnosticSeverity::Error,
            "测试错误".to_string(),
        );
        
        assert_eq!(diag.range, range);
        assert_eq!(diag.severity, DiagnosticSeverity::Error);
        assert_eq!(diag.message, "测试错误");
        assert!(diag.is_error());
        assert!(!diag.is_warning());
    }

    #[test]
    fn test_diagnostic_generator() {
        let mut gen = DiagnosticGenerator::new("file:///test.hl".to_string());
        
        gen.error(
            Range::new(Position::new(0, 0), Position::new(0, 5)),
            codes::E001_UNDEFINED_VARIABLE,
            "未定义的变量",
        );
        
        gen.warning(
            Range::new(Position::new(1, 0), Position::new(1, 5)),
            codes::W001_UNUSED_VARIABLE,
            "未使用的变量",
        );
        
        assert_eq!(gen.error_count(), 1);
        assert_eq!(gen.warning_count(), 1);
    }

    #[test]
    fn test_undefined_variable() {
        let range = Range::new(Position::new(0, 0), Position::new(0, 5));
        let diag = undefined_variable("file:///test.hl", range, "变量");
        
        assert!(diag.is_error());
        assert!(diag.message.contains("变量"));
    }

    #[test]
    fn test_type_mismatch() {
        let range = Range::new(Position::new(0, 0), Position::new(0, 5));
        let diag = type_mismatch("file:///test.hl", range, "整数", "字符串");
        
        assert!(diag.is_error());
        assert!(diag.message.contains("整数"));
        assert!(diag.message.contains("字符串"));
    }
}

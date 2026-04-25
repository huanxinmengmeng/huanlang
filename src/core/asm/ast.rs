// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
//
// 幻语内联汇编 AST 定义
// 实现规范第9.4节中的完整内联汇编功能

use std::collections::HashSet;
use crate::core::lexer::token::SourceSpan;
use crate::core::ast::Type;

/// 内联汇编完整表示
#[derive(Debug, Clone, PartialEq)]
pub struct Asm {
    /// 汇编模板字符串列表
    pub templates: Vec<String>,
    /// 输出操作数列表
    pub outputs: Vec<AsmOutput>,
    /// 输入操作数列表
    pub inputs: Vec<AsmInput>,
    /// 破坏列表
    pub clobbers: Vec<AsmClobber>,
    /// 选项列表
    pub options: Vec<AsmOption>,
    /// 源码位置
    pub span: SourceSpan,
}

/// 输出操作数
#[derive(Debug, Clone, PartialEq)]
pub struct AsmOutput {
    /// 约束字符串（如 "=r"、"+r"、"=&r"）
    pub constraint: String,
    /// 变量名（可选）
    pub name: Option<String>,
    /// 变量类型（可选，用于类型验证）
    pub ty: Option<Type>,
    /// 对应的表达式（可选，用于初始值或返回值）
    pub expr: Option<crate::core::ast::Expr>,
}

/// 输入操作数
#[derive(Debug, Clone, PartialEq)]
pub struct AsmInput {
    /// 约束字符串（如 "r"、"m"、"i"）
    pub constraint: String,
    /// 对应表达式
    pub expr: crate::core::ast::Expr,
}

/// 破坏项
#[derive(Debug, Clone, PartialEq)]
pub enum AsmClobber {
    /// 寄存器被破坏
    Register(String),
    /// 条件码/标志被破坏
    Flags,
    /// 内存被写入
    Memory,
}

/// 汇编选项
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AsmOption {
    /// 纯函数：无副作用
    Pure,
    /// 无内存破坏：不读写内存
    Nomem,
    /// 保留标志：不修改条件码
    PreservesFlags,
    /// 不可达：不返回
    Noreturn,
    /// 对齐栈：栈需要对齐
    Alignstack,
    /// 使用Intel语法（x86/x86_64）
    IntelSyntax,
    /// 易失：禁止优化重排
    Volatile,
}

impl Asm {
    /// 创建空的汇编节点
    pub fn new(span: SourceSpan) -> Self {
        Asm {
            templates: Vec::new(),
            outputs: Vec::new(),
            inputs: Vec::new(),
            clobbers: Vec::new(),
            options: Vec::new(),
            span,
        }
    }

    /// 添加汇编模板
    pub fn add_template(&mut self, template: String) {
        self.templates.push(template);
    }

    /// 添加输出操作数
    pub fn add_output(&mut self, constraint: String, name: Option<String>, ty: Option<Type>, expr: Option<crate::core::ast::Expr>) {
        self.outputs.push(AsmOutput {
            constraint,
            name,
            ty,
            expr,
        });
    }

    /// 添加输入操作数
    pub fn add_input(&mut self, constraint: String, expr: crate::core::ast::Expr) {
        self.inputs.push(AsmInput { constraint, expr });
    }

    /// 添加破坏项
    pub fn add_clobber(&mut self, clobber: AsmClobber) {
        self.clobbers.push(clobber);
    }

    /// 添加选项
    pub fn add_option(&mut self, option: AsmOption) {
        self.options.push(option);
    }

    /// 获取选项集合（用于检测重复选项）
    pub fn get_options_set(&self) -> HashSet<AsmOption> {
        self.options.clone().into_iter().collect()
    }

    /// 检测是否有内存相关选项冲突
    pub fn has_conflicting_options(&self) -> bool {
        let has_pure = self.options.contains(&AsmOption::Pure);
        let _has_nomem = self.options.contains(&AsmOption::Nomem);
        let has_memory_clobber = self.clobbers.iter().any(|c| c == &AsmClobber::Memory);

        // 纯函数不应该写内存
        if has_pure && has_memory_clobber {
            return true;
        }

        false
    }

    /// 获取所有被破坏的寄存器
    pub fn get_clobbered_registers(&self) -> Vec<String> {
        self.clobbers
            .iter()
            .filter_map(|c| {
                if let AsmClobber::Register(reg) = c {
                    Some(reg.clone())
                } else {
                    None
                }
            })
            .collect()
    }

    /// 检查是否标记为 volatile
    pub fn is_volatile(&self) -> bool {
        self.options.contains(&AsmOption::Volatile)
    }
}

impl AsmOption {
    /// 从中文关键词转换为选项类型
    pub fn from_chinese(name: &str) -> Option<Self> {
        match name.trim() {
            "纯" | "pure" => Some(AsmOption::Pure),
            "无内存" | "nomem" => Some(AsmOption::Nomem),
            "保留标志" | "preserves_flags" => Some(AsmOption::PreservesFlags),
            "不可达" | "noreturn" => Some(AsmOption::Noreturn),
            "对齐栈" | "alignstack" => Some(AsmOption::Alignstack),
            "英特尔语法" | "intel_syntax" => Some(AsmOption::IntelSyntax),
            "易失" | "volatile" => Some(AsmOption::Volatile),
            _ => None,
        }
    }

    /// 获取选项的字符串表示
    pub fn to_string(&self) -> &'static str {
        match self {
            AsmOption::Pure => "pure",
            AsmOption::Nomem => "nomem",
            AsmOption::PreservesFlags => "preserves_flags",
            AsmOption::Noreturn => "noreturn",
            AsmOption::Alignstack => "alignstack",
            AsmOption::IntelSyntax => "intel_syntax",
            AsmOption::Volatile => "volatile",
        }
    }
}



impl std::hash::Hash for AsmOption {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ast::Expr;

    #[test]
    fn test_asm_new() {
        let span = SourceSpan::dummy();
        let asm = Asm::new(span);
        assert!(asm.templates.is_empty());
        assert!(asm.outputs.is_empty());
        assert!(asm.inputs.is_empty());
        assert!(asm.clobbers.is_empty());
        assert!(asm.options.is_empty());
    }

    #[test]
    fn test_asm_add_template() {
        let span = SourceSpan::dummy();
        let mut asm = Asm::new(span);
        asm.add_template("mov {0}, {1}".into());
        assert_eq!(asm.templates.len(), 1);
        assert_eq!(asm.templates[0], "mov {0}, {1}");
    }

    #[test]
    fn test_asm_add_options() {
        let span = SourceSpan::dummy();
        let mut asm = Asm::new(span);
        asm.add_option(AsmOption::Volatile);
        asm.add_clobber(AsmClobber::Memory);
        asm.add_option(AsmOption::Pure);
        assert_eq!(asm.options.len(), 2);
        assert_eq!(asm.clobbers.len(), 1);
        assert!(asm.is_volatile());
    }

    #[test]
    fn test_option_from_chinese() {
        assert_eq!(AsmOption::from_chinese("纯"), Some(AsmOption::Pure));
        assert_eq!(AsmOption::from_chinese("易失"), Some(AsmOption::Volatile));
        assert_eq!(AsmOption::from_chinese("其他"), None);
    }
}

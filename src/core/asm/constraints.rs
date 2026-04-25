// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
//
// 汇编操作数约束验证
// 实现规范第9.4.3节中的操作数约束系统

use crate::core::ast::Type;
use super::arch::Arch;

/// 约束类型分类
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstraintType {
    /// 输出约束
    Output,
    /// 读写约束（输入输出）
    InOut,
    /// 输入约束
    Input,
    /// 早期破坏输出（避免与输入共享寄存器）
    EarlyClobber,
}

/// 约束表示
#[derive(Debug, Clone, PartialEq)]
pub struct Constraint {
    pub raw: String,
    pub constraint_type: ConstraintType,
    pub constraint_chars: String,
}

impl Constraint {
    /// 从原始约束字符串解析
    pub fn parse(raw: &str) -> Result<Self, String> {
        let mut chars = raw.chars().peekable();
        let mut constraint_type = ConstraintType::Input;

        // 检查前缀
        let mut prefix = String::new();
        while let Some(c) = chars.peek() {
            match c {
                '=' => {
                    if constraint_type == ConstraintType::Input {
                        constraint_type = ConstraintType::Output;
                    } else {
                        return Err(format!("重复前缀 '=' 在约束 '{}' 中", raw));
                    }
                    prefix.push(*c);
                    chars.next();
                },
                '+' => {
                    if constraint_type == ConstraintType::Input {
                        constraint_type = ConstraintType::InOut;
                    } else {
                        return Err(format!("重复前缀 '+' 在约束 '{}' 中", raw));
                    }
                    prefix.push(*c);
                    chars.next();
                },
                '&' => {
                    if constraint_type == ConstraintType::Input {
                        return Err(format!("'&' 前缀不能用于输入约束 '{}'", raw));
                    }
                    constraint_type = ConstraintType::EarlyClobber;
                    prefix.push(*c);
                    chars.next();
                },
                _ => break,
            }
        }

        // 提取剩余字符
        let constraint_chars: String = chars.collect();
        if constraint_chars.is_empty() {
            return Err(format!("约束字符串 '{}' 没有有效的约束字符", raw));
        }

        Ok(Constraint {
            raw: raw.to_string(),
            constraint_type,
            constraint_chars,
        })
    }
}

/// 验证约束是否对特定类型有效
pub fn validate_constraint(constraint: &Constraint, ty: &Type, arch: Arch) -> Result<bool, String> {
    match arch {
        Arch::X86 => validate_x86_constraint(constraint, ty),
        Arch::X86_64 => validate_x86_64_constraint(constraint, ty),
        Arch::ARM => validate_arm_constraint(constraint, ty),
        Arch::RISCV => validate_riscv_constraint(constraint, ty),
        _ => Ok(true), // 对于未知架构，保守地认为是有效的
    }
}

/// x86约束验证
fn validate_x86_constraint(constraint: &Constraint, ty: &Type) -> Result<bool, String> {
    let chars = &constraint.constraint_chars;

    if chars.contains('m') {
        // 内存操作数
        return Ok(true);
    }

    match chars.as_str() {
        "r" => Ok(is_integer_or_ptr_type(ty)),
        "a" | "b" | "c" | "d" => Ok(is_integer_or_ptr_type(ty)),
        "i" => Ok(true), // 立即数
        "f" => Ok(is_float_type(ty)),
        _ => Ok(true), // 保守处理
    }
}

/// x86_64约束验证
fn validate_x86_64_constraint(constraint: &Constraint, ty: &Type) -> Result<bool, String> {
    let chars = &constraint.constraint_chars;

    if chars.contains('m') {
        // 内存操作数
        return Ok(true);
    }

    match chars.as_str() {
        "r" => Ok(is_integer_or_ptr_type(ty)),
        "a" | "b" | "c" | "d" | "S" | "D" => Ok(is_integer_or_ptr_type(ty)),
        "i" => Ok(true), // 立即数
        "f" | "x" | "y" => Ok(is_float_type(ty)),
        _ => Ok(true), // 保守处理
    }
}

/// ARM约束验证
fn validate_arm_constraint(constraint: &Constraint, ty: &Type) -> Result<bool, String> {
    let chars = &constraint.constraint_chars;

    if chars.contains('m') {
        // 内存操作数
        return Ok(true);
    }

    match chars.as_str() {
        "r" | "l" | "h" | "w" => Ok(is_integer_or_ptr_type(ty)),
        "i" => Ok(true), // 立即数
        "f" => Ok(is_float_type(ty)),
        _ => Ok(true), // 保守处理
    }
}

/// RISC-V约束验证
fn validate_riscv_constraint(constraint: &Constraint, ty: &Type) -> Result<bool, String> {
    let chars = &constraint.constraint_chars;

    if chars.contains('m') {
        // 内存操作数
        return Ok(true);
    }

    match chars.as_str() {
        "r" => Ok(is_integer_or_ptr_type(ty)),
        "i" => Ok(true), // 立即数
        "f" => Ok(is_float_type(ty)),
        _ => Ok(true), // 保守处理
    }
}

/// 类型是否是整数或指针类型
fn is_integer_or_ptr_type(ty: &Type) -> bool {
    matches!(ty,
        Type::Int | Type::I8 | Type::I16 | Type::I32 | Type::I64 |
        Type::U8 | Type::U16 | Type::U32 | Type::U64 | Type::Ptr(_))
}

/// 类型是否是浮点类型
fn is_float_type(ty: &Type) -> bool {
    matches!(ty, Type::F32 | Type::F64)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ast::Type;

    #[test]
    fn test_parse_constraint_input() {
        let constraint = Constraint::parse("r").unwrap();
        assert_eq!(constraint.constraint_type, ConstraintType::Input);
        assert_eq!(constraint.constraint_chars, "r");
    }

    #[test]
    fn test_parse_constraint_output() {
        let constraint = Constraint::parse("=r").unwrap();
        assert_eq!(constraint.constraint_type, ConstraintType::Output);
        assert_eq!(constraint.constraint_chars, "r");
    }

    #[test]
    fn test_parse_constraint_inout() {
        let constraint = Constraint::parse("+r").unwrap();
        assert_eq!(constraint.constraint_type, ConstraintType::InOut);
        assert_eq!(constraint.constraint_chars, "r");
    }

    #[test]
    fn test_parse_constraint_early_clobber() {
        let constraint = Constraint::parse("=&r").unwrap();
        assert_eq!(constraint.constraint_type, ConstraintType::EarlyClobber);
        assert_eq!(constraint.constraint_chars, "r");
    }

    #[test]
    fn test_validate_x86_64() {
        let constraint = Constraint::parse("r").unwrap();
        assert!(validate_constraint(&constraint, &Type::Int, Arch::X86_64).unwrap());
        assert!(validate_constraint(&constraint, &Type::I64, Arch::X86_64).unwrap());
        assert!(!validate_constraint(&constraint, &Type::F64, Arch::X86_64).unwrap());

        let float_constraint = Constraint::parse("f").unwrap();
        assert!(validate_constraint(&float_constraint, &Type::F64, Arch::X86_64).unwrap());
    }
}

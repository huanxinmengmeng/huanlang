// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
// 类型检查功能已移至 sema 模块，此处保留兼容性

use crate::core::ast::*;
use crate::core::sema::{TypeInfer, SymbolTable, TypeError};

#[derive(Debug, Clone, PartialEq)]
pub enum TypeErrorCompat {
    Mismatch {
        expected: String,
        found: String,
    },
    Undefined {
        name: String,
    },
    InvalidOperation {
        message: String,
    },
}

pub struct TypeChecker {
    type_infer: TypeInfer,
    errors: Vec<TypeErrorCompat>,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            type_infer: TypeInfer::new(SymbolTable::new()),
            errors: Vec::new(),
        }
    }

    pub fn check_program(&mut self, program: &Program) -> Result<(), Vec<TypeErrorCompat>> {
        match self.type_infer.infer_program(program) {
            Ok(()) => Ok(()),
            Err(errs) => {
                self.errors = errs.into_iter().map(|e| match e {
                    TypeError::Mismatch { expected, found, .. } => TypeErrorCompat::Mismatch {
                        expected: format!("{:?}", expected),
                        found: format!("{:?}", found),
                    },
                    _ => TypeErrorCompat::InvalidOperation {
                        message: format!("{:?}", e),
                    },
                }).collect();
                Err(self.errors.clone())
            }
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_checker_new() {
        let checker = TypeChecker::new();
        assert!(checker.errors.is_empty());
    }
}

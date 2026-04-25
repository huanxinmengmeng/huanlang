// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::collections::HashMap;
use crate::core::lexer::token::SourceSpan;

/// 变量标识符
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VarId(pub u64);

/// 借用状态
#[derive(Debug, Clone)]
pub enum LoanState {
    /// 未被借用
    None,
    /// 被只读借用 n 次
    Shared(usize),
    /// 被可变借用
    Mutable,
}

/// 借用检查器
pub struct BorrowChecker {
    /// 变量唯一标识符 → 借用状态
    loans: HashMap<VarId, LoanState>,
    /// 当前作用域深度
    scope_depth: usize,
    /// 作用域栈（用于追踪变量离开作用域）
    scope_vars: Vec<Vec<VarId>>,
}

/// 借用检查错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum BorrowError {
    UseAfterMove { span: SourceSpan },
    CannotBorrowSharedWhileMutable { span: SourceSpan },
    CannotBorrowMutableWhileShared { span: SourceSpan },
    AlreadyMutableBorrowed { span: SourceSpan },
    CannotMoveWhileBorrowed { span: SourceSpan },
}

impl BorrowChecker {
    pub fn new() -> Self {
        Self {
            loans: HashMap::new(),
            scope_depth: 0,
            scope_vars: vec![Vec::new()],
        }
    }

    /// 进入新作用域
    pub fn enter_scope(&mut self) {
        self.scope_depth += 1;
        self.scope_vars.push(Vec::new());
    }

    /// 离开作用域，释放该作用域内声明的变量
    pub fn exit_scope(&mut self) {
        for var_id in self.scope_vars.pop().unwrap_or_default() {
            self.loans.remove(&var_id);
        }
        self.scope_depth -= 1;
    }

    /// 声明新变量
    pub fn declare_var(&mut self, var_id: VarId) {
        self.loans.insert(var_id, LoanState::None);
        self.scope_vars.last_mut().unwrap().push(var_id);
    }

    /// 检查只读借用
    pub fn check_shared_borrow(&mut self, var_id: VarId, span: SourceSpan) -> Result<(), BorrowError> {
        match self.loans.get(&var_id) {
            Some(LoanState::Mutable) => Err(BorrowError::CannotBorrowSharedWhileMutable { span }),
            Some(LoanState::Shared(n)) => {
                self.loans.insert(var_id, LoanState::Shared(n + 1));
                Ok(())
            }
            Some(LoanState::None) => {
                self.loans.insert(var_id, LoanState::Shared(1));
                Ok(())
            }
            None => Err(BorrowError::UseAfterMove { span }),
        }
    }

    /// 检查可变借用
    pub fn check_mutable_borrow(&mut self, var_id: VarId, span: SourceSpan) -> Result<(), BorrowError> {
        match self.loans.get(&var_id) {
            Some(LoanState::Mutable) => Err(BorrowError::AlreadyMutableBorrowed { span }),
            Some(LoanState::Shared(_)) => Err(BorrowError::CannotBorrowMutableWhileShared { span }),
            Some(LoanState::None) => {
                self.loans.insert(var_id, LoanState::Mutable);
                Ok(())
            }
            None => Err(BorrowError::UseAfterMove { span }),
        }
    }

    /// 释放借用（当引用离开作用域时调用）
    pub fn release_borrow(&mut self, var_id: VarId, mutable: bool) {
        if let Some(state) = self.loans.get_mut(&var_id) {
            match state {
                LoanState::Shared(n) if !mutable && *n > 1 => {
                    *state = LoanState::Shared(*n - 1);
                }
                LoanState::Shared(_) | LoanState::Mutable => {
                    *state = LoanState::None;
                }
                _ => {}
            }
        }
    }

    /// 检查变量移动
    pub fn check_move(&mut self, var_id: VarId, span: SourceSpan) -> Result<(), BorrowError> {
        match self.loans.get(&var_id) {
            Some(LoanState::Shared(_)) => Err(BorrowError::CannotMoveWhileBorrowed { span }),
            Some(LoanState::Mutable) => Err(BorrowError::CannotMoveWhileBorrowed { span }),
            Some(LoanState::None) => {
                self.loans.remove(&var_id);
                Ok(())
            }
            None => Err(BorrowError::UseAfterMove { span }),
        }
    }
}

impl Default for BorrowChecker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::lexer::token::SourceSpan;

    fn dummy_span() -> SourceSpan {
        SourceSpan::dummy()
    }

    #[test]
    fn test_borrow_checker_shared() {
        let mut checker = BorrowChecker::new();
        checker.enter_scope();
        checker.declare_var(VarId(0));

        assert!(checker.check_shared_borrow(VarId(0), dummy_span()).is_ok());
        assert!(checker.check_shared_borrow(VarId(0), dummy_span()).is_ok()); // 多次共享借用允许
        assert!(checker.check_mutable_borrow(VarId(0), dummy_span()).is_err()); // 共享借用时不能可变借用
    }

    #[test]
    fn test_borrow_checker_mutable() {
        let mut checker = BorrowChecker::new();
        checker.enter_scope();
        checker.declare_var(VarId(0));

        assert!(checker.check_mutable_borrow(VarId(0), dummy_span()).is_ok());
        assert!(checker.check_mutable_borrow(VarId(0), dummy_span()).is_err()); // 不能重复可变借用
        assert!(checker.check_shared_borrow(VarId(0), dummy_span()).is_err()); // 可变借用时不能共享借用
    }

    #[test]
    fn test_borrow_checker_move() {
        let mut checker = BorrowChecker::new();
        checker.enter_scope();
        checker.declare_var(VarId(0));

        assert!(checker.check_move(VarId(0), dummy_span()).is_ok());
        assert!(checker.check_move(VarId(0), dummy_span()).is_err()); // 不能移动已移动的变量
    }

    #[test]
    fn test_borrow_checker_scope() {
        let mut checker = BorrowChecker::new();
        checker.enter_scope();
        checker.declare_var(VarId(0));
        checker.enter_scope();
        checker.declare_var(VarId(1));
        
        assert!(checker.loans.contains_key(&VarId(0)));
        assert!(checker.loans.contains_key(&VarId(1)));
        
        checker.exit_scope(); // 离开内部作用域，释放 VarId(1)
        assert!(checker.loans.contains_key(&VarId(0)));
        assert!(!checker.loans.contains_key(&VarId(1)));
        
        checker.exit_scope(); // 离开外部作用域，释放 VarId(0)
        assert!(!checker.loans.contains_key(&VarId(0)));
    }
}

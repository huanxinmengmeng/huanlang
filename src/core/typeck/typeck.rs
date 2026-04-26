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

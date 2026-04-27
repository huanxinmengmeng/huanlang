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
//
// LLVM 代码生成完整测试套件
// 覆盖常见代码模式、边界条件及特殊语法结构

#[cfg(test)]
mod tests {
    use crate::core::ast::*;
    use crate::core::ast::BinaryOp::*;
    use crate::core::ast::UnaryOp::*;
    use crate::core::lexer::token::SourceSpan;
    use crate::core::backend::llvm::ast_to_llvm::{AstToLlvmCodeGen, validate_llvm_ir};
    use crate::core::backend::error::CodeGenError;

    fn dummy_span() -> SourceSpan {
        SourceSpan::dummy()
    }

    fn create_program(items: Vec<Item>) -> Program {
        items
    }

    // =============================================================================
    // 基础类型测试
    // =============================================================================

    #[test]
    fn test_int_literal_generation() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::IntLit(42, dummy_span()))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("define"));
        assert!(ir.contains("ret"));
    }

    #[test]
    fn test_float_literal_generation() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::F64,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::FloatLit(3.14159, dummy_span()))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("double") || ir.contains("3.14159"));
    }

    #[test]
    fn test_string_literal_generation() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Expr(
                        Box::new(Expr::Call {
                            func: Box::new(Expr::Ident(Ident::dummy("print"))),
                            args: vec![Expr::StringLit("Hello".to_string(), dummy_span())],
                            span: dummy_span(),
                        }),
                        dummy_span(),
                    ),
                    Stmt::Return(Some(Box::new(Expr::IntLit(0, dummy_span()))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_bool_literal_generation() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Bool,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BoolLit(true, dummy_span()))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    // =============================================================================
    // 算术运算测试
    // =============================================================================

    #[test]
    fn test_addition_generation() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Add,
                        left: Box::new(Expr::IntLit(10, dummy_span())),
                        right: Box::new(Expr::IntLit(20, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("add"));
    }

    #[test]
    fn test_subtraction_generation() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Sub,
                        left: Box::new(Expr::IntLit(100, dummy_span())),
                        right: Box::new(Expr::IntLit(30, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("sub"));
    }

    #[test]
    fn test_multiplication_generation() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Mul,
                        left: Box::new(Expr::IntLit(6, dummy_span())),
                        right: Box::new(Expr::IntLit(7, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("mul"));
    }

    #[test]
    fn test_division_generation() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Div,
                        left: Box::new(Expr::IntLit(100, dummy_span())),
                        right: Box::new(Expr::IntLit(4, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("sdiv"));
    }

    #[test]
    fn test_modulo_generation() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Mod,
                        left: Box::new(Expr::IntLit(17, dummy_span())),
                        right: Box::new(Expr::IntLit(5, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("srem"));
    }

    // =============================================================================
    // 比较运算测试
    // =============================================================================

    #[test]
    fn test_equality_comparison() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Bool,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Eq,
                        left: Box::new(Expr::IntLit(10, dummy_span())),
                        right: Box::new(Expr::IntLit(10, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("icmp eq"));
    }

    #[test]
    fn test_inequality_comparison() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Bool,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Ne,
                        left: Box::new(Expr::IntLit(10, dummy_span())),
                        right: Box::new(Expr::IntLit(20, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("icmp ne"));
    }

    #[test]
    fn test_greater_than_comparison() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Bool,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Gt,
                        left: Box::new(Expr::IntLit(20, dummy_span())),
                        right: Box::new(Expr::IntLit(10, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("icmp sgt"));
    }

    #[test]
    fn test_less_than_comparison() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Bool,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Lt,
                        left: Box::new(Expr::IntLit(10, dummy_span())),
                        right: Box::new(Expr::IntLit(20, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("icmp slt"));
    }

    #[test]
    fn test_greater_than_or_equal_comparison() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Bool,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Ge,
                        left: Box::new(Expr::IntLit(10, dummy_span())),
                        right: Box::new(Expr::IntLit(10, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("icmp sge"));
    }

    #[test]
    fn test_less_than_or_equal_comparison() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Bool,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Le,
                        left: Box::new(Expr::IntLit(10, dummy_span())),
                        right: Box::new(Expr::IntLit(10, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("icmp sle"));
    }

    // =============================================================================
    // 逻辑运算测试
    // =============================================================================

    #[test]
    fn test_logical_and() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Bool,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: And,
                        left: Box::new(Expr::BoolLit(true, dummy_span())),
                        right: Box::new(Expr::BoolLit(true, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("and"));
    }

    #[test]
    fn test_logical_or() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Bool,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Or,
                        left: Box::new(Expr::BoolLit(false, dummy_span())),
                        right: Box::new(Expr::BoolLit(true, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("or"));
    }

    #[test]
    fn test_logical_not() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Bool,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::UnaryOp {
                        op: Not,
                        expr: Box::new(Expr::BoolLit(true, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("xor"));
    }

    #[test]
    fn test_negation() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::UnaryOp {
                        op: Neg,
                        expr: Box::new(Expr::IntLit(42, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("sub"));
    }

    // =============================================================================
    // 变量声明与赋值测试
    // =============================================================================

    #[test]
    fn test_variable_declaration() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Let {
                        name: Ident::dummy("x"),
                        ty: Some(Type::Int),
                        value: Box::new(Expr::IntLit(10, dummy_span())),
                        span: dummy_span(),
                    },
                    Stmt::Return(Some(Box::new(Expr::Ident(Ident::dummy("x")))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("alloca"));
    }

    #[test]
    fn test_variable_assignment() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Let {
                        name: Ident::dummy("x"),
                        ty: Some(Type::Int),
                        value: Box::new(Expr::IntLit(10, dummy_span())),
                        span: dummy_span(),
                    },
                    Stmt::Assign {
                        target: Box::new(Expr::Ident(Ident::dummy("x"))),
                        value: Box::new(Expr::IntLit(20, dummy_span())),
                        span: dummy_span(),
                    },
                    Stmt::Return(Some(Box::new(Expr::Ident(Ident::dummy("x")))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    // =============================================================================
    // 控制流测试
    // =============================================================================

    #[test]
    fn test_if_statement() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::If {
                        cond: Box::new(Expr::BoolLit(true, dummy_span())),
                        then_block: vec![
                            Stmt::Return(Some(Box::new(Expr::IntLit(1, dummy_span()))), dummy_span()),
                        ],
                        else_ifs: vec![],
                        else_block: Some(vec![
                            Stmt::Return(Some(Box::new(Expr::IntLit(0, dummy_span()))), dummy_span()),
                        ]),
                        span: dummy_span(),
                    },
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("br"));
    }

    #[test]
    fn test_while_loop() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::While {
                        cond: Box::new(Expr::BoolLit(false, dummy_span())),
                        body: vec![
                            Stmt::Return(Some(Box::new(Expr::IntLit(1, dummy_span()))), dummy_span()),
                        ],
                        span: dummy_span(),
                    },
                    Stmt::Return(Some(Box::new(Expr::IntLit(0, dummy_span()))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_repeat_loop() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Repeat {
                        count: Box::new(Expr::IntLit(5, dummy_span())),
                        body: vec![],
                        span: dummy_span(),
                    },
                    Stmt::Return(Some(Box::new(Expr::IntLit(0, dummy_span()))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    // =============================================================================
    // 函数调用测试
    // =============================================================================

    #[test]
    fn test_function_call() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("print"),
                generics: vec![],
                params: vec![(Ident::dummy("msg"), Type::String)],
                return_type: Type::Unit,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![],
                span: dummy_span(),
            }),
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Expr(
                        Box::new(Expr::Call {
                            func: Box::new(Expr::Ident(Ident::dummy("print"))),
                            args: vec![Expr::StringLit("Hello".to_string(), dummy_span())],
                            span: dummy_span(),
                        }),
                        dummy_span(),
                    ),
                    Stmt::Return(Some(Box::new(Expr::IntLit(0, dummy_span()))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    // =============================================================================
    // 嵌套表达式测试
    // =============================================================================

    #[test]
    fn test_nested_binary_operations() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Add,
                        left: Box::new(Expr::BinaryOp {
                            op: Mul,
                            left: Box::new(Expr::IntLit(2, dummy_span())),
                            right: Box::new(Expr::IntLit(3, dummy_span())),
                            span: dummy_span(),
                        }),
                        right: Box::new(Expr::BinaryOp {
                            op: Sub,
                            left: Box::new(Expr::IntLit(10, dummy_span())),
                            right: Box::new(Expr::IntLit(4, dummy_span())),
                            span: dummy_span(),
                        }),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("mul"));
        assert!(ir.contains("sub"));
        assert!(ir.contains("add"));
    }

    #[test]
    fn test_complex_expression() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Gt,
                        left: Box::new(Expr::BinaryOp {
                            op: Add,
                            left: Box::new(Expr::IntLit(5, dummy_span())),
                            right: Box::new(Expr::IntLit(3, dummy_span())),
                            span: dummy_span(),
                        }),
                        right: Box::new(Expr::IntLit(7, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    // =============================================================================
    // 边界条件测试
    // =============================================================================

    #[test]
    fn test_zero_division() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Div,
                        left: Box::new(Expr::IntLit(10, dummy_span())),
                        right: Box::new(Expr::IntLit(0, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_max_int_value() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::IntLit(i64::MAX, dummy_span()))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_min_int_value() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::IntLit(i64::MIN, dummy_span()))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_negative_numbers() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Add,
                        left: Box::new(Expr::IntLit(-100, dummy_span())),
                        right: Box::new(Expr::IntLit(-50, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    // =============================================================================
    // 多函数定义测试
    // =============================================================================

    #[test]
    fn test_multiple_functions() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("add"),
                generics: vec![],
                params: vec![(Ident::dummy("a"), Type::Int), (Ident::dummy("b"), Type::Int)],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Add,
                        left: Box::new(Expr::Ident(Ident::dummy("a"))),
                        right: Box::new(Expr::Ident(Ident::dummy("b"))),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::IntLit(0, dummy_span()))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
        let ir = result.unwrap();
        assert!(ir.contains("@add"));
        assert!(ir.contains("@main"));
    }

    // =============================================================================
    // 类型转换测试
    // =============================================================================

    #[test]
    fn test_type_to_llvm_i32() {
        let ty = Type::I32;
        let llvm_ty = AstToLlvmCodeGen::type_to_llvm(&ty);
        assert_eq!(llvm_ty, "i32");
    }

    #[test]
    fn test_type_to_llvm_i64() {
        let ty = Type::I64;
        let llvm_ty = AstToLlvmCodeGen::type_to_llvm(&ty);
        assert_eq!(llvm_ty, "i64");
    }

    #[test]
    fn test_type_to_llvm_f32() {
        let ty = Type::F32;
        let llvm_ty = AstToLlvmCodeGen::type_to_llvm(&ty);
        assert_eq!(llvm_ty, "float");
    }

    #[test]
    fn test_type_to_llvm_f64() {
        let ty = Type::F64;
        let llvm_ty = AstToLlvmCodeGen::type_to_llvm(&ty);
        assert_eq!(llvm_ty, "double");
    }

    #[test]
    fn test_type_to_llvm_bool() {
        let ty = Type::Bool;
        let llvm_ty = AstToLlvmCodeGen::type_to_llvm(&ty);
        assert_eq!(llvm_ty, "i1");
    }

    #[test]
    fn test_type_to_llvm_string() {
        let ty = Type::String;
        let llvm_ty = AstToLlvmCodeGen::type_to_llvm(&ty);
        assert_eq!(llvm_ty, "i8*");
    }

    #[test]
    fn test_type_to_llvm_unit() {
        let ty = Type::Unit;
        let llvm_ty = AstToLlvmCodeGen::type_to_llvm(&ty);
        assert_eq!(llvm_ty, "void");
    }

    // =============================================================================
    // LLVM IR 验证测试
    // =============================================================================

    #[test]
    fn test_validate_llvm_ir_valid() {
        let valid_ir = r#"
; ModuleID = 'test_module'
source_filename = "test"
define i32 @main() {
ret i32 0
}
"#;
        let result = validate_llvm_ir(valid_ir);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_llvm_ir_empty() {
        let result = validate_llvm_ir("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_llvm_ir_missing_function() {
        let invalid_ir = r#"
; ModuleID = 'test_module'
source_filename = "test"
"%1 = add i32 1, 2
"#;
        let result = validate_llvm_ir(invalid_ir);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_llvm_ir_mismatched_braces() {
        let invalid_ir = r#"
define i32 @main() {
ret i32 0
"#;
        let result = validate_llvm_ir(invalid_ir);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_llvm_ir_multiple_functions() {
        let valid_ir = r#"
; ModuleID = 'test_module'
source_filename = "test"
define i32 @add(i32 %a, i32 %b) {
%result = add i32 %a, %b
ret i32 %result
}
define i32 @main() {
ret i32 0
}
"#;
        let result = validate_llvm_ir(valid_ir);
        assert!(result.is_ok());
    }

    #[test]
    fn test_generate_with_multiple_basic_blocks() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::If {
                        cond: Box::new(Expr::BoolLit(true, dummy_span())),
                        then_block: vec![
                            Stmt::Return(Some(Box::new(Expr::IntLit(1, dummy_span()))), dummy_span()),
                        ],
                        else_ifs: vec![],
                        else_block: None,
                        span: dummy_span(),
                    },
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    // =============================================================================
    // 列表和字典字面量测试
    // =============================================================================

    #[test]
    fn test_list_literal() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::List(Box::new(Type::Int)),
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::List(
                        vec![
                            Expr::IntLit(1, dummy_span()),
                            Expr::IntLit(2, dummy_span()),
                            Expr::IntLit(3, dummy_span()),
                        ],
                        dummy_span(),
                    ))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_map_literal() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Map(Box::new(Type::String), Box::new(Type::Int)),
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::Map(
                        vec![
                            (
                                Expr::StringLit("a".to_string(), dummy_span()),
                                Expr::IntLit(1, dummy_span()),
                            ),
                            (
                                Expr::StringLit("b".to_string(), dummy_span()),
                                Expr::IntLit(2, dummy_span()),
                            ),
                        ],
                        dummy_span(),
                    ))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    // =============================================================================
    // 空值和可选类型测试
    // =============================================================================

    #[test]
    fn test_null_literal() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Unit,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(None, dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    // =============================================================================
    // 字符字面量测试
    // =============================================================================

    #[test]
    fn test_char_literal() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Char,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::CharLit('A', dummy_span()))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    // =============================================================================
    // 额外边界条件测试
    // =============================================================================

    #[test]
    fn test_overflow_scenario() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Add,
                        left: Box::new(Expr::IntLit(i64::MAX, dummy_span())),
                        right: Box::new(Expr::IntLit(1, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_underflow_scenario() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Sub,
                        left: Box::new(Expr::IntLit(i64::MIN, dummy_span())),
                        right: Box::new(Expr::IntLit(1, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_nested_conditionals() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::If {
                        cond: Box::new(Expr::BinaryOp {
                            op: Gt,
                            left: Box::new(Expr::IntLit(10, dummy_span())),
                            right: Box::new(Expr::IntLit(5, dummy_span())),
                            span: dummy_span(),
                        }),
                        then_block: vec![
                            Stmt::If {
                                cond: Box::new(Expr::BinaryOp {
                                    op: Lt,
                                    left: Box::new(Expr::IntLit(5, dummy_span())),
                                    right: Box::new(Expr::IntLit(10, dummy_span())),
                                    span: dummy_span(),
                                }),
                                then_block: vec![
                                    Stmt::Return(Some(Box::new(Expr::IntLit(1, dummy_span()))), dummy_span()),
                                ],
                                else_ifs: vec![],
                                else_block: None,
                                span: dummy_span(),
                            },
                        ],
                        else_ifs: vec![],
                        else_block: Some(vec![
                            Stmt::Return(Some(Box::new(Expr::IntLit(0, dummy_span()))), dummy_span()),
                        ]),
                        span: dummy_span(),
                    },
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_function_body() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Unit,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_variable_declarations() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Let {
                        name: Ident::dummy("a"),
                        ty: Some(Type::Int),
                        value: Box::new(Expr::IntLit(10, dummy_span())),
                        span: dummy_span(),
                    },
                    Stmt::Let {
                        name: Ident::dummy("b"),
                        ty: Some(Type::Int),
                        value: Box::new(Expr::IntLit(20, dummy_span())),
                        span: dummy_span(),
                    },
                    Stmt::Let {
                        name: Ident::dummy("c"),
                        ty: Some(Type::Int),
                        value: Box::new(Expr::IntLit(30, dummy_span())),
                        span: dummy_span(),
                    },
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Add,
                        left: Box::new(Expr::Ident(Ident::dummy("a"))),
                        right: Box::new(Expr::BinaryOp {
                            op: Add,
                            left: Box::new(Expr::Ident(Ident::dummy("b"))),
                            right: Box::new(Expr::Ident(Ident::dummy("c"))),
                            span: dummy_span(),
                        }),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_float_precision() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::F64,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: Div,
                        left: Box::new(Expr::FloatLit(22.0, dummy_span())),
                        right: Box::new(Expr::FloatLit(7.0, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_bitwise_operations() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: BitAnd,
                        left: Box::new(Expr::IntLit(0b1100, dummy_span())),
                        right: Box::new(Expr::IntLit(0b1010, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_ternary_like_expression() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::If {
                        cond: Box::new(Expr::BoolLit(true, dummy_span())),
                        then_block: vec![
                            Stmt::Return(Some(Box::new(Expr::IntLit(1, dummy_span()))), dummy_span()),
                        ],
                        else_ifs: vec![],
                        else_block: Some(vec![
                            Stmt::Return(Some(Box::new(Expr::IntLit(0, dummy_span()))), dummy_span()),
                        ]),
                        span: dummy_span(),
                    },
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn test_multiple_returns() {
        let program = create_program(vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("main"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::If {
                        cond: Box::new(Expr::BoolLit(true, dummy_span())),
                        then_block: vec![
                            Stmt::Return(Some(Box::new(Expr::IntLit(10, dummy_span()))), dummy_span()),
                        ],
                        else_ifs: vec![],
                        else_block: None,
                        span: dummy_span(),
                    },
                    Stmt::Return(Some(Box::new(Expr::IntLit(0, dummy_span()))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }
}
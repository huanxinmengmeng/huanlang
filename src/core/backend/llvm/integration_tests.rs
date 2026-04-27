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
// LLVM 代码生成端到端集成测试
// 验证从 AST 到代码生成的完整流程

#[cfg(test)]
mod integration_tests {
    use crate::core::backend::llvm::ast_to_llvm::AstToLlvmCodeGen;
    use crate::core::ast::*;
    use crate::core::lexer::token::SourceSpan;

    fn dummy_span() -> SourceSpan {
        SourceSpan::dummy()
    }

    fn create_function(return_expr: Expr) -> Program {
        vec![
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
                    Stmt::Return(Some(Box::new(return_expr)), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ]
    }

    #[test]
    fn test_simple_integer_program() {
        let program = create_function(Expr::IntLit(42, dummy_span()));
        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok(), "编译失败: {:?}", result.err());
        let ir = result.unwrap();
        assert!(ir.contains("define"));
        assert!(ir.contains("ret"));
    }

    #[test]
    fn test_arithmetic_expression() {
        let program = create_function(Expr::BinaryOp {
            op: BinaryOp::Add,
            left: Box::new(Expr::IntLit(10, dummy_span())),
            right: Box::new(Expr::BinaryOp {
                op: BinaryOp::Mul,
                left: Box::new(Expr::IntLit(20, dummy_span())),
                right: Box::new(Expr::IntLit(3, dummy_span())),
                span: dummy_span(),
            }),
            span: dummy_span(),
        });

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok(), "编译失败: {:?}", result.err());
    }

    #[test]
    fn test_variable_with_assignment() {
        let program = vec![
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
                        value: Box::new(Expr::BinaryOp {
                            op: BinaryOp::Add,
                            left: Box::new(Expr::Ident(Ident::dummy("x"))),
                            right: Box::new(Expr::IntLit(5, dummy_span())),
                            span: dummy_span(),
                        }),
                        span: dummy_span(),
                    },
                    Stmt::Return(Some(Box::new(Expr::Ident(Ident::dummy("x")))), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ];

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok(), "编译失败: {:?}", result.err());
    }

    #[test]
    fn test_if_else_branch() {
        let program = vec![
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
                            op: BinaryOp::Gt,
                            left: Box::new(Expr::IntLit(5, dummy_span())),
                            right: Box::new(Expr::IntLit(3, dummy_span())),
                            span: dummy_span(),
                        }),
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
        ];

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok(), "编译失败: {:?}", result.err());
    }

    #[test]
    fn test_function_with_params() {
        let program = vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("add"),
                generics: vec![],
                params: vec![
                    (Ident::dummy("a"), Type::Int),
                    (Ident::dummy("b"), Type::Int),
                ],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![
                    Stmt::Return(Some(Box::new(Expr::BinaryOp {
                        op: BinaryOp::Add,
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
        ];

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok(), "编译失败: {:?}", result.err());
    }

    #[test]
    fn test_nested_expressions() {
        let program = create_function(Expr::BinaryOp {
            op: BinaryOp::Div,
            left: Box::new(Expr::BinaryOp {
                op: BinaryOp::Mul,
                left: Box::new(Expr::BinaryOp {
                    op: BinaryOp::Add,
                    left: Box::new(Expr::IntLit(10, dummy_span())),
                    right: Box::new(Expr::IntLit(20, dummy_span())),
                    span: dummy_span(),
                }),
                right: Box::new(Expr::BinaryOp {
                    op: BinaryOp::Sub,
                    left: Box::new(Expr::IntLit(30, dummy_span())),
                    right: Box::new(Expr::IntLit(15, dummy_span())),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            }),
            right: Box::new(Expr::IntLit(5, dummy_span())),
            span: dummy_span(),
        });

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok(), "编译失败: {:?}", result.err());
    }

    #[test]
    fn test_boolean_operations() {
        let program = vec![
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
                        op: BinaryOp::And,
                        left: Box::new(Expr::BoolLit(true, dummy_span())),
                        right: Box::new(Expr::BoolLit(false, dummy_span())),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ];

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok(), "编译失败: {:?}", result.err());
    }

    #[test]
    fn test_comparison_chain() {
        let program = vec![
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
                        op: BinaryOp::And,
                        left: Box::new(Expr::BinaryOp {
                            op: BinaryOp::Lt,
                            left: Box::new(Expr::IntLit(10, dummy_span())),
                            right: Box::new(Expr::IntLit(20, dummy_span())),
                            span: dummy_span(),
                        }),
                        right: Box::new(Expr::BinaryOp {
                            op: BinaryOp::Lt,
                            left: Box::new(Expr::IntLit(20, dummy_span())),
                            right: Box::new(Expr::IntLit(30, dummy_span())),
                            span: dummy_span(),
                        }),
                        span: dummy_span(),
                    })), dummy_span()),
                ],
                span: dummy_span(),
            }),
        ];

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok(), "编译失败: {:?}", result.err());
    }
}

#[cfg(test)]
mod regression_tests {
    use crate::core::backend::llvm::ast_to_llvm::AstToLlvmCodeGen;
    use crate::core::ast::*;
    use crate::core::lexer::token::SourceSpan;

    fn dummy_span() -> SourceSpan {
        SourceSpan::dummy()
    }

    fn create_simple_function(name: &str, body: Vec<Stmt>) -> Program {
        vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy(name),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body,
                span: dummy_span(),
            }),
        ]
    }

    #[test]
    fn regression_test_addition() {
        let program = create_simple_function("test_add", vec![
            Stmt::Return(Some(Box::new(Expr::BinaryOp {
                op: BinaryOp::Add,
                left: Box::new(Expr::IntLit(1, dummy_span())),
                right: Box::new(Expr::IntLit(2, dummy_span())),
                span: dummy_span(),
            })), dummy_span()),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn regression_test_subtraction() {
        let program = create_simple_function("test_sub", vec![
            Stmt::Return(Some(Box::new(Expr::BinaryOp {
                op: BinaryOp::Sub,
                left: Box::new(Expr::IntLit(5, dummy_span())),
                right: Box::new(Expr::IntLit(3, dummy_span())),
                span: dummy_span(),
            })), dummy_span()),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn regression_test_multiplication() {
        let program = create_simple_function("test_mul", vec![
            Stmt::Return(Some(Box::new(Expr::BinaryOp {
                op: BinaryOp::Mul,
                left: Box::new(Expr::IntLit(6, dummy_span())),
                right: Box::new(Expr::IntLit(7, dummy_span())),
                span: dummy_span(),
            })), dummy_span()),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn regression_test_division() {
        let program = create_simple_function("test_div", vec![
            Stmt::Return(Some(Box::new(Expr::BinaryOp {
                op: BinaryOp::Div,
                left: Box::new(Expr::IntLit(100, dummy_span())),
                right: Box::new(Expr::IntLit(4, dummy_span())),
                span: dummy_span(),
            })), dummy_span()),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn regression_test_modulo() {
        let program = create_simple_function("test_mod", vec![
            Stmt::Return(Some(Box::new(Expr::BinaryOp {
                op: BinaryOp::Mod,
                left: Box::new(Expr::IntLit(17, dummy_span())),
                right: Box::new(Expr::IntLit(5, dummy_span())),
                span: dummy_span(),
            })), dummy_span()),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn regression_test_comparison_eq() {
        let program = create_simple_function("test_eq", vec![
            Stmt::Return(Some(Box::new(Expr::BinaryOp {
                op: BinaryOp::Eq,
                left: Box::new(Expr::IntLit(10, dummy_span())),
                right: Box::new(Expr::IntLit(10, dummy_span())),
                span: dummy_span(),
            })), dummy_span()),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn regression_test_comparison_ne() {
        let program = create_simple_function("test_ne", vec![
            Stmt::Return(Some(Box::new(Expr::BinaryOp {
                op: BinaryOp::Ne,
                left: Box::new(Expr::IntLit(10, dummy_span())),
                right: Box::new(Expr::IntLit(20, dummy_span())),
                span: dummy_span(),
            })), dummy_span()),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn regression_test_logical_and() {
        let program = create_simple_function("test_and", vec![
            Stmt::Return(Some(Box::new(Expr::BinaryOp {
                op: BinaryOp::And,
                left: Box::new(Expr::BoolLit(true, dummy_span())),
                right: Box::new(Expr::BoolLit(true, dummy_span())),
                span: dummy_span(),
            })), dummy_span()),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn regression_test_logical_or() {
        let program = create_simple_function("test_or", vec![
            Stmt::Return(Some(Box::new(Expr::BinaryOp {
                op: BinaryOp::Or,
                left: Box::new(Expr::BoolLit(false, dummy_span())),
                right: Box::new(Expr::BoolLit(true, dummy_span())),
                span: dummy_span(),
            })), dummy_span()),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn regression_test_unary_neg() {
        let program = create_simple_function("test_neg", vec![
            Stmt::Return(Some(Box::new(Expr::UnaryOp {
                op: UnaryOp::Neg,
                expr: Box::new(Expr::IntLit(42, dummy_span())),
                span: dummy_span(),
            })), dummy_span()),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn regression_test_unary_not() {
        let program = create_simple_function("test_not", vec![
            Stmt::Return(Some(Box::new(Expr::UnaryOp {
                op: UnaryOp::Not,
                expr: Box::new(Expr::BoolLit(true, dummy_span())),
                span: dummy_span(),
            })), dummy_span()),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }

    #[test]
    fn regression_test_nested_binary_ops() {
        let program = create_simple_function("test_nested", vec![
            Stmt::Return(Some(Box::new(Expr::BinaryOp {
                op: BinaryOp::Add,
                left: Box::new(Expr::BinaryOp {
                    op: BinaryOp::Mul,
                    left: Box::new(Expr::IntLit(2, dummy_span())),
                    right: Box::new(Expr::IntLit(3, dummy_span())),
                    span: dummy_span(),
                }),
                right: Box::new(Expr::BinaryOp {
                    op: BinaryOp::Sub,
                    left: Box::new(Expr::IntLit(10, dummy_span())),
                    right: Box::new(Expr::IntLit(4, dummy_span())),
                    span: dummy_span(),
                }),
                span: dummy_span(),
            })), dummy_span()),
        ]);

        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod performance_tests {
    use crate::core::backend::llvm::ast_to_llvm::AstToLlvmCodeGen;
    use crate::core::ast::*;
    use crate::core::lexer::token::SourceSpan;
    use std::time::Instant;

    fn dummy_span() -> SourceSpan {
        SourceSpan::dummy()
    }

    fn create_many_variables_test(num_vars: usize) -> Program {
        let mut body = Vec::new();
        for i in 0..num_vars {
            body.push(Stmt::Let {
                name: Ident::dummy(&format!("x{}", i)),
                ty: Some(Type::Int),
                value: Box::new(Expr::IntLit(i as i64, dummy_span())),
                span: dummy_span(),
            });
        }
        body.push(Stmt::Return(Some(Box::new(Expr::IntLit(num_vars as i64, dummy_span()))), dummy_span()));

        vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("many_vars"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body,
                span: dummy_span(),
            }),
        ]
    }

    fn create_complex_expression_test(depth: usize) -> Program {
        fn build_nested_expr(depth: usize) -> Expr {
            if depth == 0 {
                Expr::IntLit(1, dummy_span())
            } else {
                Expr::BinaryOp {
                    op: BinaryOp::Add,
                    left: Box::new(Expr::IntLit(1, dummy_span())),
                    right: Box::new(build_nested_expr(depth - 1)),
                    span: dummy_span(),
                }
            }
        }

        vec![
            Item::Function(Function {
                public: false,
                name: Ident::dummy("nested_expr"),
                generics: vec![],
                params: vec![],
                return_type: Type::Int,
                where_clause: vec![],
                preconditions: vec![],
                postconditions: vec![],
                body: vec![Stmt::Return(Some(Box::new(build_nested_expr(depth))), dummy_span())],
                span: dummy_span(),
            }),
        ]
    }

    #[test]
    fn test_many_variables_performance() {
        let program = create_many_variables_test(100);

        let start = Instant::now();
        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        let elapsed = start.elapsed();

        assert!(result.is_ok());
        println!("生成 100 个变量的 IR 耗时: {:?}", elapsed);
    }

    #[test]
    fn test_nested_expression_performance() {
        let program = create_complex_expression_test(50);

        let start = Instant::now();
        let mut codegen = AstToLlvmCodeGen::new();
        let result = codegen.generate_program(&program, "x86_64-unknown-linux-gnu");
        let elapsed = start.elapsed();

        assert!(result.is_ok());
        println!("生成深度为 50 的嵌套表达式 IR 耗时: {:?}", elapsed);
    }
}
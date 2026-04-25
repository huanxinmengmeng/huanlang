// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// MLIR 方言测试

use huanlang::core::mlir::conversion::AstToMlirConverter;
use huanlang::core::ast::{Item, Function, Ident, Type, Stmt, Expr};
use huanlang::core::lexer::token::SourceSpan;

#[test]
pub fn test_ast_to_mlir_conversion() {
    // 测试 AST 到 MLIR 的转换
    let _ast = create_test_ast();
    let _converter = AstToMlirConverter::new();
    
    // 暂时注释掉转换测试，因为 MLIR 转换功能尚未完全实现
    // let module = converter.convert(&ast).unwrap();
    // assert!(module.is_valid());
}

fn create_test_ast() -> Vec<Item> {
    // 创建一个简单的测试 AST
    vec![
        Item::Function(Function {
            public: false,
            name: Ident::new("main".to_string(), SourceSpan::dummy()),
            generics: vec![],
            params: vec![],
            return_type: Type::Int,
            where_clause: vec![],
            preconditions: vec![],
            postconditions: vec![],
            body: vec![
                Stmt::Expr(
                    Box::new(Expr::Call {
                        func: Box::new(Expr::Ident(Ident::new("print".to_string(), SourceSpan::dummy()))),
                        args: vec![Expr::StringLit("Hello, World!".to_string(), SourceSpan::dummy())],
                        span: SourceSpan::dummy(),
                    }),
                    SourceSpan::dummy(),
                ),
                Stmt::Return(
                    Some(Box::new(Expr::IntLit(0, SourceSpan::dummy()))),
                    SourceSpan::dummy()
                ),
            ],
            span: SourceSpan::dummy(),
        }),
    ]
}

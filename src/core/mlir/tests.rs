// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
//
// MLIR 完整集成测试

#[cfg(test)]
mod tests {
    use crate::core::mlir::*;
    use crate::core::mlir::ops::*;
    use crate::core::mlir::types::*;
    use crate::core::mlir::passes::*;
    use crate::core::mlir::lowering::*;
    use crate::core::lexer::token::SourceSpan;

    #[test]
    fn test_dialect_definition() {
        let _dialect = HuanDialect;
        assert_eq!(HuanDialect::NAME, "huan");
        assert_eq!(HuanDialect::SUMMARY, "幻语（HuanLang）高级编程语言方言");
    }

    #[test]
    fn test_basic_types() {
        let int_ty = IntType;
        let i32_ty = I32Type;
        let f64_ty = F64Type;
        let bool_ty = BoolType;
        
        assert_eq!(int_ty.mnemonic(), "int");
        assert_eq!(i32_ty.mnemonic(), "i32");
        assert_eq!(f64_ty.mnemonic(), "f64");
        assert_eq!(bool_ty.mnemonic(), "bool");
    }

    #[test]
    fn test_basic_ops() {
        let span = SourceSpan::dummy();
        
        // 创建整数常量
        let a = Box::new(IntLitOp { value: 2, span: span.clone() });
        let b = Box::new(IntLitOp { value: 3, span: span.clone() });
        
        // 创建加法操作
        let add = JiaOp {
            lhs: a,
            rhs: b,
            span: span.clone(),
        };
        
        assert_eq!(add.mnemonic(), "jia");
        assert_eq!(add.summary(), "加法");
    }

    #[test]
    fn test_pass_pipeline() {
        let span = SourceSpan::dummy();
        
        // 创建一些操作
        let a = Box::new(IntLitOp { value: 10, span: span.clone() });
        let b = Box::new(IntLitOp { value: 20, span: span.clone() });
        
        let add: Box<dyn HuanOp> = Box::new(JiaOp {
            lhs: a,
            rhs: b,
            span: span.clone(),
        });
        
        let mut ops = vec![add];
        
        // 创建并运行降级管线
        let mut pipeline = LowerHuanToLLVMPass::new();
        let result = pipeline.run(&mut ops);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_type_conversion() {
        let mut converter = HuanToLLVMTypeConverter::new();
        
        let int_ty: Box<dyn HuanType> = Box::new(IntType);
        let llvm_int = converter.convert_type(&int_ty);
        assert!(llvm_int.is_ok());
        
        let bool_ty: Box<dyn HuanType> = Box::new(BoolType);
        let llvm_bool = converter.convert_type(&bool_ty);
        assert!(llvm_bool.is_ok());
        
        let f64_ty: Box<dyn HuanType> = Box::new(F64Type);
        let llvm_f64 = converter.convert_type(&f64_ty);
        assert!(llvm_f64.is_ok());
    }

    #[test]
    fn test_complete_flow() {
        let span = SourceSpan::dummy();
        
        // 1. 创建一些操作
        let x: Box<dyn HuanOp> = Box::new(IntLitOp { value: 42, span: span.clone() });
        
        // 2. 创建条件
        let cond = Box::new(BoolLitOp { value: true, span: span.clone() });
        
        // 3. 创建 If 操作
        let if_op: Box<dyn HuanOp> = Box::new(RuoOp {
            condition: cond,
            then_block: vec![],
            else_block: Some(vec![]),
            span: span.clone(),
        });
        
        let mut ops = vec![if_op, x];
        
        // 4. 运行优化
        let mut optimize = OptimizationPipeline::new();
        optimize.run(&mut ops).expect("优化失败");
        
        // 5. 运行降级
        let mut lower = LowerHuanToLLVMPass::new();
        lower.run(&mut ops).expect("降级失败");
        
        assert_eq!(ops.len(), 2);
    }

    #[test]
    fn test_list_and_string_types() {
        let list_ty = ListType {
            element_type: Box::new(I32Type),
        };
        
        let string_ty = StringType;
        
        assert_eq!(list_ty.mnemonic(), "list");
        assert_eq!(string_ty.mnemonic(), "string");
        assert_eq!(list_ty.to_string(), "list<i32>");
    }

    #[test]
    fn test_identifier_and_variable_definition() {
        let span = SourceSpan::dummy();
        
        let var = IdentOp {
            name: "x".to_string(),
            span: span.clone(),
        };
        
        let val = Box::new(IntLitOp { value: 100, span: span.clone() });
        
        let let_op = LingOp {
            sym_name: "x".to_string(),
            var_type: Box::new(I32Type),
            value: val,
            span,
        };
        
        assert_eq!(let_op.mnemonic(), "ling");
        assert_eq!(var.mnemonic(), "ident");
    }
}

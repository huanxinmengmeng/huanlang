
#[cfg(test)]
mod tests {
    use super::*;
    use crate::tools::hla::types::*;
    use crate::tools::hla::parser::HlaParser;
    use crate::tools::hla::serializer::HlaSerializer;

    #[test]
    fn test_opcode_from_str() {
        assert_eq!(Opcode::from_str("LING"), Opcode::Ling);
        assert_eq!(Opcode::from_str("DING"), Opcode::Ding);
        assert_eq!(Opcode::from_str("JIA"), Opcode::Jia);
        assert_eq!(Opcode::from_str("JIAN"), Opcode::Jian);
        assert_eq!(Opcode::from_str("UNKNOWN"), Opcode::Unknown);
    }

    #[test]
    fn test_opcode_to_str() {
        assert_eq!(Opcode::Ling.to_str(), "LING");
        assert_eq!(Opcode::Ding.to_str(), "DING");
        assert_eq!(Opcode::Jia.to_str(), "JIA");
    }

    #[test]
    fn test_parse_simple_let() {
        let source = "L001 LING 年龄 整数 25";
        let mut parser = HlaParser::new();
        let result = parser.parse(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_function() {
        let source = r#"
L001 HANSHU 相加 甲:整数 乙:整数 返回:整数
L002 LING 结果 整数 0
L003 JIA 结果 甲 乙
L004 FANHUI 结果
L005 JIESHU
"#;
        let mut parser = HlaParser::new();
        let result = parser.parse(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_type_parsing() {
        let parser = HlaParser::new();
        match parser.parse_type("整数") {
            crate::core::ast::Type::Int => assert!(true),
            _ => assert!(false),
        }
        
        match parser.parse_type("列表[整数]") {
            crate::core::ast::Type::List(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_parser_value_parsing() {
        let parser = HlaParser::new();
        let val = parser.parse_value("真");
        assert!(matches!(val, crate::core::ast::Expr::BoolLit(true, _)));
        
        let val = parser.parse_value("42");
        assert!(matches!(val, crate::core::ast::Expr::IntLit(42, _)));
        
        let val = parser.parse_value("\"Hello\"");
        assert!(matches!(val, crate::core::ast::Expr::StringLit(_, _)));
    }

    #[test]
    fn test_serializer_new() {
        let mut serializer = HlaSerializer::new();
        // 测试序列化器创建成功，不直接访问私有字段
        let program: crate::core::ast::Program = vec![];
        let result = serializer.serialize(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_serializer_metadata() {
        let mut serializer = HlaSerializer::new();
        let program: crate::core::ast::Program = vec![];
        let result = serializer.serialize(&program);
        assert!(result.is_ok());
    }

    #[test]
    fn test_label_parsing() {
        // 由于parse_label是私有方法，我们通过解析完整的HLA代码来测试标签解析
        let source = "L001 LING 年龄 整数 25";
        let mut parser = HlaParser::new();
        let result = parser.parse(source);
        assert!(result.is_ok());
    }

    #[test]
    fn test_error_handling() {
        let source = "L001 UNKNOWN 42";
        let mut parser = HlaParser::new();
        let result = parser.parse(source);
        assert!(result.is_err());
    }
}

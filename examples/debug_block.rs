use huanlang::core::lexer::Lexer;
use huanlang::core::parser::Parser;

fn main() {
    let source = "{ 返回 x }";
    println!("解析代码: {}", source);
    
    let mut lexer = Lexer::new(source);
    let (tokens, errors) = lexer.tokenize();
    
    println!("\n词法分析:");
    println!("错误: {:?}", errors);
    for (i, token) in tokens.iter().enumerate() {
        println!("Token {}: {:?}, 词素: {:?}", i, token.kind, token.lexeme);
    }
    
    let mut parser = Parser::new(tokens);
    match parser.parse_block() {
        Ok(block) => {
            println!("\n块解析成功!");
            println!("语句数量: {}", block.len());
            for (i, stmt) in block.iter().enumerate() {
                println!("语句 {}: {:?}", i, stmt);
            }
        }
        Err(e) => {
            println!("\n块解析失败: {:?}", e);
        }
    }
}

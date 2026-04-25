use huanlang::core::lexer::Lexer;
use huanlang::core::parser::Parser;

fn main() {
    let source = "函数 test() -> 整数 { 返回 42 }";
    let mut lexer = Lexer::new(source);
    let (tokens, errors) = lexer.tokenize();
    
    println!("词法分析结果:");
    println!("错误: {:?}", errors);
    println!("Token数量: {}", tokens.len());
    
    for (i, token) in tokens.iter().enumerate() {
        println!("Token {}: {:?}, 词素: {:?}", i, token.kind, token.lexeme);
    }
    
    println!("\n开始解析...");
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("解析成功! 程序包含 {} 个项", program.len());
            for (i, item) in program.iter().enumerate() {
                println!("项 {}: {:?}", i, item);
            }
        }
        Err(e) => {
            println!("解析失败: {:?}", e);
        }
    }
}
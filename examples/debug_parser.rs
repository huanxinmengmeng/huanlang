use huanlang::core::parser::Parser;
use huanlang::core::lexer::Lexer;

fn main() {
    let source = "函数 test(x: 整数) -> 整数 { 返回 x; }";
    println!("解析代码: {}", source);
    
    // 首先进行词法分析
    let mut lexer = Lexer::new(source);
    let (tokens, errors) = lexer.tokenize();
    
    println!("词法分析结果:");
    println!("错误: {:?}", errors);
    println!("Token数量: {}", tokens.len());
    
    for (i, token) in tokens.iter().enumerate() {
        println!("Token {}: {:?}, 词素: {:?}", i, token.kind, token.lexeme);
    }
    
    // 然后进行语法分析
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(program) => {
            println!("\n解析成功!");
            println!("程序包含 {} 个项目", program.len());
        }
        Err(e) => {
            println!("\n解析失败: {:?}", e);
        }
    }
}
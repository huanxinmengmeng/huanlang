use huanlang::core::lexer::Lexer;

fn main() {
    let source = "函数 test(x: 整数) -> 整数 { 返回 x }";
    let mut lexer = Lexer::new(source);
    let (tokens, errors) = lexer.tokenize();
    
    println!("词法分析结果:");
    println!("错误: {:?}", errors);
    println!("Token数量: {}", tokens.len());
    
    for (i, token) in tokens.iter().enumerate() {
        println!("Token {}: {:?}, 词素: {:?}", i, token.kind, token.lexeme);
    }
}
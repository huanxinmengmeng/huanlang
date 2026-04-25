use huanlang::core::lexer::Lexer;
use huanlang::core::parser::Parser;

fn main() {
    // 测试一个简单的函数定义
    let code = "函数 test(x: 整数) -> 整数 { 返回 x }";
    println!("解析代码: {}", code);
    
    // 首先进行词法分析
    let mut lexer = Lexer::new(code);
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
            println!("程序项数量: {}", program.len());
            for (i, item) in program.iter().enumerate() {
                println!("项 {}: {:?}", i, item);
            }
        }
        Err(e) => {
            println!("\n解析失败: {:?}", e);
        }
    }
    
    // 测试英文关键字
    let code2 = "fn test(x: int) -> int { return x }";
    println!("\n\n解析代码: {}", code2);
    let mut lexer2 = Lexer::new(code2);
    let (tokens2, _) = lexer2.tokenize();
    let mut parser2 = Parser::new(tokens2);
    match parser2.parse() {
        Ok(program) => println!("解析成功! 程序项数量: {}", program.len()),
        Err(e) => println!("解析失败: {:?}", e),
    }
    
    // 测试拼音关键字
    let code3 = "hanshu test(x: zhengshu) -> zhengshu { fanhui x }";
    println!("\n\n解析代码: {}", code3);
    let mut lexer3 = Lexer::new(code3);
    let (tokens3, _) = lexer3.tokenize();
    let mut parser3 = Parser::new(tokens3);
    match parser3.parse() {
        Ok(program) => println!("解析成功! 程序项数量: {}", program.len()),
        Err(e) => println!("解析失败: {:?}", e),
    }
}

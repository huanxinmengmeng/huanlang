use huanlang::core::lexer::keywords::KeywordTable;
use huanlang::core::lexer::token::TokenKind;

fn main() {
    let table = KeywordTable::new();
    
    println!("测试 KeywordTable:");
    
    // 测试中文关键词
    let func_kind = table.get("函数");
    println!("关键词 '函数' 对应的 TokenKind: {:?}", func_kind);
    
    // 测试英文关键词
    let func_kind_en = table.get("func");
    println!("关键词 'func' 对应的 TokenKind: {:?}", func_kind_en);
    
    // 测试拼音关键词
    let func_kind_py = table.get("hanshu");
    println!("关键词 'hanshu' 对应的 TokenKind: {:?}", func_kind_py);
    
    // 测试是否相等
    println!("三个关键词对应的 TokenKind 是否相同: {:?}", func_kind == func_kind_en && func_kind == func_kind_py);
    
    // 测试 TokenKind::Func
    println!("TokenKind::Func: {:?}", TokenKind::Func);
}
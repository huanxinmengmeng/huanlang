
// 幻语解释器完整测试
// 测试解释器的所有功能

use huanlang::interpreter::Interpreter;

#[test]
fn test_interpreter_basic() {
    let mut interpreter = Interpreter::new();

    // 简单程序
    let source = "函数 主() -> 整数32
开始
    打印(\"Hello, HuanLang!\")
    返回 0
结束";

    let result = interpreter.run_source(source);

    assert!(result.is_ok());
    println!("简单测试通过！");
}

#[test]
fn test_variable_assignment() {
    let mut interpreter = Interpreter::new();

    let source = "函数 主() -> 整数32
开始
    令 a 为 42
    令 b 为 100
    打印(a)
    打印(b)
    返回 a + b
结束";

    let result = interpreter.run_source(source);
    assert!(result.is_ok());
    println!("变量赋值测试通过！");
}

#[test]
fn test_arithmetic_operations() {
    let mut interpreter = Interpreter::new();

    let source = "函数 主() -> 整数32
开始
    令 a 为 10
    令 b 为 3
    打印(a + b)
    打印(a - b)
    打印(a * b)
    打印(a / b)
    打印(a % b)
    返回 0
结束";

    let result = interpreter.run_source(source);
    assert!(result.is_ok());
    println!("算术运算测试通过！");
}

#[test]
fn test_function_call() {
    let mut interpreter = Interpreter::new();

    let source = "函数 加法(x:整数32, y:整数32) -> 整数32
开始
    返回 x + y
结束

函数 主() -> 整数32
开始
    令 result = 加法(5, 3)
    打印(result)
    返回 result
结束";

    let result = interpreter.run_source(source);
    assert!(result.is_ok());
    println!("函数调用测试通过！");
}

#[test]
fn test_loop() {
    let mut interpreter = Interpreter::new();

    let source = "函数 主() -> 整数32
开始
    令 i 为 0
    令 sum 为 0

    重复 5 次
    开始
        sum = sum + i
        i = i + 1
    结束

    打印(sum)
    返回 sum
结束";

    let result = interpreter.run_source(source);
    assert!(result.is_ok());
    println!("循环测试通过！");
}

#[test]
fn test_conditional() {
    let mut interpreter = Interpreter::new();

    let source = "函数 主() -> 整数32
开始
    令 score 为 85
    如果 score >= 90
    则
        打印(\"优秀\")
    或者如果 score >= 80
    则
        打印(\"良好\")
    否则
        打印(\"其他\")
    返回 0
结束";

    let result = interpreter.run_source(source);
    assert!(result.is_ok());
    println!("条件判断测试通过！");
}

#[test]
fn test_list_operations() {
    let mut interpreter = Interpreter::new();

    let source = "函数 主() -> 整数32
开始
    令 list = [1, 2, 3, 4, 5]
    打印(list)
    打印(list.长度())
    打印(list[0])
    打印(list[2])
    返回 0
结束";

    let result = interpreter.run_source(source);
    assert!(result.is_ok());
    println!("列表操作测试通过！");
}

// 运行所有测试
fn main() {
    println!("=== 幻语解释器完整测试 ===\n");

    let _ = test_interpreter_basic();
    let _ = test_variable_assignment();
    let _ = test_arithmetic_operations();
    let _ = test_function_call();
    let _ = test_loop();
    let _ = test_conditional();
    let _ = test_list_operations();

    println!("\n=== 所有测试完成！ ===");
}

// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::fmt;

/// 显示特征
pub trait Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result;
}

/// 可比较特征
pub trait PartialOrd {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>;
}

/// 排序枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ordering {
    Less,
    Equal,
    Greater,
}

/// 可哈希特征
pub trait Hash {
    fn hash<H: Hasher>(&self, state: &mut H);
}

/// 哈希器
pub trait Hasher {
    fn finish(&self) -> u64;
    fn write(&mut self, bytes: &[u8]);
}

/// 克隆特征
pub trait Clone {
    fn clone(&self) -> Self;
}

/// 默认特征
pub trait Default {
    fn default() -> Self;
}

/// 结果类型
#[derive(Debug, Clone, PartialEq)]
pub enum HuanResult<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> HuanResult<T, E> {
    pub fn is_ok(&self) -> bool {
        matches!(self, HuanResult::Ok(_))
    }
    
    pub fn is_err(&self) -> bool {
        matches!(self, HuanResult::Err(_))
    }
    
    pub fn unwrap(self) -> T {
        match self {
            HuanResult::Ok(t) => t,
            HuanResult::Err(_) => panic!("called `HuanResult::unwrap()` on an `Err` value"),
        }
    }
    
    pub fn expect(self, msg: &str) -> T {
        match self {
            HuanResult::Ok(t) => t,
            HuanResult::Err(_) => panic!("{}", msg),
        }
    }
    
    pub fn unwrap_err(self) -> E {
        match self {
            HuanResult::Ok(_) => panic!("called `HuanResult::unwrap_err()` on an `Ok` value"),
            HuanResult::Err(e) => e,
        }
    }
    
    pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> HuanResult<U, E> {
        match self {
            HuanResult::Ok(t) => HuanResult::Ok(op(t)),
            HuanResult::Err(e) => HuanResult::Err(e),
        }
    }
    
    pub fn map_err<U, F: FnOnce(E) -> U>(self, op: F) -> HuanResult<T, U> {
        match self {
            HuanResult::Ok(t) => HuanResult::Ok(t),
            HuanResult::Err(e) => HuanResult::Err(op(e)),
        }
    }
    
    pub fn or(self, res: HuanResult<T, E>) -> HuanResult<T, E> {
        match self {
            HuanResult::Ok(t) => HuanResult::Ok(t),
            HuanResult::Err(_) => res,
        }
    }
}

/// 可选类型
#[derive(Debug, Clone, PartialEq)]
pub enum Option<T> {
    Some(T),
    None,
}

impl<T> Option<T> {
    pub fn is_some(&self) -> bool {
        matches!(self, Option::Some(_))
    }
    
    pub fn is_none(&self) -> bool {
        matches!(self, Option::None)
    }
    
    pub fn unwrap(self) -> T {
        match self {
            Option::Some(t) => t,
            Option::None => panic!("called `Option::unwrap()` on an `None` value"),
        }
    }
    
    pub fn expect(self, msg: &str) -> T {
        match self {
            Option::Some(t) => t,
            Option::None => panic!("{}", msg),
        }
    }
    
    pub fn map<U, F: FnOnce(T) -> U>(self, op: F) -> Option<U> {
        match self {
            Option::Some(t) => Option::Some(op(t)),
            Option::None => Option::None,
        }
    }
    
    pub fn and_then<U, F: FnOnce(T) -> Option<U>>(self, op: F) -> Option<U> {
        match self {
            Option::Some(t) => op(t),
            Option::None => Option::None,
        }
    }
    
    pub fn or(self, res: Option<T>) -> Option<T> {
        match self {
            Option::Some(t) => Option::Some(t),
            Option::None => res,
        }
    }
    
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Option::Some(t) => t,
            Option::None => default,
        }
    }
}

/// 向标准输出打印
pub fn 显示<T: fmt::Display>(value: T) {
    print!("{}", value);
}

/// 向标准输出打印并换行
pub fn 显示行<T: fmt::Display>(value: T) {
    println!("{}", value);
}

/// 向标准错误打印
pub fn 错误<T: fmt::Display>(value: T) {
    eprint!("{}", value);
}

/// 向标准错误打印并换行
pub fn 错误行<T: fmt::Display>(value: T) {
    eprintln!("{}", value);
}

/// 格式化字符串
pub fn 格式化(template: &str, _args: fmt::Arguments) -> String {
    format!("{}", template)
}

/// 终止程序
pub fn 终止(code: i32) -> ! {
    std::process::exit(code);
}

/// 调试断言
pub fn 断言(condition: bool, msg: &str) {
    if !condition {
        panic!("断言失败：{}", msg);
    }
}

/// 不可达代码标记
pub fn 不可达() -> ! {
    panic!("不可达代码被执行");
}

/// 获取类型名称
pub fn 类型名<T: 'static>() -> &'static str {
    std::any::type_name::<T>()
}

/// 获取类型大小（字节）
pub fn 大小<T: 'static>() -> usize {
    std::mem::size_of::<T>()
}

/// 获取类型对齐
pub fn 对齐<T: 'static>() -> usize {
    std::mem::align_of::<T>()
}

/// 为基本类型实现显示特征
impl<T: fmt::Display> Display for T {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

/// 整数转换函数
pub trait IntExtensions {
    fn 转为浮点64(self) -> f64;
    fn 转为字符串(self) -> String;
}

impl IntExtensions for i64 {
    fn 转为浮点64(self) -> f64 {
        self as f64
    }
    
    fn 转为字符串(self) -> String {
        self.to_string()
    }
}

/// 浮点数转换函数
pub trait FloatExtensions {
    fn 转为整数(self) -> i64;
    fn 取整(self) -> i64;
    fn 转为字符串(self) -> String;
}

impl FloatExtensions for f64 {
    fn 转为整数(self) -> i64 {
        self as i64
    }
    
    fn 取整(self) -> i64 {
        self.round() as i64
    }
    
    fn 转为字符串(self) -> String {
        self.to_string()
    }
}

/// 字符串转换函数
pub trait StringExtensions {
    fn 转为整数(&self) -> HuanResult<i64, String>;
    fn 转为浮点64(&self) -> HuanResult<f64, String>;
}

impl StringExtensions for String {
    fn 转为整数(&self) -> HuanResult<i64, String> {
        match self.parse() {
            Ok(val) => HuanResult::Ok(val),
            Err(e) => HuanResult::Err(format!("解析失败：{}", e)),
        }
    }
    
    fn 转为浮点64(&self) -> HuanResult<f64, String> {
        match self.parse() {
            Ok(val) => HuanResult::Ok(val),
            Err(e) => HuanResult::Err(format!("解析失败：{}", e)),
        }
    }
}

/// 布尔转换函数
pub trait BoolExtensions {
    fn 转为字符串(self) -> String;
}

impl BoolExtensions for bool {
    fn 转为字符串(self) -> String {
        if self { "真".to_string() } else { "假".to_string() }
    }
}

//! 幻语标准库使用示例
//! 
//! 本文件演示如何使用幻语的标准库模块
//! 包括：集合、IO、字符串、数学、时间、随机数、加密等

// 导入标准库
use huanlang::stdlib::prelude::*;
use huanlang::stdlib::collections::{列表, 字典, 集合};
use huanlang::stdlib::io::{路径, 读取文件, 写入文件};
use huanlang::stdlib::math;
use huanlang::stdlib::time;
use huanlang::stdlib::random;
use huanlang::stdlib::crypto;
use huanlang::stdlib::string::字符串;

/// 主函数 - 演示标准库的各种功能
#[cfg(test)]
mod tests {
    use super::*;

    /// 测试集合模块
    #[test]
    fn test_collections() {
        println!("=== 集合模块示例 ===");

        // 测试列表
        let mut list = 列表::新建();
        list.追加(1);
        list.追加(2);
        list.追加(3);

        println!("列表长度: {}", list.长度());
        assert_eq!(list.长度(), 3);

        // 测试字典
        let mut dict = 字典::新建();
        dict.插入("a".to_string(), 100);
        dict.插入("b".to_string(), 200);

        println!("字典长度: {}", dict.长度());
        assert_eq!(dict.长度(), 2);

        // 测试集合
        let mut set = 集合::新建();
        set.插入(1);
        set.插入(2);
        set.插入(1);  // 重复元素不会被插入

        println!("集合长度: {}", set.长度());
        assert_eq!(set.长度(), 2);
    }

    /// 测试数学模块
    #[test]
    fn test_math() {
        println!("\n=== 数学模块示例 ===");

        let a = 10;
        let b = 3;

        println!("{} + {} = {}", a, b, math::add(a, b));
        println!("{} - {} = {}", a, b, math::sub(a, b));
        println!("{} * {} = {}", a, b, math::mul(a, b));
        println!("{} / {} = {}", a, b, math::div(a, b));
        println!("{} % {} = {}", a, b, math::modulus(a, b));

        println!("平方根 of {} = {}", 16, math::sqrt(16.0));
        println!("绝对数值 of {} = {}", -5, math::abs(-5));
        println!("最大值({}, {}) = {}", 10, 20, math::max(10, 20));
        println!("最小值({}, {}) = {}", 10, 20, math::min(10, 20));
    }

    /// 测试时间模块
    #[test]
    fn test_time() {
        println!("\n=== 时间模块示例 ===");

        let now = time::时间点::现在();
        println!("当前时间: {:?}", now);

        let duration = time::时长::从秒(5);
        println!("时长: {} 秒", duration.秒());
    }

    /// 测试随机数模块
    #[test]
    fn test_random() {
        println!("\n=== 随机数模块示例 ===");

        let rng = random::随机数生成器::新();

        let r1 = rng.整数(0, 100);
        let r2 = rng.浮点数(0.0, 1.0);

        println!("随机整数(0-100): {}", r1);
        println!("随机浮点数(0-1): {}", r2);
    }

    /// 测试加密模块
    #[test]
    fn test_crypto() {
        println!("\n=== 加密模块示例 ===");

        let password = "my_password".to_string();
        let hashed = crypto::哈希::sha256(password);
        println!("SHA256 哈希: {}", hashed);

        let data = "机密数据".to_string();
        let key = "1234567890123456".to_string();
        let encrypted = crypto::加密::aes_encrypt(&data, &key);
        println!("加密结果长度: {}", encrypted.len());
    }

    /// 测试字符串模块
    #[test]
    fn test_string() {
        println!("\n=== 字符串模块示例 ===");

        let s = 字符串::从("Hello, World!");
        println!("字符串: {}", s);
        println!("长度: {}", s.长度());
        println!("是否为空: {}", s.是否为空());

        let upper = s.转换为大写();
        println!("大写: {}", upper);

        let lower = s.转换为小写();
        println!("小写: {}", lower);

        let parts = s.分割(", ");
        println!("分割为 {:?} 个部分", parts.长度());
    }

    /// 测试系统模块
    #[test]
    fn test_system() {
        println!("\n=== 系统模块示例 ===");

        let args = std::env::args().collect::<Vec<_>>();
        println!("命令行参数: {:?}", args);

        let current_dir = std::env::current_dir().unwrap();
        println!("当前目录: {:?}", current_dir);
    }
}

/// 如果直接运行本示例
fn main() {
    println!("幻语标准库使用示例");
    println!("请使用 `cargo test --test stdlib_examples` 来运行所有测试");
}

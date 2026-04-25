// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use num_traits;

/// 圆周率常量
pub const PI: f64 = 3.14159265358979323846;

/// 自然对数底
pub const E: f64 = 2.71828182845904523536;

/// 正无穷
pub const 无穷大: f64 = f64::INFINITY;

/// 负无穷
pub const 负无穷大: f64 = f64::NEG_INFINITY;

/// 非数字
pub const 非数: f64 = f64::NAN;

/// 绝对值
pub fn 绝对值<T: PartialOrd + num_traits::Signed>(x: T) -> T {
    if x < num_traits::zero() { -x } else { x }
}

/// 符号函数
pub fn 符号<T: PartialOrd + num_traits::Signed + num_traits::Zero + num_traits::One>(x: T) -> i32 {
    if x > num_traits::zero() { 1 } 
    else if x < num_traits::zero() { -1 }
    else { 0 }
}

/// 最小值
pub fn 最小值<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

/// 最大值
pub fn 最大值<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

/// 限制范围
pub fn 限制<T: PartialOrd>(值: T, 最小: T, 最大: T) -> T {
    if 值 < 最小 { 最小 }
    else if 值 > 最大 { 最大 }
    else { 值 }
}

/// 幂运算
pub fn 幂(底数: f64, 指数: f64) -> f64 {
    底数.powf(指数)
}

/// 平方
pub fn 平方<T: std::ops::Mul<Output = T> + Copy>(值: T) -> T {
    值 * 值
}

/// 立方
pub fn 立方<T: std::ops::Mul<Output = T> + Copy>(值: T) -> T {
    值 * 值 * 值
}

/// 开平方
pub fn 开平方(值: f64) -> f64 {
    值.sqrt()
}

/// 立方根
pub fn 立方根(值: f64) -> f64 {
    值.cbrt()
}

/// 指数函数 (e^x)
pub fn 指数(值: f64) -> f64 {
    值.exp()
}

/// 自然对数
pub fn 自然对数(值: f64) -> f64 {
    值.ln()
}

/// 对数（指定底数）
pub fn 对数(值: f64, 底数: f64) -> f64 {
    值.log(底数)
}

/// 常用对数 (base 10)
pub fn 常用对数(值: f64) -> f64 {
    值.log10()
}

/// 向下取整
pub fn 向下取整(值: f64) -> f64 {
    值.floor()
}

/// 向上取整
pub fn 向上取整(值: f64) -> f64 {
    值.ceil()
}

/// 四舍五入
pub fn 四舍五入(值: f64) -> f64 {
    值.round()
}

/// 截断
pub fn 截断(值: f64) -> f64 {
    值.trunc()
}

/// 整数除法
pub fn 整除(分子: i64, 分母: i64) -> i64 {
    分子 / 分母
}

/// 取余数
pub fn 取余(分子: i64, 分母: i64) -> i64 {
    分子 % 分母
}

/// 整除取余，返回 (商, 余数)
pub fn 整除余数(分子: i64, 分母: i64) -> (i64, i64) {
    (分子 / 分母, 分子 % 分母)
}

/// 正弦函数
pub fn 正弦(弧度: f64) -> f64 {
    弧度.sin()
}

/// 余弦函数
pub fn 余弦(弧度: f64) -> f64 {
    弧度.cos()
}

/// 正切函数
pub fn 正切(弧度: f64) -> f64 {
    弧度.tan()
}

/// 反正弦函数
pub fn 反正弦(值: f64) -> f64 {
    值.asin()
}

/// 反余弦函数
pub fn 反余弦(值: f64) -> f64 {
    值.acos()
}

/// 反正切函数
pub fn 反正切(值: f64) -> f64 {
    值.atan()
}

/// 反正切函数 (atan2)
pub fn 反正切2(纵: f64, 横: f64) -> f64 {
    纵.atan2(横)
}

/// 双曲正弦
pub fn 双曲正弦(值: f64) -> f64 {
    值.sinh()
}

/// 双曲余弦
pub fn 双曲余弦(值: f64) -> f64 {
    值.cosh()
}

/// 双曲正切
pub fn 双曲正切(值: f64) -> f64 {
    值.tanh()
}

/// 弧度转角度
pub fn 弧度转角度(弧度: f64) -> f64 {
    弧度 * (180.0 / PI)
}

/// 角度转弧度
pub fn 角度转弧度(角度: f64) -> f64 {
    角度 * (PI / 180.0)
}

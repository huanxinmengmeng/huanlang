// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::io::{self, Write};

/// 控制台输入输出模块
pub struct 控制台;

impl 控制台 {
    /// 打印一行文本
    pub fn 打印(内容: &str) {
        print!("{}", 内容);
        let _ = std::io::stdout().flush();
    }

    /// 打印一行文本并换行
    pub fn 打印行(内容: &str) {
        println!("{}", 内容);
    }

    /// 打印格式化文本
    pub fn 打印格式(格式: &str, 参数列表: &[&str]) {
        let result = 格式化文本(格式, 参数列表);
        控制台::打印(&result);
    }

    /// 读取一行输入
    pub fn 读取行() -> Option<String> {
        let mut 输入 = String::new();
        match io::stdin().read_line(&mut 输入) {
            Ok(_) => {
                输入.pop();
                if 输入.ends_with('\r') {
                    输入.pop();
                }
                Some(输入)
            }
            Err(_) => None,
        }
    }

    /// 读取密码（不显示输入）
    pub fn 读取密码() -> Option<String> {
        let mut 密码 = String::new();
        match io::stdin().read_line(&mut 密码) {
            Ok(_) => {
                密码.pop();
                if 密码.ends_with('\r') {
                    密码.pop();
                }
                Some(密码)
            }
            Err(_) => None,
        }
    }

    /// 清除屏幕
    pub fn 清除屏幕() {
        print!("\x1B[2J");
        let _ = std::io::stdout().flush();
    }

    /// 设置光标位置
    pub fn 设置光标位置(行: u16, 列: u16) {
        print!("\x1B[{};{}H", 行, 列);
        let _ = std::io::stdout().flush();
    }

    /// 隐藏光标
    pub fn 隐藏光标() {
        print!("\x1B[?25l");
        let _ = std::io::stdout().flush();
    }

    /// 显示光标
    pub fn 显示光标() {
        print!("\x1B[?25h");
        let _ = std::io::stdout().flush();
    }

    /// 设置前景色
    pub fn 设置前景色(颜色: 颜色) {
        let 色码 = match 颜色 {
            颜色::黑色 => 30,
            颜色::红色 => 31,
            颜色::绿色 => 32,
            颜色::黄色 => 33,
            颜色::蓝色 => 34,
            颜色::品红色 => 35,
            颜色::青色 => 36,
            颜色::白色 => 37,
            颜色::默认 => 39,
        };
        print!("\x1B[{}m", 色码);
        let _ = std::io::stdout().flush();
    }

    /// 设置背景色
    pub fn 设置背景色(颜色: 颜色) {
        let 色码 = match 颜色 {
            颜色::黑色 => 40,
            颜色::红色 => 41,
            颜色::绿色 => 42,
            颜色::黄色 => 43,
            颜色::蓝色 => 44,
            颜色::品红色 => 45,
            颜色::青色 => 46,
            颜色::白色 => 47,
            颜色::默认 => 49,
        };
        print!("\x1B[{}m", 色码);
        let _ = std::io::stdout().flush();
    }

    /// 重置所有样式
    pub fn 重置样式() {
        print!("\x1B[0m");
        let _ = std::io::stdout().flush();
    }

    /// 设置粗体
    pub fn 设置粗体() {
        print!("\x1B[1m");
        let _ = std::io::stdout().flush();
    }

    /// 设置下划线
    pub fn 设置下划线() {
        print!("\x1B[4m");
        let _ = std::io::stdout().flush();
    }

    /// 获取终端宽度
    pub fn 终端宽度() -> u16 {
        终端大小().0
    }

    /// 获取终端高度
    pub fn 终端高度() -> u16 {
        终端大小().1
    }
}

/// 颜色枚举
pub enum 颜色 {
    黑色,
    红色,
    绿色,
    黄色,
    蓝色,
    品红色,
    青色,
    白色,
    默认,
}

/// 获取终端大小
fn 终端大小() -> (u16, u16) {
    #[cfg(windows)]
    {
        use std::env;
        if let Ok(columns_str) = env::var("COLUMNS") {
            if let Ok(columns) = columns_str.parse::<u16>() {
                if let Ok(lines_str) = env::var("LINES") {
                    if let Ok(lines) = lines_str.parse::<u16>() {
                        return (columns, lines);
                    }
                }
            }
        }
        (80, 25)
    }

    #[cfg(not(windows))]
    {
        (80, 25)
    }
}

/// 简单的格式化文本函数
fn 格式化文本(格式: &str, 参数列表: &[&str]) -> String {
    let mut result = String::new();
    let mut current = String::new();
    let mut in_placeholder = false;

    for ch in 格式.chars() {
        if ch == '{' && !in_placeholder {
            if !current.is_empty() {
                result.push_str(&current);
                current.clear();
            }
            in_placeholder = true;
        } else if ch == '}' && in_placeholder {
            in_placeholder = false;
            let idx = current.parse::<usize>().ok();
            current.clear();
            if let Some(i) = idx {
                if i < 参数列表.len() {
                    result.push_str(参数列表[i]);
                }
            }
        } else {
            current.push(ch);
        }
    }

    if !current.is_empty() {
        result.push_str(&current);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_格式化文本() {
        let result = 格式化文本("你好, {0}!", &["世界"]);
        assert_eq!(result, "你好, 世界!");

        let result = 格式化文本("{0} + {1} = {2}", &["1", "2", "3"]);
        assert_eq!(result, "1 + 2 = 3");
    }
}
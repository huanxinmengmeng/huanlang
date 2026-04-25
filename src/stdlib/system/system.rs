// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::env;
use std::process;
use libloading::{Library, Symbol};

type Result<T> = std::result::Result<T, 系统错误>;
use crate::stdlib::collections::字典;
use crate::stdlib::collections::列表;
use crate::stdlib::string::字符串;
use crate::stdlib::io::系统错误;

pub fn 获取环境变量(键: &str) -> Option<字符串> {
    match env::var(键) {
        Ok(value) => Option::Some(字符串::从(&value)),
        Err(_) => Option::None,
    }
}

pub fn 设置环境变量(键: &str, 值: &str) -> Result<()> {
    env::set_var(键, 值);
    Result::Ok(())
}

pub fn 删除环境变量(键: &str) -> Result<()> {
    unsafe {
        env::remove_var(键);
    }
    Result::Ok(())
}

pub fn 所有环境变量() -> 字典<字符串, 字符串> {
    let mut 字典 = 字典::新建();
    for (key, value) in env::vars() {
        字典.插入(字符串::从(&key), 字符串::从(&value));
    }
    字典
}

pub fn 参数() -> 列表<字符串> {
    let mut 列表 = 列表::新建();
    for arg in env::args() {
        列表.追加(字符串::从(&arg));
    }
    列表
}

pub fn 参数数量() -> usize {
    env::args().count()
}

#[derive(Debug, Clone)]
pub enum 选项类型 {
    标志,
    选项 { 需要值: bool, 默认值: Option<String> },
}

#[derive(Debug, Clone)]
pub struct 选项定义 {
    pub 短名: Option<char>,
    pub 长名: String,
    pub 描述: String,
    pub 选项类型: 选项类型,
}

#[derive(Debug, Clone)]
pub struct 子命令定义 {
    pub 名称: String,
    pub 描述: String,
    pub 选项列表: Vec<选项定义>,
}

pub struct 命令行解析器 {
    选项列表: Vec<选项定义>,
    子命令列表: Vec<子命令定义>,
    当前子命令: Option<String>,
}

impl 命令行解析器 {
    pub fn 新建() -> Self {
        命令行解析器 {
            选项列表: Vec::new(),
            子命令列表: Vec::new(),
            当前子命令: None,
        }
    }

    pub fn 添加标志(&mut self, 短名: Option<char>, 长名: &str, 描述: &str) -> &mut Self {
        self.选项列表.push(选项定义 {
            短名,
            长名: 长名.to_string(),
            描述: 描述.to_string(),
            选项类型: 选项类型::标志,
        });
        self
    }

    pub fn 添加选项(&mut self, 短名: Option<char>, 长名: &str, 描述: &str, 需要值: bool) -> &mut Self {
        self.选项列表.push(选项定义 {
            短名,
            长名: 长名.to_string(),
            描述: 描述.to_string(),
            选项类型: 选项类型::选项 { 需要值, 默认值: None },
        });
        self
    }

    pub fn 添加选项_带默认值(&mut self, 短名: Option<char>, 长名: &str, 描述: &str, 默认值: &str) -> &mut Self {
        self.选项列表.push(选项定义 {
            短名,
            长名: 长名.to_string(),
            描述: 描述.to_string(),
            选项类型: 选项类型::选项 { 需要值: true, 默认值: Some(默认值.to_string()) },
        });
        self
    }

    pub fn 添加子命令(&mut self, 名称: &str, 描述: &str) -> &mut Self {
        self.子命令列表.push(子命令定义 {
            名称: 名称.to_string(),
            描述: 描述.to_string(),
            选项列表: Vec::new(),
        });
        self
    }

    pub fn 获取子命令(&self) -> Option<&子命令定义> {
        if let Some(ref name) = self.当前子命令 {
            self.子命令列表.iter().find(|c| &c.名称 == name)
        } else {
            None
        }
    }

    pub fn 解析(&self) -> Result<命令行匹配> {
        let args: Vec<String> = env::args().collect();
        self.解析参数(&args[1..])
    }

    pub fn 解析参数(&self, args: &[String]) -> Result<命令行匹配> {
        let mut 匹配 = 命令行匹配 {
            子命令: None,
            标志: std::collections::HashMap::new(),
            选项值: std::collections::HashMap::new(),
            位置参数: Vec::new(),
        };

        // 初始化默认值
        for opt in &self.选项列表 {
            if let 选项类型::选项 { 需要值: true, 默认值: Some(ref default) } = opt.选项类型 {
                匹配.选项值.insert(opt.长名.clone(), default.clone());
            }
        }

        let mut i = 0;
        while i < args.len() {
            let arg = &args[i];

            // 跳过第一个参数（程序名）
            if i == 0 {
                i += 1;
                continue;
            }

            if !arg.starts_with('-') && self.子命令列表.iter().any(|c| &c.名称 == arg) {
                匹配.子命令 = Some(arg.clone());
                i += 1;
                continue;
            }

            if arg.starts_with("--") {
                let name = &arg[2..];
                if let Some(opt) = self.查找选项(name) {
                    match opt.选项类型 {
                        选项类型::标志 => {
                            匹配.标志.insert(name.to_string(), true);
                        }
                        选项类型::选项 { 需要值, .. } => {
                            if 需要值 {
                                if i + 1 < args.len() {
                                    i += 1;
                                    匹配.选项值.insert(name.to_string(), args[i].clone());
                                    i += 1;
                                    continue;
                                } else {
                                    return Result::Err(系统错误::IO错误(format!("选项 --{} 需要一个值", name)));
                                }
                            } else {
                                匹配.选项值.insert(name.to_string(), "true".to_string());
                            }
                        }
                    }
                }
            } else if arg.starts_with('-') {
                let name = &arg[1..];
                if name.len() == 1 {
                    if let Some(opt) = self.查找选项_by_short(name.chars().next().unwrap()) {
                        match opt.选项类型 {
                            选项类型::标志 => {
                                匹配.标志.insert(opt.长名.clone(), true);
                            }
                            选项类型::选项 { 需要值, .. } => {
                                if 需要值 {
                                    if i + 1 < args.len() {
                                        i += 1;
                                        匹配.选项值.insert(opt.长名.clone(), args[i].clone());
                                        i += 1;
                                        continue;
                                    } else {
                                        return Result::Err(系统错误::IO错误(format!("选项 -{} 需要一个值", name)));
                                    }
                                } else {
                                    匹配.选项值.insert(opt.长名.clone(), "true".to_string());
                                }
                            }
                        }
                    }
                } else {
                    匹配.位置参数.push(arg.clone());
                }
            } else {
                匹配.位置参数.push(arg.clone());
            }
            i += 1;
        }

        Result::Ok(匹配)
    }

    fn 查找选项(&self, 长名: &str) -> Option<&选项定义> {
        self.选项列表.iter().find(|o| &o.长名 == 长名)
    }

    fn 查找选项_by_short(&self, 短名: char) -> Option<&选项定义> {
        self.选项列表.iter().find(|o| o.短名 == Some(短名))
    }

    pub fn 生成帮助(&self) -> String {
        let mut help = String::new();
        help.push_str("用法: 命令 [选项] [参数]\n\n");

        if !self.子命令列表.is_empty() {
            help.push_str("子命令:\n");
            for cmd in &self.子命令列表 {
                help.push_str(&format!("  {} - {}\n", cmd.名称, cmd.描述));
            }
            help.push_str("\n");
        }

        if !self.选项列表.is_empty() {
            help.push_str("选项:\n");
            for opt in &self.选项列表 {
                let short = opt.短名.map(|c| format!("-{}", c)).unwrap_or_else(|| "  ".to_string());
                let long = format!("--{}", opt.长名);
                match opt.选项类型 {
                    选项类型::标志 => {
                        help.push_str(&format!("  {} {}\n", short, long));
                    }
                    选项类型::选项 { 需要值, .. } => {
                        if 需要值 {
                            help.push_str(&format!("  {} {} <值>\n", short, long));
                        } else {
                            help.push_str(&format!("  {} {}\n", short, long));
                        }
                    }
                }
                help.push_str(&format!("    {}\n", opt.描述));
            }
        }

        help
    }
}

impl Default for 命令行解析器 {
    fn default() -> Self {
        Self::新建()
    }
}

#[derive(Debug, Clone)]
pub struct 命令行匹配 {
    pub 子命令: Option<String>,
    pub 标志: std::collections::HashMap<String, bool>,
    pub 选项值: std::collections::HashMap<String, String>,
    pub 位置参数: Vec<String>,
}

impl 命令行匹配 {
    pub fn 是否匹配(&self, 名称: &str) -> bool {
        self.标志.get(名称).copied().unwrap_or(false)
    }

    pub fn 有选项(&self, 名称: &str) -> bool {
        self.选项值.contains_key(名称)
    }

    pub fn 获取值(&self, 名称: &str) -> Option<字符串> {
        self.选项值.get(名称).map(|s| 字符串::从(s))
    }

    pub fn 获取值_或默认值(&self, 名称: &str, 默认值: &str) -> String {
        self.选项值.get(名称).cloned().unwrap_or_else(|| 默认值.to_string())
    }

    pub fn 获取值_作为整数(&self, 名称: &str) -> Option<i64> {
        self.选项值.get(名称).and_then(|v| v.parse().ok())
    }

    pub fn 获取值_作为浮点数(&self, 名称: &str) -> Option<f64> {
        self.选项值.get(名称).and_then(|v| v.parse().ok())
    }

    pub fn 位置参数(&self) -> &Vec<String> {
        &self.位置参数
    }

    pub fn 位置参数_at(&self, 索引: usize) -> Option<&String> {
        self.位置参数.get(索引)
    }

    pub fn 位置参数数量(&self) -> usize {
        self.位置参数.len()
    }

    pub fn 有子命令(&self) -> bool {
        self.子命令.is_some()
    }

    pub fn 子命令(&self) -> Option<&String> {
        self.子命令.as_ref()
    }
}

pub fn 执行(命令: &str, 参数: &[字符串]) -> Result<命令输出> {
    let mut cmd = process::Command::new(命令);
    for arg in 参数 {
        cmd.arg(arg.作为字符串());
    }
    match cmd.output() {
        Ok(output) => Result::Ok(命令输出::从(output)),
        Err(e) => Result::Err(系统错误::IO错误(e.to_string())),
    }
}

pub fn 执行并捕获(命令: &str, 参数: &[字符串]) -> Result<命令输出> {
    执行(命令, 参数)
}

pub struct 命令输出 {
    status: process::Output,
}

impl 命令输出 {
    pub fn 从(output: process::Output) -> Self {
        命令输出 { status: output }
    }

    pub fn 状态码(&self) -> i32 {
        self.status.status.code().unwrap_or(-1)
    }

    pub fn 标准输出(&self) -> 字符串 {
        字符串::从(&String::from_utf8_lossy(&self.status.stdout).to_string())
    }

    pub fn 标准错误(&self) -> 字符串 {
        字符串::从(&String::from_utf8_lossy(&self.status.stderr).to_string())
    }

    pub fn 是否成功(&self) -> bool {
        self.status.status.success()
    }
}

pub fn 当前进程_id() -> u32 {
    process::id()
}

pub fn 退出(代码: i32) -> ! {
    process::exit(代码)
}

pub struct 动态库 {
    library: Library,
    path: String,
}

impl 动态库 {
    pub fn 加载(路径: &str) -> Result<动态库> {
        unsafe {
            Library::new(路径)
                .map(|library| 动态库 {
                    library,
                    path: 路径.to_string(),
                })
                .map_err(|e| 系统错误::IO错误(format!("加载动态库失败: {}", e)))
        }
    }

    #[allow(unsafe_code)]
    pub fn 获取符号<T>(&self, 名称: &str) -> Result<*const T> {
        unsafe {
            let symbol: Symbol<T> = self.library.get(名称.as_bytes())
                .map_err(|e| 系统错误::IO错误(format!("找不到符号 {}: {}", 名称, e)))?;
            let ptr = (&*symbol) as *const T;
            Ok(ptr)
        }
    }

    #[allow(unsafe_code)]
    pub fn 获取函数指针<F>(&self, 名称: &str) -> Result<Symbol<'_, F>> {
        unsafe {
            self.library.get(名称.as_bytes())
                .map_err(|e| 系统错误::IO错误(format!("找不到函数指针 {}: {}", 名称, e)))
        }
    }

    #[allow(unsafe_code)]
    pub fn 获取数据<D>(&self, 名称: &str) -> Result<*const D> {
        unsafe {
            let symbol: Symbol<D> = self.library.get(名称.as_bytes())
                .map_err(|e| 系统错误::IO错误(format!("找不到数据符号 {}: {}", 名称, e)))?;
            let ptr = std::mem::transmute_copy(&symbol);
            Ok(ptr)
        }
    }

    #[allow(unsafe_code)]
    pub fn 获取可变数据<D>(&self, 名称: &str) -> Result<*mut D> {
        unsafe {
            let symbol: Symbol<D> = self.library.get(名称.as_bytes())
                .map_err(|e| 系统错误::IO错误(format!("找不到可变数据符号 {}: {}", 名称, e)))?;
            let ptr = std::mem::transmute_copy(&symbol);
            Ok(ptr)
        }
    }

    pub fn 卸载(&mut self) -> Result<()> {
        let _ = &mut self.library;
        Ok(())
    }

    pub fn 路径(&self) -> &str {
        &self.path
    }
}

impl Drop for 动态库 {
    fn drop(&mut self) {
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_命令行解析器_基本使用() {
        let mut parser = 命令行解析器::新建();
        parser
            .添加标志(std::option::Option::Some('v'), "verbose", "详细输出")
            .添加选项(std::option::Option::Some('o'), "output", "输出文件", true);

        let args = vec!["prog".to_string(), "-v".to_string(), "-o".to_string(), "file.txt".to_string(), "input.txt".to_string()];
        let result = parser.解析参数(&args);

        assert!(result.is_ok());
        let 匹配 = result.unwrap();
        assert!(匹配.是否匹配("verbose"));
        assert_eq!(匹配.获取值("output"), std::option::Option::Some(字符串::从("file.txt")));
        assert_eq!(匹配.位置参数_at(0), std::option::Option::Some(&"input.txt".to_string()));
    }

    #[test]
    fn test_命令行解析器_长选项() {
        let mut parser = 命令行解析器::新建();
        parser
            .添加标志(std::option::Option::None, "verbose", "详细输出")
            .添加选项(std::option::Option::None, "output", "输出文件", true);

        let args: Vec<String> = vec!["prog", "--verbose", "--output", "file.txt"].iter().map(|s| s.to_string()).collect();
        let result = parser.解析参数(&args);

        assert!(result.is_ok());
        let 匹配 = result.unwrap();
        assert!(匹配.是否匹配("verbose"));
        assert_eq!(匹配.获取值("output"), std::option::Option::Some(字符串::从("file.txt")));
    }

    #[test]
    fn test_命令行解析器_子命令() {
        let mut parser = 命令行解析器::新建();
        parser.添加子命令("build", "构建项目");
        parser.添加子命令("run", "运行项目");

        let args: Vec<String> = vec!["prog", "build"].iter().map(|s| s.to_string()).collect();
        let result = parser.解析参数(&args);

        assert!(result.is_ok());
        let 匹配 = result.unwrap();
        assert!(匹配.有子命令());
        assert_eq!(匹配.子命令(), std::option::Option::Some(&"build".to_string()));
    }

    #[test]
    fn test_命令行匹配_默认值() {
        let mut parser = 命令行解析器::新建();
        parser.添加选项_带默认值(std::option::Option::None, "port", "端口号", "8080");

        let args: Vec<String> = vec!["prog"].iter().map(|s| s.to_string()).collect();
        let result = parser.解析参数(&args);

        assert!(result.is_ok());
        let 匹配 = result.unwrap();
        assert_eq!(匹配.获取值_或默认值("port", "80"), "8080");
    }

    #[test]
    fn test_命令行匹配_整数解析() {
        let mut parser = 命令行解析器::新建();
        parser.添加选项(std::option::Option::None, "port", "端口号", true);

        let args: Vec<String> = vec!["prog", "--port", "8080"].iter().map(|s| s.to_string()).collect();
        let result = parser.解析参数(&args);

        assert!(result.is_ok());
        let 匹配 = result.unwrap();
        assert_eq!(匹配.获取值_作为整数("port"), std::option::Option::Some(8080));
    }
}
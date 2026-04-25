// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use crate::stdlib::collections::列表;
use crate::stdlib::core::Option;

/// 幻语字符串类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct 字符串 {
    data: String,
}

impl 字符串 {
    /// 创建新的空字符串
    pub fn 新建() -> Self {
        Self { data: String::new() }
    }
    
    /// 创建具有指定容量的空字符串
    pub fn 从容量(容量: usize) -> Self {
        Self { data: String::with_capacity(容量) }
    }
    
    /// 从 UTF8 字节创建字符串
    pub fn 从_utf8(字节: &[u8]) -> crate::stdlib::core::HuanResult<字符串, &'static str> {
        match std::str::from_utf8(字节) {
            Ok(s) => crate::stdlib::core::HuanResult::Ok(字符串 { data: s.to_string() }),
            Err(_) => crate::stdlib::core::HuanResult::Err("无效的 UTF8 编码"),
        }
    }
    
    /// 从字符串切片创建
    pub fn 从(源: &str) -> Self {
        Self { data: 源.to_string() }
    }
    
    /// 获取字符长度（Unicode 标量值）
    pub fn 长度(&self) -> usize {
        self.data.chars().count()
    }
    
    /// 获取字节长度
    pub fn 字节长度(&self) -> usize {
        self.data.len()
    }
    
    /// 检查字符串是否为空
    pub fn 是否为空(&self) -> bool {
        self.data.is_empty()
    }
    
    /// 获取容量
    pub fn 容量(&self) -> usize {
        self.data.capacity()
    }
    
    /// 获取指定位置的字符
    pub fn 字符(&self, 索引: usize) -> Option<char> {
        match self.data.chars().nth(索引) {
            Some(c) => Option::Some(c),
            None => Option::None,
        }
    }
    
    /// 获取子字符串
    pub fn 子串(&self, 开始: usize, 结束: usize) -> Option<字符串> {
        let mut chars = self.data.chars();
        let start_char = chars.by_ref().take(开始).count();
        if start_char != 开始 {
            return Option::None;
        }
        let substr: String = chars.take(结束 - 开始).collect();
        Option::Some(字符串 { data: substr })
    }
    
    /// 获取切片
    pub fn 切片(&self, 范围: std::ops::Range<usize>) -> 字符串 {
        字符串 { data: self.data.chars().skip(范围.start).take(范围.end - 范围.start).collect() }
    }
    
    /// 追加可显示值
    pub fn 追加<T: std::fmt::Display>(&mut self, 值: T) {
        self.data.push_str(&值.to_string());
    }
    
    /// 追加字符串
    pub fn 追加字符串(&mut self, 其他: &字符串) {
        self.data.push_str(&其他.data);
    }
    
    /// 在指定位置插入可显示值
    pub fn 插入<T: std::fmt::Display>(&mut self, 索引: usize, 值: T) {
        let char_index = self.data.char_indices().nth(索引).map(|(i, _)| i).unwrap_or(self.data.len());
        self.data.insert_str(char_index, &值.to_string());
    }
    
    /// 移除指定位置的字符
    pub fn 移除(&mut self, 索引: usize) -> Option<char> {
        if let Some((byte_index, _)) = self.data.char_indices().nth(索引) {
            Option::Some(self.data.remove(byte_index))
        } else {
            Option::None
        }
    }
    
    /// 清空字符串
    pub fn 清空(&mut self) {
        self.data.clear();
    }
    
    /// 检查是否包含子字符串
    pub fn 包含(&self, 子串: &str) -> bool {
        self.data.contains(子串)
    }
    
    /// 查找子字符串第一次出现的位置
    pub fn 查找(&self, 子串: &str) -> Option<usize> {
        match self.data.find(子串) {
            Some(byte_index) => {
                let char_index = self.data[..byte_index].chars().count();
                Option::Some(char_index)
            }
            None => Option::None,
        }
    }
    
    /// 反向查找子字符串
    pub fn 反向查找(&self, 子串: &str) -> Option<usize> {
        match self.data.rfind(子串) {
            Some(byte_index) => {
                let char_index = self.data[..byte_index].chars().count();
                Option::Some(char_index)
            }
            None => Option::None,
        }
    }
    
    /// 检查是否以指定前缀开头
    pub fn 是否以(&self, 前缀: &str) -> bool {
        self.data.starts_with(前缀)
    }
    
    /// 检查是否以指定后缀结尾
    pub fn 结束(&self, 后缀: &str) -> bool {
        self.data.ends_with(后缀)
    }
    
    /// 替换第一次出现的子字符串
    pub fn 替换(&self, 旧: &str, 新: &str) -> 字符串 {
        字符串 { data: self.data.replacen(旧, 新, 1) }
    }
    
    /// 替换所有出现的子字符串
    pub fn 替换所有(&self, 旧: &str, 新: &str) -> 字符串 {
        字符串 { data: self.data.replace(旧, 新) }
    }
    
    /// 分割字符串
    pub fn 分割(&self, 分隔符: &str) -> 列表<字符串> {
        let mut 列表 = 列表::新建();
        for 部分 in self.data.split(分隔符) {
            列表.追加(字符串::从(部分));
        }
        列表
    }
    
    /// 按空白分割
    pub fn 分割空白(&self) -> 列表<字符串> {
        let mut 列表 = 列表::新建();
        for 部分 in self.data.split_whitespace() {
            列表.追加(字符串::从(部分));
        }
        列表
    }
    
    /// 按行分割
    pub fn 分割行(&self) -> 列表<字符串> {
        let mut 列表 = 列表::新建();
        for 行 in self.data.lines() {
            列表.追加(字符串::从(行));
        }
        列表
    }
    
    /// 连接可显示值
    pub fn 连接<可显示: std::fmt::Display, 迭代器: Iterator<Item = 可显示>>(迭代器: 迭代器, 分隔符: &str) -> 字符串 {
        let mut result = String::new();
        let mut first = true;
        for item in 迭代器 {
            if !first {
                result.push_str(分隔符);
            }
            first = false;
            result.push_str(&item.to_string());
        }
        字符串 { data: result }
    }
    
    /// 修剪首尾空白
    pub fn 修剪(&self) -> 字符串 {
        字符串 { data: self.data.trim().to_string() }
    }
    
    /// 修剪开头空白
    pub fn 修剪开始(&self) -> 字符串 {
        字符串 { data: self.data.trim_start().to_string() }
    }
    
    /// 修剪结尾空白
    pub fn 修剪结束(&self) -> 字符串 {
        字符串 { data: self.data.trim_end().to_string() }
    }
    
    /// 转换为大写
    pub fn 转大写(&self) -> 字符串 {
        字符串 { data: self.data.to_uppercase() }
    }
    
    /// 转换为小写
    pub fn 转小写(&self) -> 字符串 {
        字符串 { data: self.data.to_lowercase() }
    }
    
    /// 转换为字节
    pub fn 转为字节(&self) -> 列表<u8> {
        let mut 列表 = 列表::新建();
        for 字节 in self.data.bytes() {
            列表.追加(字节);
        }
        列表
    }
    
    /// 从字节创建
    pub fn 从字节(字节: &[u8]) -> crate::stdlib::core::HuanResult<字符串, &'static str> {
        字符串::从_utf8(字节)
    }
    
    /// 字符迭代器
    pub fn 字符迭代(&self) -> std::str::Chars<'_> {
        self.data.chars()
    }
    
    /// 字节迭代器
    pub fn 字节迭代(&self) -> std::str::Bytes<'_> {
        self.data.bytes()
    }
    
    /// 行迭代器
    pub fn 行迭代(&self) -> std::str::Lines<'_> {
        self.data.lines()
    }
    
    /// 获取内部字符串
    pub fn 作为字符串(&self) -> &str {
        &self.data
    }
}

impl std::fmt::Display for 字符串 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.data, f)
    }
}



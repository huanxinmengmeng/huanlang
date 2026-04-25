// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use serde::{Serialize, Deserialize};
use serde_json;


/// JSON 值类型
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum JSON值 {
    空,
    布尔(bool),
    数字(f64),
    字符串(String),
    数组(Vec<JSON值>),
    对象(std::collections::HashMap<String, JSON值>),
}

impl Serialize for JSON值 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            JSON值::空 => serializer.serialize_none(),
            JSON值::布尔(b) => serializer.serialize_bool(*b),
            JSON值::数字(n) => serializer.serialize_f64(*n),
            JSON值::字符串(s) => serializer.serialize_str(s),
            JSON值::数组(arr) => arr.serialize(serializer),
            JSON值::对象(obj) => obj.serialize(serializer),
        }
    }
}

/// 可序列化为 JSON 的特征
pub trait 可序列化为JSON {
    fn 转为_json(&self) -> JSON值;
}

/// 可从 JSON 反序列化的特征
pub trait 可从JSON反序列化: Sized {
    fn 从_json(json: &JSON值) -> Result<Self, JSON错误>;
}

/// JSON 错误类型
#[derive(Debug, Clone)]
pub enum JSON错误 {
    解析错误(String),
    序列化错误(String),
    类型不匹配(String),
    缺少字段(String),
}

impl std::fmt::Display for JSON错误 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JSON错误::解析错误(msg) => write!(f, "JSON 解析错误：{}", msg),
            JSON错误::序列化错误(msg) => write!(f, "JSON 序列化错误：{}", msg),
            JSON错误::类型不匹配(msg) => write!(f, "JSON 类型不匹配：{}", msg),
            JSON错误::缺少字段(msg) => write!(f, "JSON 缺少字段：{}", msg),
        }
    }
}

/// 解析 JSON 字符串
pub fn 解析_json(输入: &str) -> Result<JSON值, JSON错误> {
    match serde_json::from_str(输入) {
        Ok(value) => Ok(转换为_json值(&value)),
        Err(e) => Err(JSON错误::解析错误(e.to_string())),
    }
}

/// 将 serde_json::Value 转换为我们的 JSON值
fn 转换为_json值(value: &serde_json::Value) -> JSON值 {
    match value {
        serde_json::Value::Null => JSON值::空,
        serde_json::Value::Bool(b) => JSON值::布尔(*b),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                JSON值::数字(i as f64)
            } else if let Some(f) = n.as_f64() {
                JSON值::数字(f)
            } else {
                JSON值::数字(0.0)
            }
        }
        serde_json::Value::String(s) => JSON值::字符串(s.clone()),
        serde_json::Value::Array(arr) => {
            let mut 数组 = Vec::new();
            for item in arr {
                数组.push(转换为_json值(item));
            }
            JSON值::数组(数组)
        }
        serde_json::Value::Object(obj) => {
            let mut 对象 = std::collections::HashMap::new();
            for (k, v) in obj {
                对象.insert(k.clone(), 转换为_json值(v));
            }
            JSON值::对象(对象)
        }
    }
}

/// 将 JSON值 转换为字符串
pub fn 转为_json字符串(值: &JSON值) -> crate::stdlib::string::字符串 {
    crate::stdlib::string::字符串::从(&serde_json::to_string(&值).unwrap_or("".to_string()))
}

/// 将 JSON值 转换为美化的字符串
pub fn 美化_json(值: &JSON值) -> crate::stdlib::string::字符串 {
    crate::stdlib::string::字符串::从(&serde_json::to_string_pretty(&值).unwrap_or("".to_string()))
}

/// 为基本类型实现 JSON 序列化
impl 可序列化为JSON for bool {
    fn 转为_json(&self) -> JSON值 {
        JSON值::布尔(*self)
    }
}

impl 可序列化为JSON for i64 {
    fn 转为_json(&self) -> JSON值 {
        JSON值::数字(*self as f64)
    }
}

impl 可序列化为JSON for f64 {
    fn 转为_json(&self) -> JSON值 {
        JSON值::数字(*self)
    }
}

impl 可序列化为JSON for String {
    fn 转为_json(&self) -> JSON值 {
        JSON值::字符串(self.clone())
    }
}

impl<'a> 可序列化为JSON for &'a str {
    fn 转为_json(&self) -> JSON值 {
        JSON值::字符串(self.to_string())
    }
}

impl<T: 可序列化为JSON> 可序列化为JSON for Vec<T> {
    fn 转为_json(&self) -> JSON值 {
        let mut 数组 = Vec::new();
        for item in self {
            数组.push(item.转为_json());
        }
        JSON值::数组(数组)
    }
}

/// 二进制序列化特征
pub trait 可序列化为二进制 {
    fn 序列化(&self, 写入器: &mut 二进制写入器) -> Result<(), 二进制错误>;
    fn 反序列化(读取器: &mut 二进制读取器) -> Result<Self, 二进制错误> where Self: Sized;
}

/// 二进制写入器
pub struct 二进制写入器 {
    数据: Vec<u8>,
}

impl 二进制写入器 {
    pub fn 新建() -> Self {
        二进制写入器 { 数据: Vec::new() }
    }
    
    pub fn 写入整数8(&mut self, 值: i8) {
        self.数据.extend_from_slice(&值.to_be_bytes());
    }
    
    pub fn 写入整数16(&mut self, 值: i16) {
        self.数据.extend_from_slice(&值.to_be_bytes());
    }
    
    pub fn 写入整数32(&mut self, 值: i32) {
        self.数据.extend_from_slice(&值.to_be_bytes());
    }
    
    pub fn 写入整数64(&mut self, 值: i64) {
        self.数据.extend_from_slice(&值.to_be_bytes());
    }
    
    pub fn 写入浮点32(&mut self, 值: f32) {
        self.数据.extend_from_slice(&值.to_be_bytes());
    }
    
    pub fn 写入浮点64(&mut self, 值: f64) {
        self.数据.extend_from_slice(&值.to_be_bytes());
    }
    
    pub fn 写入布尔(&mut self, 值: bool) {
        self.数据.push(if 值 { 1 } else { 0 });
    }
    
    pub fn 写入字符串(&mut self, 值: &str) {
        self.写入整数32(值.len() as i32);
        self.数据.extend_from_slice(值.as_bytes());
    }
    
    pub fn 写入字节(&mut self, 数据: &[u8]) {
        self.写入整数32(数据.len() as i32);
        self.数据.extend_from_slice(数据);
    }
    
    pub fn 转为字节(&self) -> &[u8] {
        &self.数据
    }
}

/// 二进制读取器
pub struct 二进制读取器 {
    数据: Vec<u8>,
    位置: usize,
}

impl 二进制读取器 {
    pub fn 从字节(数据: &[u8]) -> Self {
        二进制读取器 { 数据: 数据.to_vec(), 位置: 0 }
    }
    
    fn 读取字节(&mut self, count: usize) -> Result<&[u8], 二进制错误> {
        if self.位置 + count > self.数据.len() {
            return Err(二进制错误::数据不足);
        }
        let slice = &self.数据[self.位置..self.位置 + count];
        self.位置 += count;
        Ok(slice)
    }
    
    pub fn 读取整数8(&mut self) -> Result<i8, 二进制错误> {
        let bytes = self.读取字节(1)?;
        Ok(i8::from_be_bytes([bytes[0]]))
    }
    
    pub fn 读取整数16(&mut self) -> Result<i16, 二进制错误> {
        let bytes = self.读取字节(2)?;
        Ok(i16::from_be_bytes([bytes[0], bytes[1]]))
    }
    
    pub fn 读取整数32(&mut self) -> Result<i32, 二进制错误> {
        let bytes = self.读取字节(4)?;
        Ok(i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    
    pub fn 读取整数64(&mut self) -> Result<i64, 二进制错误> {
        let bytes = self.读取字节(8)?;
        Ok(i64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]))
    }
    
    pub fn 读取浮点32(&mut self) -> Result<f32, 二进制错误> {
        let bytes = self.读取字节(4)?;
        Ok(f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }
    
    pub fn 读取浮点64(&mut self) -> Result<f64, 二进制错误> {
        let bytes = self.读取字节(8)?;
        Ok(f64::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7]]))
    }
    
    pub fn 读取布尔(&mut self) -> Result<bool, 二进制错误> {
        let byte = self.读取字节(1)?[0];
        Ok(byte != 0)
    }
    
    pub fn 读取字符串(&mut self) -> Result<String, 二进制错误> {
        let len = self.读取整数32()? as usize;
        let bytes = self.读取字节(len)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| 二进制错误::编码错误)
    }
    
    pub fn 读取字节数组(&mut self, 长度: usize) -> Result<Vec<u8>, 二进制错误> {
        let bytes = self.读取字节(长度)?;
        Ok(bytes.to_vec())
    }
    
    pub fn 剩余字节(&self) -> usize {
        self.数据.len() - self.位置
    }
}

/// 二进制错误
#[derive(Debug, Clone)]
pub enum 二进制错误 {
    数据不足,
    编码错误,
    验证错误(String),
}

impl std::fmt::Display for 二进制错误 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            二进制错误::数据不足 => write!(f, "二进制读取错误：数据不足"),
            二进制错误::编码错误 => write!(f, "二进制读取错误：编码错误"),
            二进制错误::验证错误(msg) => write!(f, "二进制验证错误：{}", msg),
        }
    }
}

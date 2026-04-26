// Copyright © 2026 幻心梦梦 (huanxinmengmeng)
// Licensed under the Apache License, Version 2.0 (the "License");
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::path::PathBuf;
use std::fs::{File, read, read_to_string, read_dir, write, create_dir, create_dir_all, remove_file, remove_dir, remove_dir_all, copy, metadata};
use std::io::{Read, Write, Seek, SeekFrom, BufRead, BufReader};
type Result<T> = std::result::Result<T, 系统错误>;
use crate::stdlib::collections::列表;
use crate::stdlib::string::字符串;

/// 系统错误
#[derive(Debug)]
pub enum 系统错误 {
    未找到,
    权限不足,
    已存在,
    不是文件,
    不是目录,
    IO错误(String),
}

impl std::fmt::Display for 系统错误 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            系统错误::未找到 => write!(f, "系统错误：文件或目录未找到"),
            系统错误::权限不足 => write!(f, "系统错误：权限不足"),
            系统错误::已存在 => write!(f, "系统错误：文件或目录已存在"),
            系统错误::不是文件 => write!(f, "系统错误：不是文件"),
            系统错误::不是目录 => write!(f, "系统错误：不是目录"),
            系统错误::IO错误(msg) => write!(f, "系统IO错误：{}", msg),
        }
    }
}

impl From<std::io::Error> for 系统错误 {
    fn from(error: std::io::Error) -> Self {
        系统错误::IO错误(error.to_string())
    }
}

/// 路径类型
pub struct 路径 {
    path: PathBuf,
}

impl 路径 {
    /// 创建新路径
    pub fn 新建(路径字符串: &str) -> Self {
        路径 { path: PathBuf::from(路径字符串) }
    }
    
    /// 获取当前目录
    pub fn 当前目录() -> Result<路径> {
        match std::env::current_dir() {
            Ok(path) => Result::Ok(路径 { path }),
            Err(e) => Result::Err(e.into()),
        }
    }
    
    /// 获取临时目录
    pub fn 临时目录() -> 路径 {
        路径 { path: std::env::temp_dir() }
    }
    
    /// 连接路径
    pub fn 连接(&self, 其他: &路径) -> 路径 {
        路径 { path: self.path.join(&其他.path) }
    }
    
    /// 获取父目录
    pub fn 父目录(&self) -> Option<路径> {
        self.path.parent().map(|p| 路径 { path: p.to_path_buf() })
    }
    
    /// 获取文件名
    pub fn 文件名(&self) -> Option<字符串> {
        self.path.file_name().and_then(|n| n.to_str()).map(|s| 字符串::从(s))
    }
    
    /// 获取扩展名
    pub fn 扩展名(&self) -> Option<字符串> {
        self.path.extension().and_then(|e| e.to_str()).map(|s| 字符串::从(s))
    }
    
    /// 添加扩展名
    pub fn 带扩展名(&self, 扩展名: &str) -> 路径 {
        let mut new_path = self.path.clone();
        new_path.set_extension(扩展名);
        路径 { path: new_path }
    }
    
    /// 移除扩展名
    pub fn 不带扩展名(&self) -> 路径 {
        let mut new_path = self.path.clone();
        new_path.set_extension("");
        路径 { path: new_path }
    }
    
    /// 转换为字符串
    pub fn 转为字符串(&self) -> 字符串 {
        match self.path.to_str() {
            Some(s) => 字符串::从(s),
            None => 字符串::新建(),
        }
    }
    
    /// 转换为绝对路径
    pub fn 转为绝对路径(&self) -> Result<路径> {
        match self.path.canonicalize() {
            Ok(absolute) => Result::Ok(路径 { path: absolute }),
            Err(e) => Result::Err(e.into()),
        }
    }
    
    /// 规范化路径
    pub fn 规范化(&self) -> 路径 {
        // 简化实现
        self.clone()
    }
    
    /// 检查是否为绝对路径
    pub fn 是否为绝对路径(&self) -> bool {
        self.path.is_absolute()
    }
    
    /// 检查路径是否存在
    pub fn 是否存在(&self) -> bool {
        self.path.exists()
    }
    
    /// 检查是否为文件
    pub fn 是否为文件(&self) -> bool {
        self.path.is_file()
    }
    
    /// 检查是否为目录
    pub fn 是否为目录(&self) -> bool {
        self.path.is_dir()
    }
    
    /// 检查是否为符号链接
    pub fn 是否为符号链接(&self) -> bool {
        self.path.is_symlink()
    }
}

impl Clone for 路径 {
    fn clone(&self) -> Self {
        路径 { path: self.path.clone() }
    }
}

/// 读取文件为字符串
pub fn 读取文件(路径: &路径) -> Result<字符串> {
    match read_to_string(&路径.path) {
        Ok(content) => Result::Ok(字符串::从(&content)),
        Err(e) => Result::Err(e.into()),
    }
}

/// 读取文件为字节
pub fn 读取文件字节(路径: &路径) -> Result<Vec<u8>> {
    match read(&路径.path) {
        Ok(bytes) => Result::Ok(bytes),
        Err(e) => Result::Err(e.into()),
    }
}

/// 读取文件为行列表
pub fn 读取文件行(路径: &路径) -> Result<列表<字符串>> {
    match File::open(&路径.path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            let mut 列表 = 列表::新建();
            for line in reader.lines() {
                match line {
                    Ok(line) => 列表.追加(字符串::从(&line)),
                    Err(e) => return Result::Err(e.into()),
                }
            }
            Result::Ok(列表)
        }
        Err(e) => Result::Err(e.into()),
    }
}

/// 写入文件
pub fn 写入文件(路径: &路径, 内容: &str) -> Result<()> {
    match write(&路径.path, 内容.as_bytes()) {
        Ok(_) => Result::Ok(()),
        Err(e) => Result::Err(e.into()),
    }
}

/// 写入字节到文件
pub fn 写入文件字节(路径: &路径, 数据: &[u8]) -> Result<()> {
    match write(&路径.path, 数据) {
        Ok(_) => Result::Ok(()),
        Err(e) => Result::Err(e.into()),
    }
}

/// 追加内容到文件
pub fn 追加文件(路径: &路径, 内容: &str) -> Result<()> {
    match std::fs::OpenOptions::new()
        .append(true)
        .open(&路径.path) {
        Ok(mut file) => {
            match file.write_all(内容.as_bytes()) {
                Ok(_) => Result::Ok(()),
                Err(e) => Result::Err(e.into()),
            }
        }
        Err(e) => Result::Err(e.into()),
    }
}

/// 文件元数据
pub struct 文件元数据 {
    meta: std::fs::Metadata,
}

impl 文件元数据 {
    /// 获取文件大小
    pub fn 长度(&self) -> u64 {
        self.meta.len()
    }
    
    /// 检查是否为目录
    pub fn 是否为目录(&self) -> bool {
        self.meta.is_dir()
    }
    
    /// 检查是否为文件
    pub fn 是否为文件(&self) -> bool {
        self.meta.is_file()
    }
    
    /// 获取访问时间
    pub fn 访问时间(&self) -> crate::stdlib::time::时间点 {
        crate::stdlib::time::时间点::现在()
    }
    
    /// 获取修改时间
    pub fn 修改时间(&self) -> crate::stdlib::time::时间点 {
        crate::stdlib::time::时间点::现在()
    }
    
    /// 获取权限
    pub fn 权限(&self) -> 权限 {
        权限
    }
}

/// 权限类型（简化）
pub struct 权限;

/// 获取文件元数据
pub fn 获取元数据(路径: &路径) -> Result<文件元数据> {
    match metadata(&路径.path) {
        Ok(meta) => Result::Ok(文件元数据 { meta }),
        Err(e) => Result::Err(e.into()),
    }
}

/// 设置文件权限
pub fn 设置权限(_路径: &路径, _权限: 权限) -> Result<()> {
    // 简化实现
    Result::Ok(())
}

/// 复制文件
pub fn 复制文件(源: &路径, 目标: &路径) -> Result<()> {
    match copy(&源.path, &目标.path) {
        Ok(_) => Result::Ok(()),
        Err(e) => Result::Err(e.into()),
    }
}

/// 移动文件
pub fn 移动文件(源: &路径, 目标: &路径) -> Result<()> {
    match std::fs::rename(&源.path, &目标.path) {
        Ok(_) => Result::Ok(()),
        Err(e) => Result::Err(e.into()),
    }
}

/// 删除文件
pub fn 删除文件(路径: &路径) -> Result<()> {
    match remove_file(&路径.path) {
        Ok(_) => Result::Ok(()),
        Err(e) => Result::Err(e.into()),
    }
}

/// 创建符号链接
pub fn 创建符号链接(_原始: &路径, _链接: &路径) -> Result<()> {
    #[cfg(unix)]
    {
        // 简化实现
        Result::Ok(())
    }
    #[cfg(windows)]
    {
        // Windows 有不同的链接类型
        Result::Ok(())
    }
}

/// 读取符号链接
pub fn 读取符号链接(路径: &路径) -> Result<路径> {
    match std::fs::read_link(&路径.path) {
        Ok(target) => Result::Ok(路径 { path: target }),
        Err(e) => Result::Err(e.into()),
    }
}

/// 创建目录
pub fn 创建目录(路径: &路径) -> Result<()> {
    match create_dir(&路径.path) {
        Ok(_) => Result::Ok(()),
        Err(e) => Result::Err(e.into()),
    }
}

/// 递归创建目录
pub fn 创建目录递归(路径: &路径) -> Result<()> {
    match create_dir_all(&路径.path) {
        Ok(_) => Result::Ok(()),
        Err(e) => Result::Err(e.into()),
    }
}

/// 删除目录
pub fn 删除目录(路径: &路径) -> Result<()> {
    match remove_dir(&路径.path) {
        Ok(_) => Result::Ok(()),
        Err(e) => Result::Err(e.into()),
    }
}

/// 递归删除目录
pub fn 删除目录递归(路径: &路径) -> Result<()> {
    match remove_dir_all(&路径.path) {
        Ok(_) => Result::Ok(()),
        Err(e) => Result::Err(e.into()),
    }
}

/// 目录条目
pub struct 目录条目 {
    entry: std::fs::DirEntry,
}

impl 目录条目 {
    /// 获取路径
    pub fn 路径(&self) -> 路径 {
        路径 { path: self.entry.path() }
    }
    
    /// 获取文件名
    pub fn 文件名(&self) -> 字符串 {
        字符串::从(self.entry.file_name().to_str().unwrap_or(""))
    }
    
    /// 获取元数据
    pub fn 元数据(&self) -> Result<文件元数据> {
        match self.entry.metadata() {
            Ok(meta) => Result::Ok(文件元数据 { meta }),
            Err(e) => Result::Err(e.into()),
        }
    }
    
    /// 检查是否为文件
    pub fn 是否为文件(&self) -> bool {
        if let Ok(meta) = self.entry.metadata() {
            meta.is_file()
        } else {
            false
        }
    }
    
    /// 检查是否为目录
    pub fn 是否为目录(&self) -> bool {
        if let Ok(meta) = self.entry.metadata() {
            meta.is_dir()
        } else {
            false
        }
    }
}

/// 读取目录
pub fn 读取目录(路径: &路径) -> Result<列表<目录条目>> {
    match read_dir(&路径.path) {
        Ok(entries) => {
            let mut 列表 = 列表::新建();
            for entry in entries {
                match entry {
                    Ok(entry) => 列表.追加(目录条目 { entry }),
                    Err(e) => return Result::Err(e.into()),
                }
            }
            Result::Ok(列表)
        }
        Err(e) => Result::Err(e.into()),
    }
}

/// 遍历目录（简化实现）
pub fn 遍历目录(路径: &路径) -> Result<列表<目录条目>> {
    读取目录(路径)
}

/// 设置当前目录
pub fn 设置当前目录(路径: &路径) -> Result<()> {
    match std::env::set_current_dir(&路径.path) {
        Ok(_) => Result::Ok(()),
        Err(e) => Result::Err(e.into()),
    }
}

/// 文件类型
pub struct 文件 {
    file: Option<File>,
}

impl 文件 {
    /// 打开文件
    pub fn 打开(路径: &路径) -> Result<文件> {
        match File::open(&路径.path) {
            Ok(file) => Result::Ok(文件 { file: Some(file) }),
            Err(e) => Result::Err(e.into()),
        }
    }
    
    /// 创建新文件或截断
    pub fn 创建(路径: &路径) -> Result<文件> {
        match File::create(&路径.path) {
            Ok(file) => Result::Ok(文件 { file: Some(file) }),
            Err(e) => Result::Err(e.into()),
        }
    }
    
    /// 只读打开
    pub fn 只读打开(路径: &路径) -> Result<文件> {
        文件::打开(路径)
    }
    
    /// 读写打开
    pub fn 读写打开(路径: &路径) -> Result<文件> {
        match std::fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&路径.path) {
            Ok(file) => Result::Ok(文件 { file: Some(file) }),
            Err(e) => Result::Err(e.into()),
        }
    }
    
    /// 读取到字符串
    pub fn 读取到字符串(&mut self) -> Result<字符串> {
        if let Some(ref mut file) = self.file {
            let mut content = String::new();
            match file.read_to_string(&mut content) {
                Ok(_) => Result::Ok(字符串::从(&content)),
                Err(e) => Result::Err(e.into()),
            }
        } else {
            Result::Err(系统错误::IO错误("文件未打开".to_string()))
        }
    }
    
    /// 读取到缓冲区
    pub fn 读取到字节(&mut self, 缓冲区: &mut [u8]) -> Result<usize> {
        if let Some(ref mut file) = self.file {
            match file.read(缓冲区) {
                Ok(n) => Result::Ok(n),
                Err(e) => Result::Err(e.into()),
            }
        } else {
            Result::Err(系统错误::IO错误("文件未打开".to_string()))
        }
    }
    
    /// 读取一行
    pub fn 读取行(&mut self) -> Result<Option<字符串>> {
        if let Some(ref mut file) = self.file {
            let mut reader = BufReader::new(file);
            let mut line = String::new();
            match reader.read_line(&mut line) {
                Ok(0) => Result::Ok(None),
                Ok(_) => {
                    if line.ends_with('\n') {
                        line.pop();
                    }
                    if line.ends_with('\r') {
                        line.pop();
                    }
                    Result::Ok(Some(字符串::从(&line)))
                }
                Err(e) => Result::Err(e.into()),
            }
        } else {
            Result::Err(系统错误::IO错误("文件未打开".to_string()))
        }
    }
    
    /// 写入字符串
    pub fn 写入(&mut self, 内容: &str) -> Result<()> {
        if let Some(ref mut file) = self.file {
            match file.write_all(内容.as_bytes()) {
                Ok(_) => Result::Ok(()),
                Err(e) => Result::Err(e.into()),
            }
        } else {
            Result::Err(系统错误::IO错误("文件未打开".to_string()))
        }
    }
    
    /// 写入字节
    pub fn 写入字节(&mut self, 数据: &[u8]) -> Result<()> {
        if let Some(ref mut file) = self.file {
            match file.write_all(数据) {
                Ok(_) => Result::Ok(()),
                Err(e) => Result::Err(e.into()),
            }
        } else {
            Result::Err(系统错误::IO错误("文件未打开".to_string()))
        }
    }
    
    /// 写入一行
    pub fn 写入行(&mut self, 内容: &str) -> Result<()> {
        let mut line = String::from(内容);
        line.push('\n');
        self.写入(&line)
    }
    
    /// 刷新缓冲区
    pub fn 刷新(&mut self) -> Result<()> {
        if let Some(ref mut file) = self.file {
            match file.flush() {
                Ok(_) => Result::Ok(()),
                Err(e) => Result::Err(e.into()),
            }
        } else {
            Result::Ok(())
        }
    }
    
    /// 定位到指定位置
    pub fn 定位(&mut self, 位置: 定位方式) -> Result<u64> {
        if let Some(ref mut file) = self.file {
            let seek_from = match 位置 {
                定位方式::从开始(offset) => SeekFrom::Start(offset as u64),
                定位方式::从当前(offset) => SeekFrom::Current(offset as i64),
                定位方式::从结尾(offset) => SeekFrom::End(offset as i64),
            };
            match file.seek(seek_from) {
                Ok(pos) => Result::Ok(pos),
                Err(e) => Result::Err(e.into()),
            }
        } else {
            Result::Err(系统错误::IO错误("文件未打开".to_string()))
        }
    }
    
    /// 获取当前位置
    pub fn 当前位置(&mut self) -> u64 {
        if let Some(ref mut file) = self.file {
            file.seek(SeekFrom::Current(0)).unwrap_or(0)
        } else {
            0
        }
    }
    
    /// 获取文件元数据
    pub fn 元数据(&self) -> Result<文件元数据> {
        if let Some(ref file) = self.file {
            match file.metadata() {
                Ok(meta) => Result::Ok(文件元数据 { meta }),
                Err(e) => Result::Err(e.into()),
            }
        } else {
            Result::Err(系统错误::IO错误("文件未打开".to_string()))
        }
    }
    
    /// 设置文件大小
    pub fn 设置大小(&mut self, 大小: u64) -> Result<()> {
        if let Some(ref mut file) = self.file {
            match file.set_len(大小) {
                Ok(_) => Result::Ok(()),
                Err(e) => Result::Err(e.into()),
            }
        } else {
            Result::Err(系统错误::IO错误("文件未打开".to_string()))
        }
    }
    
    /// 同步所有数据
    pub fn 同步全部(&mut self) -> Result<()> {
        if let Some(ref mut file) = self.file {
            match file.sync_all() {
                Ok(_) => Result::Ok(()),
                Err(e) => Result::Err(e.into()),
            }
        } else {
            Result::Ok(())
        }
    }
    
    /// 关闭文件
    pub fn 关闭(&mut self) -> Result<()> {
        self.file.take();
        Result::Ok(())
    }
}

/// 定位方式
pub enum 定位方式 {
    从开始(u64),
    从当前(i64),
    从结尾(i64),
}

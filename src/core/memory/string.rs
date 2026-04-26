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

use std::ptr;
use std::str;
use super::allocator::{Allocator, default_allocator};


/// UTF-8 编码的字符串
#[repr(C)]
pub struct HuanString {
    /// UTF-8 数据指针
    data: *mut u8,
    /// 字节长度（不含结尾空字符）
    length: usize,
    /// 分配器指针
    allocator: *const dyn Allocator,
}

impl HuanString {
    /// 创建新的空字符串
    pub fn new() -> Self {
        Self::new_in(default_allocator())
    }

    /// 创建新的空字符串，使用指定分配器
    pub fn new_in(allocator: &'static dyn Allocator) -> Self {
        HuanString {
            data: ptr::null_mut(),
            length: 0,
            allocator: allocator as *const _,
        }
    }

    /// 从 Rust 字符串创建
    pub fn from_str(s: &str) -> Self {
        let mut result = Self::new();
        result.push_str(s);
        result
    }

    /// 从 Rust 字符串创建，使用指定分配器
    pub fn from_str_in(s: &str, allocator: &'static dyn Allocator) -> Self {
        let mut result = Self::new_in(allocator);
        result.push_str(s);
        result
    }

    /// 获取当前长度
    pub fn len(&self) -> usize {
        self.length
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// 获取字节切片
    pub fn as_bytes(&self) -> &[u8] {
        if self.data.is_null() {
            &[]
        } else {
            unsafe {
                std::slice::from_raw_parts(self.data, self.length)
            }
        }
    }

    /// 获取字符串切片
    pub fn as_str(&self) -> &str {
        str::from_utf8(self.as_bytes()).unwrap_or("")
    }

    /// 追加字符串
    pub fn push_str(&mut self, s: &str) {
        let new_length = self.length + s.len();
        self.reserve(new_length);
        
        unsafe {
            ptr::copy_nonoverlapping(
                s.as_ptr(),
                self.data.add(self.length),
                s.len()
            );
        }
        self.length = new_length;
    }

    /// 追加单个字符
    pub fn push(&mut self, c: char) {
        let mut buffer = [0u8; 4];
        let s = c.encode_utf8(&mut buffer);
        self.push_str(s);
    }

    /// 清空字符串
    pub fn clear(&mut self) {
        self.length = 0;
    }

    /// 保留至少指定的空间
    fn reserve(&mut self, new_capacity: usize) {
        // 简化实现，每次都重新分配
        let align = std::mem::align_of::<u8>();
        let allocator = unsafe { &*self.allocator };
        
        let new_ptr = if self.data.is_null() {
            allocator.alloc(new_capacity, align)
        } else {
            let old_size = self.length;
            allocator.realloc(self.data, old_size, new_capacity, align)
        };
        
        self.data = new_ptr;
    }
}

impl Drop for HuanString {
    fn drop(&mut self) {
        if !self.data.is_null() {
            let allocator = unsafe { &*self.allocator };
            // 简化释放，实际应用中需要知道实际分配的大小
            // 这里为了简化，我们直接按当前长度释放
            allocator.free(self.data, self.length, 1);
        }
    }
}

impl Default for HuanString {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for HuanString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_basic() {
        let mut s = HuanString::new();
        assert!(s.is_empty());
        
        s.push_str("Hello");
        assert_eq!(s.as_str(), "Hello");
        assert_eq!(s.len(), 5);
        
        s.push(' ');
        s.push_str("World");
        assert_eq!(s.as_str(), "Hello World");
        assert_eq!(s.len(), 11);
        
        s.clear();
        assert!(s.is_empty());
    }
}

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
use std::marker::PhantomData;
use super::allocator::{Allocator, default_allocator};

/// 哈希表条目
#[repr(C)]
pub struct MapEntry {
    hash: u64,
    occupied: bool,
    // 实际数据紧跟在后面，这里是一个占位表示
}

/// 使用开放寻址的哈希表
#[repr(C)]
pub struct HuanMap<K, V> {
    /// 桶数组指针
    entries: *mut MapEntry,
    /// 桶数量（2 的幂）
    capacity: usize,
    /// 已占用桶数量
    length: usize,
    /// 键大小
    key_size: usize,
    /// 值大小
    value_size: usize,
    /// 分配器指针
    allocator: *const dyn Allocator,
    _marker_k: PhantomData<K>,
    _marker_v: PhantomData<V>,
}

impl<K, V> HuanMap<K, V> {
    /// 创建新的空哈希表
    pub fn new() -> Self {
        Self::new_in(default_allocator())
    }

    /// 创建新的空哈希表，使用指定分配器
    pub fn new_in(allocator: &'static dyn Allocator) -> Self {
        HuanMap {
            entries: ptr::null_mut(),
            capacity: 0,
            length: 0,
            key_size: std::mem::size_of::<K>(),
            value_size: std::mem::size_of::<V>(),
            allocator: allocator as *const _,
            _marker_k: PhantomData,
            _marker_v: PhantomData,
        }
    }

    /// 获取当前长度
    pub fn len(&self) -> usize {
        self.length
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// 获取当前容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 检查是否包含键（简化实现，需要键支持 Hash + Eq）
    /// 实际实现需要 Hash trait，但为了简化我们先不实现
    pub fn insert(&mut self, _key: K, _value: V) {
        // 简化实现，实际需要完整的哈希查找和插入逻辑
        if self.length * 4 >= self.capacity * 3 { // 负载因子 0.75
            self.grow();
        }
        // 实际的插入逻辑省略，需要实现哈希、查找冲突处理等
    }

    /// 扩容哈希表
    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            8
        } else {
            self.capacity * 2
        };
        
        // 简化实现
        self.capacity = new_capacity;
        // 实际需要重新分配内存和重新哈希所有条目
    }
}

impl<K, V> Default for HuanMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K, V> Drop for HuanMap<K, V> {
    fn drop(&mut self) {
        // 简化实现，实际需要正确析构所有条目和释放内存
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_basic() {
        let mut map = HuanMap::<i32, String>::new();
        assert_eq!(map.len(), 0);
        assert_eq!(map.capacity(), 0);
        
        // map.insert(1, "one".to_string());
        // assert_eq!(map.len(), 1);
    }
}

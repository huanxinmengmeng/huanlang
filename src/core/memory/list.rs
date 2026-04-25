// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::ptr;
use std::marker::PhantomData;
use super::allocator::{Allocator, default_allocator};

/// 动态增长的列表（类似 Vec）
#[repr(C)]
pub struct HuanList<T> {
    /// 数据指针
    data: *mut T,
    /// 当前元素数量
    length: usize,
    /// 已分配容量（元素个数）
    capacity: usize,
    /// 单个元素大小（字节）
    elem_size: usize,
    /// 分配器指针
    allocator: *const dyn Allocator,
    _marker: PhantomData<T>,
}

impl<T> HuanList<T> {
    /// 创建新的空列表，使用默认分配器
    pub fn new() -> Self {
        Self::new_in(default_allocator())
    }

    /// 创建新的空列表，使用指定分配器
    pub fn new_in(allocator: &'static dyn Allocator) -> Self {
        HuanList {
            data: ptr::null_mut(),
            length: 0,
            capacity: 0,
            elem_size: std::mem::size_of::<T>(),
            allocator: allocator as *const _,
            _marker: PhantomData,
        }
    }

    /// 获取当前长度
    pub fn len(&self) -> usize {
        self.length
    }

    /// 检查列表是否为空
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// 获取当前容量
    pub fn capacity(&self) -> usize {
        self.capacity
    }

    /// 获取元素引用
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.length {
            unsafe { Some(&*self.data.add(index)) }
        } else {
            None
        }
    }

    /// 获取元素可变引用
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.length {
            unsafe { Some(&mut *self.data.add(index)) }
        } else {
            None
        }
    }

    /// 在列表末尾添加元素
    pub fn push(&mut self, value: T) {
        if self.length >= self.capacity {
            self.grow();
        }
        unsafe {
            ptr::write(self.data.add(self.length), value);
            self.length += 1;
        }
    }

    /// 从列表末尾移除并返回元素
    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.length -= 1;
        unsafe { Some(ptr::read(self.data.add(self.length))) }
    }

    /// 清空列表
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }

    /// 扩容列表
    fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4 // 初始容量为4
        } else {
            self.capacity * 2 // 否则翻倍
        };
        
        let new_size = new_capacity * self.elem_size;
        let align = std::mem::align_of::<T>();
        let allocator = unsafe { &*self.allocator };

        let new_data = if self.data.is_null() {
            allocator.alloc(new_size, align) as *mut T
        } else {
            let old_size = self.capacity * self.elem_size;
            allocator.realloc(self.data as *mut u8, old_size, new_size, align) as *mut T
        };

        self.data = new_data;
        self.capacity = new_capacity;
    }
}

impl<T> Drop for HuanList<T> {
    fn drop(&mut self) {
        // 首先析构所有元素
        self.clear();
        
        // 然后释放内存
        if !self.data.is_null() {
            let old_size = self.capacity * self.elem_size;
            let align = std::mem::align_of::<T>();
            let allocator = unsafe { &*self.allocator };
            allocator.free(self.data as *mut u8, old_size, align);
        }
    }
}

impl<T> Default for HuanList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_growth() {
        let mut list = HuanList::new();
        assert_eq!(list.len(), 0);
        assert_eq!(list.capacity(), 0);

        list.push(42);
        assert_eq!(list.len(), 1);
        assert!(list.capacity() >= 1);

        for i in 0..100 {
            list.push(i);
        }
        assert_eq!(list.len(), 101);
        assert!(list.capacity() >= 101);
    }

    #[test]
    fn test_list_push_pop() {
        let mut list = HuanList::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.len(), 3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn test_list_get() {
        let mut list = HuanList::new();
        list.push(10);
        list.push(20);
        list.push(30);

        assert_eq!(list.get(0), Some(&10));
        assert_eq!(list.get(1), Some(&20));
        assert_eq!(list.get(2), Some(&30));
        assert_eq!(list.get(3), None);
    }
}

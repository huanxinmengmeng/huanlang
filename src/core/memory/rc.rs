// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::ptr;
use std::cell::Cell;
use std::marker::PhantomData;
use super::allocator::{Allocator, default_allocator};


/// Rc 内部数据结构
#[derive(Debug)]
struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>,
}

/// 单线程引用计数智能指针
#[derive(Debug)]
pub struct Rc<T> {
    ptr: *mut RcInner<T>,
    allocator: *const dyn Allocator,
    _marker: PhantomData<T>,
}

impl<T> Rc<T> {
    /// 使用默认分配器创建新的 Rc
    pub fn new(value: T) -> Self {
        Self::new_in(value, default_allocator())
    }

    /// 使用指定分配器创建新的 Rc
    pub fn new_in(value: T, allocator: &'static dyn Allocator) -> Self {
        let size = std::mem::size_of::<RcInner<T>>();
        let align = std::mem::align_of::<RcInner<T>>();
        let ptr = unsafe {
            let raw_ptr = allocator.alloc(size, align) as *mut RcInner<T>;
            if !raw_ptr.is_null() {
                ptr::write(raw_ptr, RcInner {
                    value,
                    ref_count: Cell::new(1),
                });
            }
            raw_ptr
        };

        Rc {
            ptr,
            allocator: allocator as *const _,
            _marker: PhantomData,
        }
    }

    /// 获取强引用计数
    pub fn strong_count(this: &Self) -> usize {
        unsafe { (*this.ptr).ref_count.get() }
    }

    /// 尝试解引用，如果引用计数为1则返回所有权
    pub fn try_unwrap(this: Self) -> Result<T, Self> {
        if Self::strong_count(&this) == 1 {
            // 安全：只有一个引用，可以安全取出
            unsafe {
                let value = ptr::read(&(*this.ptr).value);
                
                // 释放内存但不调用 drop
                let size = std::mem::size_of::<RcInner<T>>();
                let align = std::mem::align_of::<RcInner<T>>();
                let allocator = &*this.allocator;
                allocator.free(this.ptr as *mut u8, size, align);
                
                // 防止 drop 被调用
                std::mem::forget(this);
                
                Ok(value)
            }
        } else {
            Err(this)
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe {
            let ref_count = &(*self.ptr).ref_count;
            ref_count.set(ref_count.get() + 1);
        }
        Rc {
            ptr: self.ptr,
            allocator: self.allocator,
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &(*self.ptr).value }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        unsafe {
            let ref_count = &(*self.ptr).ref_count;
            let count = ref_count.get();
            
            if count == 1 {
                // 最后一个引用，析构数据并释放内存
                ptr::drop_in_place(self.ptr);
                
                let size = std::mem::size_of::<RcInner<T>>();
                let align = std::mem::align_of::<RcInner<T>>();
                let allocator = &*self.allocator;
                allocator.free(self.ptr as *mut u8, size, align);
            } else {
                // 减少引用计数
                ref_count.set(count - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_basic() {
        let rc = Rc::new(42);
        assert_eq!(*rc, 42);
        assert_eq!(Rc::strong_count(&rc), 1);

        let rc2 = rc.clone();
        assert_eq!(*rc2, 42);
        assert_eq!(Rc::strong_count(&rc), 2);

        drop(rc);
        assert_eq!(Rc::strong_count(&rc2), 1);
    }

    #[test]
    fn test_rc_try_unwrap() {
        let rc = Rc::new("hello".to_string());
        assert_eq!(Rc::strong_count(&rc), 1);
        
        let rc2 = rc.clone();
        assert!(Rc::try_unwrap(rc).is_err());
        
        let value = Rc::try_unwrap(rc2).unwrap();
        assert_eq!(value, "hello");
    }
}

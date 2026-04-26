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

/// 堆上分配的智能指针
pub struct Box<T> {
    ptr: *mut T,
    allocator: *const dyn Allocator,
    _marker: PhantomData<T>,
}

impl<T> Box<T> {
    /// 使用默认分配器创建新的 Box
    pub fn new(value: T) -> Self {
        Self::new_in(value, default_allocator())
    }

    /// 使用指定的分配器创建新的 Box
    pub fn new_in(value: T, allocator: &'static dyn Allocator) -> Self {
        let size = std::mem::size_of::<T>();
        let align = std::mem::align_of::<T>();
        let ptr = unsafe {
            let raw_ptr = allocator.alloc(size, align) as *mut T;
            if !raw_ptr.is_null() {
                ptr::write(raw_ptr, value);
            }
            raw_ptr
        };

        Box {
            ptr,
            allocator: allocator as *const _,
            _marker: PhantomData,
        }
    }

    /// 获取原始指针
    pub fn as_ptr(&self) -> *const T {
        self.ptr
    }

    /// 获取可变原始指针
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.ptr
    }

    /// 获取引用
    pub fn as_ref(&self) -> &T {
        unsafe { &*self.ptr }
    }

    /// 获取可变引用
    pub fn as_mut(&mut self) -> &mut T {
        unsafe { &mut *self.ptr }
    }
}

impl<T> Drop for Box<T> {
    fn drop(&mut self) {
        unsafe {
            // 首先调用析构函数
            ptr::drop_in_place(self.ptr);
            
            // 然后释放内存
            let size = std::mem::size_of::<T>();
            let align = std::mem::align_of::<T>();
            let allocator = &*self.allocator;
            allocator.free(self.ptr as *mut u8, size, align);
        }
    }
}

impl<T> std::ops::Deref for Box<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> std::ops::DerefMut for Box<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_box() {
        let mut b = Box::new(42);
        assert_eq!(*b, 42);
        
        *b = 100;
        assert_eq!(*b, 100);
    }
}

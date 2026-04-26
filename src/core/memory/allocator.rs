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

use std::alloc::Layout;
use std::cell::UnsafeCell;
use std::ptr::null_mut;

pub trait Allocator {
    fn alloc(&self, size: usize, align: usize) -> *mut u8;
    fn free(&self, ptr: *mut u8, size: usize, align: usize);
    fn realloc(&self, ptr: *mut u8, old_size: usize, new_size: usize, align: usize) -> *mut u8;
}

pub struct GlobalAllocator;

impl Allocator for GlobalAllocator {
    fn alloc(&self, size: usize, align: usize) -> *mut u8 {
        unsafe {
            let layout = Layout::from_size_align(size, align).unwrap();
            std::alloc::alloc(layout)
        }
    }

    fn free(&self, ptr: *mut u8, size: usize, align: usize) {
        unsafe {
            let layout = Layout::from_size_align(size, align).unwrap();
            std::alloc::dealloc(ptr, layout);
        }
    }

    fn realloc(&self, ptr: *mut u8, old_size: usize, new_size: usize, align: usize) -> *mut u8 {
        unsafe {
            let layout = Layout::from_size_align(old_size, align).unwrap();
            let new_ptr = std::alloc::realloc(ptr, layout, new_size);
            new_ptr
        }
    }
}

pub struct FirstFitAllocator {
    heap_start: usize,
    heap_size: usize,
    free_list: *mut FreeBlock,
}

#[repr(C)]
struct FreeBlock {
    size: usize,
    next: *mut FreeBlock,
}

impl FirstFitAllocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        Self {
            heap_start,
            heap_size,
            free_list: core::ptr::null_mut(),
        }
    }

    pub unsafe fn init(&mut self) {
        let block = self.heap_start as *mut FreeBlock;
        (*block).size = self.heap_size;
        (*block).next = core::ptr::null_mut();
        self.free_list = block;
    }

    unsafe fn update_free_list(&mut self, new_head: *mut FreeBlock) {
        self.free_list = new_head;
    }

    unsafe fn alloc_inner(&mut self, size: usize, align: usize) -> *mut u8 {
        let mut current = self.free_list;
        let mut prev: *mut *mut FreeBlock = core::ptr::null_mut();

        while !current.is_null() {
            let block = &mut *current;
            let block_addr = current as usize;

            let aligned_addr = (block_addr + align - 1) & !(align - 1);
            let padding = aligned_addr - block_addr;

            if block.size >= size + padding {
                let remaining = block.size - size - padding;

                if remaining > core::mem::size_of::<FreeBlock>() {
                    let new_free = (aligned_addr + size) as *mut FreeBlock;
                    (*new_free).size = remaining;
                    (*new_free).next = block.next;

                    if prev.is_null() {
                        self.update_free_list(new_free);
                    } else {
                        *prev = new_free;
                    }
                } else {
                    if prev.is_null() {
                        self.update_free_list(block.next);
                    } else {
                        *prev = block.next;
                    }
                }
                return aligned_addr as *mut u8;
            }

            prev = &mut block.next;
            current = block.next;
        }

        null_mut()
    }

    unsafe fn free_inner(&mut self, ptr: *mut u8, size: usize, _align: usize) {
        if ptr.is_null() {
            return;
        }

        let block = ptr as *mut FreeBlock;
        (*block).size = size;
        (*block).next = self.free_list;

        self.free_list = block;

        self.coalesce();
    }

    unsafe fn coalesce(&mut self) {
        let mut current = self.free_list;

        while !current.is_null() {
            let block = &mut *current;

            while !block.next.is_null() {
                let next_block = &mut *block.next;
                let current_end = current as usize + (*block).size;
                let next_start = block.next as usize;

                if current_end == next_start {
                    block.size += next_block.size;
                    block.next = next_block.next;
                } else {
                    break;
                }
            }

            current = block.next;
        }
    }
}

impl Allocator for FirstFitAllocator {
    fn alloc(&self, size: usize, align: usize) -> *mut u8 {
        unsafe {
            let mut this = Self::new(self.heap_start, self.heap_size);
            this.free_list = self.free_list;
            this.alloc_inner(size, align)
        }
    }

    fn free(&self, ptr: *mut u8, size: usize, align: usize) {
        unsafe {
            let mut this = Self::new(self.heap_start, self.heap_size);
            this.free_list = self.free_list;
            this.free_inner(ptr, size, align);
        }
    }

    fn realloc(&self, ptr: *mut u8, old_size: usize, new_size: usize, align: usize) -> *mut u8 {
        if new_size <= old_size {
            return ptr;
        }

        let new_ptr = self.alloc(new_size, align);
        if !new_ptr.is_null() {
            unsafe {
                core::ptr::copy_nonoverlapping(ptr, new_ptr, old_size);
            }
            self.free(ptr, old_size, align);
        }
        new_ptr
    }
}

pub fn default_allocator() -> &'static dyn Allocator {
    static GLOBAL: GlobalAllocator = GlobalAllocator;
    &GLOBAL
}

pub struct BumpAllocator {
    heap_start: usize,
    heap_size: usize,
    current: UnsafeCell<usize>,
}

impl BumpAllocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        Self {
            heap_start,
            heap_size,
            current: UnsafeCell::new(heap_start),
        }
    }

    pub unsafe fn init(&mut self) {
        *self.current.get() = self.heap_start;
    }
}

impl Allocator for BumpAllocator {
    fn alloc(&self, size: usize, align: usize) -> *mut u8 {
        unsafe {
            let current = *self.current.get();
            let aligned = (current + align - 1) & !(align - 1);
            let new_current = aligned + size;

            if new_current <= self.heap_start + self.heap_size {
                *self.current.get() = new_current;
                aligned as *mut u8
            } else {
                null_mut()
            }
        }
    }

    fn free(&self, _ptr: *mut u8, _size: usize, _align: usize) {
    }

    fn realloc(&self, ptr: *mut u8, old_size: usize, new_size: usize, align: usize) -> *mut u8 {
        if new_size <= old_size {
            return ptr;
        }

        let new_ptr = self.alloc(new_size, align);
        if !new_ptr.is_null() {
            unsafe {
                core::ptr::copy_nonoverlapping(ptr, new_ptr, old_size);
            }
        }
        new_ptr
    }
}

pub struct PoolAllocator {
    block_size: usize,
    block_count: usize,
    free_list: UnsafeCell<*mut FreeBlock>,
}

impl PoolAllocator {
    pub fn new(block_size: usize, block_count: usize, memory: *mut u8) -> Self {
        let mut pool = Self {
            block_size,
            block_count,
            free_list: UnsafeCell::new(core::ptr::null_mut()),
        };

        unsafe {
            pool.init(memory);
        }

        pool
    }

    unsafe fn init(&mut self, memory: *mut u8) {
        *self.free_list.get() = memory as *mut FreeBlock;

        let mut current = *self.free_list.get();
        for i in 0..self.block_count {
            let block = current;
            (*block).size = self.block_size;

            if i < self.block_count - 1 {
                let next = (memory as usize + (i + 1) * self.block_size) as *mut FreeBlock;
                (*block).next = next;
                current = next;
            } else {
                (*block).next = core::ptr::null_mut();
            }
        }
    }
}

impl Allocator for PoolAllocator {
    fn alloc(&self, size: usize, _align: usize) -> *mut u8 {
        if size > self.block_size {
            return null_mut();
        }

        unsafe {
            let free_list = &mut *self.free_list.get();
            if (*free_list).is_null() {
                return null_mut();
            }

            let block = *free_list;
            *free_list = (*block).next;

            block as *mut u8
        }
    }

    fn free(&self, ptr: *mut u8, _size: usize, _align: usize) {
        if ptr.is_null() {
            return;
        }

        unsafe {
            let block = ptr as *mut FreeBlock;
            let free_list = &mut *self.free_list.get();
            (*block).next = *free_list;
            *free_list = block;
        }
    }

    fn realloc(&self, ptr: *mut u8, old_size: usize, new_size: usize, align: usize) -> *mut u8 {
        if new_size <= old_size && new_size <= self.block_size {
            return ptr;
        }

        if new_size <= self.block_size {
            let new_ptr = self.alloc(new_size, align);
            if !new_ptr.is_null() {
                unsafe {
                    core::ptr::copy_nonoverlapping(ptr, new_ptr, old_size.min(new_size));
                }
            }
            return new_ptr;
        }

        null_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_allocator() {
        let alloc = default_allocator();

        let ptr = alloc.alloc(1024, 8);
        assert!(!ptr.is_null());

        alloc.free(ptr, 1024, 8);
    }

    #[test]
    fn test_bump_allocator() {
        let mut alloc = BumpAllocator::new(0x1000, 4096);
        unsafe {
            alloc.init();
        }

        let ptr1 = alloc.alloc(100, 8);
        assert!(!ptr1.is_null());

        let ptr2 = alloc.alloc(100, 8);
        assert!(!ptr2.is_null());

        assert!(ptr1 != ptr2);
    }

    #[test]
    fn test_pool_allocator() {
        use std::alloc::{alloc, dealloc, Layout};

        let layout = Layout::from_size_align(1024, 8).unwrap();
        let memory = unsafe { alloc(layout) };

        let mut alloc = PoolAllocator::new(64, 16, memory);

        let ptr1 = alloc.alloc(64, 8);
        assert!(!ptr1.is_null());

        let ptr2 = alloc.alloc(64, 8);
        assert!(!ptr2.is_null());

        alloc.free(ptr1, 64, 8);

        let ptr3 = alloc.alloc(64, 8);
        assert!(!ptr3.is_null());
        assert_eq!(ptr1, ptr3);

        unsafe {
            dealloc(memory, layout);
        }
    }
}
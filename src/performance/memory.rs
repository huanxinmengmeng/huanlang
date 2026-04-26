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

//! 内存优化模块
//!
//! 提供内存使用追踪和优化功能：
//! - 内存分配追踪
//! - 内存池管理
//! - 对象池复用
//! - 小对象优化
//! - 内存对齐优化

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// 内存追踪器
pub struct MemoryTracker {
    total_allocations: AtomicU64,
    total_bytes: AtomicU64,
    current_bytes: AtomicU64,
    peak_bytes: AtomicU64,
    allocation_count: AtomicU64,
    deallocation_count: AtomicU64,
}

/// 内存统计信息
#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_allocations: u64,
    pub total_bytes: u64,
    pub current_bytes: u64,
    pub peak_bytes: u64,
    pub allocation_count: u64,
    pub deallocation_count: u64,
}

/// 内存池配置
#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub chunk_size: usize,
    pub initial_chunks: usize,
    pub max_chunks: usize,
    pub growth_factor: f32,
}

/// 内存池
pub struct MemoryPool {
    config: PoolConfig,
    free_chunks: Vec<Vec<u8>>,
    allocated_chunks: Vec<Vec<u8>>,
    chunk_size: usize,
}

/// 小对象分配器
pub struct SmallObjectAllocator {
    pools: HashMap<usize, MemoryPool>,
    page_size: usize,
}

/// 内存对齐工具
pub struct MemoryAligner;

impl MemoryTracker {
    /// 创建新的内存追踪器（全局单例）
    pub fn new() -> Self {
        MemoryTracker {
            total_allocations: AtomicU64::new(0),
            total_bytes: AtomicU64::new(0),
            current_bytes: AtomicU64::new(0),
            peak_bytes: AtomicU64::new(0),
            allocation_count: AtomicU64::new(0),
            deallocation_count: AtomicU64::new(0),
        }
    }

    /// 记录分配
    pub fn track_allocation(&self, size: u64) {
        self.total_allocations.fetch_add(1, Ordering::Relaxed);
        self.total_bytes.fetch_add(size, Ordering::Relaxed);
        self.current_bytes.fetch_add(size, Ordering::Relaxed);
        self.allocation_count.fetch_add(1, Ordering::Relaxed);

        // 更新峰值
        let current = self.current_bytes.load(Ordering::Relaxed);
        let peak = self.peak_bytes.load(Ordering::Relaxed);
        if current > peak {
            self.peak_bytes.store(current, Ordering::Relaxed);
        }
    }

    /// 记录释放
    pub fn track_deallocation(&self, size: u64) {
        self.current_bytes.fetch_sub(size, Ordering::Relaxed);
        self.deallocation_count.fetch_add(1, Ordering::Relaxed);
    }

    /// 获取统计信息
    pub fn get_stats(&self) -> MemoryStats {
        MemoryStats {
            total_allocations: self.total_allocations.load(Ordering::Relaxed),
            total_bytes: self.total_bytes.load(Ordering::Relaxed),
            current_bytes: self.current_bytes.load(Ordering::Relaxed),
            peak_bytes: self.peak_bytes.load(Ordering::Relaxed),
            allocation_count: self.allocation_count.load(Ordering::Relaxed),
            deallocation_count: self.deallocation_count.load(Ordering::Relaxed),
        }
    }

    /// 重置统计
    pub fn reset(&self) {
        self.total_allocations.store(0, Ordering::Relaxed);
        self.total_bytes.store(0, Ordering::Relaxed);
        self.current_bytes.store(0, Ordering::Relaxed);
        self.peak_bytes.store(0, Ordering::Relaxed);
        self.allocation_count.store(0, Ordering::Relaxed);
        self.deallocation_count.store(0, Ordering::Relaxed);
    }
}

impl Default for MemoryTracker {
    fn default() -> Self {
        Self::new()
    }
}

impl PoolConfig {
    /// 创建默认配置
    pub fn default_config(chunk_size: usize) -> Self {
        PoolConfig {
            chunk_size,
            initial_chunks: 16,
            max_chunks: 1024,
            growth_factor: 1.5,
        }
    }

    /// 创建小对象池配置
    pub fn small_object_config() -> Self {
        PoolConfig {
            chunk_size: 64,
            initial_chunks: 256,
            max_chunks: 8192,
            growth_factor: 2.0,
        }
    }

    /// 创建中等对象池配置
    pub fn medium_object_config() -> Self {
        PoolConfig {
            chunk_size: 1024,
            initial_chunks: 128,
            max_chunks: 4096,
            growth_factor: 1.5,
        }
    }
}

impl MemoryPool {
    /// 创建新的内存池
    pub fn new(config: PoolConfig) -> Self {
        let mut pool = MemoryPool {
            config: config.clone(),
            free_chunks: Vec::with_capacity(config.max_chunks),
            allocated_chunks: Vec::with_capacity(config.max_chunks),
            chunk_size: config.chunk_size,
        };

        // 预分配初始块
        for _ in 0..config.initial_chunks {
            pool.free_chunks.push(vec![0u8; config.chunk_size]);
        }

        pool
    }

    /// 分配内存
    pub fn allocate(&mut self) -> Option<Vec<u8>> {
        // 尝试从空闲列表获取
        if let Some(chunk) = self.free_chunks.pop() {
            self.allocated_chunks.push(chunk.clone());
            return Some(chunk);
        }

        // 如果没有空闲块且可以增长，则分配新块
        if self.allocated_chunks.len() < self.config.max_chunks {
            let new_chunk = vec![0u8; self.chunk_size];
            self.allocated_chunks.push(new_chunk.clone());
            return Some(new_chunk);
        }

        None
    }

    /// 释放内存
    pub fn deallocate(&mut self, chunk: Vec<u8>) {
        if self.free_chunks.len() < self.config.max_chunks {
            // 重置内存内容
            let mut chunk = chunk;
            chunk.resize(self.chunk_size, 0);
            self.free_chunks.push(chunk);
        }
        
        // 从已分配列表中移除
        self.allocated_chunks.retain(|c| c.as_ptr() != chunk.as_ptr());
    }

    /// 获取空闲块数量
    pub fn free_count(&self) -> usize {
        self.free_chunks.len()
    }

    /// 获取已分配块数量
    pub fn allocated_count(&self) -> usize {
        self.allocated_chunks.len()
    }

    /// 获取内存使用量
    pub fn memory_usage(&self) -> usize {
        (self.free_chunks.len() + self.allocated_chunks.len()) * self.chunk_size
    }

    /// 清除所有空闲块
    pub fn trim(&mut self) {
        let target_size = (self.config.initial_chunks as f32 * self.config.growth_factor) as usize;
        while self.free_chunks.len() > target_size {
            self.free_chunks.pop();
        }
    }
}

impl SmallObjectAllocator {
    /// 创建新的小对象分配器
    pub fn new() -> Self {
        SmallObjectAllocator {
            pools: HashMap::new(),
            page_size: 4096,
        }
    }

    /// 获取指定大小的池
    fn get_pool(&mut self, size: usize) -> &mut MemoryPool {
        if !self.pools.contains_key(&size) {
            let config = PoolConfig::default_config(size);
            self.pools.insert(size, MemoryPool::new(config));
        }
        self.pools.get_mut(&size).unwrap()
    }

    /// 分配小对象
    pub fn allocate(&mut self, size: usize) -> Option<Vec<u8>> {
        if size == 0 {
            return Some(Vec::new());
        }

        // 对齐到 8 字节
        let aligned_size = (size + 7) & !7;
        self.get_pool(aligned_size).allocate()
    }

    /// 释放小对象
    pub fn deallocate(&mut self, chunk: Vec<u8>) {
        let size = chunk.capacity();
        if size == 0 {
            return;
        }

        let aligned_size = (size + 7) & !7;
        if let Some(pool) = self.pools.get_mut(&aligned_size) {
            pool.deallocate(chunk);
        }
    }

    /// 获取内存使用统计
    pub fn get_memory_usage(&self) -> usize {
        self.pools.values().map(|p| p.memory_usage()).sum()
    }

    /// 优化内存使用
    pub fn trim(&mut self) {
        for pool in self.pools.values_mut() {
            pool.trim();
        }
    }
}

impl Default for SmallObjectAllocator {
    fn default() -> Self {
        Self::new()
    }
}

impl MemoryAligner {
    /// 对齐到指定边界
    pub fn align<T>(value: usize, alignment: usize) -> usize {
        if alignment == 0 {
            return value;
        }
        (value + alignment - 1) & !(alignment - 1)
    }

    /// 检查是否对齐
    pub fn is_aligned<T>(value: usize) -> bool {
        let alignment = std::mem::align_of::<T>();
        value % alignment == 0
    }

    /// 计算结构体对齐后的大小
    pub fn aligned_size<T>() -> usize {
        let size = std::mem::size_of::<T>();
        let alignment = std::mem::align_of::<T>();
        Self::align(size, alignment)
    }

    /// 计算结构体数组的对齐大小
    pub fn aligned_array_size<T>(count: usize) -> usize {
        Self::aligned_size::<T>() * count
    }

    /// 缓存行对齐
    pub fn cache_line_align(size: usize) -> usize {
        Self::align(size, 64)
    }
}

/// 对象池 - 用于复用大型对象
pub struct ObjectPool<T> {
    available: Vec<Box<T>>,
    in_use: usize,
    factory: Box<dyn Fn() -> Box<T>>,
}

impl<T: 'static> ObjectPool<T> {
    /// 创建新的对象池
    pub fn new(factory: impl Fn() -> T + 'static) -> Self {
        ObjectPool {
            available: Vec::new(),
            in_use: 0,
            factory: Box::new(move || Box::new(factory())),
        }
    }

    /// 获取对象
    pub fn acquire(&mut self) -> Box<T> {
        self.in_use += 1;
        self.available.pop().unwrap_or_else(|| (self.factory)())
    }

    /// 归还对象
    pub fn release(&mut self, obj: Box<T>) {
        self.in_use -= 1;
        self.available.push(obj);
    }

    /// 预热池
    pub fn warm(&mut self, count: usize) {
        for _ in 0..count {
            self.available.push((self.factory)());
        }
    }

    /// 获取可用对象数量
    pub fn available_count(&self) -> usize {
        self.available.len()
    }

    /// 获取正在使用的对象数量
    pub fn in_use_count(&self) -> usize {
        self.in_use
    }

    /// 清理池
    pub fn clear(&mut self) {
        self.available.clear();
    }
}

/// 环形缓冲区 - 用于高效的生产者-消费者模式
pub struct RingBuffer<T> {
    buffer: Vec<T>,
    read_index: usize,
    write_index: usize,
    capacity: usize,
}

impl<T: Default + Clone> RingBuffer<T> {
    /// 创建新的环形缓冲区
    pub fn new(capacity: usize) -> Self {
        let buffer = vec![T::default(); capacity];
        RingBuffer {
            buffer,
            read_index: 0,
            write_index: 0,
            capacity,
        }
    }

    /// 写入数据
    pub fn write(&mut self, item: T) -> bool {
        let next_write = (self.write_index + 1) % self.capacity;
        if next_write == self.read_index {
            return false; // 缓冲区已满
        }
        self.buffer[self.write_index] = item;
        self.write_index = next_write;
        true
    }

    /// 读取数据
    pub fn read(&mut self) -> Option<T> {
        if self.read_index == self.write_index {
            return None; // 缓冲区为空
        }
        let item = self.buffer[self.read_index].clone();
        self.read_index = (self.read_index + 1) % self.capacity;
        Some(item)
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.read_index == self.write_index
    }

    /// 检查是否已满
    pub fn is_full(&self) -> bool {
        (self.write_index + 1) % self.capacity == self.read_index
    }

    /// 获取可用空间
    pub fn available_space(&self) -> usize {
        if self.write_index >= self.read_index {
            self.capacity - (self.write_index - self.read_index) - 1
        } else {
            self.read_index - self.write_index - 1
        }
    }

    /// 获取已使用空间
    pub fn used_space(&self) -> usize {
        self.capacity - self.available_space() - 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_tracker() {
        let tracker = MemoryTracker::new();
        
        tracker.track_allocation(100);
        tracker.track_allocation(200);
        tracker.track_deallocation(100);
        
        let stats = tracker.get_stats();
        assert_eq!(stats.total_allocations, 2);
        assert_eq!(stats.total_bytes, 300);
        assert_eq!(stats.current_bytes, 200);
    }

    #[test]
    fn test_memory_pool() {
        let config = PoolConfig::default_config(64);
        let mut pool = MemoryPool::new(config);
        
        assert_eq!(pool.free_count(), 16);
        
        let chunk = pool.allocate();
        assert!(chunk.is_some());
        assert_eq!(pool.free_count(), 15);
        
        if let Some(c) = chunk {
            pool.deallocate(c);
        }
        assert_eq!(pool.free_count(), 16);
    }

    #[test]
    fn test_small_object_allocator() {
        let mut allocator = SmallObjectAllocator::new();
        
        let chunk = allocator.allocate(50);
        assert!(chunk.is_some());
        assert!(allocator.get_memory_usage() >= 64);
    }

    #[test]
    fn test_memory_aligner() {
        assert_eq!(MemoryAligner::align(10, 8), 16);
        assert_eq!(MemoryAligner::align(16, 8), 16);
        assert_eq!(MemoryAligner::align(17, 8), 24);
    }

    #[test]
    fn test_object_pool() {
        let mut pool = ObjectPool::new(|| Vec::<i32>::new());
        
        let mut obj1 = pool.acquire();
        obj1.push(1);
        pool.release(obj1);
        
        let mut obj2 = pool.acquire();
        obj2.push(2);
        
        assert_eq!(pool.in_use_count(), 1);
        assert_eq!(pool.available_count(), 0);
    }

    #[test]
    fn test_ring_buffer() {
        let mut buffer = RingBuffer::new(5);
        
        assert!(buffer.is_empty());
        assert!(!buffer.is_full());
        
        buffer.write(1);
        buffer.write(2);
        buffer.write(3);
        
        assert_eq!(buffer.read(), Some(1));
        assert_eq!(buffer.read(), Some(2));
        
        buffer.write(4);
        buffer.write(5);
        assert!(buffer.is_full());
        
        buffer.write(6); // 应该失败，缓冲区已满
        assert_eq!(buffer.used_space(), 4);
    }
}

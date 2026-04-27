use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_allocations: usize,
    pub total_deallocations: usize,
    pub current_bytes: usize,
    pub peak_bytes: usize,
    pub allocation_sizes: HashMap<usize, usize>,
}

pub struct MemoryTracker {
    total_allocations: usize,
    total_deallocations: usize,
    current_bytes: usize,
    peak_bytes: usize,
    allocation_sizes: HashMap<usize, usize>,
}

impl MemoryTracker {
    pub fn new() -> Self {
        MemoryTracker {
            total_allocations: 0,
            total_deallocations: 0,
            current_bytes: 0,
            peak_bytes: 0,
            allocation_sizes: HashMap::new(),
        }
    }

    pub fn track_allocation(&mut self, size: usize) {
        self.total_allocations += 1;
        self.current_bytes += size;
        if self.current_bytes > self.peak_bytes {
            self.peak_bytes = self.current_bytes;
        }
        *self.allocation_sizes.entry(size).or_insert(0) += 1;
    }

    pub fn track_deallocation(&mut self, size: usize) {
        self.total_deallocations += 1;
        if self.current_bytes >= size {
            self.current_bytes -= size;
        }
    }

    pub fn get_stats(&self) -> MemoryStats {
        MemoryStats {
            total_allocations: self.total_allocations,
            total_deallocations: self.total_deallocations,
            current_bytes: self.current_bytes,
            peak_bytes: self.peak_bytes,
            allocation_sizes: self.allocation_sizes.clone(),
        }
    }

    pub fn reset(&mut self) {
        self.total_allocations = 0;
        self.total_deallocations = 0;
        self.current_bytes = 0;
        self.peak_bytes = 0;
        self.allocation_sizes.clear();
    }
}

impl Default for MemoryTracker {
    fn default() -> Self {
        MemoryTracker::new()
    }
}

#[derive(Debug, Clone)]
pub struct PoolConfig {
    pub chunk_size: usize,
    pub initial_chunks: usize,
    pub max_chunks: usize,
    pub alignment: usize,
}

impl PoolConfig {
    pub fn default_config(chunk_size: usize) -> Self {
        PoolConfig {
            chunk_size,
            initial_chunks: 16,
            max_chunks: 1024,
            alignment: 8,
        }
    }

    pub fn small_object_config() -> Self {
        PoolConfig::default_config(64)
    }

    pub fn medium_object_config() -> Self {
        PoolConfig::default_config(1024)
    }
}

pub struct MemoryPool {
    config: PoolConfig,
    free_chunks: VecDeque<Vec<u8>>,
    allocated_count: usize,
}

impl MemoryPool {
    pub fn new(config: PoolConfig) -> Self {
        let mut pool = MemoryPool {
            config,
            free_chunks: VecDeque::new(),
            allocated_count: 0,
        };
        pool.preallocate();
        pool
    }

    fn preallocate(&mut self) {
        for _ in 0..self.config.initial_chunks {
            self.free_chunks.push_back(vec![0u8; self.config.chunk_size]);
        }
    }

    pub fn allocate(&mut self) -> Option<Vec<u8>> {
        if let Some(chunk) = self.free_chunks.pop_front() {
            self.allocated_count += 1;
            return Some(chunk);
        }
        if self.allocated_count < self.config.max_chunks {
            self.allocated_count += 1;
            return Some(vec![0u8; self.config.chunk_size]);
        }
        None
    }

    pub fn deallocate(&mut self, mut chunk: Vec<u8>) {
        chunk.resize(self.config.chunk_size, 0);
        self.free_chunks.push_back(chunk);
        self.allocated_count = self.allocated_count.saturating_sub(1);
    }

    pub fn free_count(&self) -> usize {
        self.free_chunks.len()
    }

    pub fn allocated_count(&self) -> usize {
        self.allocated_count
    }
}

pub struct SmallObjectAllocator {
    pools: HashMap<usize, MemoryPool>,
    tracker: MemoryTracker,
}

impl SmallObjectAllocator {
    pub fn new() -> Self {
        SmallObjectAllocator {
            pools: HashMap::new(),
            tracker: MemoryTracker::new(),
        }
    }

    pub fn allocate(&mut self, size: usize) -> Option<Vec<u8>> {
        let aligned_size = self.align(size, 8);
        let pool_size = Self::get_pool_size(aligned_size);
        
        if !self.pools.contains_key(&pool_size) {
            let config = PoolConfig::default_config(pool_size);
            self.pools.insert(pool_size, MemoryPool::new(config));
        }
        
        self.tracker.track_allocation(aligned_size);
        self.pools.get_mut(&pool_size).and_then(|pool| pool.allocate())
    }

    pub fn deallocate(&mut self, mut chunk: Vec<u8>) {
        let size = chunk.len();
        let pool_size = Self::get_pool_size(size);
        if let Some(pool) = self.pools.get_mut(&pool_size) {
            self.tracker.track_deallocation(size);
            chunk.resize(pool_size, 0);
            pool.deallocate(chunk);
        }
    }

    pub fn get_memory_usage(&self) -> usize {
        self.tracker.get_stats().current_bytes
    }

    fn align(&self, size: usize, alignment: usize) -> usize {
        (size + alignment - 1) & !(alignment - 1)
    }

    fn get_pool_size(size: usize) -> usize {
        let mut pool_size = 8;
        while pool_size < size {
            pool_size *= 2;
        }
        pool_size
    }
}

impl Default for SmallObjectAllocator {
    fn default() -> Self {
        SmallObjectAllocator::new()
    }
}

pub struct ObjectPool<T> {
    factory: Box<dyn Fn() -> T>,
    available: VecDeque<T>,
    in_use: usize,
}

impl<T> ObjectPool<T> {
    pub fn new(factory: impl Fn() -> T + 'static) -> Self {
        ObjectPool {
            factory: Box::new(factory),
            available: VecDeque::new(),
            in_use: 0,
        }
    }

    pub fn warm(&mut self, count: usize) {
        for _ in 0..count {
            self.available.push_back((self.factory)());
        }
    }

    pub fn acquire(&mut self) -> T {
        self.in_use += 1;
        self.available.pop_front().unwrap_or_else(|| (self.factory)())
    }

    pub fn release(&mut self, obj: T) {
        self.available.push_back(obj);
        self.in_use = self.in_use.saturating_sub(1);
    }

    pub fn available_count(&self) -> usize {
        self.available.len()
    }

    pub fn in_use_count(&self) -> usize {
        self.in_use
    }
}

pub struct RingBuffer<T> {
    buffer: Vec<Option<T>>,
    capacity: usize,
    read_pos: usize,
    write_pos: usize,
    count: usize,
}

impl<T> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        RingBuffer {
            buffer: (0..capacity).map(|_| None).collect(),
            capacity,
            read_pos: 0,
            write_pos: 0,
            count: 0,
        }
    }

    pub fn write(&mut self, value: T) -> bool {
        if self.count >= self.capacity {
            return false;
        }
        self.buffer[self.write_pos] = Some(value);
        self.write_pos = (self.write_pos + 1) % self.capacity;
        self.count += 1;
        true
    }

    pub fn read(&mut self) -> Option<T> {
        if self.count == 0 {
            return None;
        }
        let value = self.buffer[self.read_pos].take();
        self.read_pos = (self.read_pos + 1) % self.capacity;
        self.count -= 1;
        value
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn is_full(&self) -> bool {
        self.count >= self.capacity
    }

    pub fn used_space(&self) -> usize {
        self.count
    }
}

pub struct MemoryAligner;

impl MemoryAligner {
    pub fn align(size: usize, alignment: usize) -> usize {
        (size + alignment - 1) & !(alignment - 1)
    }

    pub fn is_aligned<T>(ptr: *const T) -> bool {
        (ptr as usize) % std::mem::align_of::<T>() == 0
    }

    pub fn aligned_size<T>() -> usize {
        std::mem::size_of::<T>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_tracker() {
        let mut tracker = MemoryTracker::new();
        tracker.track_allocation(100);
        tracker.track_allocation(200);
        tracker.track_deallocation(100);
        
        let stats = tracker.get_stats();
        assert_eq!(stats.total_allocations, 2);
        assert_eq!(stats.total_deallocations, 1);
        assert_eq!(stats.current_bytes, 200);
    }

    #[test]
    fn test_memory_pool() {
        let config = PoolConfig::default_config(64);
        let mut pool = MemoryPool::new(config);

        let chunk = pool.allocate();
        assert!(chunk.is_some());

        pool.deallocate(chunk.unwrap());
        assert_eq!(pool.free_count(), 16);
    }

    #[test]
    fn test_ring_buffer() {
        let mut buffer = RingBuffer::<i32>::new(3);
        assert!(buffer.is_empty());
        
        buffer.write(1);
        buffer.write(2);
        buffer.write(3);
        assert!(buffer.is_full());
        
        assert_eq!(buffer.read(), Some(1));
        assert_eq!(buffer.read(), Some(2));
        assert_eq!(buffer.read(), Some(3));
        assert_eq!(buffer.read(), None);
    }

    #[test]
    fn test_memory_aligner() {
        assert_eq!(MemoryAligner::align(10, 8), 16);
        assert_eq!(MemoryAligner::align(8, 8), 8);
    }
}

// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::sync::{
    Mutex as StdMutex, MutexGuard as StdMutexGuard,
    RwLock as StdRwLock, RwLockReadGuard as StdRwLockReadGuard,
    RwLockWriteGuard as StdRwLockWriteGuard,
    atomic::{AtomicI32 as StdAtomicI32, AtomicBool as StdAtomicBool, Ordering as StdOrdering},
    Barrier as StdBarrier, Once as StdOnce
};
use std::ops::{Deref, DerefMut};

/// 互斥锁
pub struct Mutex<T> {
    inner: StdMutex<T>,
}

/// 互斥锁守卫
pub struct MutexGuard<'a, T> {
    guard: StdMutexGuard<'a, T>,
}

/// 读写锁
pub struct RwLock<T> {
    inner: StdRwLock<T>,
}

/// 读写锁读守卫
pub struct RwLockReadGuard<'a, T> {
    guard: StdRwLockReadGuard<'a, T>,
}

/// 读写锁写守卫
pub struct RwLockWriteGuard<'a, T> {
    guard: StdRwLockWriteGuard<'a, T>,
}

/// 原子整数（32位）
pub struct AtomicI32 {
    inner: StdAtomicI32,
}

/// 原子布尔
pub struct AtomicBool {
    inner: StdAtomicBool,
}

/// 内存顺序
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ordering {
    Relaxed,
    Acquire,
    Release,
    AcqRel,
    SeqCst,
}

/// 同步屏障
pub struct Barrier {
    inner: StdBarrier,
}

/// 同步屏障等待结果
pub struct BarrierWaitResult {
    pub is_leader: bool,
}

/// 一次性执行
pub struct Once {
    inner: StdOnce,
}

impl<T> Mutex<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: StdMutex::new(value),
        }
    }

    /// 获取锁，返回守卫
    pub fn lock(&self) -> MutexGuard<'_, T> {
        MutexGuard {
            guard: self.inner.lock().unwrap(),
        }
    }

    /// 尝试获取锁
    pub fn try_lock(&self) -> Option<MutexGuard<'_, T>> {
        self.inner.try_lock().ok().map(|g| MutexGuard { guard: g })
    }
}

impl<T> Deref for MutexGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T> DerefMut for MutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

impl<T> RwLock<T> {
    pub fn new(value: T) -> Self {
        Self {
            inner: StdRwLock::new(value),
        }
    }

    pub fn read(&self) -> RwLockReadGuard<'_, T> {
        RwLockReadGuard {
            guard: self.inner.read().unwrap(),
        }
    }

    pub fn write(&self) -> RwLockWriteGuard<'_, T> {
        RwLockWriteGuard {
            guard: self.inner.write().unwrap(),
        }
    }
}

impl<T> Deref for RwLockReadGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T> Deref for RwLockWriteGuard<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.guard
    }
}

impl<T> DerefMut for RwLockWriteGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.guard
    }
}

impl AtomicI32 {
    pub fn new(value: i32) -> Self {
        Self {
            inner: StdAtomicI32::new(value),
        }
    }

    pub fn load(&self, ordering: Ordering) -> i32 {
        self.inner.load(ordering.into())
    }

    pub fn store(&self, value: i32, ordering: Ordering) {
        self.inner.store(value, ordering.into());
    }

    pub fn fetch_add(&self, value: i32, ordering: Ordering) -> i32 {
        self.inner.fetch_add(value, ordering.into())
    }

    pub fn compare_exchange(
        &self,
        current: i32,
        new: i32,
        success: Ordering,
        failure: Ordering,
    ) -> Result<i32, i32> {
        self.inner.compare_exchange(current, new, success.into(), failure.into())
    }
}

impl AtomicBool {
    pub fn new(value: bool) -> Self {
        Self {
            inner: StdAtomicBool::new(value),
        }
    }

    pub fn load(&self, ordering: Ordering) -> bool {
        self.inner.load(ordering.into())
    }

    pub fn store(&self, value: bool, ordering: Ordering) {
        self.inner.store(value, ordering.into());
    }

    pub fn fetch_or(&self, value: bool, ordering: Ordering) -> bool {
        self.inner.fetch_or(value, ordering.into())
    }
}

impl From<Ordering> for StdOrdering {
    fn from(ordering: Ordering) -> Self {
        match ordering {
            Ordering::Relaxed => StdOrdering::Relaxed,
            Ordering::Acquire => StdOrdering::Acquire,
            Ordering::Release => StdOrdering::Release,
            Ordering::AcqRel => StdOrdering::AcqRel,
            Ordering::SeqCst => StdOrdering::SeqCst,
        }
    }
}

impl Barrier {
    pub fn new(n: usize) -> Self {
        Self {
            inner: StdBarrier::new(n),
        }
    }

    pub fn wait(&self) -> BarrierWaitResult {
        BarrierWaitResult {
            is_leader: self.inner.wait().is_leader(),
        }
    }
}

impl Once {
    pub const fn new() -> Self {
        Self {
            inner: StdOnce::new(),
        }
    }

    pub fn call_once<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        self.inner.call_once(f);
    }
}

// 自动实现 Send/Sync 标记
unsafe impl<T: Send> Send for Mutex<T> {}
unsafe impl<T: Send> Sync for Mutex<T> {}
unsafe impl<T: Send> Send for RwLock<T> {}
unsafe impl<T: Send> Sync for RwLock<T> {}
unsafe impl Send for AtomicI32 {}
unsafe impl Sync for AtomicI32 {}
unsafe impl Send for AtomicBool {}
unsafe impl Sync for AtomicBool {}
unsafe impl Send for Barrier {}
unsafe impl Sync for Barrier {}
unsafe impl Send for Once {}
unsafe impl Sync for Once {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use crate::core::concurrent::tasks::TaskGroup;

    #[test]
    fn test_mutex() {
        let mutex = Arc::new(Mutex::new(0));
        let mut group = TaskGroup::new();

        for _ in 0..10 {
            let m = mutex.clone();
            group.spawn(move || {
                let mut guard = m.lock();
                *guard += 1;
            });
        }

        group.wait_all();
        assert_eq!(*mutex.lock(), 10);
    }

    #[test]
    fn test_atomic() {
        let counter = Arc::new(AtomicI32::new(0));
        let mut group = TaskGroup::new();

        for _ in 0..100 {
            let c = counter.clone();
            group.spawn(move || {
                c.fetch_add(1, Ordering::Relaxed);
            });
        }

        group.wait_all();
        assert_eq!(counter.load(Ordering::Relaxed), 100);
    }
}

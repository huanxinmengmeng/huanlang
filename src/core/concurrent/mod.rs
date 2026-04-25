// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

pub mod tasks;
pub mod channel;
pub mod sync;
pub mod error;

pub use tasks::{TaskId, TaskHandle, TaskGroup, CancellationToken};
pub use channel::{Channel, Sender, Receiver, Iter};
pub use sync::{
    Mutex, MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard,
    AtomicI32, AtomicBool, Ordering, Barrier, BarrierWaitResult, Once
};
pub use error::{ConcurrentError, TaskError};

// 注释掉自定义的 Send 和 Sync traits，使用 Rust 标准库提供的
// unsafe auto trait Send {}
// unsafe auto trait Sync {}

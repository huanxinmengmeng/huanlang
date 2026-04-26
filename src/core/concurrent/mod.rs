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

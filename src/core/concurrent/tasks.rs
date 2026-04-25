// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::sync::mpsc;
use std::thread::{self, JoinHandle};
use crate::core::concurrent::error::TaskError;

/// 任务标识符
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(u64);

/// 任务句柄，用于获取任务结果或取消任务
pub struct TaskHandle<T> {
    _id: TaskId,
    receiver: mpsc::Receiver<Result<T, TaskError>>,
}

/// 任务组，管理子任务的生命周期
pub struct TaskGroup {
    handles: Vec<JoinHandle<()>>,
    _cancel_sender: Option<mpsc::Sender<()>>,
    next_id: u64,
}

impl TaskGroup {
    /// 创建新的任务组
    pub fn new() -> Self {
        Self {
            handles: Vec::new(),
            _cancel_sender: None,
            next_id: 0,
        }
    }

    /// 创建带取消支持的任务组
    pub fn with_cancellation() -> (Self, CancellationToken) {
        let (tx, rx) = mpsc::channel();
        let token = CancellationToken { receiver: Some(rx) };
        let group = Self {
            handles: Vec::new(),
            _cancel_sender: Some(tx),
            next_id: 0,
        };
        (group, token)
    }

    /// 在任务组内创建子任务
    pub fn spawn<F, T>(&mut self, f: F) -> TaskHandle<T>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        let (tx, rx) = mpsc::channel();
        let task_id = TaskId(self.next_id);
        self.next_id += 1;

        let handle = thread::spawn(move || {
            let result = f();
            let _ = tx.send(Ok(result));
        });
        self.handles.push(handle);

        TaskHandle {
            _id: task_id,
            receiver: rx,
        }
    }

    /// 等待所有子任务完成
    pub fn wait_all(mut self) -> Vec<Result<(), Box<dyn std::any::Any + Send + 'static>>> {
        std::mem::take(&mut self.handles)
            .into_iter()
            .map(|h| h.join())
            .collect()
    }
}

impl<T> TaskHandle<T> {
    /// 获取任务结果（阻塞直到任务完成）
    pub fn result(self) -> Result<T, TaskError> {
        self.receiver.recv().unwrap_or(Err(TaskError::Cancelled))
    }
}

/// 取消令牌，用于发出取消信号
pub struct CancellationToken {
    receiver: Option<mpsc::Receiver<()>>,
}

impl CancellationToken {
    /// 检查是否已取消
    pub fn is_cancelled(&self) -> bool {
        self.receiver.as_ref().map_or(false, |rx| rx.try_recv().is_ok())
    }
}

/// 在任务组离开作用域时自动等待
impl Drop for TaskGroup {
    fn drop(&mut self) {
        for handle in self.handles.drain(..) {
            let _ = handle.join();
        }
    }
}

impl Default for TaskGroup {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_group_spawn() {
        let mut group = TaskGroup::new();
        let handle = group.spawn(|| 42);
        group.wait_all();
        assert_eq!(handle.result().unwrap(), 42);
    }
}

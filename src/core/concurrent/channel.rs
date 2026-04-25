// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::sync::mpsc::{self, SendError, TrySendError, RecvError, TryRecvError};


/// 通道，提供任务间的消息传递
pub struct Channel<T> {
    sender: Sender<T>,
    receiver: Receiver<T>,
}

/// 通道发送端
pub struct Sender<T>(mpsc::SyncSender<T>);

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

/// 通道接收端
pub struct Receiver<T>(mpsc::Receiver<T>);

/// 接收迭代器
pub struct Iter<'a, T> {
    receiver: &'a Receiver<T>,
}

impl<T> Channel<T> {
    /// 创建有界通道
    pub fn bounded(capacity: usize) -> Self {
        let (tx, rx) = mpsc::sync_channel(capacity);
        Self {
            sender: Sender(tx),
            receiver: Receiver(rx),
        }
    }

    /// 创建无界通道
    pub fn unbounded() -> Self {
        let (tx, rx) = mpsc::sync_channel(0); // 0 表示无界通道
        Self {
            sender: Sender(tx),
            receiver: Receiver(rx),
        }
    }

    /// 获取发送端
    pub fn sender(&self) -> Sender<T> {
        self.sender.clone()
    }

    /// 获取接收端
    pub fn receiver(&self) -> &Receiver<T> {
        &self.receiver
    }
}

impl<T> Sender<T> {
    /// 发送消息（阻塞）
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        self.0.send(value)
    }

    /// 尝试发送（非阻塞）
    pub fn try_send(&self, value: T) -> Result<(), TrySendError<T>> {
        self.0.try_send(value)
    }
}

impl<T> Receiver<T> {
    /// 接收消息（阻塞）
    pub fn recv(&self) -> Result<T, RecvError> {
        self.0.recv()
    }

    /// 尝试接收（非阻塞）
    pub fn try_recv(&self) -> Result<T, TryRecvError> {
        self.0.try_recv()
    }

    /// 迭代接收消息（阻塞，直到通道关闭）
    pub fn iter(&self) -> Iter<'_, T> {
        Iter { receiver: self }
    }
}



impl<'a, T> Iterator for Iter<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.receiver.recv().ok()
    }
}

// 自动实现 Send/Sync 标记
unsafe impl<T: Send> Send for Sender<T> {}
unsafe impl<T: Send> Send for Receiver<T> {}
unsafe impl<T: Send> Sync for Sender<T> {}
unsafe impl<T: Send> Sync for Receiver<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::concurrent::tasks::TaskGroup;

    #[test]
    fn test_channel_communication() {
        let channel = Channel::<i32>::unbounded();
        let sender = channel.sender();
        let receiver = channel.receiver();

        let mut group = TaskGroup::new();
        group.spawn(move || {
            for i in 0..10 {
                sender.send(i).unwrap();
            }
        });

        let mut sum = 0;
        for _ in 0..10 {
            sum += receiver.recv().unwrap();
        }
        
        group.wait_all();
        assert_eq!(sum, 45);
    }

    #[test]
    fn test_bounded_channel() {
        let channel = Channel::<i32>::bounded(2);
        let sender = channel.sender();
        let receiver = channel.receiver();

        let mut group = TaskGroup::new();
        group.spawn(move || {
            for i in 0..5 {
                sender.send(i).unwrap();
            }
        });

        let mut sum = 0;
        for _ in 0..5 {
            sum += receiver.recv().unwrap();
        }
        
        group.wait_all();
        assert_eq!(sum, 10);
    }
}

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

/// 并发操作中的错误
#[derive(Debug, Clone, PartialEq)]
pub enum ConcurrentError {
    /// 任务 panic
    TaskPanic(String),
    /// 任务被取消
    TaskCancelled,
    /// 通道发送失败（接收端已关闭）
    SendError,
    /// 通道接收失败（发送端已关闭）
    RecvError,
    /// 获取锁时发生 poison
    PoisonError,
}

/// 任务执行错误
#[derive(Debug, Clone, PartialEq)]
pub enum TaskError {
    /// 任务 panic
    Panic(String),
    /// 任务被取消
    Cancelled,
    /// 自定义错误
    Custom(String),
}

impl From<TaskError> for ConcurrentError {
    fn from(err: TaskError) -> Self {
        match err {
            TaskError::Panic(s) => ConcurrentError::TaskPanic(s),
            TaskError::Cancelled => ConcurrentError::TaskCancelled,
            TaskError::Custom(s) => ConcurrentError::TaskPanic(s),
        }
    }
}

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

//! 性能优化模块
//!
//! 本模块提供性能优化和调试支持功能，包括：
//! - 性能剖析和计时
//! - 内存优化和管理
//! - 调试工具和日志系统
//! - 基准测试框架

pub mod profiler;
pub mod memory;
pub mod debug;
pub mod logger;
pub mod bench;

pub use profiler::*;
pub use memory::*;
pub use debug::*;
pub use logger::*;
pub use bench::*;

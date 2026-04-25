// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

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

// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 幻语测试框架模块
//!
//! 本模块提供完整的单元测试、集成测试和基准测试框架，包括：
//! - 声明式测试定义
//! - 丰富的断言 API
//! - 并行测试执行
//! - 性能基准测试
//! - 代码覆盖率报告
//!
//! # 使用示例
//!
//! ```hl
//! 导入 幻语.测试 为 测试
//!
//! 测试.模块("示例测试", || {
//!     测试.测试("加法测试", || {
//!         测试.断言相等(2 + 3, 5)
//!     })
//! })
//! ```

pub mod assertion;
pub mod bench;
pub mod config;
pub mod error;
pub mod fuzz;
pub mod property;
pub mod registry;
pub mod result;
pub mod runner;
pub mod test;
pub mod utils;

pub use assertion::*;
pub use bench::*;
pub use config::*;
pub use error::{TestResult as TestErrorResult, *};
pub use fuzz::*;
pub use property::*;
pub use registry::*;
pub use result::{TestResult, *};
pub use runner::*;
pub use test::*;
pub use utils::*;

/// 测试框架初始化
///
/// 在程序启动时调用以初始化测试环境
pub fn init() {
    std::env::set_var("HL_TEST", "1");
}

/// 测试框架主入口
pub fn main() -> ! {
    let config = TestConfig::from_args(&std::env::args().collect::<Vec<_>>());
    
    let mut runner = TestRunner::new(config);
    
    let paths = vec![std::path::PathBuf::from("./")];
    let _count = runner.discover(&paths).unwrap_or(0);
    
    let summary = runner.run();
    
    summary.print();
    
    std::process::exit(if summary.failed > 0 { 1 } else { 0 });
}

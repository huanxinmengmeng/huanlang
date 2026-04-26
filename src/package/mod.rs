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

//! 幻语包管理器模块
//!
//! 本模块提供了完整的包管理功能，包括：
//! - 项目初始化和管理
//! - 依赖解析和版本控制
//! - 包的发布和安装
//! - 工作区管理
//! - 外部库集成
//!
//! # 核心功能
//!
//! - **包描述文件**：`幻语包.toml` 配置解析
//!
//! - **依赖管理**：版本约束解析，依赖树构建
//!
//! - **注册表交互**：包的上传、下载、搜索
//!
//! - **缓存系统**：本地包缓存，构建缓存
//!
//! - **工作区**：多包协同管理

pub mod manifest;
pub mod lockfile;
pub mod registry;
pub mod resolver;
pub mod cache;
pub mod workspace;
pub mod commands;
pub mod error;
pub mod config;
pub mod dependency;
pub mod security;

pub use manifest::*;
pub use lockfile::*;
pub use registry::*;
pub use resolver::*;
pub use cache::*;
pub use workspace::*;
pub use commands::*;
pub use error::*;
pub use config::*;
pub use dependency::*;
pub use security::*;

/// 包管理器主函数
pub fn main() -> ! {
    let args: Vec<String> = std::env::args().collect();
    let exit_code = Command::execute(&args).unwrap_or(1);
    std::process::exit(exit_code);
}

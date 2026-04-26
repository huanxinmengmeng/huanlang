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

//! 幻语工具链命令行接口模块
//! 
//! 本模块提供了幻语编程语言的完整命令行工具链，包括：
//! - 构建命令（build）：编译源文件生成可执行文件或库
//! - 运行命令（run）：编译并运行源文件
//! - 交互命令（repl）：启动交互式编程环境
//! - 检查命令（check）：仅执行语法和类型检查
//! - 格式化命令（fmt）：格式化源代码
//! - 编辑命令（edit）：启动内置编辑器
//! - 转换命令（transpile）：代码语言转换
//! - 导入库命令（import-lib）：导入外部库
//! - 汇编/反汇编命令（asm/disasm）：汇编器操作
//! - 烧录命令（flash）：烧录到嵌入式设备
//! - 调试命令（debug）：启动调试会话
//! - 服务命令（serve）：启动语言服务器
//! - 包管理命令（package）：包管理
//! - 测试命令（test）：运行测试
//! - 文档命令（doc）：生成文档
//! - 清理命令（clean）：清理构建产物
//! - 版本命令（version）：显示版本信息

pub mod commands;
pub mod config;
pub mod error;
pub mod exit_codes;

pub use commands::*;
pub use config::*;
pub use error::*;
pub use exit_codes::*;

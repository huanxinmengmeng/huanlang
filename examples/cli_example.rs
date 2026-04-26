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

//! 幻语工具链示例
//!
//! 本文件展示了如何使用幻语工具链 CLI 框架的各种命令。

/// 主函数
fn main() {
    println!("幻语工具链示例");
    println!("================");
    println!();

    // 演示如何使用 CLI 命令结构
    demonstrate_build_command();
    demonstrate_run_command();
    demonstrate_package_config();
    demonstrate_version_command();
}

/// 演示构建命令
fn demonstrate_build_command() {
    println!("1. 构建命令示例");
    println!("---------------");

    println!("  输入文件: program.hl");
    println!("  输出文件: program");
    println!("  优化级别: Level3");
    println!("  目标平台: x86_64-unknown-linux-gnu");
    println!("  发布模式: true");
    println!("  调试信息: false");
    println!("  并行任务: 4");
    println!("  生成类型: Link");
    println!();
}

/// 演示运行命令
fn demonstrate_run_command() {
    println!("2. 运行命令示例");
    println!("---------------");

    println!("  输入文件: program.hl");
    println!("  跳过构建: false");
    println!("  程序参数:");
    println!("    - --help");
    println!("    - arg1");
    println!();
}

/// 演示包配置
fn demonstrate_package_config() {
    println!("3. 包配置示例");
    println!("--------------");

    println!("  包名称: 我的项目");
    println!("  版本: 0.1.0");
    println!("  规范版本: 1.2");
    println!("  作者: [作者 <author@example.com>]");
    println!("  描述: 这是一个幻语项目示例");
    println!("  许可证: MIT");
    println!("  关键词风格: Chinese");
    println!("  依赖:");
    println!("    网络: 0.3");
    println!("    序列化: 1.0");
    println!("    本地依赖: path=../本地包");
    println!();
}

/// 演示版本命令
fn demonstrate_version_command() {
    println!("4. 版本命令示例");
    println!("----------------");

    println!("  详细输出: true");
    println!("  JSON 格式: false");
    println!();
}

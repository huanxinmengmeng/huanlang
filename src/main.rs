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

use std::process;
use huanlang::tools::cli::Cli;
use huanlang::core::performance::logger;

fn main() {
    // 初始化日志系统
    logger::init();
    logger::info("幻语编译器启动");
    
    if let Err(e) = Cli::run() {
        logger::error(&format!("错误: {}", e));
        process::exit(1);
    }
    
    logger::info("编译完成");
}

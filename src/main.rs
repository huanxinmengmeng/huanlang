// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::process;
use huanlang::tools::cli::Cli;

fn main() {
    if let Err(e) = Cli::run() {
        eprintln!("错误: {}", e);
        process::exit(1);
    }
}

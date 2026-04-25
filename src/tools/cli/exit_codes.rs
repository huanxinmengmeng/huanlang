// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 退出码定义

/// 退出码常量
pub mod exit_code {
    /// 成功
    pub const SUCCESS: i32 = 0;
    /// 一般错误（编译失败、运行时错误）
    pub const GENERAL_ERROR: i32 = 1;
    /// 命令行参数错误
    pub const ARGUMENT_ERROR: i32 = 2;
    /// 配置文件错误
    pub const CONFIG_ERROR: i32 = 3;
    /// 内部编译器错误（ICE）
    pub const INTERNAL_ERROR: i32 = 101;
    /// 被用户中断（Ctrl+C）
    pub const INTERRUPTED: i32 = 130;
}

/// 退出码工具
pub struct ExitStatus;

impl ExitStatus {
    /// 成功退出
    pub fn success() -> ! {
        std::process::exit(exit_code::SUCCESS);
    }

    /// 错误退出
    pub fn error(code: i32) -> ! {
        std::process::exit(code);
    }

    /// 从错误类型退出
    pub fn from_error(error: &crate::tools::cli::CliError) -> ! {
        std::process::exit(error.exit_code());
    }

    /// 中断退出
    pub fn interrupted() -> ! {
        std::process::exit(exit_code::INTERRUPTED);
    }
}

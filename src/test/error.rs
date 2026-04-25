// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 测试框架错误类型定义

use std::fmt;

/// 测试错误类型
#[derive(Debug, Clone, PartialEq)]
pub enum TestError {
    /// 断言失败
    AssertionFailed {
        message: String,
        file: String,
        line: u32,
        column: u32,
    },
    /// 测试超时
    Timeout {
        duration: std::time::Duration,
    },
    /// 测试被忽略
    Ignored {
        reason: Option<String>,
    },
    /// 测试失败
    Failed {
        message: String,
    },
    /// 配置错误
    ConfigError {
        message: String,
    },
    /// IO 错误
    IoError {
        message: String,
        file: String,
        line: u32,
        column: u32,
    },
    /// 模糊测试失败
    FuzzTestFailed {
        message: String,
        file: String,
        line: u32,
        column: u32,
    },
    /// 属性测试失败
    PropertyTestFailed {
        message: String,
        file: String,
        line: u32,
        column: u32,
    },
}

impl fmt::Display for TestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TestError::AssertionFailed {
                message,
                file,
                line,
                column,
            } => {
                write!(f, "断言失败: {}\n    at {}:{}:{}", message, file, line, column)
            }
            TestError::Timeout { duration } => {
                write!(f, "测试超时: {} ms", duration.as_millis())
            }
            TestError::Ignored { reason } => {
                write!(f, "测试被忽略")?;
                if let Some(r) = reason {
                    write!(f, ": {}", r)?;
                }
                Ok(())
            }
            TestError::Failed { message } => {
                write!(f, "测试失败: {}", message)
            }
            TestError::ConfigError { message } => {
                write!(f, "配置错误: {}", message)
            }
            TestError::IoError { message, file, line, column } => {
                write!(f, "IO 错误: {}\n    at {}:{}:{}", message, file, line, column)
            }
            TestError::FuzzTestFailed { message, file, line, column } => {
                write!(f, "模糊测试失败: {}\n    at {}:{}:{}", message, file, line, column)
            }
            TestError::PropertyTestFailed { message, file, line, column } => {
                write!(f, "属性测试失败: {}\n    at {}:{}:{}", message, file, line, column)
            }
        }
    }
}

impl std::error::Error for TestError {}

/// 测试结果类型
pub type TestResult<T> = std::result::Result<T, TestError>;

/// 断言宏
#[macro_export]
macro_rules! test_assert {
    ($cond:expr) => {
        $crate::test::assertion::assert_impl(
            $cond,
            file!().to_string(),
            line!(),
            column!(),
        )
    };
    ($cond:expr, $msg:expr) => {
        $crate::test::assertion::assert_with_message_impl(
            $cond,
            $msg,
            file!().to_string(),
            line!(),
            column!(),
        )
    };
}

/// 相等断言宏
#[macro_export]
macro_rules! test_assert_eq {
    ($actual:expr, $expected:expr) => {
        $crate::test::assertion::assert_eq_impl(
            $actual,
            $expected,
            file!().to_string(),
            line!(),
            column!(),
        )
    };
    ($actual:expr, $expected:expr, $msg:expr) => {
        $crate::test::assertion::assert_eq_with_message_impl(
            $actual,
            $expected,
            $msg,
            file!().to_string(),
            line!(),
            column!(),
        )
    };
}

/// 不相等断言宏
#[macro_export]
macro_rules! test_assert_ne {
    ($actual:expr, $expected:expr) => {
        $crate::test::assertion::assert_ne_impl(
            $actual,
            $expected,
            file!().to_string(),
            line!(),
            column!(),
        )
    };
    ($actual:expr, $expected:expr, $msg:expr) => {
        $crate::test::assertion::assert_ne_with_message_impl(
            $actual,
            $expected,
            $msg,
            file!().to_string(),
            line!(),
            column!(),
        )
    };
}

/// 快照测试断言宏
#[macro_export]
macro_rules! test_assert_snapshot {
    ($actual:expr, $snapshot_name:expr) => {
        $crate::test::assertion::assert_snapshot_impl(
            $actual,
            $snapshot_name,
            file!().to_string(),
            line!(),
            column!(),
        )
    };
}

/// 更新快照宏
#[macro_export]
macro_rules! test_update_snapshot {
    ($actual:expr, $snapshot_name:expr) => {
        $crate::test::assertion::update_snapshot_impl(
            $actual,
            $snapshot_name,
            file!().to_string(),
            line!(),
            column!(),
        )
    };
}

/// 模糊测试宏
#[macro_export]
macro_rules! test_fuzz {
    ($name:expr, $input_type:ty, $test_fn:expr) => {
        $crate::test::test::Test::new($name.to_string(), $crate::test::test::SourceLocation::new(file!().to_string(), line!(), column!()))
            .fuzz()
            .with_module(module_path!())
    };
}

/// 属性测试宏
#[macro_export]
macro_rules! test_property {
    ($name:expr, $input_type:ty, $test_fn:expr) => {
        $crate::test::test::Test::new($name.to_string(), $crate::test::test::SourceLocation::new(file!().to_string(), line!(), column!()))
            .property()
            .with_module(module_path!())
    };
}

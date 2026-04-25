// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 断言 API 模块
//!
//! 提供丰富的断言函数，用于测试中的验证。所有断言失败时都会输出详细信息。

use crate::test::error::*;
use std::fmt::Debug;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

/// 基本断言 - 断言条件为真
pub fn assert_impl(cond: bool, file: String, line: u32, column: u32) -> TestResult<()> {
    if !cond {
        return Err(TestError::AssertionFailed {
            message: "断言失败".to_string(),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 带自定义消息的基本断言
pub fn assert_with_message_impl(
    cond: bool,
    message: &str,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if !cond {
        return Err(TestError::AssertionFailed {
            message: message.to_string(),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言相等
pub fn assert_eq_impl<T: PartialEq + Debug>(
    actual: T,
    expected: T,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if actual != expected {
        return Err(TestError::AssertionFailed {
            message: format!("断言相等失败，期望: {:?}，实际: {:?}", expected, actual),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 带自定义消息的断言相等
pub fn assert_eq_with_message_impl<T: PartialEq + Debug>(
    actual: T,
    expected: T,
    message: &str,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if actual != expected {
        return Err(TestError::AssertionFailed {
            message: format!("{}，期望: {:?}，实际: {:?}", message, expected, actual),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言不相等
pub fn assert_ne_impl<T: PartialEq + Debug>(
    actual: T,
    expected: T,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if actual == expected {
        return Err(TestError::AssertionFailed {
            message: format!("断言不相等失败，两者都等于: {:?}", expected),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 带自定义消息的断言不相等
pub fn assert_ne_with_message_impl<T: PartialEq + Debug>(
    actual: T,
    expected: T,
    message: &str,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if actual == expected {
        return Err(TestError::AssertionFailed {
            message: format!("{}，两者都等于: {:?}", message, expected),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言值为真
pub fn assert_true_impl(value: bool, file: String, line: u32, column: u32) -> TestResult<()> {
    if !value {
        return Err(TestError::AssertionFailed {
            message: "断言为真失败".to_string(),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言值为假
pub fn assert_false_impl(value: bool, file: String, line: u32, column: u32) -> TestResult<()> {
    if value {
        return Err(TestError::AssertionFailed {
            message: "断言为假失败".to_string(),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言可选值为空
pub fn assert_none_impl<T: Debug>(
    value: Option<T>,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if value.is_some() {
        return Err(TestError::AssertionFailed {
            message: format!("断言为空失败，实际值: {:?}", value),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言可选值不为空
pub fn assert_some_impl<T: Debug>(
    value: Option<T>,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if value.is_none() {
        return Err(TestError::AssertionFailed {
            message: "断言不为空失败，实际值: 空".to_string(),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言结果为成功
pub fn assert_ok_impl<T: Debug, E: Debug>(
    value: Result<T, E>,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    match value {
        Ok(_) => Ok(()),
        Err(e) => Err(TestError::AssertionFailed {
            message: format!("断言成功失败，实际错误: {:?}", e),
            file,
            line,
            column,
        }),
    }
}

/// 断言结果为错误
pub fn assert_err_impl<T: Debug, E: Debug>(
    value: Result<T, E>,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    match value {
        Err(_) => Ok(()),
        Ok(v) => Err(TestError::AssertionFailed {
            message: format!("断言错误失败，实际值: {:?}", v),
            file,
            line,
            column,
        }),
    }
}

/// 断言浮点数近似相等
pub fn assert_approx_impl(
    actual: f64,
    expected: f64,
    tolerance: f64,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    let diff = (actual - expected).abs();
    if diff > tolerance {
        return Err(TestError::AssertionFailed {
            message: format!(
                "断言近似失败，期望: {}, 实际: {}, 误差: {}, 容忍度: {}",
                expected, actual, diff, tolerance
            ),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言大于
pub fn assert_gt_impl<T: PartialOrd + Debug>(
    left: T,
    right: T,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if left <= right {
        return Err(TestError::AssertionFailed {
            message: format!("断言大于失败，期望: {:?} > {:?}", left, right),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言小于
pub fn assert_lt_impl<T: PartialOrd + Debug>(
    left: T,
    right: T,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if left >= right {
        return Err(TestError::AssertionFailed {
            message: format!("断言小于失败，期望: {:?} < {:?}", left, right),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言大于等于
pub fn assert_ge_impl<T: PartialOrd + Debug>(
    left: T,
    right: T,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if left < right {
        return Err(TestError::AssertionFailed {
            message: format!("断言大于等于失败，期望: {:?} >= {:?}", left, right),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言小于等于
pub fn assert_le_impl<T: PartialOrd + Debug>(
    left: T,
    right: T,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if left > right {
        return Err(TestError::AssertionFailed {
            message: format!("断言小于等于失败，期望: {:?} <= {:?}", left, right),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言字符串包含子串
pub fn assert_contains_impl(
    str: &str,
    substr: &str,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if !str.contains(substr) {
        return Err(TestError::AssertionFailed {
            message: format!(
                "断言包含失败，字符串: {:?} 不包含子串: {:?}",
                str, substr
            ),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言字符串不包含子串
pub fn assert_not_contains_impl(
    str: &str,
    substr: &str,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if str.contains(substr) {
        return Err(TestError::AssertionFailed {
            message: format!(
                "断言不包含失败，字符串: {:?} 包含子串: {:?}",
                str, substr
            ),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言字符串以指定前缀开头
pub fn assert_starts_with_impl(
    str: &str,
    prefix: &str,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if !str.starts_with(prefix) {
        return Err(TestError::AssertionFailed {
            message: format!(
                "断言以...开头失败，字符串: {:?} 不以: {:?} 开头",
                str, prefix
            ),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言字符串以指定后缀结尾
pub fn assert_ends_with_impl(
    str: &str,
    suffix: &str,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if !str.ends_with(suffix) {
        return Err(TestError::AssertionFailed {
            message: format!(
                "断言以...结尾失败，字符串: {:?} 不以: {:?} 结尾",
                str, suffix
            ),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言列表包含指定元素
pub fn assert_list_contains_impl<T: PartialEq + Debug>(
    list: &[T],
    element: &T,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    if !list.contains(element) {
        return Err(TestError::AssertionFailed {
            message: format!("断言列表包含失败，元素: {:?} 不在列表: {:?} 中", element, list),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言闭包执行时触发错误
pub fn assert_panics_impl<F: FnOnce()>(
    func: F,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(func));
    if result.is_ok() {
        return Err(TestError::AssertionFailed {
            message: "断言触发错误失败，闭包没有触发错误".to_string(),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言闭包不触发错误
pub fn assert_no_panic_impl<F: FnOnce()>(
    func: F,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(func));
    if result.is_err() {
        return Err(TestError::AssertionFailed {
            message: "断言不触发错误失败，闭包触发了错误".to_string(),
            file,
            line,
            column,
        });
    }
    Ok(())
}

/// 断言闭包触发指定错误消息
pub fn assert_panics_with_impl<F: FnOnce()>(
    func: F,
    expected_message: &str,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(func));
    match result {
        Ok(_) => Err(TestError::AssertionFailed {
            message: "断言触发错误消息失败，闭包没有触发错误".to_string(),
            file,
            line,
            column,
        }),
        Err(e) => {
            let message_str = if let Some(s) = e.downcast_ref::<&str>() {
                s.to_string()
            } else if let Some(s) = e.downcast_ref::<String>() {
                s.clone()
            } else {
                "未知错误".to_string()
            };
            if !message_str.contains(expected_message) {
                Err(TestError::AssertionFailed {
                    message: format!(
                        "断言触发错误消息失败，期望包含: {:?}，实际消息: {:?}",
                        expected_message, message_str
                    ),
                    file,
                    line,
                    column,
                })
            } else {
                Ok(())
            }
        }
    }
}

/// 快照测试 - 断言值与保存的快照匹配
pub fn assert_snapshot_impl<T: Debug>(
    actual: T,
    snapshot_name: &str,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    let snapshot_dir = Path::new("./snapshots");
    if !snapshot_dir.exists() {
        fs::create_dir_all(snapshot_dir).map_err(|e| TestError::IoError {
            message: format!("创建快照目录失败: {:?}", e),
            file: file.clone(),
            line,
            column,
        })?;
    }
    
    let snapshot_path = snapshot_dir.join(format!("{}.snap", snapshot_name));
    
    let actual_str = format!("{:?}", actual);
    
    if snapshot_path.exists() {
        let expected_str = fs::read_to_string(&snapshot_path).map_err(|e| TestError::IoError {
            message: format!("读取快照文件失败: {:?}", e),
            file: file.clone(),
            line,
            column,
        })?;
        
        if actual_str != expected_str {
            return Err(TestError::AssertionFailed {
                message: format!("快照测试失败，实际值与快照不匹配\n期望:\n{}\n实际:\n{}", expected_str, actual_str),
                file,
                line,
                column,
            });
        }
    } else {
        // 创建新快照
        let mut file = File::create(&snapshot_path).map_err(|e| TestError::IoError {
            message: format!("创建快照文件失败: {:?}", e),
            file: snapshot_path.to_string_lossy().to_string(),
            line,
            column,
        })?;
        file.write_all(actual_str.as_bytes()).map_err(|e| TestError::IoError {
            message: format!("写入快照文件失败: {:?}", e),
            file: snapshot_path.to_string_lossy().to_string(),
            line,
            column,
        })?;
    }
    
    Ok(())
}

/// 更新快照 - 强制更新保存的快照
pub fn update_snapshot_impl<T: Debug>(
    actual: T,
    snapshot_name: &str,
    file: String,
    line: u32,
    column: u32,
) -> TestResult<()> {
    let snapshot_dir = Path::new("./snapshots");
    if !snapshot_dir.exists() {
        fs::create_dir_all(snapshot_dir).map_err(|e| TestError::IoError {
            message: format!("创建快照目录失败: {:?}", e),
            file: file.clone(),
            line,
            column,
        })?;
    }
    
    let snapshot_path = snapshot_dir.join(format!("{}.snap", snapshot_name));
    
    let actual_str = format!("{:?}", actual);
    
    let mut file = File::open(&snapshot_path).map_err(|e| TestError::IoError {
        message: format!("打开快照文件失败: {:?}", e),
        file: snapshot_path.to_string_lossy().to_string(),
        line,
        column,
    })?;
    file.write_all(actual_str.as_bytes()).map_err(|e| TestError::IoError {
        message: format!("写入快照文件失败: {:?}", e),
        file: snapshot_path.to_string_lossy().to_string(),
        line,
        column,
    })?;
    
    Ok(())
}

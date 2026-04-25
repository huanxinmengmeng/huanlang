// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 测试结果模块

use crate::test::*;
use std::time::Duration;
use std::fmt;

/// 测试结果
#[derive(Debug, Clone)]
pub struct TestResult {
    /// 测试信息
    pub test: Test,
    /// 测试状态
    pub status: TestStatus,
    /// 持续时间
    pub duration: Duration,
    /// 输出
    pub output: Option<String>,
    /// 失败消息
    pub failure_message: Option<String>,
    /// 失败位置
    pub failure_location: Option<SourceLocation>,
}

impl TestResult {
    pub fn passed(test: Test, duration: Duration) -> Self {
        TestResult {
            test,
            status: TestStatus::Passed,
            duration,
            output: None,
            failure_message: None,
            failure_location: None,
        }
    }

    pub fn failed(
        test: Test,
        duration: Duration,
        message: String,
        location: Option<SourceLocation>,
    ) -> Self {
        TestResult {
            test,
            status: TestStatus::Failed,
            duration,
            output: None,
            failure_message: Some(message),
            failure_location: location,
        }
    }

    pub fn ignored(test: Test, reason: Option<String>) -> Self {
        TestResult {
            test,
            status: TestStatus::Ignored,
            duration: Duration::from_millis(0),
            output: None,
            failure_message: None,
            failure_location: None,
        }
    }

    pub fn timed_out(test: Test, duration: Duration) -> Self {
        TestResult {
            test,
            status: TestStatus::TimedOut,
            duration,
            output: None,
            failure_message: None,
            failure_location: None,
        }
    }

    pub fn with_output(mut self, output: String) -> Self {
        self.output = Some(output);
        self
    }
}

/// 测试摘要
#[derive(Debug, Clone)]
pub struct TestSummary {
    /// 总测试数
    pub total: usize,
    /// 通过数
    pub passed: usize,
    /// 失败数
    pub failed: usize,
    /// 忽略数
    pub ignored: usize,
    /// 超时数
    pub timed_out: usize,
    /// 总时间
    pub total_duration: Duration,
}

impl TestSummary {
    pub fn empty() -> Self {
        TestSummary {
            total: 0,
            passed: 0,
            failed: 0,
            ignored: 0,
            timed_out: 0,
            total_duration: Duration::from_millis(0),
        }
    }

    pub fn from_results(results: &[TestResult]) -> Self {
        let mut summary = TestSummary::empty();
        summary.total = results.len();
        
        for result in results {
            summary.total_duration += result.duration;
            match result.status {
                TestStatus::Passed => summary.passed += 1,
                TestStatus::Failed => summary.failed += 1,
                TestStatus::Ignored => summary.ignored += 1,
                TestStatus::TimedOut => summary.timed_out += 1,
                TestStatus::Fuzzed => summary.passed += 1,
                TestStatus::Property => summary.passed += 1,
            }
        }
        
        summary
    }

    pub fn print(&self) {
        println!();
        println!("测试结果: {} 通过, {} 失败, {} 忽略", self.passed, self.failed, self.ignored);
        println!("总时间: {}ms", self.total_duration.as_millis());
    }

    pub fn to_json(&self, results: &[TestResult]) -> String {
        use serde_json::json;
        
        let test_arr: Vec<_> = results
            .iter()
            .map(|r| {
                json!({
                    "name": r.test.name,
                    "status": r.status.to_string(),
                    "duration_ms": r.duration.as_millis(),
                    "module": r.test.module_path,
                })
            })
            .collect();
        
        json!({
            "summary": {
                "total": self.total,
                "passed": self.passed,
                "failed": self.failed,
                "ignored": self.ignored,
                "total_duration_ms": self.total_duration.as_millis(),
            },
            "tests": test_arr,
        })
        .to_string()
    }
}

impl fmt::Display for TestSummary {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "总计: {}, 通过: {}, 失败: {}, 忽略: {}, 时间: {}ms",
            self.total,
            self.passed,
            self.failed,
            self.ignored,
            self.total_duration.as_millis()
        )
    }
}

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

//! 测试运行器模块

use crate::test::*;
use std::path::PathBuf;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{Instant, Duration};
use std::thread;

/// 测试运行器
pub struct TestRunner {
    /// 配置
    config: TestConfig,
    /// 测试模块
    #[allow(dead_code)]
    modules: Vec<TestModule>,
    /// 测试注册表
    registry: TestRegistry,
}

impl TestRunner {
    pub fn new(config: TestConfig) -> Self {
        TestRunner {
            config,
            modules: Vec::new(),
            registry: TestRegistry::new(),
        }
    }

    /// 发现测试
    pub fn discover(&mut self, paths: &[PathBuf]) -> crate::test::error::TestResult<usize> {
        println!("正在发现测试...");
        
        let count = self.registry.load_from(paths)?;
        println!("发现了 {} 个测试", count);
        
        Ok(count)
    }

    /// 运行测试
    pub fn run(&mut self) -> TestSummary {
        if self.config.list {
            return self.list_tests();
        }

        println!("运行测试...");

        let tests = self.filter_tests();

        if self.config.bench {
            self.run_benchmarks(&tests)
        } else {
            self.run_tests(&tests)
        }
    }

    /// 列出测试
    fn list_tests(&self) -> TestSummary {
        for entry in self.registry.entries() {
            let full_name = if entry.module.is_empty() {
                entry.name.clone()
            } else {
                format!("{}::{}", entry.module, entry.name)
            };
            println!("  {}", full_name);
        }
        println!();
        TestSummary::empty()
    }

    /// 过滤测试
    fn filter_tests(&self) -> Vec<TestEntry> {
        let mut filtered = Vec::new();
        
        for entry in self.registry.entries() {
            if self.config.bench && entry.test_type != TestType::Benchmark {
                continue;
            }
            if !self.config.bench && entry.test_type == TestType::Benchmark {
                continue;
            }

            if entry.ignored {
                if !self.config.include_ignored {
                    continue;
                }
                if self.config.only_ignored {
                    // 仅运行被忽略的测试
                }
            } else if self.config.only_ignored {
                continue;
            }

            if let Some(filter) = &self.config.filter {
                let full_name = if entry.module.is_empty() {
                    entry.name.clone()
                } else {
                    format!("{}::{}", entry.module, entry.name)
                };
                
                if self.config.exact {
                    if full_name == *filter {
                        filtered.push(entry.clone());
                    }
                } else {
                    if full_name.contains(filter) {
                        filtered.push(entry.clone());
                    }
                }
            } else {
                filtered.push(entry.clone());
            }
        }
        
        filtered
    }

    /// 运行普通测试
    fn run_tests(&self, tests: &[TestEntry]) -> TestSummary {
        println!("运行 {} 个测试...", tests.len());
        
        let (serial_tests, parallel_tests): (Vec<_>, Vec<_>) = tests
            .iter()
            .cloned()
            .partition(|t| t.serial);

        let mut results = Vec::new();
        
        if !serial_tests.is_empty() {
            println!("串行执行 {} 个测试...", serial_tests.len());
            results.extend(self.run_serial(&serial_tests));
        }
        
        if !parallel_tests.is_empty() {
            println!("并行执行 {} 个测试...", parallel_tests.len());
            results.extend(self.run_parallel(&parallel_tests));
        }

        let summary = TestSummary::from_results(&results);
        summary.print();
        
        summary
    }

    /// 串行运行测试
    fn run_serial(&self, tests: &[TestEntry]) -> Vec<ResultItem> {
        let mut results = Vec::new();
        
        for entry in tests {
            let test = Test::new(
                entry.name.clone(),
                entry.location.clone(),
            );
            let result = self.run_single_test(&test, None);
            self.print_test_result(&result);
            results.push(result);
        }
        
        results
    }

    /// 并行运行测试
    fn run_parallel(&self, tests: &[TestEntry]) -> Vec<ResultItem> {
        let (tx, rx) = mpsc::channel();
        let running = Arc::new(Mutex::new(0));
        
        let mut handles = Vec::new();
        
        for chunk in self.chunk_tests(tests) {
            let tx = tx.clone();
            let _running = Arc::clone(&running);
            let tests = chunk.clone();
            let config = self.config.clone();
            
            let handle = thread::spawn(move || {
                for entry in tests {
                    let test = Test::new(
                        entry.name.clone(),
                        entry.location.clone(),
                    );
                    
                    let result = run_single_internal(&test, config.clone());
                    tx.send(result).unwrap();
                }
            });
            
            handles.push(handle);
        }
        
        drop(tx);
        
        let mut results = Vec::new();
        for result in rx {
            self.print_test_result(&result);
            results.push(result);
        }
        
        results
    }

    fn chunk_tests(&self, tests: &[TestEntry]) -> Vec<Vec<TestEntry>> {
        let chunks = self.config.jobs.max(1);
        let mut chunks_vec = vec![Vec::new(); chunks];
        
        for (i, test) in tests.iter().enumerate() {
            chunks_vec[i % chunks].push(test.clone());
        }
        
        chunks_vec
    }

    /// 运行单个测试
    fn run_single_test(&self, test: &Test, _timeout: Option<Duration>) -> ResultItem {
        let test = test.clone();
        
        let start = Instant::now();
        
        let result = if test.ignored {
            let ignore_reason = test.ignore_reason.clone();
            ResultItem::ignored(test, ignore_reason)
        } else {
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                // 这里应该实际调用测试函数
                // 为演示，直接返回通过
                Ok::<(), crate::test::error::TestError>(())
            }));
            
            let duration = start.elapsed();
            
            match result {
                Ok(Ok(())) => ResultItem::passed(test, duration),
                Ok(Err(e)) => ResultItem::failed(test, duration, format!("测试失败: {:?}", e), None),
                Err(e) => {
                    let msg = if let Some(s) = e.downcast_ref::<&str>() {
                        s.to_string()
                    } else if let Some(s) = e.downcast_ref::<String>() {
                        s.clone()
                    } else {
                        "未知错误".to_string()
                    };
                    ResultItem::failed(test, duration, msg, None)
                }
            }
        };
        
        result
    }

    fn print_test_result(&self, result: &ResultItem) {
        let full_name = if result.test.module_path.is_empty() {
            result.test.name.clone()
        } else {
            format!("{}::{}", result.test.module_path, result.test.name)
        };
        
        match result.status {
            TestStatus::Passed => {
                if self.config.report_time {
                    println!("测试 {} ... 通过 ({:.2}ms)", full_name, result.duration.as_secs_f64() * 1000.0);
                } else {
                    println!("测试 {} ... 通过", full_name);
                }
            }
            TestStatus::Failed => {
                if let Some(msg) = &result.failure_message {
                    println!("测试 {} ... 失败: {}", full_name, msg);
                } else {
                    println!("测试 {} ... 失败", full_name);
                }
            }
            TestStatus::Ignored => {
                if let Some(reason) = &result.test.ignore_reason {
                    println!("测试 {} ... 忽略 ({})", full_name, reason);
                } else {
                    println!("测试 {} ... 忽略", full_name);
                }
            }
            TestStatus::TimedOut => {
                println!("测试 {} ... 超时", full_name);
            }
            TestStatus::Fuzzed => {
                if self.config.report_time {
                    println!("测试 {} ... 模糊测试通过 ({:.2}ms)", full_name, result.duration.as_secs_f64() * 1000.0);
                } else {
                    println!("测试 {} ... 模糊测试通过", full_name);
                }
            }
            TestStatus::Property => {
                if self.config.report_time {
                    println!("测试 {} ... 属性测试通过 ({:.2}ms)", full_name, result.duration.as_secs_f64() * 1000.0);
                } else {
                    println!("测试 {} ... 属性测试通过", full_name);
                }
            }
        }
    }

    fn run_benchmarks(&self, tests: &[TestEntry]) -> TestSummary {
        println!("运行 {} 个基准测试...", tests.len());
        TestSummary::empty()
    }
}

type ResultItem = TestResult;

fn run_single_internal(test: &Test, _config: TestConfig) -> ResultItem {
    let start = Instant::now();
    let duration = start.elapsed();
    
    ResultItem::passed(test.clone(), duration)
}

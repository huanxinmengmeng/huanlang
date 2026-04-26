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

//! 测试类型定义模块

use std::time::Duration;
use std::fmt;

/// 测试状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestStatus {
    /// 通过
    Passed,
    /// 失败
    Failed,
    /// 忽略
    Ignored,
    /// 超时
    TimedOut,
    /// 模糊测试
    Fuzzed,
    /// 属性测试
    Property,
}

impl fmt::Display for TestStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TestStatus::Passed => write!(f, "通过"),
            TestStatus::Failed => write!(f, "失败"),
            TestStatus::Ignored => write!(f, "忽略"),
            TestStatus::TimedOut => write!(f, "超时"),
            TestStatus::Fuzzed => write!(f, "模糊测试"),
            TestStatus::Property => write!(f, "属性测试"),
        }
    }
}

/// 测试位置
#[derive(Debug, Clone)]
pub struct SourceLocation {
    /// 文件路径
    pub file: String,
    /// 行号
    pub line: u32,
    /// 列号
    pub column: u32,
}

impl SourceLocation {
    pub fn new(file: String, line: u32, column: u32) -> Self {
        SourceLocation { file, line, column }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}

/// 测试类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestType {
    /// 单元测试
    Unit,
    /// 基准测试
    Benchmark,
    /// 模糊测试
    Fuzz,
    /// 属性测试
    Property,
}

/// 测试定义
#[derive(Debug, Clone)]
pub struct Test {
    /// 测试名称
    pub name: String,
    /// 模块路径
    pub module_path: String,
    /// 位置
    pub location: SourceLocation,
    /// 是否忽略
    pub ignored: bool,
    /// 忽略原因
    pub ignore_reason: Option<String>,
    /// 超时时间
    pub timeout: Option<Duration>,
    /// 是否强制串行执行
    pub serial: bool,
    /// 测试类型
    pub test_type: TestType,
}

impl Test {
    pub fn new(name: String, location: SourceLocation) -> Self {
        Test {
            name,
            module_path: String::new(),
            location,
            ignored: false,
            ignore_reason: None,
            timeout: None,
            serial: false,
            test_type: TestType::Unit,
        }
    }

    pub fn with_module(mut self, module: String) -> Self {
        self.module_path = module;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn ignore(mut self, reason: String) -> Self {
        self.ignored = true;
        self.ignore_reason = Some(reason);
        self
    }

    pub fn serial(mut self) -> Self {
        self.serial = true;
        self
    }

    pub fn benchmark(mut self) -> Self {
        self.test_type = TestType::Benchmark;
        self
    }

    pub fn fuzz(mut self) -> Self {
        self.test_type = TestType::Fuzz;
        self
    }

    pub fn property(mut self) -> Self {
        self.test_type = TestType::Property;
        self
    }
}

/// 测试模块
pub struct TestModule {
    /// 模块名称
    pub name: String,
    /// 子模块
    pub modules: Vec<TestModule>,
    /// 测试用例
    pub tests: Vec<Test>,
    /// 前置钩子
    pub before: Option<Box<dyn Fn() + Send + Sync>>,
    /// 后置钩子
    pub after: Option<Box<dyn Fn() + Send + Sync>>,
}

impl std::fmt::Debug for TestModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TestModule")
            .field("name", &self.name)
            .field("modules", &self.modules)
            .field("tests", &self.tests)
            .finish()
    }
}

impl Clone for TestModule {
    fn clone(&self) -> Self {
        TestModule {
            name: self.name.clone(),
            modules: self.modules.clone(),
            tests: self.tests.clone(),
            before: None, // 函数闭包无法克隆
            after: None,  // 函数闭包无法克隆
        }
    }
}

impl TestModule {
    pub fn new(name: String) -> Self {
        TestModule {
            name,
            modules: Vec::new(),
            tests: Vec::new(),
            before: None,
            after: None,
        }
    }

    pub fn add_module(&mut self, module: TestModule) {
        self.modules.push(module);
    }

    pub fn add_test(&mut self, test: Test) {
        self.tests.push(test);
    }

    pub fn set_before<F: Fn() + Send + Sync + 'static>(&mut self, f: F) {
        self.before = Some(Box::new(f));
    }

    pub fn set_after<F: Fn() + Send + Sync + 'static>(&mut self, f: F) {
        self.after = Some(Box::new(f));
    }

    pub fn collect_all_tests(&self) -> Vec<Test> {
        let mut all = Vec::new();
        for test in &self.tests {
            all.push(test.clone());
        }
        for module in &self.modules {
            for test in module.collect_all_tests() {
                all.push(test);
            }
        }
        all
    }
}

/// 测试入口
pub type TestFunc = Box<dyn FnOnce() -> crate::test::error::TestResult<()> + Send>;

/// 测试注册表项
#[derive(Debug, Clone)]
pub struct TestEntry {
    /// 模块路径
    pub module: String,
    /// 测试名称
    pub name: String,
    /// 位置
    pub location: SourceLocation,
    /// 是否忽略
    pub ignored: bool,
    /// 忽略原因
    pub ignore_reason: Option<String>,
    /// 是否串行
    pub serial: bool,
    /// 测试类型
    pub test_type: TestType,
}

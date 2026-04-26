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

//! 测试注册表模块

use crate::test::*;
use std::path::PathBuf;
use std::sync::Mutex;

/// 测试注册表
pub struct TestRegistry {
    entries: Vec<TestEntry>,
}

impl TestRegistry {
    pub fn new() -> Self {
        TestRegistry {
            entries: Vec::new(),
        }
    }

    pub fn register(&mut self, entry: TestEntry) {
        self.entries.push(entry);
    }

    pub fn entries(&self) -> &[TestEntry] {
        &self.entries
    }

    pub fn load_from(&mut self, _paths: &[PathBuf]) -> Result<usize, TestError> {
        // 实际实现应该扫描目录、编译模块、收集测试
        // 为演示，返回空
        Ok(0)
    }
}

impl Default for TestRegistry {
    fn default() -> Self {
        Self::new()
    }
}

lazy_static::lazy_static! {
    static ref GLOBAL_REGISTRY: Mutex<TestRegistry> = Mutex::new(TestRegistry::new());
}

pub fn register_test(entry: TestEntry) {
    GLOBAL_REGISTRY.lock().unwrap().register(entry);
}

pub fn global_registry() -> &'static Mutex<TestRegistry> {
    &GLOBAL_REGISTRY
}

/// 测试模块构建器
pub struct TestModuleBuilder {
    name: String,
    tests: Vec<TestEntry>,
    submodules: Vec<TestModuleBuilder>,
}

impl TestModuleBuilder {
    pub fn new(name: String) -> Self {
        TestModuleBuilder {
            name,
            tests: Vec::new(),
            submodules: Vec::new(),
        }
    }

    pub fn test(mut self, name: String, location: SourceLocation) -> Self {
        self.tests.push(TestEntry {
            module: self.name.clone(),
            name,
            location,
            ignored: false,
            ignore_reason: None,
            serial: false,
            test_type: TestType::Unit,
        });
        self
    }

    pub fn benchmark(mut self, name: String, location: SourceLocation) -> Self {
        self.tests.push(TestEntry {
            module: self.name.clone(),
            name,
            location,
            ignored: false,
            ignore_reason: None,
            serial: false,
            test_type: TestType::Benchmark,
        });
        self
    }

    pub fn submodule(mut self, module: TestModuleBuilder) -> Self {
        self.submodules.push(module);
        self
    }

    pub fn build(self) -> Vec<TestEntry> {
        let mut entries = Vec::new();
        
        for test in self.tests {
            entries.push(test);
        }
        
        for submodule in self.submodules {
            let mut sub_entries = submodule.build();
            entries.append(&mut sub_entries);
        }
        
        entries
    }
}

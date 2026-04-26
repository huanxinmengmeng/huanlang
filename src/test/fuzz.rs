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

//! 模糊测试模块
//! 
//! 提供模糊测试功能，用于发现代码中的边界情况和安全漏洞。

use crate::test::error::*;
use std::fmt::Debug;
use std::time::Duration;

/// 模糊测试输入生成器
pub trait FuzzerInput: Debug + Clone {
    /// 生成随机输入
    fn generate() -> Self;
    /// 变异输入
    fn mutate(&self) -> Self;
}

/// 模糊测试结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuzzResult {
    /// 测试通过
    Passed,
    /// 测试失败
    Failed(String),
    /// 测试超时
    TimedOut,
}

/// 模糊测试运行器
pub struct FuzzRunner<T: FuzzerInput> {
    /// 测试函数
    test_fn: Box<dyn Fn(&T) -> FuzzResult + Send + Sync>,
    /// 运行时间限制
    timeout: Duration,
    /// 最大执行次数
    max_runs: u64,
    /// 发现的崩溃数
    crashes: u64,
}

impl<T: FuzzerInput> FuzzRunner<T> {
    /// 创建新的模糊测试运行器
    pub fn new<F>(test_fn: F, timeout: Duration, max_runs: u64) -> Self
    where
        F: Fn(&T) -> FuzzResult + Send + Sync + 'static,
    {
        Self {
            test_fn: Box::new(test_fn),
            timeout,
            max_runs,
            crashes: 0,
        }
    }

    /// 运行模糊测试
    pub fn run(&mut self) -> TestResult<()> {
        for run in 0..self.max_runs {
            // 生成随机输入
            let input = T::generate();
            
            // 运行测试
            let result = self.run_with_input(&input);
            
            match result {
                FuzzResult::Failed(msg) => {
                    self.crashes += 1;
                    eprintln!("模糊测试失败 (运行 {}): {:?}\n错误: {}", run, input, msg);
                }
                FuzzResult::TimedOut => {
                    self.crashes += 1;
                    eprintln!("模糊测试超时 (运行 {}): {:?}", run, input);
                }
                FuzzResult::Passed => {
                    // 测试通过，继续
                }
            }
            
            // 变异输入并再次测试
            for _ in 0..10 { // 每个输入变异10次
                let mutated_input = input.mutate();
                let result = self.run_with_input(&mutated_input);
                
                match result {
                    FuzzResult::Failed(msg) => {
                        self.crashes += 1;
                        eprintln!("模糊测试失败 (变异 {}): {:?}\n错误: {}", run, mutated_input, msg);
                    }
                    FuzzResult::TimedOut => {
                        self.crashes += 1;
                        eprintln!("模糊测试超时 (变异 {}): {:?}", run, mutated_input);
                    }
                    FuzzResult::Passed => {
                        // 测试通过，继续
                    }
                }
            }
        }
        
        if self.crashes > 0 {
            Err(TestError::FuzzTestFailed {
                message: format!("模糊测试失败，发现 {} 个崩溃", self.crashes),
                file: "fuzz_test".to_string(),
                line: 0,
                column: 0,
            })
        } else {
            Ok(())
        }
    }

    /// 运行单个输入的测试
    fn run_with_input(&self, input: &T) -> FuzzResult {
        let start = std::time::Instant::now();
        
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (self.test_fn)(input)
        }));
        
        let duration = start.elapsed();
        
        if duration > self.timeout {
            return FuzzResult::TimedOut;
        }
        
        match result {
            Ok(fuzz_result) => fuzz_result,
            Err(e) => {
                let error_msg = if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "未知错误".to_string()
                };
                FuzzResult::Failed(error_msg)
            }
        }
    }
}

/// 基本类型的模糊测试输入实现

impl FuzzerInput for u8 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn mutate(&self) -> Self {
        let mut value = *self;
        // 随机变异
        match rand::random::<u8>() % 3 {
            0 => value = value.wrapping_add(rand::random::<u8>()),
            1 => value = value.wrapping_sub(rand::random::<u8>()),
            2 => value = rand::random(),
            _ => {}
        }
        value
    }
}

impl FuzzerInput for u16 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn mutate(&self) -> Self {
        let mut value = *self;
        match rand::random::<u8>() % 3 {
            0 => value = value.wrapping_add(rand::random::<u16>()),
            1 => value = value.wrapping_sub(rand::random::<u16>()),
            2 => value = rand::random(),
            _ => {}
        }
        value
    }
}

impl FuzzerInput for u32 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn mutate(&self) -> Self {
        let mut value = *self;
        match rand::random::<u8>() % 3 {
            0 => value = value.wrapping_add(rand::random::<u32>()),
            1 => value = value.wrapping_sub(rand::random::<u32>()),
            2 => value = rand::random(),
            _ => {}
        }
        value
    }
}

impl FuzzerInput for u64 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn mutate(&self) -> Self {
        let mut value = *self;
        match rand::random::<u8>() % 3 {
            0 => value = value.wrapping_add(rand::random::<u64>()),
            1 => value = value.wrapping_sub(rand::random::<u64>()),
            2 => value = rand::random(),
            _ => {}
        }
        value
    }
}

impl FuzzerInput for i8 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn mutate(&self) -> Self {
        let mut value = *self;
        match rand::random::<u8>() % 3 {
            0 => value = value.wrapping_add(rand::random::<i8>()),
            1 => value = value.wrapping_sub(rand::random::<i8>()),
            2 => value = rand::random(),
            _ => {}
        }
        value
    }
}

impl FuzzerInput for i16 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn mutate(&self) -> Self {
        let mut value = *self;
        match rand::random::<u8>() % 3 {
            0 => value = value.wrapping_add(rand::random::<i16>()),
            1 => value = value.wrapping_sub(rand::random::<i16>()),
            2 => value = rand::random(),
            _ => {}
        }
        value
    }
}

impl FuzzerInput for i32 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn mutate(&self) -> Self {
        let mut value = *self;
        match rand::random::<u8>() % 3 {
            0 => value = value.wrapping_add(rand::random::<i32>()),
            1 => value = value.wrapping_sub(rand::random::<i32>()),
            2 => value = rand::random(),
            _ => {}
        }
        value
    }
}

impl FuzzerInput for i64 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn mutate(&self) -> Self {
        let mut value = *self;
        match rand::random::<u8>() % 3 {
            0 => value = value.wrapping_add(rand::random::<i64>()),
            1 => value = value.wrapping_sub(rand::random::<i64>()),
            2 => value = rand::random(),
            _ => {}
        }
        value
    }
}

impl FuzzerInput for f32 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn mutate(&self) -> Self {
        let mut value = *self;
        match rand::random::<u8>() % 3 {
            0 => value += rand::random::<f32>(),
            1 => value -= rand::random::<f32>(),
            2 => value = rand::random(),
            _ => {}
        }
        value
    }
}

impl FuzzerInput for f64 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn mutate(&self) -> Self {
        let mut value = *self;
        match rand::random::<u8>() % 3 {
            0 => value += rand::random::<f64>(),
            1 => value -= rand::random::<f64>(),
            2 => value = rand::random(),
            _ => {}
        }
        value
    }
}

impl FuzzerInput for bool {
    fn generate() -> Self {
        rand::random()
    }
    
    fn mutate(&self) -> Self {
        !*self
    }
}

impl FuzzerInput for char {
    fn generate() -> Self {
        rand::random::<u32>().try_into().unwrap_or(' ')
    }
    
    fn mutate(&self) -> Self {
        let mut code = *self as u32;
        match rand::random::<u8>() % 3 {
            0 => code = code.wrapping_add(rand::random::<u32>() % 100),
            1 => code = code.wrapping_sub(rand::random::<u32>() % 100),
            2 => code = rand::random(),
            _ => {}
        }
        code.try_into().unwrap_or(' ')
    }
}

impl FuzzerInput for String {
    fn generate() -> Self {
        let length = rand::random::<u8>() % 100;
        (0..length).map(|_| generate_char()).collect()
    }
    
    fn mutate(&self) -> Self {
        let mut result = self.clone();
        match rand::random::<u8>() % 4 {
            0 => {
                // 添加字符
                let pos = rand::random::<usize>() % (result.len() + 1);
                let c = generate_char();
                result.insert(pos, c);
            }
            1 => {
                // 删除字符
                if !result.is_empty() {
                    let pos = rand::random::<usize>() % result.len();
                    result.remove(pos);
                }
            }
            2 => {
                // 修改字符
                if !result.is_empty() {
                    let pos = rand::random::<usize>() % result.len();
                    let c = generate_char();
                    result.replace_range(pos..pos+1, &c.to_string());
                }
            }
            3 => {
                // 完全随机
                return Self::generate();
            }
            _ => {}
        }
        result
    }
}

fn generate_char() -> char {
    rand::random::<u32>().try_into().unwrap_or(' ')
}

/// 为元组实现 FuzzerInput
macro_rules! impl_fuzzer_input_tuple {
    (A) => {
        impl<A: FuzzerInput> FuzzerInput for (A,) {
            fn generate() -> Self {
                (A::generate(),)
            }
            
            fn mutate(&self) -> Self {
                let (ref a,) = *self;
                (A::mutate(a),)
            }
        }
    };
    (A, B) => {
        impl<A: FuzzerInput, B: FuzzerInput> FuzzerInput for (A, B) {
            fn generate() -> Self {
                (A::generate(), B::generate())
            }
            
            fn mutate(&self) -> Self {
                let (ref a, ref b) = *self;
                match rand::random::<usize>() % 2 {
                    0 => (A::mutate(a), B::clone(b)),
                    1 => (A::clone(a), B::mutate(b)),
                    _ => unreachable!(),
                }
            }
        }
    };
    (A, B, C) => {
        impl<A: FuzzerInput, B: FuzzerInput, C: FuzzerInput> FuzzerInput for (A, B, C) {
            fn generate() -> Self {
                (A::generate(), B::generate(), C::generate())
            }
            
            fn mutate(&self) -> Self {
                let (ref a, ref b, ref c) = *self;
                match rand::random::<usize>() % 3 {
                    0 => (A::mutate(a), B::clone(b), C::clone(c)),
                    1 => (A::clone(a), B::mutate(b), C::clone(c)),
                    2 => (A::clone(a), B::clone(b), C::mutate(c)),
                    _ => unreachable!(),
                }
            }
        }
    };
    (A, B, C, D) => {
        impl<A: FuzzerInput, B: FuzzerInput, C: FuzzerInput, D: FuzzerInput> FuzzerInput for (A, B, C, D) {
            fn generate() -> Self {
                (A::generate(), B::generate(), C::generate(), D::generate())
            }
            
            fn mutate(&self) -> Self {
                let (ref a, ref b, ref c, ref d) = *self;
                match rand::random::<usize>() % 4 {
                    0 => (A::mutate(a), B::clone(b), C::clone(c), D::clone(d)),
                    1 => (A::clone(a), B::mutate(b), C::clone(c), D::clone(d)),
                    2 => (A::clone(a), B::clone(b), C::mutate(c), D::clone(d)),
                    3 => (A::clone(a), B::clone(b), C::clone(c), D::mutate(d)),
                    _ => unreachable!(),
                }
            }
        }
    };
    (A, B, C, D, E) => {
        impl<A: FuzzerInput, B: FuzzerInput, C: FuzzerInput, D: FuzzerInput, E: FuzzerInput> FuzzerInput for (A, B, C, D, E) {
            fn generate() -> Self {
                (A::generate(), B::generate(), C::generate(), D::generate(), E::generate())
            }
            
            fn mutate(&self) -> Self {
                let (ref a, ref b, ref c, ref d, ref e) = *self;
                match rand::random::<usize>() % 5 {
                    0 => (A::mutate(a), B::clone(b), C::clone(c), D::clone(d), E::clone(e)),
                    1 => (A::clone(a), B::mutate(b), C::clone(c), D::clone(d), E::clone(e)),
                    2 => (A::clone(a), B::clone(b), C::mutate(c), D::clone(d), E::clone(e)),
                    3 => (A::clone(a), B::clone(b), C::clone(c), D::mutate(d), E::clone(e)),
                    4 => (A::clone(a), B::clone(b), C::clone(c), D::clone(d), E::mutate(e)),
                    _ => unreachable!(),
                }
            }
        }
    };
}

// 为常见元组实现 FuzzerInput
impl_fuzzer_input_tuple!(A, B);
impl_fuzzer_input_tuple!(A, B, C);
impl_fuzzer_input_tuple!(A, B, C, D);
impl_fuzzer_input_tuple!(A, B, C, D, E);

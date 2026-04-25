// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 属性测试模块
//! 
//! 提供属性测试功能，用于验证代码是否满足某些属性。

use crate::test::error::*;
use std::fmt::Debug;
use std::time::Duration;

/// 属性测试输入生成器
pub trait PropertyInput: Debug + Clone {
    /// 生成随机输入
    fn generate() -> Self;
    /// 生成简化的输入（用于缩小失败案例）
    fn simplify(&self) -> Option<Self>;
}

/// 属性测试结果
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropertyResult {
    /// 测试通过
    Passed,
    /// 测试失败
    Failed(String),
    /// 测试超时
    TimedOut,
}

/// 属性测试运行器
pub struct PropertyRunner<T: PropertyInput> {
    /// 测试函数
    test_fn: Box<dyn Fn(&T) -> PropertyResult + Send + Sync>,
    /// 运行时间限制
    timeout: Duration,
    /// 最大执行次数
    max_runs: u64,
    /// 发现的失败数
    failures: u64,
    /// 最小失败案例
    minimal_failure: Option<Box<T>>,
}

impl<T: PropertyInput> PropertyRunner<T> {
    /// 创建新的属性测试运行器
    pub fn new<F>(test_fn: F, timeout: Duration, max_runs: u64) -> Self
    where
        F: Fn(&T) -> PropertyResult + Send + Sync + 'static,
    {
        Self {
            test_fn: Box::new(test_fn),
            timeout,
            max_runs,
            failures: 0,
            minimal_failure: None,
        }
    }

    /// 运行属性测试
    pub fn run(&mut self) -> TestResult<()> {
        for run in 0..self.max_runs {
            // 生成随机输入
            let input = T::generate();
            
            // 运行测试
            let result = self.run_with_input(&input);
            
            match result {
                PropertyResult::Failed(msg) => {
                    self.failures += 1;
                    eprintln!("属性测试失败 (运行 {}): {:?}\n错误: {}", run, input, msg);
                    
                    // 尝试缩小失败案例
                    self.minimize_failure(input, msg);
                }
                PropertyResult::TimedOut => {
                    self.failures += 1;
                    eprintln!("属性测试超时 (运行 {}): {:?}", run, input);
                }
                PropertyResult::Passed => {
                    // 测试通过，继续
                }
            }
        }
        
        if self.failures > 0 {
            let message = if let Some(failure) = &self.minimal_failure {
                format!("属性测试失败，发现 {} 个失败案例，最小失败案例: {:?}", self.failures, failure)
            } else {
                format!("属性测试失败，发现 {} 个失败案例", self.failures)
            };
            
            Err(TestError::PropertyTestFailed {
                message,
                file: "property_test".to_string(),
                line: 0,
                column: 0,
            })
        } else {
            Ok(())
        }
    }

    /// 运行单个输入的测试
    fn run_with_input(&self, input: &T) -> PropertyResult {
        let start = std::time::Instant::now();
        
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            (self.test_fn)(input)
        }));
        
        let duration = start.elapsed();
        
        if duration > self.timeout {
            return PropertyResult::TimedOut;
        }
        
        match result {
            Ok(property_result) => property_result,
            Err(e) => {
                let error_msg = if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "未知错误".to_string()
                };
                PropertyResult::Failed(error_msg)
            }
        }
    }

    /// 缩小失败案例
    fn minimize_failure(&mut self, input: T, _error_msg: String) {
        let mut current_input = input;
        
        // 尝试通过简化输入来找到最小失败案例
        while let Some(simplified) = current_input.simplify() {
            let result = self.run_with_input(&simplified);
            match result {
                PropertyResult::Failed(_msg) => {
                    // 简化后的输入仍然失败，继续简化
                    current_input = simplified;
                }
                _ => {
                    // 简化后的输入通过了测试，停止简化
                    break;
                }
            }
        }
        
        // 更新最小失败案例
        if self.minimal_failure.is_none() {
            self.minimal_failure = Some(Box::new(current_input));
        }
    }
}

/// 基本类型的属性测试输入实现

impl PropertyInput for u8 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn simplify(&self) -> Option<Self> {
        if *self == 0 {
            None
        } else if *self == 1 {
            Some(0)
        } else {
            Some(*self / 2)
        }
    }
}

impl PropertyInput for u16 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn simplify(&self) -> Option<Self> {
        if *self == 0 {
            None
        } else if *self == 1 {
            Some(0)
        } else {
            Some(*self / 2)
        }
    }
}

impl PropertyInput for u32 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn simplify(&self) -> Option<Self> {
        if *self == 0 {
            None
        } else if *self == 1 {
            Some(0)
        } else {
            Some(*self / 2)
        }
    }
}

impl PropertyInput for u64 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn simplify(&self) -> Option<Self> {
        if *self == 0 {
            None
        } else if *self == 1 {
            Some(0)
        } else {
            Some(*self / 2)
        }
    }
}

impl PropertyInput for i8 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn simplify(&self) -> Option<Self> {
        if *self == 0 {
            None
        } else if *self == 1 || *self == -1 {
            Some(0)
        } else {
            Some(*self / 2)
        }
    }
}

impl PropertyInput for i16 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn simplify(&self) -> Option<Self> {
        if *self == 0 {
            None
        } else if *self == 1 || *self == -1 {
            Some(0)
        } else {
            Some(*self / 2)
        }
    }
}

impl PropertyInput for i32 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn simplify(&self) -> Option<Self> {
        if *self == 0 {
            None
        } else if *self == 1 || *self == -1 {
            Some(0)
        } else {
            Some(*self / 2)
        }
    }
}

impl PropertyInput for i64 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn simplify(&self) -> Option<Self> {
        if *self == 0 {
            None
        } else if *self == 1 || *self == -1 {
            Some(0)
        } else {
            Some(*self / 2)
        }
    }
}

impl PropertyInput for f32 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn simplify(&self) -> Option<Self> {
        if *self == 0.0 {
            None
        } else if *self == 1.0 || *self == -1.0 {
            Some(0.0)
        } else {
            Some(*self / 2.0)
        }
    }
}

impl PropertyInput for f64 {
    fn generate() -> Self {
        rand::random()
    }
    
    fn simplify(&self) -> Option<Self> {
        if *self == 0.0 {
            None
        } else if *self == 1.0 || *self == -1.0 {
            Some(0.0)
        } else {
            Some(*self / 2.0)
        }
    }
}

impl PropertyInput for bool {
    fn generate() -> Self {
        rand::random()
    }
    
    fn simplify(&self) -> Option<Self> {
        // 布尔值只有两种可能，无法进一步简化
        None
    }
}

impl PropertyInput for char {
    fn generate() -> Self {
        rand::random::<u32>().try_into().unwrap_or(' ')
    }
    
    fn simplify(&self) -> Option<Self> {
        if *self == ' ' {
            None
        } else {
            Some(' ')
        }
    }
}

impl PropertyInput for String {
    fn generate() -> Self {
        let length = rand::random::<u8>() % 100;
        (0..length).map(|_| generate_char()).collect()
    }
    
    fn simplify(&self) -> Option<Self> {
        if self.is_empty() {
            None
        } else if self.len() == 1 {
            Some(String::new())
        } else {
            // 尝试减少字符串长度
            Some(self[..self.len()/2].to_string())
        }
    }
}

fn generate_char() -> char {
    rand::random::<u32>().try_into().unwrap_or(' ')
}

/// 为元组实现 PropertyInput
macro_rules! impl_property_input_tuple {
    (A) => {
        impl<A: PropertyInput> PropertyInput for (A,) {
            fn generate() -> Self {
                (A::generate(),)
            }
            
            fn simplify(&self) -> Option<Self> {
                let (ref a,) = *self;
                if let Some(simplified) = a.simplify() {
                    Some((simplified,))
                } else {
                    None
                }
            }
        }
    };
    (A, B) => {
        impl<A: PropertyInput, B: PropertyInput> PropertyInput for (A, B) {
            fn generate() -> Self {
                (A::generate(), B::generate())
            }
            
            fn simplify(&self) -> Option<Self> {
                let (ref a, ref b) = *self;
                if let Some(simplified) = a.simplify() {
                    Some((simplified, B::clone(b)))
                } else if let Some(simplified) = b.simplify() {
                    Some((A::clone(a), simplified))
                } else {
                    None
                }
            }
        }
    };
    (A, B, C) => {
        impl<A: PropertyInput, B: PropertyInput, C: PropertyInput> PropertyInput for (A, B, C) {
            fn generate() -> Self {
                (A::generate(), B::generate(), C::generate())
            }
            
            fn simplify(&self) -> Option<Self> {
                let (ref a, ref b, ref c) = *self;
                if let Some(simplified) = a.simplify() {
                    Some((simplified, B::clone(b), C::clone(c)))
                } else if let Some(simplified) = b.simplify() {
                    Some((A::clone(a), simplified, C::clone(c)))
                } else if let Some(simplified) = c.simplify() {
                    Some((A::clone(a), B::clone(b), simplified))
                } else {
                    None
                }
            }
        }
    };
    (A, B, C, D) => {
        impl<A: PropertyInput, B: PropertyInput, C: PropertyInput, D: PropertyInput> PropertyInput for (A, B, C, D) {
            fn generate() -> Self {
                (A::generate(), B::generate(), C::generate(), D::generate())
            }
            
            fn simplify(&self) -> Option<Self> {
                let (ref a, ref b, ref c, ref d) = *self;
                if let Some(simplified) = a.simplify() {
                    Some((simplified, B::clone(b), C::clone(c), D::clone(d)))
                } else if let Some(simplified) = b.simplify() {
                    Some((A::clone(a), simplified, C::clone(c), D::clone(d)))
                } else if let Some(simplified) = c.simplify() {
                    Some((A::clone(a), B::clone(b), simplified, D::clone(d)))
                } else if let Some(simplified) = d.simplify() {
                    Some((A::clone(a), B::clone(b), C::clone(c), simplified))
                } else {
                    None
                }
            }
        }
    };
    (A, B, C, D, E) => {
        impl<A: PropertyInput, B: PropertyInput, C: PropertyInput, D: PropertyInput, E: PropertyInput> PropertyInput for (A, B, C, D, E) {
            fn generate() -> Self {
                (A::generate(), B::generate(), C::generate(), D::generate(), E::generate())
            }
            
            fn simplify(&self) -> Option<Self> {
                let (ref a, ref b, ref c, ref d, ref e) = *self;
                if let Some(simplified) = a.simplify() {
                    Some((simplified, B::clone(b), C::clone(c), D::clone(d), E::clone(e)))
                } else if let Some(simplified) = b.simplify() {
                    Some((A::clone(a), simplified, C::clone(c), D::clone(d), E::clone(e)))
                } else if let Some(simplified) = c.simplify() {
                    Some((A::clone(a), B::clone(b), simplified, D::clone(d), E::clone(e)))
                } else if let Some(simplified) = d.simplify() {
                    Some((A::clone(a), B::clone(b), C::clone(c), simplified, E::clone(e)))
                } else if let Some(simplified) = e.simplify() {
                    Some((A::clone(a), B::clone(b), C::clone(c), D::clone(d), simplified))
                } else {
                    None
                }
            }
        }
    };
}

// 为常见元组实现 PropertyInput
impl_property_input_tuple!(A, B);
impl_property_input_tuple!(A, B, C);
impl_property_input_tuple!(A, B, C, D);
impl_property_input_tuple!(A, B, C, D, E);

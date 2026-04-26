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

//! 基准测试模块
//!
//! 提供微基准测试框架：
//! - 测量代码执行时间
//! - 防止编译器优化
//! - 统计多次运行结果
//! - 生成基准测试报告
//! - 比较基准测试结果

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// 基准测试结果
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub mean_ns: f64,
    pub median_ns: f64,
    pub std_dev_ns: f64,
    pub min_ns: u64,
    pub max_ns: u64,
    pub iterations: u64,
}

/// 基准测试组
pub struct BenchmarkGroup {
    name: String,
    benchmarks: HashMap<String, Vec<u64>>,
    iterations: u64,
    warmup_iterations: u64,
}

/// 基准测试配置
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub iterations: u64,
    pub warmup_iterations: u64,
    pub confidence_level: f64,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        BenchmarkConfig {
            iterations: 1000,
            warmup_iterations: 100,
            confidence_level: 0.95,
        }
    }
}

/// 基准测试运行器
pub struct BenchmarkRunner {
    config: BenchmarkConfig,
    groups: HashMap<String, BenchmarkGroup>,
}

/// 基准测试防止优化辅助
pub struct Blackhole;

impl Blackhole {
    /// 消耗值，防止编译器优化掉计算
    pub fn consume<T>(_value: T) {
        // 在实际实现中，这会使用 volatile 读取或 asm 指令
        // 来确保编译器不能优化掉这个值
    }

    /// 防止返回值优化
    pub fn r#return<T>(value: T) -> T {
        Self::consume(value.clone());
        Self::consume(&value);
        value
    }
}

/// 计时器
pub struct Timer {
    start: Instant,
    name: String,
}

impl Timer {
    /// 创建新的计时器
    pub fn new(name: &str) -> Self {
        Timer {
            start: Instant::now(),
            name: name.to_string(),
        }
    }

    /// 获取经过的时间
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// 获取经过的纳秒数
    pub fn elapsed_ns(&self) -> u64 {
        self.start.elapsed().as_nanos() as u64
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        // 可以在这里自动记录时间
    }
}

/// 基准测试辅助宏
#[macro_export]
macro_rules! bench {
    ($name:expr, $iterations:expr, $block:expr) => {{
        let mut times = Vec::with_capacity($iterations);
        
        // 预热
        for _ in 0..100 {
            let start = std::time::Instant::now();
            $block;
            let _ = start.elapsed();
        }
        
        // 实际测量
        for _ in 0..$iterations {
            let start = std::time::Instant::now();
            $block;
            times.push(start.elapsed().as_nanos() as u64);
        }
        
        // 计算统计
        let sum: u64 = times.iter().sum();
        let mean = sum as f64 / $iterations as f64;
        let mut sorted = times.clone();
        sorted.sort();
        let median = sorted[$iterations as usize / 2];
        
        // 计算标准差
        let variance: f64 = times.iter()
            .map(|&t| {
                let diff = t as f64 - mean;
                diff * diff
            })
            .sum::<f64>() / $iterations as f64;
        let std_dev = variance.sqrt();
        
        BenchmarkResult {
            name: $name.to_string(),
            mean_ns: mean,
            median_ns: median as f64,
            std_dev_ns: std_dev,
            min_ns: *sorted.first().unwrap_or(&0),
            max_ns: *sorted.last().unwrap_or(&0),
            iterations: $iterations,
        }
    }};
}

impl BenchmarkGroup {
    /// 创建新的基准测试组
    pub fn new(name: &str) -> Self {
        BenchmarkGroup {
            name: name.to_string(),
            benchmarks: HashMap::new(),
            iterations: 1000,
            warmup_iterations: 100,
        }
    }

    /// 设置迭代次数
    pub fn iterations(mut self, iterations: u64) -> Self {
        self.iterations = iterations;
        self
    }

    /// 设置预热迭代次数
    pub fn warmup(mut self, iterations: u64) -> Self {
        self.warmup_iterations = iterations;
        self
    }

    /// 添加基准测试
    pub fn bench<F>(&mut self, name: &str, f: F)
    where
        F: FnMut(),
    {
        let mut times = Vec::with_capacity(self.iterations as usize);
        
        // 预热
        for _ in 0..self.warmup_iterations {
            let start = Instant::now();
            f();
            let _elapsed = start.elapsed();
        }
        
        // 实际测量
        for _ in 0..self.iterations {
            let start = Instant::now();
            f();
            times.push(start.elapsed().as_nanos() as u64);
        }
        
        self.benchmarks.insert(name.to_string(), times);
    }

    /// 添加基准测试（带参数）
    pub fn bench_with_args<A, F>(&mut self, name: &str, args: A, mut f: F)
    where
        A: Clone,
        F: FnMut(A),
    {
        let mut times = Vec::with_capacity(self.iterations as usize);
        
        // 预热
        for _ in 0..self.warmup_iterations {
            let start = Instant::now();
            f(args.clone());
            let _elapsed = start.elapsed();
        }
        
        // 实际测量
        for _ in 0..self.iterations {
            let start = Instant::now();
            f(args.clone());
            times.push(start.elapsed().as_nanos() as u64);
        }
        
        self.benchmarks.insert(name.to_string(), times);
    }

    /// 获取基准测试结果
    pub fn results(&self) -> HashMap<String, BenchmarkResult> {
        self.benchmarks.iter().map(|(name, times)| {
            let iterations = times.len() as u64;
            let sum: u64 = times.iter().sum();
            let mean = sum as f64 / iterations as f64;
            let mut sorted = times.clone();
            sorted.sort();
            let median = sorted[iterations as usize / 2];
            
            let variance: f64 = times.iter()
                .map(|&t| {
                    let diff = t as f64 - mean;
                    diff * diff
                })
                .sum::<f64>() / iterations as f64;
            let std_dev = variance.sqrt();
            
            BenchmarkResult {
                name: name.clone(),
                mean_ns: mean,
                median_ns: median as f64,
                std_dev_ns: std_dev,
                min_ns: *sorted.first().unwrap_or(&0),
                max_ns: *sorted.last().unwrap_or(&0),
                iterations,
            }
        }).collect()
    }

    /// 打印结果
    pub fn print_results(&self) {
        println!("\n基准测试组: {}", self.name);
        println!("{:=<80}", "");
        
        for (name, result) in self.results() {
            println!("\n{}", name);
            println!("  平均:   {:>10.2} ns", result.mean_ns);
            println!("  中位数: {:>10.2} ns", result.median_ns);
            println!("  标准差: {:>10.2} ns", result.std_dev_ns);
            println!("  最小值: {:>10.2} ns", result.min_ns as f64);
            println!("  最大值: {:>10.2} ns", result.max_ns as f64);
            println!("  迭代:   {}", result.iterations);
        }
    }
}

impl BenchmarkRunner {
    /// 创建新的基准测试运行器
    pub fn new() -> Self {
        BenchmarkRunner {
            config: BenchmarkConfig::default(),
            groups: HashMap::new(),
        }
    }

    /// 设置配置
    pub fn config(mut self, config: BenchmarkConfig) -> Self {
        self.config = config;
        self
    }

    /// 添加基准测试组
    pub fn add_group(&mut self, group: BenchmarkGroup) {
        self.groups.insert(group.name.clone(), group);
    }

    /// 创建基准测试组
    pub fn group(&mut self, name: &str) -> &mut BenchmarkGroup {
        self.groups.entry(name.to_string()).or_insert_with(|| {
            BenchmarkGroup::new(name)
        });
        self.groups.get_mut(name).unwrap()
    }

    /// 运行所有基准测试
    pub fn run(&mut self) {
        for group in self.groups.values_mut() {
            // 基准测试已经包含在 add_group 中了
        }
    }

    /// 获取所有结果
    pub fn results(&self) -> HashMap<String, HashMap<String, BenchmarkResult>> {
        self.groups.iter().map(|(name, group)| {
            (name.clone(), group.results())
        }).collect()
    }

    /// 打印所有结果
    pub fn print_results(&self) {
        for group in self.groups.values() {
            group.print_results();
        }
    }
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// 性能回归检测
pub struct RegressionDetector {
    baseline: HashMap<String, BenchmarkResult>,
    current: HashMap<String, BenchmarkResult>,
    threshold: f64,
}

impl RegressionDetector {
    /// 创建新的回归检测器
    pub fn new(threshold: f64) -> Self {
        RegressionDetector {
            baseline: HashMap::new(),
            current: HashMap::new(),
            threshold,
        }
    }

    /// 保存基线
    pub fn save_baseline(&mut self, name: &str, result: BenchmarkResult) {
        self.baseline.insert(name.to_string(), result);
    }

    /// 添加当前结果
    pub fn add_current(&mut self, name: &str, result: BenchmarkResult) {
        self.current.insert(name.to_string(), result);
    }

    /// 检测回归
    pub fn detect(&self) -> Vec<Regression> {
        let mut regressions = Vec::new();
        
        for (name, current_result) in &self.current {
            if let Some(baseline_result) = self.baseline.get(name) {
                let change = (current_result.mean_ns - baseline_result.mean_ns) 
                    / baseline_result.mean_ns * 100.0;
                
                if change.abs() > self.threshold * 100.0 {
                    regressions.push(Regression {
                        name: name.clone(),
                        baseline_ns: baseline_result.mean_ns,
                        current_ns: current_result.mean_ns,
                        change_percent: change,
                    });
                }
            }
        }
        
        regressions
    }

    /// 打印比较结果
    pub fn print_comparison(&self) {
        println!("\n基准测试结果比较:");
        println!("{:=<80}", "");
        println!("{:<20} {:>15} {:>15} {:>15}", "名称", "基线", "当前", "变化");
        println!("{:-<80}", "");
        
        for (name, current_result) in &self.current {
            if let Some(baseline_result) = self.baseline.get(name) {
                let change = (current_result.mean_ns - baseline_result.mean_ns) 
                    / baseline_result.mean_ns * 100.0;
                
                let change_str = if change > 0.0 {
                    format!("+{:.1}%", change)
                } else if change < 0.0 {
                    format!("{:.1}%", change)
                } else {
                    "0.0%".to_string()
                };
                
                let regression_marker = if change.abs() > self.threshold * 100.0 {
                    if change > 0.0 { " (回归)" } else { " (改进)" }
                } else {
                    ""
                };
                
                println!(
                    "{:<20} {:>15.2} ns {:>15.2} ns {:>15}{}",
                    name,
                    baseline_result.mean_ns,
                    current_result.mean_ns,
                    change_str,
                    regression_marker
                );
            }
        }
    }
}

/// 性能回归信息
#[derive(Debug, Clone)]
pub struct Regression {
    pub name: String,
    pub baseline_ns: f64,
    pub current_ns: f64,
    pub change_percent: f64,
}

/// 测量帮助宏
#[macro_export]
macro_rules! measure_time {
    ($name:expr, $block:expr) => {{
        let start = std::time::Instant::now();
        let result = $block;
        let elapsed = start.elapsed();
        println!("[测量] {} 耗时: {:?}", $name, elapsed);
        (result, elapsed)
    }};
}

/// 防止优化辅助函数
pub fn prevent_optimization<T>(value: T) -> T {
    // 使用 volatile 读取防止优化
    let mut tmp = value;
    let ptr = &mut tmp as *mut T;
    let volatile_ptr = ptr as *mut std::sync::atomic::UnsafeCell<T>;
    // 在实际代码中，这里会使用 volatile 读取
    // unsafe { (*volatile_ptr).get().read_volatile() }
    tmp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blackhole() {
        let value = 42;
        Blackhole::consume(value);
        let returned = Blackhole::r#return(value);
        assert_eq!(returned, 42);
    }

    #[test]
    fn test_timer() {
        let timer = Timer::new("test");
        std::thread::sleep(Duration::from_millis(1));
        let elapsed = timer.elapsed();
        assert!(elapsed.as_millis() >= 1);
    }

    #[test]
    fn test_benchmark_group() {
        let mut group = BenchmarkGroup::new("test");
        
        group.bench("sleep", || {
            std::thread::sleep(Duration::from_micros(100));
        });
        
        let results = group.results();
        assert!(results.contains_key("sleep"));
        
        let result = &results["sleep"];
        assert!(result.mean_ns >= 100_000.0); // 至少 100 微秒
    }

    #[test]
    fn test_regression_detector() {
        let mut detector = RegressionDetector::new(0.1); // 10% 阈值
        
        detector.save_baseline("test", BenchmarkResult {
            name: "test".to_string(),
            mean_ns: 100.0,
            median_ns: 100.0,
            std_dev_ns: 10.0,
            min_ns: 90,
            max_ns: 110,
            iterations: 100,
        });
        
        detector.add_current("test", BenchmarkResult {
            name: "test".to_string(),
            mean_ns: 120.0, // 20% 回归
            median_ns: 120.0,
            std_dev_ns: 10.0,
            min_ns: 110,
            max_ns: 130,
            iterations: 100,
        });
        
        let regressions = detector.detect();
        assert_eq!(regressions.len(), 1);
        assert_eq!(regressions[0].name, "test");
    }

    #[test]
    fn test_measure_time_macro() {
        let (result, _) = measure_time!("test operation", || {
            42
        });
        assert_eq!(result, 42);
    }
}

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

use std::time::{Duration, Instant};
use std::fmt;
use serde::{Serialize, Deserialize};

/// 基准测试配置
#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    /// 最小样本数
    pub min_samples: usize,
    /// 最小时间
    pub min_time: Duration,
    /// 最大时间
    pub max_time: Option<Duration>,
    /// 预热迭代次数
    pub warmup_iters: usize,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        BenchmarkConfig {
            min_samples: 100,
            min_time: Duration::from_secs(3),
            max_time: None,
            warmup_iters: 10,
        }
    }
}

/// 基准测试结果统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkStats {
    /// 样本数
    pub samples: usize,
    /// 平均时间
    pub mean: Duration,
    /// 中位数
    pub median: Duration,
    /// 最小值
    pub min: Duration,
    /// 最大值
    pub max: Duration,
    /// 标准差
    pub std_dev: Duration,
    /// 吞吐量（每秒操作数）
    pub throughput: f64,
}

impl BenchmarkStats {
    pub fn from_samples(mut samples: Vec<Duration>) -> Self {
        samples.sort();
        
        let n = samples.len();
        if n == 0 {
            return BenchmarkStats {
                samples: 0,
                mean: Duration::from_secs(0),
                median: Duration::from_secs(0),
                min: Duration::from_secs(0),
                max: Duration::from_secs(0),
                std_dev: Duration::from_secs(0),
                throughput: 0.0,
            };
        }
        
        let total: f64 = samples
            .iter()
            .map(|d| d.as_secs_f64())
            .sum();
        let mean_secs = total / n as f64;
        let mean = Duration::from_secs_f64(mean_secs);
        
        let median = if n % 2 == 0 {
            let a = samples[n / 2 - 1];
            let b = samples[n / 2];
            Duration::from_secs_f64((a.as_secs_f64() + b.as_secs_f64()) / 2.0)
        } else {
            samples[n / 2]
        };
        
        let min = samples.first().cloned().unwrap();
        let max = samples.last().cloned().unwrap();
        
        let variance: f64 = samples
            .iter()
            .map(|d| (d.as_secs_f64() - mean_secs).powi(2))
            .sum::<f64>()
            / n as f64;
        let std_dev_secs = variance.sqrt();
        let std_dev = Duration::from_secs_f64(std_dev_secs);
        
        let throughput = if mean_secs > 0.0 {
            1.0 / mean_secs
        } else {
            0.0
        };
        
        BenchmarkStats {
            samples: n,
            mean,
            median,
            min,
            max,
            std_dev,
            throughput,
        }
    }
}

impl fmt::Display for BenchmarkStats {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "  样本数: {}", self.samples)?;
        writeln!(f, "  平均时间: {:.3} ms", self.mean.as_secs_f64() * 1000.0)?;
        writeln!(f, "  中位数: {:.3} ms", self.median.as_secs_f64() * 1000.0)?;
        writeln!(f, "  最小值: {:.3} ms", self.min.as_secs_f64() * 1000.0)?;
        writeln!(f, "  最大值: {:.3} ms", self.max.as_secs_f64() * 1000.0)?;
        writeln!(f, "  标准差: {:.3} ms", self.std_dev.as_secs_f64() * 1000.0)?;
        write!(f, "  吞吐量: {:.2} ops/s", self.throughput)?;
        Ok(())
    }
}

/// 基准测试结果
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// 名称
    pub name: String,
    /// 统计
    pub stats: BenchmarkStats,
    /// 样本（保留用于比较）
    pub samples: Vec<Duration>,
}

impl BenchmarkResult {
    pub fn new(name: String, stats: BenchmarkStats, samples: Vec<Duration>) -> Self {
        BenchmarkResult {
            name,
            stats,
            samples,
        }
    }

    pub fn compare_with(&self, baseline: &BenchmarkStats) -> PerformanceComparison {
        let baseline_mean = baseline.mean.as_secs_f64();
        let current_mean = self.stats.mean.as_secs_f64();
        
        let difference = if baseline_mean > 0.0 {
            (current_mean - baseline_mean) / baseline_mean * 100.0
        } else {
            0.0
        };
        
        let status = if difference <= -5.0 {
            // 超过5%的性能提升
            ComparisonStatus::Improved
        } else if difference >= 5.0 {
            // 超过5%的性能下降
            ComparisonStatus::Regression
        } else {
            ComparisonStatus::Stable
        };
        
        PerformanceComparison {
            name: self.name.clone(),
            status,
            difference_percent: difference,
            baseline: Some(baseline.clone()),
            current: self.stats.clone(),
        }
    }
}

impl fmt::Display for BenchmarkResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "基准测试: {}", self.name)?;
        write!(f, "{}", self.stats)?;
        Ok(())
    }
}

/// 比较状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonStatus {
    /// 性能提升
    Improved,
    /// 性能下降
    Regression,
    /// 稳定
    Stable,
}

/// 性能比较结果
#[derive(Debug, Clone)]
pub struct PerformanceComparison {
    pub name: String,
    pub status: ComparisonStatus,
    pub difference_percent: f64,
    pub baseline: Option<BenchmarkStats>,
    pub current: BenchmarkStats,
}

impl fmt::Display for PerformanceComparison {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let symbol = match self.status {
            ComparisonStatus::Improved => "🚀",
            ComparisonStatus::Regression => "⚠️",
            ComparisonStatus::Stable => "✓",
        };
        
        write!(
            f,
            "{} {}: {:.1}% ({:.3}ms vs {:.3}ms)",
            symbol,
            self.name,
            self.difference_percent,
            self.current.mean.as_secs_f64() * 1000.0,
            if let Some(b) = &self.baseline {
                b.mean.as_secs_f64() * 1000.0
            } else {
                0.0
            }
        )?;
        
        Ok(())
    }
}

/// 基准测试运行器
pub struct BenchmarkRunner {
    /// 配置
    config: BenchmarkConfig,
    /// 结果
    results: Vec<BenchmarkResult>,
}

impl BenchmarkRunner {
    pub fn new() -> Self {
        BenchmarkRunner {
            config: BenchmarkConfig::default(),
            results: Vec::new(),
        }
    }

    pub fn with_config(mut self, config: BenchmarkConfig) -> Self {
        self.config = config;
        self
    }

    pub fn run<F: Fn()>(&mut self, name: String, f: F) -> &BenchmarkResult {
        // 预热
        for _ in 0..self.config.warmup_iters {
            black_box(f());
        }

        // 测量
        let mut samples = Vec::new();
        let start_all = Instant::now();
        
        while samples.len() < self.config.min_samples 
            || start_all.elapsed() < self.config.min_time 
        {
            if let Some(max_time) = self.config.max_time {
                if start_all.elapsed() > max_time {
                    break;
                }
            }
            
            let start = Instant::now();
            black_box(f());
            let duration = start.elapsed();
            samples.push(duration);
        }
        
        let stats = BenchmarkStats::from_samples(samples.clone());
        let result = BenchmarkResult::new(name, stats, samples);
        
        self.results.push(result);
        self.results.last().unwrap()
    }

    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }

    pub fn print_results(&self) {
        for result in &self.results {
            println!("{}", result);
            println!();
        }
    }

    pub fn compare_with_baseline(&self, _baseline: &[BenchmarkStats]) -> Vec<PerformanceComparison> {
        // 这里实现比较逻辑
        Vec::new()
    }

    pub fn save_baseline(&self, path: &str) -> std::io::Result<()> {
        let baselines: Vec<_> = self.results.iter().map(|r| r.stats.clone()).collect();
        let json = serde_json::to_string_pretty(&baselines)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_baseline(path: &str) -> std::io::Result<Vec<BenchmarkStats>> {
        let json = std::fs::read_to_string(path)?;
        let baselines: Vec<BenchmarkStats> = serde_json::from_str(&json)?;
        Ok(baselines)
    }
}

impl Default for BenchmarkRunner {
    fn default() -> Self {
        Self::new()
    }
}

/// 防止编译器优化掉值
/// 使用volatile读取来确保值不会被优化掉
#[inline(never)]
pub fn black_box<T>(dummy: T) {
    unsafe {
        std::ptr::read_volatile(&dummy);
        std::mem::forget(dummy);
    }
}

/// 基准测试宏
#[macro_export]
macro_rules! benchmark {
    ($name:expr, $body:expr) => {
        $crate::test::bench::BenchmarkRunner::new().run($name.to_string(), $body)
    };
}

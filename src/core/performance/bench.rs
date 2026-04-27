use std::time::Instant;

#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    pub name: String,
    pub mean_ns: f64,
    pub median_ns: f64,
    pub std_dev_ns: f64,
    pub min_ns: u64,
    pub max_ns: u64,
    pub iterations: usize,
}

#[derive(Debug, Clone)]
pub struct BenchmarkConfig {
    pub iterations: usize,
    pub warmup: usize,
    pub name: String,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        BenchmarkConfig {
            iterations: 1000,
            warmup: 100,
            name: "default".to_string(),
        }
    }
}

pub struct BenchmarkGroup {
    name: String,
    config: BenchmarkConfig,
    results: Vec<BenchmarkResult>,
}

impl BenchmarkGroup {
    pub fn new(name: &str) -> Self {
        BenchmarkGroup {
            name: name.to_string(),
            config: BenchmarkConfig::default(),
            results: Vec::new(),
        }
    }

    pub fn iterations(mut self, count: usize) -> Self {
        self.config.iterations = count;
        self
    }

    pub fn warmup(mut self, count: usize) -> Self {
        self.config.warmup = count;
        self
    }

    pub fn bench<F>(&mut self, name: &str, mut f: F)
    where
        F: FnMut(),
    {
        let mut times = Vec::with_capacity(self.config.iterations);

        for _ in 0..self.config.warmup {
            f();
            std::hint::black_box(());
        }

        for _ in 0..self.config.iterations {
            let start = Instant::now();
            f();
            let duration = start.elapsed();
            times.push(duration.as_nanos() as u64);
        }

        let result = Self::calculate_stats(name.to_string(), &times);
        self.results.push(result);
    }

    pub fn bench_with_result<F, T>(&mut self, name: &str, mut f: F)
    where
        F: FnMut() -> T,
    {
        let mut times = Vec::with_capacity(self.config.iterations);

        for _ in 0..self.config.warmup {
            let _ = std::hint::black_box(f());
        }

        for _ in 0..self.config.iterations {
            let start = Instant::now();
            let _ = std::hint::black_box(f());
            let duration = start.elapsed();
            times.push(duration.as_nanos() as u64);
        }

        let result = Self::calculate_stats(name.to_string(), &times);
        self.results.push(result);
    }

    fn calculate_stats(name: String, times: &[u64]) -> BenchmarkResult {
        let mut sorted_times = times.to_vec();
        sorted_times.sort_unstable();

        let sum: u64 = sorted_times.iter().sum();
        let count = sorted_times.len();
        let mean = sum as f64 / count as f64;

        let median = if count % 2 == 0 {
            let mid = count / 2;
            (sorted_times[mid - 1] + sorted_times[mid]) as f64 / 2.0
        } else {
            sorted_times[count / 2] as f64
        };

        let variance = sorted_times
            .iter()
            .map(|&t| (t as f64 - mean).powi(2))
            .sum::<f64>()
            / count as f64;
        let std_dev = variance.sqrt();

        let min = *sorted_times.first().unwrap_or(&0);
        let max = *sorted_times.last().unwrap_or(&0);

        BenchmarkResult {
            name,
            mean_ns: mean,
            median_ns: median,
            std_dev_ns: std_dev,
            min_ns: min,
            max_ns: max,
            iterations: count,
        }
    }

    pub fn print_results(&self) {
        println!("=== Benchmark Group: {} ===", self.name);
        for result in &self.results {
            println!("Benchmark: {}", result.name);
            println!("  Mean: {:.2} ns/iter", result.mean_ns);
            println!("  Median: {:.2} ns/iter", result.median_ns);
            println!("  Std Dev: {:.2} ns", result.std_dev_ns);
            println!("  Min: {} ns", result.min_ns);
            println!("  Max: {} ns", result.max_ns);
            println!("  Iterations: {}", result.iterations);
        }
    }

    pub fn results(&self) -> &[BenchmarkResult] {
        &self.results
    }
}

pub struct Blackhole;

impl Blackhole {
    #[inline(never)]
    pub fn consume<T>(value: T) {
        let _ = value;
        std::hint::black_box(());
    }

    #[inline(never)]
    pub fn return_<T>(value: T) -> T {
        value
    }
}

#[macro_export]
macro_rules! bench {
    ($name:expr, $iterations:expr, $f:expr) => {{
        let mut times = Vec::with_capacity($iterations);
        let f = $f;
        
        for _ in 0..100 {
            let _ = std::hint::black_box(f());
        }
        
        for _ in 0..$iterations {
            let start = std::time::Instant::now();
            let _ = std::hint::black_box(f());
            let duration = start.elapsed();
            times.push(duration.as_nanos() as u64);
        }
        
        let mut sorted_times = times.clone();
        sorted_times.sort_unstable();
        
        let sum: u64 = sorted_times.iter().sum();
        let count = sorted_times.len();
        let mean = sum as f64 / count as f64;
        
        let median = if count % 2 == 0 {
            let mid = count / 2;
            (sorted_times[mid - 1] + sorted_times[mid]) as f64 / 2.0
        } else {
            sorted_times[count / 2] as f64
        };
        
        let variance = sorted_times
            .iter()
            .map(|&t| (t as f64 - mean).powi(2))
            .sum::<f64>()
            / count as f64;
        let std_dev = variance.sqrt();
        
        $crate::core::performance::bench::BenchmarkResult {
            name: $name.to_string(),
            mean_ns: mean,
            median_ns: median,
            std_dev_ns: std_dev,
            min_ns: *sorted_times.first().unwrap_or(&0),
            max_ns: *sorted_times.last().unwrap_or(&0),
            iterations: count,
        }
    }};
}

pub struct RegressionDetector {
    baseline: std::collections::HashMap<String, BenchmarkResult>,
    current: std::collections::HashMap<String, BenchmarkResult>,
    threshold: f64,
}

impl RegressionDetector {
    pub fn new(threshold: f64) -> Self {
        RegressionDetector {
            baseline: std::collections::HashMap::new(),
            current: std::collections::HashMap::new(),
            threshold,
        }
    }

    pub fn save_baseline(&mut self, name: &str, result: BenchmarkResult) {
        self.baseline.insert(name.to_string(), result);
    }

    pub fn add_current(&mut self, name: &str, result: BenchmarkResult) {
        self.current.insert(name.to_string(), result);
    }

    pub fn detect(&self) -> Vec<(String, f64)> {
        let mut regressions = Vec::new();
        
        for (name, baseline) in &self.baseline {
            if let Some(current) = self.current.get(name) {
                let ratio = current.mean_ns / baseline.mean_ns;
                if ratio > (1.0 + self.threshold) {
                    regressions.push((name.clone(), ratio));
                }
            }
        }
        
        regressions
    }

    pub fn print_comparison(&self) {
        println!("=== Benchmark Comparison:");
        for (name, baseline) in &self.baseline {
            if let Some(current) = self.current.get(name) {
                let ratio = current.mean_ns / baseline.mean_ns;
                let change = if ratio > 1.0 { "slower" } else { "faster" };
                let percent = (ratio - 1.0).abs() * 100.0;
                println!(
                    "  {}: baseline {:.2} ns, current {:.2} ns ({:.1}% {})",
                    name, baseline.mean_ns, current.mean_ns, percent, change
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_group() {
        let mut group = BenchmarkGroup::new("test")
            .iterations(100)
            .warmup(10);

        group.bench("simple addition", || {
            let _ = 1 + 2;
        });

        assert_eq!(group.results().len(), 1);
    }

    #[test]
    fn test_blackhole() {
        let value = 42;
        Blackhole::consume(value);
        let returned = Blackhole::return_(value);
        assert_eq!(returned, 42);
    }

    #[test]
    fn test_regression_detector() {
        let mut detector = RegressionDetector::new(0.05);

        let baseline = BenchmarkResult {
            name: "test".to_string(),
            mean_ns: 100.0,
            median_ns: 100.0,
            std_dev_ns: 10.0,
            min_ns: 90,
            max_ns: 110,
            iterations: 100,
        };
        detector.save_baseline("test", baseline);

        let current = BenchmarkResult {
            name: "test".to_string(),
            mean_ns: 110.0,
            median_ns: 110.0,
            std_dev_ns: 10.0,
            min_ns: 100,
            max_ns: 120,
            iterations: 100,
        };
        detector.add_current("test", current);

        let regressions = detector.detect();
        assert_eq!(regressions.len(), 1);
    }
}

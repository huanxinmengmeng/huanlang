use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct TimerStats {
    pub name: String,
    pub count: usize,
    pub total_time: Duration,
    pub min_time: Duration,
    pub max_time: Duration,
    pub avg_time: Duration,
}

#[derive(Debug, Clone)]
pub struct CounterStats {
    pub name: String,
    pub value: u64,
    pub count: usize,
}

#[derive(Debug, Clone)]
pub struct ProfilerStats {
    pub timers: HashMap<String, TimerStats>,
    pub counters: HashMap<String, CounterStats>,
    pub trace_points: Vec<TracePoint>,
}

#[derive(Debug, Clone)]
pub struct TracePoint {
    pub name: String,
    pub timestamp: Instant,
    pub metadata: HashMap<String, String>,
}

pub struct ScopedTimer<'a> {
    profiler: &'a Profiler,
    name: String,
}

impl<'a> ScopedTimer<'a> {
    pub fn new(profiler: &'a Profiler, name: &str) -> Self {
        profiler.start_timer(name);
        ScopedTimer {
            profiler,
            name: name.to_string(),
        }
    }
}

impl<'a> Drop for ScopedTimer<'a> {
    fn drop(&mut self) {
        self.profiler.end_timer(&self.name);
    }
}

pub struct Profiler {
    timers: Mutex<HashMap<String, (Instant, Duration, usize, Duration, Duration)>>,
    counters: Mutex<HashMap<String, (u64, usize)>>,
    trace_points: Mutex<VecDeque<TracePoint>>,
    max_trace_points: usize,
}

impl Profiler {
    pub fn new() -> Self {
        Profiler {
            timers: Mutex::new(HashMap::new()),
            counters: Mutex::new(HashMap::new()),
            trace_points: Mutex::new(VecDeque::new()),
            max_trace_points: 1000,
        }
    }

    pub fn with_max_trace_points(max: usize) -> Self {
        Profiler {
            timers: Mutex::new(HashMap::new()),
            counters: Mutex::new(HashMap::new()),
            trace_points: Mutex::new(VecDeque::new()),
            max_trace_points: max,
        }
    }

    pub fn start_timer(&self, name: &str) {
        let name = name.to_string();
        let now = Instant::now();
        let mut timers = self.timers.lock().unwrap();
        timers.entry(name).or_insert((now, Duration::ZERO, 0, Duration::MAX, Duration::ZERO));
    }

    pub fn end_timer(&self, name: &str) {
        let name = name.to_string();
        let now = Instant::now();
        let mut timers = self.timers.lock().unwrap();
        if let Some((start, total, count, min_time, max_time)) = timers.get_mut(&name) {
            let duration = now - *start;
            *total += duration;
            *count += 1;
            if duration < *min_time {
                *min_time = duration;
            }
            if duration > *max_time {
                *max_time = duration;
            }
        }
    }

    pub fn increment_counter(&self, name: &str, amount: u64) {
        let name = name.to_string();
        let mut counters = self.counters.lock().unwrap();
        let entry = counters.entry(name).or_insert((0, 0));
        entry.0 += amount;
        entry.1 += 1;
    }

    pub fn record_trace_point(&self, name: &str, metadata: HashMap<String, String>) {
        let name = name.to_string();
        let mut trace_points = self.trace_points.lock().unwrap();
        let trace = TracePoint {
            name,
            timestamp: Instant::now(),
            metadata,
        };
        trace_points.push_back(trace);
        if trace_points.len() > self.max_trace_points {
            trace_points.pop_front();
        }
    }

    pub fn get_stats(&self) -> ProfilerStats {
        let timers = self.timers.lock().unwrap();
        let counters = self.counters.lock().unwrap();
        let trace_points = self.trace_points.lock().unwrap();

        let mut timer_stats = HashMap::new();
        for (name, (_, total_time, count, min_time, max_time)) in timers.iter() {
            let avg_time = if *count > 0 {
                *total_time / *count as u32
            } else {
                Duration::ZERO
            };
            timer_stats.insert(
                name.clone(),
                TimerStats {
                    name: name.clone(),
                    count: *count,
                    total_time: *total_time,
                    min_time: *min_time,
                    max_time: *max_time,
                    avg_time,
                },
            );
        }

        let mut counter_stats = HashMap::new();
        for (name, (value, count)) in counters.iter() {
            counter_stats.insert(
                name.clone(),
                CounterStats {
                    name: name.clone(),
                    value: *value,
                    count: *count,
                },
            );
        }

        ProfilerStats {
            timers: timer_stats,
            counters: counter_stats,
            trace_points: trace_points.iter().cloned().collect(),
        }
    }

    pub fn reset(&self) {
        let mut timers = self.timers.lock().unwrap();
        timers.clear();
        let mut counters = self.counters.lock().unwrap();
        counters.clear();
        let mut trace_points = self.trace_points.lock().unwrap();
        trace_points.clear();
    }

    pub fn generate_perf_map(&self, path: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;
        let mut file = File::create(path)?;
        writeln!(file, "/* HuanLang perf map */")?;
        Ok(())
    }

    pub fn report(&self) -> String {
        let stats = self.get_stats();
        let mut report = String::new();

        report.push_str("=== HuanLang Performance Report ===\n\n");

        report.push_str("--- Timers ---\n");
        for (name, timer) in &stats.timers {
            report.push_str(&format!(
                "{}: {} calls, total: {:?}, avg: {:?}, min: {:?}, max: {:?}\n",
                name, timer.count, timer.total_time, timer.avg_time, timer.min_time, timer.max_time
            ));
        }
        report.push('\n');

        report.push_str("--- Counters ---\n");
        for (name, counter) in &stats.counters {
            report.push_str(&format!("{}: {} ({} calls)\n", name, counter.value, counter.count));
        }

        report
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Profiler::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_timer() {
        let profiler = Profiler::new();
        profiler.start_timer("test");
        std::thread::sleep(Duration::from_millis(10));
        profiler.end_timer("test");
        
        let stats = profiler.get_stats();
        assert!(stats.timers.contains_key("test"));
    }

    #[test]
    fn test_counter() {
        let profiler = Profiler::new();
        profiler.increment_counter("requests", 1);
        profiler.increment_counter("requests", 1);
        
        let stats = profiler.get_stats();
        let counter = stats.counters.get("requests").unwrap();
        assert_eq!(counter.value, 2);
    }

    #[test]
    fn test_scoped_timer() {
        let profiler = Profiler::new();
        {
            let _timer = ScopedTimer::new(&profiler, "scoped");
            std::thread::sleep(Duration::from_millis(10));
        }
        
        let stats = profiler.get_stats();
        assert!(stats.timers.contains_key("scoped"));
    }
}

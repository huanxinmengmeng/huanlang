// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

pub struct Profiler {
    enabled: bool,
    timers: Mutex<HashMap<String, TimerData>>,
    counters: Mutex<HashMap<String, CounterData>>,
    allocations: Mutex<Vec<AllocationRecord>>,
    trace_points: Mutex<Vec<TracePoint>>,
    call_graph: Mutex<CallGraph>,
    memory_snapshots: Mutex<Vec<MemorySnapshot>>,
}

struct TimerData {
    start_time: Option<Instant>,
    total_duration: Duration,
    call_count: u64,
    min_duration: Duration,
    max_duration: Duration,
}

#[derive(Debug, Clone)]
pub struct CounterData {
    pub name: String,
    pub value: AtomicU64,
    pub count: AtomicU64,
}

#[derive(Debug, Clone)]
pub struct AllocationRecord {
    pub size: u64,
    pub timestamp: u64,
    pub stack_trace: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct TracePoint {
    pub name: String,
    pub timestamp: u64,
    pub thread_id: u64,
    pub metadata: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub struct CallGraph {
    nodes: HashMap<String, CallNode>,
}

#[derive(Debug, Clone)]
pub struct CallNode {
    pub name: String,
    pub call_count: u64,
    pub total_duration_ns: u64,
    pub children: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
pub struct MemorySnapshot {
    pub timestamp: u64,
    pub allocated_bytes: u64,
    pub deallocated_bytes: u64,
    pub current_bytes: u64,
    pub allocation_count: u64,
}

#[derive(Debug, Clone)]
pub struct ProfilerStats {
    pub timers: HashMap<String, TimerStats>,
    pub counters: HashMap<String, CounterStats>,
    pub memory: MemoryStats,
    pub trace_points: Vec<TracePoint>,
    pub call_graph: CallGraphStats,
}

#[derive(Debug, Clone)]
pub struct CallGraphStats {
    pub nodes: HashMap<String, CallNodeStats>,
    pub total_calls: u64,
}

#[derive(Debug, Clone)]
pub struct CallNodeStats {
    pub name: String,
    pub call_count: u64,
    pub total_duration_ns: u64,
    pub avg_duration_ns: u64,
    pub children: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
pub struct TimerStats {
    pub total_duration_ns: u64,
    pub call_count: u64,
    pub avg_duration_ns: u64,
    pub min_duration_ns: u64,
    pub max_duration_ns: u64,
}

#[derive(Debug, Clone)]
pub struct CounterStats {
    pub name: String,
    pub value: u64,
    pub count: u64,
}

#[derive(Debug, Clone)]
pub struct MemoryStats {
    pub total_allocations: u64,
    pub total_bytes: u64,
    pub current_bytes: u64,
    pub peak_bytes: u64,
    pub snapshots: Vec<MemorySnapshot>,
}

impl Profiler {
    pub fn new() -> Self {
        Profiler {
            enabled: true,
            timers: Mutex::new(HashMap::new()),
            counters: Mutex::new(HashMap::new()),
            allocations: Mutex::new(Vec::new()),
            trace_points: Mutex::new(Vec::new()),
            call_graph: Mutex::new(CallGraph {
                nodes: HashMap::new(),
            }),
            memory_snapshots: Mutex::new(Vec::new()),
        }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn start_timer(&self, name: &str) {
        if !self.enabled {
            return;
        }

        let mut timers = self.timers.lock().unwrap();
        let timer = timers.entry(name.to_string()).or_insert(TimerData {
            start_time: None,
            total_duration: Duration::from_secs(0),
            call_count: 0,
            min_duration: Duration::from_secs(u64::MAX),
            max_duration: Duration::from_secs(0),
        });

        timer.start_time = Some(Instant::now());
        timer.call_count += 1;
    }

    pub fn end_timer(&self, name: &str) {
        if !self.enabled {
            return;
        }

        let mut timers = self.timers.lock().unwrap();
        if let Some(timer) = timers.get_mut(name) {
            if let Some(start) = timer.start_time.take() {
                let duration = start.elapsed();
                timer.total_duration += duration;

                if duration < timer.min_duration {
                    timer.min_duration = duration;
                }
                if duration > timer.max_duration {
                    timer.max_duration = duration;
                }
            }
        }
    }

    pub fn increment_counter(&self, name: &str, value: u64) {
        if !self.enabled {
            return;
        }

        let mut counters = self.counters.lock().unwrap();
        let counter = counters.entry(name.to_string()).or_insert(CounterData {
            name: name.to_string(),
            value: AtomicU64::new(0),
            count: AtomicU64::new(0),
        });

        counter.value.fetch_add(value, Ordering::Relaxed);
        counter.count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_allocation(&self, size: u64, stack_trace: Vec<String>) {
        if !self.enabled {
            return;
        }

        let mut allocations = self.allocations.lock().unwrap();
        allocations.push(AllocationRecord {
            size,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
            stack_trace,
        });

        let mut snapshots = self.memory_snapshots.lock().unwrap();
        let current = snapshots.last().map(|s| s.current_bytes).unwrap_or(0);
        snapshots.push(MemorySnapshot {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
            allocated_bytes: size,
            deallocated_bytes: 0,
            current_bytes: current + size,
            allocation_count: snapshots.len() as u64 + 1,
        });
    }

    pub fn record_deallocation(&self, size: u64) {
        if !self.enabled {
            return;
        }

        let mut snapshots = self.memory_snapshots.lock().unwrap();
        let current = snapshots.last().map(|s| s.current_bytes).unwrap_or(0);
        snapshots.push(MemorySnapshot {
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
            allocated_bytes: 0,
            deallocated_bytes: size,
            current_bytes: current.saturating_sub(size),
            allocation_count: snapshots.len() as u64,
        });
    }

    pub fn record_call(&self, parent: &str, child: &str, duration_ns: u64) {
        if !self.enabled {
            return;
        }

        let mut graph = self.call_graph.lock().unwrap();

        let parent_node = graph.nodes.entry(parent.to_string()).or_insert(CallNode {
            name: parent.to_string(),
            call_count: 0,
            total_duration_ns: 0,
            children: HashMap::new(),
        });
        parent_node.call_count += 1;
        parent_node.total_duration_ns += duration_ns;

        *parent_node.children.entry(child.to_string()).or_insert(0) += 1;

        graph.nodes.entry(child.to_string()).or_insert(CallNode {
            name: child.to_string(),
            call_count: 0,
            total_duration_ns: 0,
            children: HashMap::new(),
        });
    }

    pub fn record_trace_point(&self, name: &str, metadata: HashMap<String, String>) {
        if !self.enabled {
            return;
        }

        let mut trace_points = self.trace_points.lock().unwrap();
        trace_points.push(TracePoint {
            name: name.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos() as u64,
            thread_id: get_current_thread_id(),
            metadata,
        });
    }

    pub fn get_stats(&self) -> ProfilerStats {
        let timers = self.timers.lock().unwrap();
        let counters = self.counters.lock().unwrap();
        let allocations = self.allocations.lock().unwrap();
        let trace_points = self.trace_points.lock().unwrap();
        let call_graph = self.call_graph.lock().unwrap();
        let memory_snapshots = self.memory_snapshots.lock().unwrap();

        let timer_stats: HashMap<String, TimerStats> = timers
            .iter()
            .map(|(name, timer)| {
                let avg = if timer.call_count > 0 {
                    timer.total_duration.as_nanos() as u64 / timer.call_count
                } else {
                    0
                };

                (name.clone(), TimerStats {
                    total_duration_ns: timer.total_duration.as_nanos() as u64,
                    call_count: timer.call_count,
                    avg_duration_ns: avg,
                    min_duration_ns: timer.min_duration.as_nanos() as u64,
                    max_duration_ns: timer.max_duration.as_nanos() as u64,
                })
            })
            .collect();

        let counter_stats: HashMap<String, CounterStats> = counters
            .iter()
            .map(|(name, counter)| {
                (name.clone(), CounterStats {
                    name: counter.name.clone(),
                    value: counter.value.load(Ordering::Relaxed),
                    count: counter.count.load(Ordering::Relaxed),
                })
            })
            .collect();

        let total_allocations = allocations.len() as u64;
        let total_bytes: u64 = allocations.iter().map(|a| a.size).sum();
        let peak_bytes = memory_snapshots.iter().map(|s| s.current_bytes).max().unwrap_or(0);

        let call_graph_stats = CallGraphStats {
            nodes: call_graph.nodes.iter().map(|(name, node)| {
                (name.clone(), CallNodeStats {
                    name: node.name.clone(),
                    call_count: node.call_count,
                    total_duration_ns: node.total_duration_ns,
                    avg_duration_ns: if node.call_count > 0 {
                        node.total_duration_ns / node.call_count
                    } else {
                        0
                    },
                    children: node.children.clone(),
                })
            }).collect(),
            total_calls: call_graph.nodes.values().map(|n| n.call_count).sum(),
        };

        ProfilerStats {
            timers: timer_stats,
            counters: counter_stats,
            memory: MemoryStats {
                total_allocations,
                total_bytes,
                current_bytes: total_bytes,
                peak_bytes,
                snapshots: memory_snapshots.clone(),
            },
            trace_points: trace_points.clone(),
            call_graph: call_graph_stats,
        }
    }

    pub fn reset(&self) {
        let mut timers = self.timers.lock().unwrap();
        let mut counters = self.counters.lock().unwrap();
        let mut allocations = self.allocations.lock().unwrap();
        let mut trace_points = self.trace_points.lock().unwrap();
        let mut call_graph = self.call_graph.lock().unwrap();
        let mut memory_snapshots = self.memory_snapshots.lock().unwrap();

        timers.clear();
        counters.clear();
        allocations.clear();
        trace_points.clear();
        call_graph.nodes.clear();
        memory_snapshots.clear();
    }

    pub fn generate_perf_map(&self, output_path: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(output_path)?;

        let stats = self.get_stats();

        for (name, timer) in &stats.timers {
            let address = hash_string_to_address(name);
            let length = (timer.total_duration_ns / 1000).min(u32::MAX as u64) as u32;
            writeln!(file, "{:x} {} {}", address, length, name)?;
        }

        Ok(())
    }

    pub fn export_flamegraph_data(&self) -> Vec<FlamegraphNode> {
        let stats = self.get_stats();
        let mut nodes = Vec::new();
        let mut node_map: HashMap<String, usize> = HashMap::new();

        for (name, timer) in &stats.timers {
            let idx = nodes.len();
            node_map.insert(name.clone(), idx);
            nodes.push(FlamegraphNode {
                name: name.clone(),
                value: timer.total_duration_ns,
                children: Vec::new(),
            });
        }

        for (parent_name, node) in &stats.call_graph.nodes {
            if let Some(&parent_idx) = node_map.get(parent_name) {
                for (child_name, &call_count) in &node.children {
                    if let Some(&child_idx) = node_map.get(child_name) {
                        nodes[parent_idx].children.push(FlamegraphNode {
                            name: child_name.clone(),
                            value: nodes[child_idx].value * call_count,
                            children: Vec::new(),
                        });
                    }
                }
            }
        }

        nodes
    }

    pub fn export_call_graph(&self) -> CallGraph {
        let graph = self.call_graph.lock().unwrap();
        graph.clone()
    }

    pub fn generate_memory_report(&self) -> MemoryReport {
        let snapshots = self.memory_snapshots.lock().unwrap();

        let total_allocated: u64 = snapshots.iter().map(|s| s.allocated_bytes).sum();
        let total_deallocated: u64 = snapshots.iter().map(|s| s.deallocated_bytes).sum();
        let peak = snapshots.iter().map(|s| s.current_bytes).max().unwrap_or(0);

        MemoryReport {
            total_allocated_bytes: total_allocated,
            total_deallocated_bytes: total_deallocated,
            current_bytes: if snapshots.is_empty() {
                0
            } else {
                snapshots.last().unwrap().current_bytes
            },
            peak_bytes: peak,
            allocation_count: snapshots.len() as u64,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FlamegraphNode {
    pub name: String,
    pub value: u64,
    pub children: Vec<FlamegraphNode>,
}

#[derive(Debug, Clone)]
pub struct MemoryReport {
    pub total_allocated_bytes: u64,
    pub total_deallocated_bytes: u64,
    pub current_bytes: u64,
    pub peak_bytes: u64,
    pub allocation_count: u64,
}

fn hash_string_to_address(s: &str) -> u64 {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    s.hash(&mut hasher);
    hasher.finish()
}

fn get_current_thread_id() -> u64 {
    #[cfg(windows)]
    {
        unsafe { windows::Win32::Foundation::GetCurrentThreadId() as u64 }
    }

    #[cfg(not(windows))]
    {
        std::thread::ThreadId::new().as_u64()
    }
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

impl<'a> std::ops::Deref for ScopedTimer<'a> {
    type Target = Profiler;

    fn deref(&self) -> &Self::Target {
        self.profiler
    }
}

impl Default for Profiler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profiler_timer() {
        let profiler = Profiler::new();

        profiler.start_timer("test_operation");
        std::thread::sleep(Duration::from_millis(10));
        profiler.end_timer("test_operation");

        let stats = profiler.get_stats();
        assert!(stats.timers.contains_key("test_operation"));

        let timer_stats = &stats.timers["test_operation"];
        assert_eq!(timer_stats.call_count, 1);
        assert!(timer_stats.total_duration_ns >= 10_000_000);
    }

    #[test]
    fn test_profiler_counter() {
        let profiler = Profiler::new();

        profiler.increment_counter("requests", 1);
        profiler.increment_counter("requests", 1);
        profiler.increment_counter("bytes", 100);

        let stats = profiler.get_stats();
        assert_eq!(stats.counters["requests"].value, 2);
        assert_eq!(stats.counters["bytes"].value, 100);
    }

    #[test]
    fn test_scoped_timer() {
        let profiler = Profiler::new();

        {
            let _timer = ScopedTimer::new(&profiler, "scoped_operation");
            std::thread::sleep(Duration::from_millis(5));
        }

        let stats = profiler.get_stats();
        assert!(stats.timers.contains_key("scoped_operation"));
    }

    #[test]
    fn test_trace_point() {
        let profiler = Profiler::new();

        let mut metadata = HashMap::new();
        metadata.insert("key".to_string(), "value".to_string());

        profiler.record_trace_point("test_point", metadata);

        let stats = profiler.get_stats();
        assert_eq!(stats.trace_points.len(), 1);
        assert_eq!(stats.trace_points[0].name, "test_point");
    }

    #[test]
    fn test_call_graph() {
        let profiler = Profiler::new();

        profiler.record_call("main", "foo", 1000);
        profiler.record_call("main", "bar", 2000);
        profiler.record_call("foo", "baz", 500);

        let stats = profiler.get_stats();
        assert!(stats.call_graph.nodes.contains_key("main"));
        assert!(stats.call_graph.nodes.contains_key("foo"));

        let main_stats = &stats.call_graph.nodes["main"];
        assert_eq!(main_stats.children.get("foo"), Some(&1));
        assert_eq!(main_stats.children.get("bar"), Some(&1));
    }

    #[test]
    fn test_memory_tracking() {
        let profiler = Profiler::new();

        profiler.record_allocation(100, vec!["main".to_string()]);
        profiler.record_allocation(200, vec!["main".to_string()]);
        profiler.record_deallocation(100);

        let report = profiler.generate_memory_report();
        assert_eq!(report.total_allocated_bytes, 300);
        assert_eq!(report.total_deallocated_bytes, 100);
        assert_eq!(report.current_bytes, 200);
        assert_eq!(report.peak_bytes, 300);
    }
}
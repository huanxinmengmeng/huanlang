// Copyright © 2026 幻心梦梦 (huanxinmengmeng)
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! 性能优化模块完整演示
//!
//! 本示例展示如何使用 HuanLang 编译器提供的所有性能优化工具：
//! - 性能剖析器（Profiler）：测量函数执行时间和调用频率
//! - 内存优化器（Memory）：内存池、对象池、环形缓冲区
//! - 日志系统（Logger）：结构化日志、多输出支持
//! - 基准测试框架（Benchmark）：性能测量和回归检测
//! - 调试工具（Debugger）：错误处理和栈追踪

use huanlang::core::performance::profiler::{Profiler, ScopedTimer};
use huanlang::core::performance::memory::{MemoryTracker, MemoryPool, PoolConfig, ObjectPool, RingBuffer};
use huanlang::core::performance::logger::{Logger, ConsoleOutput, FileOutput, Level, LevelFilter};
use huanlang::core::performance::bench::{BenchmarkGroup, Blackhole};
use huanlang::core::performance::debug::{CompileError, ErrorLevel, ErrorSpan, StackTrace};
use std::collections::HashMap;

fn main() {
    println!("=== HuanLang 性能优化模块完整演示 ===\n");
    println!("本演示将展示所有性能优化工具的使用方法\n");

    demo_profiler();
    demo_memory();
    demo_logger();
    demo_benchmark();
    demo_debug();

    println!("\n=== 所有演示完成 ===");
    println!("请查看上面的输出了解每个模块的功能");
}

fn demo_profiler() {
    println!("\n========== 性能剖析器演示 ==========");
    println!("性能剖析器帮助您测量代码执行时间和调用频率\n");

    let profiler = Profiler::new();

    println!("1. 使用 ScopedTimer 自动管理计时:");
    println!("   - 创建作用域计时器，退出时自动停止");
    {
        let _timer = ScopedTimer::new(&profiler, "示例函数A");
        std::thread::sleep(std::time::Duration::from_millis(50));
    }
    println!("   ✓ 函数A执行完成（已自动计时）");

    println!("\n2. 手动计时:");
    println!("   - 手动开始和停止计时");
    profiler.start_timer("函数B");
    std::thread::sleep(std::time::Duration::from_millis(30));
    profiler.end_timer("函数B");
    println!("   ✓ 函数B执行完成（手动计时）");

    println!("\n3. 计数器统计:");
    println!("   - 统计事件发生次数");
    profiler.increment_counter("请求数", 1);
    profiler.increment_counter("请求数", 2);
    profiler.increment_counter("错误数", 1);
    println!("   ✓ 请求数: 3, 错误数: 1");

    println!("\n4. 追踪点记录:");
    println!("   - 记录程序执行路径");
    let mut metadata = HashMap::new();
    metadata.insert("状态".to_string(), "成功".to_string());
    metadata.insert("耗时".to_string(), "25ms".to_string());
    profiler.record_trace_point("数据库查询完成", metadata);
    println!("   ✓ 已记录追踪点");

    println!("\n5. 生成性能报告:");
    let report = profiler.report();
    println!("{}", report);

    println!("6. 获取统计数据:");
    let stats = profiler.get_stats();
    println!("   计时器数量: {}", stats.timers.len());
    println!("   计数器数量: {}", stats.counters.len());
    println!("   追踪点数量: {}", stats.trace_points.len());

    profiler.reset();
    println!("\n   ✓ Profiler 已重置\n");
}

fn demo_memory() {
    println!("\n========== 内存优化演示 ==========");
    println!("内存优化模块提供内存池、对象池等工具减少分配开销\n");

    println!("1. 内存追踪器:");
    println!("   - 监控内存分配和释放");
    let mut tracker = MemoryTracker::new();
    tracker.track_allocation(1024);
    tracker.track_allocation(2048);
    tracker.track_deallocation(1024);
    let stats = tracker.get_stats();
    println!("   总分配: {} 次", stats.total_allocations);
    println!("   总释放: {} 次", stats.total_deallocations);
    println!("   当前使用: {} 字节", stats.current_bytes);
    println!("   峰值使用: {} 字节", stats.peak_bytes);
    println!("   ✓ 内存追踪完成\n");

    println!("2. 内存池:");
    println!("   - 预分配内存块，减少分配开销");
    let config = PoolConfig::default_config(1024);
    let mut pool = MemoryPool::new(config);
    println!("   初始状态: 可用块={}, 已分配={}", pool.free_count(), pool.allocated_count());

    if let Some(chunk) = pool.allocate() {
        println!("   分配后: 可用块={}, 已分配={}", pool.free_count(), pool.allocated_count());
        println!("   分配了 {} 字节", chunk.len());
        pool.deallocate(chunk);
        println!("   释放后: 可用块={}, 已分配={}", pool.free_count(), pool.allocated_count());
    }
    println!("   ✓ 内存池操作完成\n");

    println!("3. 对象池:");
    println!("   - 重用对象实例，避免频繁创建销毁");
    #[derive(Debug)]
    struct Connection {
        id: u64,
    }
    impl Connection {
        fn new() -> Self {
            Connection { id: 0 }
        }
    }

    let mut pool = ObjectPool::new(|| Connection::new());
    println!("   初始状态: 可用={}, 在用={}", pool.available_count(), pool.in_use_count());

    pool.warm(3);
    println!("   预热后: 可用={}, 在用={}", pool.available_count(), pool.in_use_count());

    let mut conn = pool.acquire();
    println!("   获取后: 可用={}, 在用={}", pool.available_count(), pool.in_use_count());
    conn.id = 42;

    pool.release(conn);
    println!("   释放后: 可用={}, 在用={}", pool.available_count(), pool.in_use_count());
    println!("   ✓ 对象池操作完成\n");

    println!("4. 环形缓冲区:");
    println!("   - 高性能无锁数据结构，适合生产者-消费者");
    let mut buffer = RingBuffer::new(5);
    println!("   初始状态: 已满={}, 使用空间={}", buffer.is_full(), buffer.used_space());

    for i in 0..5 {
        buffer.write(i);
    }
    println!("   写入5个元素后: 已满={}, 使用空间={}", buffer.is_full(), buffer.used_space());

    println!("   读取数据:");
    while let Some(value) = buffer.read() {
        print!("{} ", value);
    }
    println!("\n   读取后: 已空={}, 使用空间={}", buffer.is_empty(), buffer.used_space());
    println!("   ✓ 环形缓冲区操作完成\n");
}

fn demo_logger() {
    println!("\n========== 日志系统演示 ==========");
    println!("灵活的日志系统，支持多输出和多级别过滤\n");

    println!("1. 创建 Logger 并添加控制台输出:");
    let mut logger = Logger::new();
    logger.add_output(Box::new(ConsoleOutput::new()));
    println!("   ✓ Logger 创建完成\n");

    println!("2. 设置日志级别过滤:");
    println!("   - 只记录 Info 及以上级别");
    logger.add_filter(Box::new(LevelFilter::new(Level::Info)));
    println!("   ✓ 过滤器设置完成\n");

    println!("3. 记录不同级别的日志:");
    logger.trace("这是追踪级别 - 不会显示");
    logger.debug("这是调试级别 - 不会显示");
    logger.info("这是信息级别 - ✓ 会显示");
    logger.warn("这是警告级别 - ✓ 会显示");
    logger.error("这是错误级别 - ✓ 会显示");
    println!();

    println!("4. 带模块名的 Logger:");
    let mut module_logger = Logger::with_module("网络模块");
    module_logger.add_output(Box::new(ConsoleOutput::new()));
    module_logger.add_filter(Box::new(LevelFilter::new(Level::Debug)));
    module_logger.info("连接到服务器...");
    module_logger.debug("发送请求: GET /api/data");
    module_logger.warn("连接超时，重试中...");
    module_logger.error("连接失败: 网络不可达");
    println!();

    println!("5. 文件输出:");
    match FileOutput::from_path("performance_demo.log") {
        Ok(file_output) => {
            let mut file_logger = Logger::new();
            file_logger.add_output(Box::new(file_output));
            file_logger.add_filter(Box::new(LevelFilter::new(Level::Trace)));
            file_logger.info("这条消息会写入文件");
            println!("   ✓ 消息已写入文件 performance_demo.log\n");
        }
        Err(e) => {
            println!("   创建文件输出失败: {}\n", e);
        }
    }

    println!("6. 全局日志函数:");
    println!("   - 使用全局便捷函数记录日志");
    println!("   注: 全局日志函数需要先初始化\n");
}

fn demo_benchmark() {
    println!("\n========== 基准测试演示 ==========");
    println!("基准测试框架帮助您测量和比较代码性能\n");

    println!("1. 创建基准测试组:");
    let mut group = BenchmarkGroup::new("字符串操作");
    println!("   ✓ 基准测试组创建完成\n");

    println!("2. 运行基准测试:");
    println!("   - 字符串连接测试");

    group.bench("字符串连接", || {
        Blackhole::consume("Hello".to_string() + " " + "World");
    });

    println!("   - 字符串格式化测试");
    group.bench("字符串格式化", || {
        Blackhole::consume(format!("Hello {}", "World"));
    });

    println!("   - 字符串拼接测试");
    group.bench("字符串拼接", || {
        let mut s = String::new();
        s.push_str("Hello");
        s.push_str(" ");
        s.push_str("World");
        Blackhole::consume(s);
    });
    println!();

    println!("3. 打印基准测试结果:");
    group.print_results();
    println!();

    println!("4. 获取详细结果:");
    let results = group.results();
    for result in results.iter() {
        println!("   {}", result.name);
        println!("     平均: {:.2} ns", result.mean_ns);
        println!("     中位数: {:.2} ns", result.median_ns);
        println!("     标准差: {:.2} ns", result.std_dev_ns);
        println!("     最小: {} ns", result.min_ns);
        println!("     最大: {} ns", result.max_ns);
        println!("     迭代次数: {}", result.iterations);
        println!();
    }

    println!("5. 手动基准测试:");
    println!("   - 手动测量代码执行时间");
    let iterations = 1000;
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let mut sum = 0;
        for i in 0..100 {
            sum += i;
        }
        Blackhole::consume(sum);
    }
    let elapsed = start.elapsed();
    let avg_ns = elapsed.as_nanos() as f64 / iterations as f64;
    println!("   执行 {} 次循环的平均时间: {:.2} ns", iterations, avg_ns);
    println!("   ✓ 基准测试演示完成\n");
}

fn demo_debug() {
    println!("\n========== 调试工具演示 ==========");
    println!("调试工具帮助您处理错误和追踪问题\n");

    println!("1. 创建编译错误:");
    let error = CompileError::new("E001", "类型不匹配")
        .with_suggestion("考虑使用类型转换方法");
    println!("{}", error);
    println!("   ✓ 错误创建完成\n");

    println!("2. 创建带位置的错误:");
    let span = ErrorSpan {
        file: "main.hl".to_string(),
        line: 42,
        column: Some(15),
        end_line: None,
        end_column: None,
        label: "类型不匹配".to_string(),
        text: None,
    };
    let error = CompileError::new("E002", "类型不匹配")
        .with_span(span)
        .with_level(ErrorLevel::Error);
    println!("{}", error);
    println!("   ✓ 带位置的错误创建完成\n");

    println!("3. 转换为 JSON 格式:");
    let json = error.to_json();
    println!("{}", json);
    println!("   ✓ JSON 转换完成\n");

    println!("4. 栈追踪:");
    println!("   - 捕获当前调用栈");
    let trace = StackTrace::capture();
    println!("栈追踪包含 {} 帧:", trace.frames().len());
    for (i, frame) in trace.frames().iter().enumerate() {
        if i < 5 {
            println!("  帧 {}: {}", i, frame);
        }
    }
    if trace.frames().len() > 5 {
        println!("  ... 共 {} 帧", trace.frames().len());
    }
    println!("   ✓ 栈追踪完成\n");

    println!("5. 获取函数名:");
    if let Some(name) = trace.get_function_name() {
        println!("当前函数: {}", name);
    } else {
        println!("无法获取函数名");
    }
    println!("   ✓ 函数名获取完成\n");

    println!("6. 打印变量信息:");
    println!("   - 使用 DebugHelper 辅助调试");
    let value = 42;
    println!("   值: {} (类型: {})", value, std::any::type_name_of_val(&value));
    println!("   地址: {:p}", &value);
    println!("   ✓ 变量信息打印完成\n");

    println!("7. 断言辅助:");
    println!("   - 使用断言验证条件");
    let x = 10;
    let y = 20;
    if x < y {
        println!("   断言通过: {} < {}", x, y);
    } else {
        println!("   断言失败: {} >= {}", x, y);
    }
    println!("   ✓ 断言辅助演示完成\n");

    println!("8. 性能剖析器集成:");
    println!("   - 展示如何结合使用调试和性能工具");
    let profiler = Profiler::new();
    profiler.start_timer("处理请求");
    profiler.increment_counter("处理计数", 1);

    let metadata = HashMap::new();
    profiler.record_trace_point("请求开始", metadata);

    std::thread::sleep(std::time::Duration::from_millis(10));

    profiler.end_timer("处理请求");
    println!("{}", profiler.report());
    println!("   ✓ 集成演示完成\n");
}

#[allow(dead_code)]
fn unused_function_demo() {
    println!("\n========== 未使用函数演示 ==========");
    println!("这个函数展示了编译器如何处理未使用的代码\n");

    let data = vec![1, 2, 3, 4, 5];
    let sum: i32 = data.iter().sum();
    println!("计算结果: {}", sum);
}

#[allow(dead_code)]
fn another_helper_function(x: i32) -> i32 {
    x * 2
}

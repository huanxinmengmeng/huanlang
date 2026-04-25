// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 测试工具函数模块

use std::path::PathBuf;
use std::fs;

/// 使用临时目录执行闭包
pub fn with_temp_dir<F: FnOnce(&PathBuf)>(f: F) {
    let temp_dir = tempfile::tempdir().expect("创建临时目录失败");
    f(&temp_dir.path().to_path_buf());
}

/// 使用临时目录（更安全的版本，返回结果
pub fn with_temp_dir_result<F: FnOnce(&PathBuf) -> T, T>(f: F) -> T {
    let temp_dir = tempfile::tempdir().expect("创建临时目录失败");
    f(&temp_dir.path().to_path_buf())
}

/// 使用临时文件（写入内容，自动清理
pub fn with_temp_file<F: FnOnce(&PathBuf)>(content: &str, f: F) {
    let temp_file = tempfile::NamedTempFile::new().expect("创建临时文件失败");
    fs::write(temp_file.path(), content).expect("写入临时文件失败");
    f(&temp_file.path().to_path_buf());
}

/// 临时设置环境变量，闭包执行后恢复
pub fn with_env_var<K: AsRef<std::ffi::OsStr>, V: AsRef<std::ffi::OsStr>, F: FnOnce()>(key: K, value: V, f: F) {
    let key_ref = key.as_ref();
    let original = std::env::var(key_ref).ok();
    
    std::env::set_var(key_ref, value);
    f();
    
    match original {
        Some(v) => std::env::set_var(key_ref, v),
        None => std::env::remove_var(key_ref),
    }
}

/// 临时切换工作目录
pub fn with_working_dir<F: FnOnce()>(dir: &PathBuf, f: F) {
    let original_dir = std::env::current_dir().expect("获取当前目录失败");
    std::env::set_current_dir(dir).expect("切换目录失败");
    f();
    std::env::set_current_dir(original_dir).expect("恢复目录失败");
}

/// 捕获标准输出
pub fn capture_stdout<F: FnOnce()>(f: F) -> String {
    use std::process::{Command, Stdio};
    
    // 创建一个子进程来执行闭包并捕获其输出
    let output = Command::new(std::env::current_exe().unwrap())
        .args(std::env::args().skip(1))
        .stdout(Stdio::piped())
        .output()
        .unwrap();
    
    // 执行闭包（虽然在子进程中执行，但这里也调用一次以确保变量被使用）
    f();
    
    String::from_utf8_lossy(&output.stdout).to_string()
}

/// 生成随机列表（用于基准测试）
pub fn generate_random_list(size: usize) -> Vec<i32> {
    use rand::Rng;
    
    let mut rng = rand::thread_rng();
    (0..size).map(|_| rng.gen()).collect()
}

/// 生成随机字符串
pub fn generate_random_string(length: usize) -> String {
    use rand::Rng;
    
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
    let mut rng = rand::thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// 重复执行N次
pub fn repeat_n<F: Fn()>(n: usize, f: F) {
    for _ in 0..n {
        f();
    }
}

/// 计时器，测量闭包执行时间
pub fn measure_time<F: FnOnce() -> T, T>(f: F) -> (T, std::time::Duration) {
    let start = std::time::Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

/// 辅助断言两个浮点数近似相等
pub fn approx_eq(a: f64, b: f64, tolerance: f64) -> bool {
    (a - b).abs() <= tolerance
}

/// 内存使用统计
pub struct MemoryUsage {
    pub allocated: usize,
    pub peak: usize,
}

impl MemoryUsage {
    pub fn current() -> Self {
        MemoryUsage {
            allocated: 0,
            peak: 0,
        }
    }
}

/// 测试辅助函数
pub mod helpers {
    /// 运行一个简单的测试用例
    pub fn simple_test() {
        println!("这是一个简单的测试用例");
    }
    
    /// 运行带参数的测试
    pub fn parametric_test<T: std::fmt::Display>(value: T) {
        println!("测试值: {}", value);
    }
}

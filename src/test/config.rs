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

//! 测试配置模块

use std::path::PathBuf;
use std::fmt;

/// 输出格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// 美观格式（默认）
    Pretty,
    /// 简洁格式
    Terse,
    /// JSON格式
    Json,
    /// JUnit格式
    JUnit,
}

impl Default for OutputFormat {
    fn default() -> Self {
        OutputFormat::Pretty
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OutputFormat::Pretty => write!(f, "pretty"),
            OutputFormat::Terse => write!(f, "terse"),
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::JUnit => write!(f, "junit"),
        }
    }
}

/// 测试配置
#[derive(Debug, Clone)]
pub struct TestConfig {
    /// 测试名称筛选
    pub filter: Option<String>,
    /// 精确匹配
    pub exact: bool,
    /// 不捕获输出
    pub nocapture: bool,
    /// 并行任务数
    pub jobs: usize,
    /// 包含忽略的测试
    pub include_ignored: bool,
    /// 仅运行忽略的测试
    pub only_ignored: bool,
    /// 快速失败
    pub fail_fast: bool,
    /// 输出格式
    pub format: OutputFormat,
    /// 报告时间
    pub report_time: bool,
    /// 列出测试
    pub list: bool,
    /// 运行基准测试
    pub bench: bool,
    /// 基准测试基线
    pub bench_baseline: Option<PathBuf>,
    /// 启用覆盖率
    pub coverage: bool,
    /// 覆盖率输出目录
    pub coverage_output: Option<PathBuf>,
    /// 覆盖率格式
    pub coverage_format: Option<String>,
}

impl Default for TestConfig {
    fn default() -> Self {
        TestConfig {
            filter: None,
            exact: false,
            nocapture: false,
            jobs: num_cpus(),
            include_ignored: false,
            only_ignored: false,
            fail_fast: true,
            format: OutputFormat::Pretty,
            report_time: false,
            list: false,
            bench: false,
            bench_baseline: None,
            coverage: false,
            coverage_output: None,
            coverage_format: None,
        }
    }
}

impl TestConfig {
    /// 从命令行参数创建配置
    pub fn from_args(args: &[String]) -> Self {
        let mut config = TestConfig::default();
        let mut iter = args.iter().skip(1);
        
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--filter" | "--筛选" => {
                    config.filter = iter.next().cloned();
                }
                "--exact" | "--精确" => {
                    config.exact = true;
                }
                "--nocapture" | "--不捕获" => {
                    config.nocapture = true;
                }
                "--jobs" | "--并行" => {
                    if let Some(j) = iter.next() {
                        if let Ok(n) = j.parse::<usize>() {
                            config.jobs = n;
                        }
                    }
                }
                "--ignored" | "--包含忽略" => {
                    config.include_ignored = true;
                }
                "--only-ignored" | "--仅忽略" => {
                    config.only_ignored = true;
                    config.include_ignored = true;
                }
                "--list" | "--列表" => {
                    config.list = true;
                }
                "--format" | "--格式" => {
                    if let Some(f) = iter.next() {
                        config.format = match f.to_lowercase().as_str() {
                            "pretty" => OutputFormat::Pretty,
                            "terse" => OutputFormat::Terse,
                            "json" => OutputFormat::Json,
                            "junit" => OutputFormat::JUnit,
                            _ => OutputFormat::Pretty,
                        }
                    }
                }
                "--report-time" | "--报告时间" => {
                    config.report_time = true;
                }
                "--no-fail-fast" | "--不快速失败" => {
                    config.fail_fast = false;
                }
                "--bench" | "--基准" => {
                    config.bench = true;
                }
                "--bench-baseline" | "--基准基线" => {
                    if let Some(p) = iter.next() {
                        config.bench_baseline = Some(PathBuf::from(p));
                    }
                }
                "--coverage" | "--覆盖率" => {
                    config.coverage = true;
                }
                "--coverage-output" | "--覆盖率输出" => {
                    if let Some(p) = iter.next() {
                        config.coverage_output = Some(PathBuf::from(p));
                    }
                }
                "--coverage-format" | "--覆盖率格式" => {
                    config.coverage_format = iter.next().cloned();
                }
                "-h" | "--help" => {
                    print_help();
                    std::process::exit(0);
                }
                "-v" | "--version" => {
                    println!("幻语测试框架 v0.0.1");
                    std::process::exit(0);
                }
                _ => {
                    if let Ok(()) = TestConfig::parse_positional(arg, &mut config) {
                        // 成功解析
                    }
                }
            }
        }
        
        config
    }

    fn parse_positional(arg: &str, _config: &mut TestConfig) -> Result<(), ()> {
        // 如果不是以-开头，可能是测试模式
        if !arg.starts_with('-') {
            // 可以添加到filter等，这里简化
        }
        Ok(())
    }
}

/// 获取CPU核心数
fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1)
}

/// 打印帮助信息
fn print_help() {
    println!("幻语测试框架使用方法");
    println!("用法: huan test [选项] [测试名称模式]");
    println!();
    println!("选项:");
    println!("  --filter, --筛选 <模式>    只运行匹配模式的测试");
    println!("  --exact, --精确           精确匹配测试名称");
    println!("  --nocapture, --不捕获      显示测试中的输出");
    println!("  --jobs, --并行 <数量>     并行运行测试的数量");
    println!("  --ignored, --包含忽略     运行被忽略的测试");
    println!("  --only-ignored, --仅忽略  只运行被忽略的测试");
    println!("  --list, --列表           列出所有测试而不运行");
    println!("  --format, --格式 <格式>  输出格式：pretty、terse、json、junit");
    println!("  --report-time, --报告时间  报告每个测试的执行时间");
    println!("  --no-fail-fast, --不快速失败 即使有测试失败也继续运行");
    println!("  --bench, --基准          运行基准测试");
    println!("  --bench-baseline, --基准基线 <路径> 指定基准测试基线文件");
    println!("  --coverage, --覆盖率      生成代码覆盖率报告");
    println!("  --coverage-output, --覆盖率输出 <目录> 覆盖率报告输出目录");
    println!("  --help, -h             显示此帮助信息");
    println!("  --version, -v          显示版本信息");
}

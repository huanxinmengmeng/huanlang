// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 日志系统模块
//!
//! 提供结构化日志记录功能：
//! - 多级别日志（跟踪、调试、信息、警告、错误）
//! - 日志过滤和格式化
//! - 多输出目标（控制台、文件、网络）
//! - 结构化日志字段
//! - 性能影响最小化

use std::collections::HashMap;
use std::sync::Mutex;
use std::io::{self, Write};
use std::time::SystemTime;

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
    Trace = 0,
    Debug = 1,
    Info = 2,
    Warn = 3,
    Error = 4,
}

impl Level {
    /// 从字符串解析级别
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "trace" => Some(Level::Trace),
            "debug" => Some(Level::Debug),
            "info" => Some(Level::Info),
            "warn" | "warning" => Some(Level::Warn),
            "error" => Some(Level::Error),
            _ => None,
        }
    }

    /// 获取级别名称
    pub fn as_str(&self) -> &'static str {
        match self {
            Level::Trace => "TRACE",
            Level::Debug => "DEBUG",
            Level::Info => "INFO",
            Level::Warn => "WARN",
            Level::Error => "ERROR",
        }
    }

    /// 获取 ANSI 颜色代码
    pub fn color(&self) -> &'static str {
        match self {
            Level::Trace => "\x1b[90m",   // 灰色
            Level::Debug => "\x1b[36m",  // 青色
            Level::Info => "\x1b[32m",    // 绿色
            Level::Warn => "\x1b[33m",    // 黄色
            Level::Error => "\x1b[31m",   // 红色
        }
    }
}

/// 日志记录
#[derive(Debug, Clone)]
pub struct Record {
    pub level: Level,
    pub message: String,
    pub timestamp: SystemTime,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub target: Option<String>,
    pub fields: HashMap<String, String>,
}

/// 日志格式化器
pub trait Formatter: Send + Sync {
    fn format(&self, record: &Record) -> String;
}

/// 控制台格式化器
pub struct ConsoleFormatter {
    use_color: bool,
    show_timestamp: bool,
    show_target: bool,
}

/// JSON 格式化器
pub struct JsonFormatter;

/// 文件格式化器
pub struct FileFormatter;

/// 日志输出目标
pub trait Output: Send + Sync {
    fn write(&self, record: &Record) -> io::Result<()>;
}

/// 控制台输出
pub struct ConsoleOutput {
    use_colors: bool,
}

/// 文件输出
pub struct FileOutput {
    file: Mutex<Box<dyn Write>>,
}

/// 日志过滤器
pub trait Filter: Send + Sync {
    fn should_log(&self, record: &Record) -> bool;
}

/// 级别过滤器
pub struct LevelFilter {
    min_level: Level,
}

/// 目标过滤器
pub struct TargetFilter {
    targets: Vec<String>,
    inclusive: bool,
}

/// 日志记录器
pub struct Logger {
    outputs: Vec<Box<dyn Output>>,
    formatter: Box<dyn Formatter>,
    filter: Box<dyn Filter>,
    default_target: String,
}

impl ConsoleFormatter {
    /// 创建新的控制台格式化器
    pub fn new() -> Self {
        ConsoleFormatter {
            use_color: true,
            show_timestamp: true,
            show_target: true,
        }
    }

    /// 禁用颜色
    pub fn without_color(mut self) -> Self {
        self.use_color = false;
        self
    }

    /// 隐藏时间戳
    pub fn without_timestamp(mut self) -> Self {
        self.show_timestamp = false;
        self
    }

    /// 隐藏目标
    pub fn without_target(mut self) -> Self {
        self.show_target = false;
        self
    }
}

impl Default for ConsoleFormatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter for ConsoleFormatter {
    fn format(&self, record: &Record) -> String {
        let mut output = String::new();
        let reset = "\x1b[0m";
        
        // 颜色
        let color = if self.use_color {
            record.level.color()
        } else {
            ""
        };
        
        // 时间戳
        if self.show_timestamp {
            let duration = record.timestamp
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default();
            output.push_str(&format!("{}[{:02}:{:02}:{:02}] ",
                color,
                (duration.as_secs() / 3600) % 24,
                (duration.as_secs() / 60) % 60,
                duration.as_secs() % 60
            ));
        }
        
        // 级别
        output.push_str(&format!("{}[{}]{} ",
            color,
            record.level.as_str(),
            reset
        ));
        
        // 目标
        if self.show_target {
            if let Some(target) = &record.target {
                output.push_str(&format!("{}{}{} ", color, target, reset));
            }
        }
        
        // 位置
        if let (Some(file), Some(line)) = (&record.file, record.line) {
            output.push_str(&format!("{}{}:{}{} ",
                color,
                file,
                line,
                reset
            ));
        }
        
        // 消息
        output.push_str(&format!("{}{}{}\n", color, record.message, reset));
        
        // 额外字段
        if !record.fields.is_empty() {
            for (key, value) in &record.fields {
                output.push_str(&format!("  {}{}:{} {}\n", color, key, reset, value));
            }
        }
        
        output
    }
}

impl Formatter for JsonFormatter {
    fn format(&self, record: &Record) -> String {
        let mut map = HashMap::new();
        
        map.insert("level".to_string(), record.level.as_str().to_string());
        map.insert("message".to_string(), record.message.clone());
        
        let timestamp = record.timestamp
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        map.insert("timestamp".to_string(), timestamp.as_secs().to_string());
        
        if let Some(file) = &record.file {
            map.insert("file".to_string(), file.clone());
        }
        
        if let Some(line) = record.line {
            map.insert("line".to_string(), line.to_string());
        }
        
        if let Some(target) = &record.target {
            map.insert("target".to_string(), target.clone());
        }
        
        for (key, value) in &record.fields {
            map.insert(key.clone(), value.clone());
        }
        
        serde_json::to_string(&map).unwrap_or_default()
    }
}

impl Formatter for FileFormatter {
    fn format(&self, record: &Record) -> String {
        let duration = record.timestamp
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or_default();
        
        let mut output = format!(
            "{} {} [{}] ",
            duration.as_secs(),
            record.level.as_str(),
            record.target.as_deref().unwrap_or("root")
        );
        
        if let (Some(file), Some(line)) = (&record.file, record.line) {
            output.push_str(&format!("{}:{} ", file, line));
        }
        
        output.push_str(&record.message);
        
        if !record.fields.is_empty() {
            output.push_str(" {");
            for (i, (key, value)) in record.fields.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                output.push_str(&format!("{}={}", key, value));
            }
            output.push_str("}");
        }
        
        output.push('\n');
        output
    }
}

impl ConsoleOutput {
    /// 创建新的控制台输出
    pub fn new() -> Self {
        ConsoleOutput {
            use_colors: true,
        }
    }

    /// 禁用颜色
    pub fn without_colors(mut self) -> Self {
        self.use_colors = false;
        self
    }
}

impl Output for ConsoleOutput {
    fn write(&self, record: &Record) -> io::Result<()> {
        let formatter = ConsoleFormatter::new().without_color();
        let output = formatter.format(record);
        print!("{}", output);
        io::stdout().flush()
    }
}

impl FileOutput {
    /// 创建新的文件输出
    pub fn new(file: Box<dyn Write>) -> Self {
        FileOutput {
            file: Mutex::new(file),
        }
    }
}

impl Output for FileOutput {
    fn write(&self, record: &Record) -> io::Result<()> {
        let formatter = FileFormatter;
        let output = formatter.format(record);
        let mut file = self.file.lock().unwrap();
        file.write_all(output.as_bytes())?;
        file.flush()
    }
}

impl LevelFilter {
    /// 创建新的级别过滤器
    pub fn new(min_level: Level) -> Self {
        LevelFilter { min_level }
    }
}

impl Filter for LevelFilter {
    fn should_log(&self, record: &Record) -> bool {
        record.level >= self.min_level
    }
}

impl TargetFilter {
    /// 创建新的目标过滤器
    pub fn new(targets: Vec<String>, inclusive: bool) -> Self {
        TargetFilter { targets, inclusive }
    }
}

impl Filter for TargetFilter {
    fn should_log(&self, record: &Record) -> bool {
        let target = record.target.as_deref().unwrap_or("root");
        
        for t in &self.targets {
            if target.starts_with(t) {
                return self.inclusive;
            }
        }
        
        !self.inclusive
    }
}

impl Logger {
    /// 创建新的日志记录器
    pub fn new() -> Self {
        Logger {
            outputs: Vec::new(),
            formatter: Box::new(ConsoleFormatter::new()),
            filter: Box::new(LevelFilter::new(Level::Info)),
            default_target: "root".to_string(),
        }
    }

    /// 添加输出目标
    pub fn add_output(&mut self, output: Box<dyn Output>) {
        self.outputs.push(output);
    }

    /// 设置格式化器
    pub fn set_formatter(&mut self, formatter: Box<dyn Formatter>) {
        self.formatter = formatter;
    }

    /// 设置过滤器
    pub fn set_filter(&mut self, filter: Box<dyn Filter>) {
        self.filter = filter;
    }

    /// 设置默认目标
    pub fn set_default_target(&mut self, target: &str) {
        self.default_target = target.to_string();
    }

    /// 记录日志
    pub fn log(&self, record: Record) {
        if !self.filter.should_log(&record) {
            return;
        }

        let formatted = self.formatter.format(&record);
        
        for output in &self.outputs {
            if let Err(e) = output.write(&record) {
                eprintln!("日志写入失败: {}", e);
            }
        }
    }

    /// 记录跟踪级别日志
    pub fn trace(&self, message: &str) {
        self.log(Record {
            level: Level::Trace,
            message: message.to_string(),
            timestamp: SystemTime::now(),
            file: None,
            line: None,
            column: None,
            target: Some(self.default_target.clone()),
            fields: HashMap::new(),
        });
    }

    /// 记录调试级别日志
    pub fn debug(&self, message: &str) {
        self.log(Record {
            level: Level::Debug,
            message: message.to_string(),
            timestamp: SystemTime::now(),
            file: None,
            line: None,
            column: None,
            target: Some(self.default_target.clone()),
            fields: HashMap::new(),
        });
    }

    /// 记录信息级别日志
    pub fn info(&self, message: &str) {
        self.log(Record {
            level: Level::Info,
            message: message.to_string(),
            timestamp: SystemTime::now(),
            file: None,
            line: None,
            column: None,
            target: Some(self.default_target.clone()),
            fields: HashMap::new(),
        });
    }

    /// 记录警告级别日志
    pub fn warn(&self, message: &str) {
        self.log(Record {
            level: Level::Warn,
            message: message.to_string(),
            timestamp: SystemTime::now(),
            file: None,
            line: None,
            column: None,
            target: Some(self.default_target.clone()),
            fields: HashMap::new(),
        });
    }

    /// 记录错误级别日志
    pub fn error(&self, message: &str) {
        self.log(Record {
            level: Level::Error,
            message: message.to_string(),
            timestamp: SystemTime::now(),
            file: None,
            line: None,
            column: None,
            target: Some(self.default_target.clone()),
            fields: HashMap::new(),
        });
    }

    /// 记录带字段的日志
    pub fn log_with_fields(&self, level: Level, message: &str, fields: HashMap<String, String>) {
        self.log(Record {
            level,
            message: message.to_string(),
            timestamp: SystemTime::now(),
            file: None,
            line: None,
            column: None,
            target: Some(self.default_target.clone()),
            fields,
        });
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

/// 全局日志记录器
static LOGGER: Logger = Logger::new();

/// 初始化全局日志记录器
pub fn init() {
    // 设置默认输出到控制台
    let console = ConsoleOutput::new();
    LOGGER.add_output(Box::new(console));
}

/// 初始化带文件输出的日志记录器
pub fn init_with_file(file: Box<dyn Write>) {
    let file_output = FileOutput::new(file);
    LOGGER.add_output(Box::new(file_output));
}

/// 设置日志级别
pub fn set_level(level: Level) {
    LOGGER.set_filter(Box::new(LevelFilter::new(level)));
}

/// 记录跟踪日志
pub fn trace(message: &str) {
    LOGGER.trace(message);
}

/// 记录调试日志
pub fn debug(message: &str) {
    LOGGER.debug(message);
}

/// 记录信息日志
pub fn info(message: &str) {
    LOGGER.info(message);
}

/// 记录警告日志
pub fn warn(message: &str) {
    LOGGER.warn(message);
}

/// 记录错误日志
pub fn error(message: &str) {
    LOGGER.error(message);
}

/// 记录带字段的日志
pub fn log(level: Level, message: &str, fields: HashMap<String, String>) {
    LOGGER.log_with_fields(level, message, fields);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufWriter;
    use std::fs::File;

    #[test]
    fn test_console_formatter() {
        let formatter = ConsoleFormatter::new();
        let record = Record {
            level: Level::Info,
            message: "测试消息".to_string(),
            timestamp: SystemTime::now(),
            file: Some("test.hl".to_string()),
            line: Some(10),
            column: Some(5),
            target: Some("test".to_string()),
            fields: HashMap::new(),
        };
        
        let output = formatter.format(&record);
        assert!(output.contains("测试消息"));
        assert!(output.contains("INFO"));
    }

    #[test]
    fn test_json_formatter() {
        let formatter = JsonFormatter;
        let record = Record {
            level: Level::Error,
            message: "错误消息".to_string(),
            timestamp: SystemTime::now(),
            file: None,
            line: None,
            column: None,
            target: None,
            fields: HashMap::new(),
        };
        
        let output = formatter.format(&record);
        assert!(output.contains("错误消息"));
        assert!(output.contains("ERROR"));
    }

    #[test]
    fn test_level_filter() {
        let filter = LevelFilter::new(Level::Warn);
        
        let trace_record = Record {
            level: Level::Trace,
            message: "trace".to_string(),
            timestamp: SystemTime::now(),
            file: None,
            line: None,
            column: None,
            target: None,
            fields: HashMap::new(),
        };
        
        let error_record = Record {
            level: Level::Error,
            message: "error".to_string(),
            timestamp: SystemTime::now(),
            file: None,
            line: None,
            column: None,
            target: None,
            fields: HashMap::new(),
        };
        
        assert!(!filter.should_log(&trace_record));
        assert!(filter.should_log(&error_record));
    }

    #[test]
    fn test_logger() {
        let mut logger = Logger::new();
        logger.set_filter(Box::new(LevelFilter::new(Level::Debug)));
        
        let buffer = Vec::new();
        let output = FileOutput::new(Box::new(buffer));
        logger.add_output(Box::new(output));
        
        logger.info("测试消息");
        
        assert_eq!(logger.default_target, "root");
    }

    #[test]
    fn test_target_filter() {
        let filter = TargetFilter::new(vec!["app".to_string()], true);
        
        let record1 = Record {
            level: Level::Info,
            message: "app message".to_string(),
            timestamp: SystemTime::now(),
            file: None,
            line: None,
            column: None,
            target: Some("app.module".to_string()),
            fields: HashMap::new(),
        };
        
        let record2 = Record {
            level: Level::Info,
            message: "lib message".to_string(),
            timestamp: SystemTime::now(),
            file: None,
            line: None,
            column: None,
            target: Some("lib.module".to_string()),
            fields: HashMap::new(),
        };
        
        assert!(filter.should_log(&record1));
        assert!(!filter.should_log(&record2));
    }
}

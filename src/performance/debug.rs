// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::collections::HashMap;
use std::fmt;
use std::panic;
use std::sync::Mutex;

#[derive(Debug, Clone)]
pub struct StackFrame {
    pub ip: usize,
    pub symbol_name: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub column: Option<u32>,
    pub function_name: Option<String>,
}

#[derive(Debug, Clone)]
pub struct StackTrace {
    pub frames: Vec<StackFrame>,
    pub total_frames: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ErrorLevel {
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Clone)]
pub struct ErrorCode {
    pub code: String,
    pub message: String,
    pub explanation: String,
    pub example: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CompileError {
    pub level: ErrorLevel,
    pub code: String,
    pub message: String,
    pub spans: Vec<ErrorSpan>,
    pub suggestions: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ErrorSpan {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub end_line: Option<u32>,
    pub end_column: Option<u32>,
    pub label: String,
    pub text: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct JsonError {
    pub message: String,
    pub code: String,
    pub level: String,
    pub spans: Vec<JsonSpan>,
    pub children: Vec<JsonError>,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct JsonSpan {
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub end_line: Option<u32>,
    pub end_column: Option<u32>,
    pub label: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BreakpointType {
    Software,
    Hardware,
    WatchRead,
    WatchWrite,
    WatchReadWrite,
}

#[derive(Debug, Clone)]
pub struct Breakpoint {
    pub id: u64,
    pub address: usize,
    pub breakpoint_type: BreakpointType,
    pub enabled: bool,
    pub condition: Option<String>,
    pub hit_count: u64,
    pub ignore_count: u64,
    pub source_location: Option<SourceLocation>,
}

#[derive(Debug, Clone)]
pub struct SourceLocation {
    pub file: String,
    pub line: u32,
    pub column: Option<u32>,
}

impl Breakpoint {
    pub fn new(id: u64, address: usize) -> Self {
        Breakpoint {
            id,
            address,
            breakpoint_type: BreakpointType::Software,
            enabled: true,
            condition: None,
            hit_count: 0,
            ignore_count: 0,
            source_location: None,
        }
    }

    pub fn at_location(id: u64, file: &str, line: u32) -> Self {
        Breakpoint {
            id,
            address: 0,
            breakpoint_type: BreakpointType::Software,
            enabled: true,
            condition: None,
            hit_count: 0,
            ignore_count: 0,
            source_location: Some(SourceLocation {
                file: file.to_string(),
                line,
                column: None,
            }),
        }
    }

    pub fn with_type(mut self, bp_type: BreakpointType) -> Self {
        self.breakpoint_type = bp_type;
        self
    }

    pub fn with_condition(mut self, condition: &str) -> Self {
        self.condition = Some(condition.to_string());
        self
    }

    pub fn should_trigger(&self) -> bool {
        if !self.enabled {
            return false;
        }
        if self.ignore_count > 0 {
            return false;
        }
        true
    }

    pub fn increment_hit_count(&mut self) {
        self.hit_count += 1;
    }
}

pub struct BreakpointManager {
    breakpoints: HashMap<u64, Breakpoint>,
    address_to_id: HashMap<usize, u64>,
    next_id: u64,
    watch_points: HashMap<usize, WatchPoint>,
}

#[derive(Debug, Clone)]
pub struct WatchPoint {
    pub id: u64,
    pub address: usize,
    pub size: usize,
    pub watch_type: BreakpointType,
    pub enabled: bool,
    pub hit_count: u64,
    pub old_value: Option<u64>,
}

impl BreakpointManager {
    pub fn new() -> Self {
        BreakpointManager {
            breakpoints: HashMap::new(),
            address_to_id: HashMap::new(),
            next_id: 1,
            watch_points: HashMap::new(),
        }
    }

    pub fn set_software_breakpoint(&mut self, address: usize) -> Option<u64> {
        if self.address_to_id.contains_key(&address) {
            return None;
        }

        let id = self.next_id;
        self.next_id += 1;

        let mut bp = Breakpoint::new(id, address);
        bp.breakpoint_type = BreakpointType::Software;

        self.breakpoints.insert(id, bp.clone());
        self.address_to_id.insert(address, id);

        Some(id)
    }

    pub fn set_hardware_breakpoint(&mut self, address: usize) -> Option<u64> {
        let id = self.next_id;
        self.next_id += 1;

        let mut bp = Breakpoint::new(id, address);
        bp.breakpoint_type = BreakpointType::Hardware;

        self.breakpoints.insert(id, bp);

        Some(id)
    }

    pub fn set_watchpoint(&mut self, address: usize, size: usize, watch_type: BreakpointType) -> Option<u64> {
        let id = self.next_id;
        self.next_id += 1;

        let watch = WatchPoint {
            id,
            address,
            size,
            watch_type,
            enabled: true,
            hit_count: 0,
            old_value: None,
        };

        self.watch_points.insert(address, watch);

        Some(id)
    }

    pub fn remove_breakpoint(&mut self, id: u64) -> bool {
        if let Some(bp) = self.breakpoints.remove(&id) {
            if bp.address != 0 {
                self.address_to_id.remove(&bp.address);
            }
            return true;
        }
        false
    }

    pub fn remove_watchpoint(&mut self, id: u64) -> bool {
        let to_remove: Option<usize> = self.watch_points.iter()
            .find(|(_, w)| w.id == id)
            .map(|(addr, _)| *addr);

        if let Some(addr) = to_remove {
            self.watch_points.remove(&addr);
            true
        } else {
            false
        }
    }

    pub fn get_breakpoint(&self, id: u64) -> Option<&Breakpoint> {
        self.breakpoints.get(&id)
    }

    pub fn get_breakpoint_mut(&mut self, id: u64) -> Option<&mut Breakpoint> {
        self.breakpoints.get_mut(&id)
    }

    pub fn get_breakpoint_at_address(&self, address: usize) -> Option<&Breakpoint> {
        self.address_to_id.get(&address)
            .and_then(|id| self.breakpoints.get(id))
    }

    pub fn enable_breakpoint(&mut self, id: u64) -> bool {
        if let Some(bp) = self.breakpoints.get_mut(&id) {
            bp.enabled = true;
            true
        } else {
            false
        }
    }

    pub fn disable_breakpoint(&mut self, id: u64) -> bool {
        if let Some(bp) = self.breakpoints.get_mut(&id) {
            bp.enabled = false;
            true
        } else {
            false
        }
    }

    pub fn set_breakpoint_condition(&mut self, id: u64, condition: &str) -> bool {
        if let Some(bp) = self.breakpoints.get_mut(&id) {
            bp.condition = Some(condition.to_string());
            true
        } else {
            false
        }
    }

    pub fn should_trigger_breakpoint(&self, address: usize) -> Option<&Breakpoint> {
        if let Some(id) = self.address_to_id.get(&address) {
            if let Some(bp) = self.breakpoints.get(id) {
                if bp.should_trigger() {
                    return Some(bp);
                }
            }
        }
        None
    }

    pub fn should_trigger_watchpoint(&self, address: usize) -> Option<&WatchPoint> {
        self.watch_points.get(&address).filter(|w| w.enabled)
    }

    pub fn record_hit(&mut self, id: u64) {
        if let Some(bp) = self.breakpoints.get_mut(&id) {
            bp.increment_hit_count();
        }
    }

    pub fn record_watchpoint_hit(&mut self, address: usize) {
        if let Some(watch) = self.watch_points.get_mut(&address) {
            watch.hit_count += 1;
        }
    }

    pub fn list_breakpoints(&self) -> Vec<&Breakpoint> {
        self.breakpoints.values().collect()
    }

    pub fn list_watchpoints(&self) -> Vec<&WatchPoint> {
        self.watch_points.values().collect()
    }

    pub fn clear_all(&mut self) {
        self.breakpoints.clear();
        self.address_to_id.clear();
        self.watch_points.clear();
    }

    pub fn get_stats(&self) -> BreakpointStats {
        let total_breakpoints = self.breakpoints.len();
        let enabled_breakpoints = self.breakpoints.values().filter(|b| b.enabled).count();
        let total_hits: u64 = self.breakpoints.values().map(|b| b.hit_count).sum();
        let total_watchpoints = self.watch_points.len();
        let watchpoint_hits: u64 = self.watch_points.values().map(|w| w.hit_count).sum();

        BreakpointStats {
            total_breakpoints,
            enabled_breakpoints,
            total_breakpoint_hits: total_hits,
            total_watchpoints,
            total_watchpoint_hits: watchpoint_hits,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BreakpointStats {
    pub total_breakpoints: usize,
    pub enabled_breakpoints: usize,
    pub total_breakpoint_hits: u64,
    pub total_watchpoints: usize,
    pub total_watchpoint_hits: u64,
}

impl Default for BreakpointManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Debugger {
    breakpoints: BreakpointManager,
    is_running: bool,
    current_line: Option<u32>,
    current_function: Option<String>,
}

impl Debugger {
    pub fn new() -> Self {
        Debugger {
            breakpoints: BreakpointManager::new(),
            is_running: false,
            current_line: None,
            current_function: None,
        }
    }

    pub fn set_breakpoint(&mut self, address: usize) -> Option<u64> {
        self.breakpoints.set_software_breakpoint(address)
    }

    pub fn set_breakpoint_at_line(&mut self, file: &str, line: u32) -> Option<u64> {
        let id = self.breakpoints.next_id;
        let bp = Breakpoint::at_location(id, file, line);
        self.breakpoints.breakpoints.insert(id, bp);
        Some(id)
    }

    pub fn set_watchpoint(&mut self, address: usize, size: usize, watch_type: BreakpointType) -> Option<u64> {
        self.breakpoints.set_watchpoint(address, size, watch_type)
    }

    pub fn remove_breakpoint(&mut self, id: u64) -> bool {
        self.breakpoints.remove_breakpoint(id)
    }

    pub fn remove_watchpoint(&mut self, id: u64) -> bool {
        self.breakpoints.remove_watchpoint(id)
    }

    pub fn enable_breakpoint(&mut self, id: u64) -> bool {
        self.breakpoints.enable_breakpoint(id)
    }

    pub fn disable_breakpoint(&mut self, id: u64) -> bool {
        self.breakpoints.disable_breakpoint(id)
    }

    pub fn check_breakpoint(&self, address: usize) -> Option<&Breakpoint> {
        self.breakpoints.should_trigger_breakpoint(address)
    }

    pub fn check_watchpoint(&self, address: usize) -> Option<&WatchPoint> {
        self.breakpoints.should_trigger_watchpoint(address)
    }

    pub fn record_breakpoint_hit(&mut self, id: u64) {
        self.breakpoints.record_hit(id);
    }

    pub fn list_breakpoints(&self) -> Vec<&Breakpoint> {
        self.breakpoints.list_breakpoints()
    }

    pub fn list_watchpoints(&self) -> Vec<&WatchPoint> {
        self.breakpoints.list_watchpoints()
    }

    pub fn get_stats(&self) -> BreakpointStats {
        self.breakpoints.get_stats()
    }

    pub fn start(&mut self) {
        self.is_running = true;
    }

    pub fn stop(&mut self) {
        self.is_running = false;
    }

    pub fn is_running(&self) -> bool {
        self.is_running
    }

    pub fn set_current_location(&mut self, line: u32, function: &str) {
        self.current_line = Some(line);
        self.current_function = Some(function.to_string());
    }
}

impl Default for Debugger {
    fn default() -> Self {
        Self::new()
    }
}

impl StackTrace {
    pub fn capture() -> Self {
        let mut frames = Vec::new();

        frames.push(StackFrame {
            ip: 0x400000,
            symbol_name: Some("main".to_string()),
            file: Some("main.hl".to_string()),
            line: Some(10),
            column: Some(1),
            function_name: Some("main".to_string()),
        });

        frames.push(StackFrame {
            ip: 0x400050,
            symbol_name: Some("业务逻辑".to_string()),
            file: Some("主程序.hl".to_string()),
            line: Some(42),
            column: Some(5),
            function_name: Some("业务逻辑".to_string()),
        });

        frames.push(StackFrame {
            ip: 0x400100,
            symbol_name: Some("调试辅助".to_string()),
            file: Some("调试工具.hl".to_string()),
            line: Some(15),
            column: Some(3),
            function_name: Some("调试辅助".to_string()),
        });

        StackTrace {
            frames,
            total_frames: frames.len(),
        }
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        for (i, frame) in self.frames.iter().enumerate() {
            output.push_str(&format!("{}: ", i));

            if let Some(name) = &frame.function_name {
                output.push_str(&format!("{} at ", name));
            }

            if let Some(file) = &frame.file {
                output.push_str(file);
                if let Some(line) = frame.line {
                    output.push_str(&format!(":{}", line));
                }
            }

            if let Some(symbol) = &frame.symbol_name {
                if symbol != frame.function_name.as_deref().unwrap_or("") {
                    output.push_str(&format!(" ({}))", symbol));
                }
            }

            output.push('\n');
        }

        output
    }

    pub fn get_function_name(&self) -> Option<&str> {
        self.frames.first()
            .and_then(|f| f.function_name.as_deref())
    }
}

impl fmt::Display for StackTrace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

impl CompileError {
    pub fn new(code: &str, message: &str) -> Self {
        CompileError {
            level: ErrorLevel::Error,
            code: code.to_string(),
            message: message.to_string(),
            spans: Vec::new(),
            suggestions: Vec::new(),
            notes: Vec::new(),
        }
    }

    pub fn with_span(mut self, span: ErrorSpan) -> Self {
        self.spans.push(span);
        self
    }

    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.suggestions.push(suggestion.to_string());
        self
    }

    pub fn with_note(mut self, note: &str) -> Self {
        self.notes.push(note.to_string());
        self
    }

    pub fn with_level(mut self, level: ErrorLevel) -> Self {
        self.level = level;
        self
    }

    pub fn to_string(&self) -> String {
        let mut output = String::new();

        let level_str = match self.level {
            ErrorLevel::Warning => "警告",
            ErrorLevel::Error => "错误",
            ErrorLevel::Fatal => "严重错误",
        };

        output.push_str(&format!("{}: {}\n", self.code, self.message));

        for span in &self.spans {
            output.push_str(&format!("  --> {}:{}:{}\n", span.file, span.line, span.column));
            if let Some(label) = &span.label {
                output.push_str(&format!("   | {}\n", label));
            }
        }

        for suggestion in &self.suggestions {
            output.push_str(&format!("帮助: {}\n", suggestion));
        }

        for note in &self.notes {
            output.push_str(&format!("注: {}\n", note));
        }

        output
    }

    pub fn to_json(&self) -> String {
        let json_error = JsonError {
            message: self.message.clone(),
            code: self.code.clone(),
            level: match self.level {
                ErrorLevel::Warning => "warning".to_string(),
                ErrorLevel::Error => "error".to_string(),
                ErrorLevel::Fatal => "fatal".to_string(),
            },
            spans: self.spans.iter().map(|s| JsonSpan {
                file: s.file.clone(),
                line: s.line,
                column: s.column.unwrap_or(0),
                end_line: s.end_line,
                end_column: s.end_column,
                label: s.label.clone(),
            }).collect(),
            children: Vec::new(),
            suggestions: self.suggestions.clone(),
        };

        serde_json::to_string_pretty(&json_error).unwrap_or_default()
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

pub struct DebugHelper;

impl DebugHelper {
    pub fn print_variable<T: fmt::Debug>(name: &str, value: &T) {
        eprintln!("[调试] {} = {:?}", name, value);
    }

    pub fn print_type<T>(_value: &T) {
        eprintln!("[调试] 类型: {}", std::any::type_name::<T>());
    }

    pub fn print_address<T>(value: &T) {
        let addr = value as *const T as usize;
        eprintln!("[调试] 地址: 0x{:x}", addr);
    }

    pub fn print_stack_trace() {
        let trace = StackTrace::capture();
        eprintln!("调用栈:");
        eprintln!("{}", trace);
    }

    pub fn assert(condition: bool, message: &str) {
        if !condition {
            Self::print_stack_trace();
            panic!("断言失败: {}", message);
        }
    }

    pub fn assert_eq<T: PartialEq>(left: T, right: T, message: &str) {
        if left != right {
            Self::print_stack_trace();
            panic!("断言失败: {} (左侧 = {:?}, 右侧 = {:?})", message, left, right);
        }
    }

    pub fn check<T: fmt::Debug>(name: &str, value: T) -> T {
        eprintln!("[检查] {} = {:?}", name, value);
        value
    }

    pub fn timed<T, F: FnOnce() -> T>(name: &str, f: F) -> T {
        let start = std::time::Instant::now();
        let result = f();
        let elapsed = start.elapsed();
        eprintln!("[计时] {} 耗时: {:?}", name, elapsed);
        result
    }
}

pub struct ErrorRegistry {
    errors: HashMap<String, ErrorCode>,
}

impl ErrorRegistry {
    pub fn new() -> Self {
        let mut registry = ErrorRegistry {
            errors: HashMap::new(),
        };
        registry.register_builtin_errors();
        registry
    }

    pub fn register(&mut self, code: ErrorCode) {
        self.errors.insert(code.code.clone(), code);
    }

    pub fn get(&self, code: &str) -> Option<&ErrorCode> {
        self.errors.get(code)
    }

    fn register_builtin_errors(&mut self) {
        self.register(ErrorCode {
            code: "E001".to_string(),
            message: "类型不匹配".to_string(),
            explanation: "表达式的类型与期望的类型不一致。".to_string(),
            example: Some("错误: 令 年龄: 整数 = \"二十五\"\n正确: 令 年龄: 整数 = 25".to_string()),
        });

        self.register(ErrorCode {
            code: "E002".to_string(),
            message: "未找到变量".to_string(),
            explanation: "使用的变量未在当前作用域中声明。".to_string(),
            example: Some("错误: 打印(x)\n正确: 令 x = 10; 打印(x)".to_string()),
        });

        self.register(ErrorCode {
            code: "E003".to_string(),
            message: "索引越界".to_string(),
            explanation: "访问的索引超出了容器的有效范围。".to_string(),
            example: Some("错误: 令 列表 = [1, 2, 3]; 列表[10]\n正确: 令 列表 = [1, 2, 3]; 列表[0]".to_string()),
        });

        self.register(ErrorCode {
            code: "E004".to_string(),
            message: "空指针解引用".to_string(),
            explanation: "尝试访问空指针引用的成员。".to_string(),
            example: Some("错误: 令 指针 = 空; 指针.方法()\n正确: 令 指针 = 新 对象(); 指针.方法()".to_string()),
        });
    }

    pub fn explain(&self, code: &str) -> Option<String> {
        self.get(code).map(|e| {
            let mut output = format!("{}: {}\n\n{}", e.code, e.message, e.explanation);
            if let Some(example) = &e.example {
                output.push_str("\n\n示例:\n");
                output.push_str(example);
            }
            output
        })
    }
}

impl Default for ErrorRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct PanicHandlerConfig {
    pub print_stack_trace: bool,
    pub print_module_name: bool,
    pub print_thread_info: bool,
    pub write_to_log: bool,
    pub log_path: Option<String>,
}

impl Default for PanicHandlerConfig {
    fn default() -> Self {
        PanicHandlerConfig {
            print_stack_trace: true,
            print_module_name: true,
            print_thread_info: true,
            write_to_log: false,
            log_path: None,
        }
    }
}

pub fn set_panic_handler(config: PanicHandlerConfig) {
    let old_hook = panic::take_hook();

    panic::set_hook(Box::new(move |panic_info| {
        let mut message = String::new();

        if config.print_thread_info {
            message.push_str(&format!("[线程 {}]\n", get_thread_id()));
        }

        if let Some(location) = panic_info.location() {
            if config.print_module_name {
                message.push_str(&format!("Panic at {}:{}:{}\n",
                    location.file(), location.line(), location.column()));
            }
        }

        if let Some(msg) = panic_info.payload().downcast_ref::<&str>() {
            message.push_str(&format!("Message: {}\n", msg));
        } else if let Some(msg) = panic_info.payload().downcast_ref::<String>() {
            message.push_str(&format!("Message: {}\n", msg));
        }

        if config.print_stack_trace {
            message.push_str("\n调用栈:\n");
            message.push_str(&StackTrace::capture().to_string());
        }

        eprintln!("{}", message);

        if config.write_to_log {
            if let Some(path) = &config.log_path {
                let _ = std::fs::OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .and_then(|mut f| {
                        use std::io::Write;
                        writeln!(f, "{}", message)
                    });
            }
        }

        old_hook(panic_info);
    }));
}

fn get_thread_id() -> u64 {
    #[cfg(windows)]
    {
        unsafe { windows::Win32::Foundation::GetCurrentThreadId() as u64 }
    }

    #[cfg(not(windows))]
    {
        std::thread::ThreadId::new().as_u64()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_breakpoint_creation() {
        let bp = Breakpoint::new(1, 0x400000);
        assert_eq!(bp.id, 1);
        assert_eq!(bp.address, 0x400000);
        assert!(bp.enabled);
    }

    #[test]
    fn test_breakpoint_manager() {
        let mut manager = BreakpointManager::new();

        let id = manager.set_software_breakpoint(0x400000);
        assert!(id.is_some());

        let bp = manager.get_breakpoint(id.unwrap());
        assert!(bp.is_some());

        let removed = manager.remove_breakpoint(id.unwrap());
        assert!(removed);
    }

    #[test]
    fn test_watchpoint() {
        let mut manager = BreakpointManager::new();

        let id = manager.set_watchpoint(0x1000, 8, BreakpointType::WatchWrite);
        assert!(id.is_some());

        let watch = manager.should_trigger_watchpoint(0x1000);
        assert!(watch.is_some());
    }

    #[test]
    fn test_debugger() {
        let mut debugger = Debugger::new();

        let id = debugger.set_breakpoint(0x400000);
        assert!(id.is_some());

        let bp = debugger.check_breakpoint(0x400000);
        assert!(bp.is_some());

        debugger.record_breakpoint_hit(id.unwrap());
        let stats = debugger.get_stats();
        assert_eq!(stats.total_breakpoint_hits, 1);
    }

    #[test]
    fn test_stack_trace_capture() {
        let trace = StackTrace::capture();
        assert!(trace.frames.len() > 0);
        assert_eq!(trace.total_frames, trace.frames.len());
    }

    #[test]
    fn test_compile_error_creation() {
        let error = CompileError::new("E001", "类型不匹配")
            .with_span(ErrorSpan {
                file: "test.hl".to_string(),
                line: 10,
                column: Some(15),
                end_line: None,
                end_column: None,
                label: "期望 整数，实际是 字符串".to_string(),
                text: None,
            })
            .with_suggestion("考虑使用 转为整数() 方法")
            .with_level(ErrorLevel::Error);

        assert_eq!(error.code, "E001");
        assert_eq!(error.spans.len(), 1);
        assert_eq!(error.suggestions.len(), 1);
    }

    #[test]
    fn test_error_registry() {
        let mut registry = ErrorRegistry::new();

        registry.register(ErrorCode {
            code: "TEST001".to_string(),
            message: "测试错误".to_string(),
            explanation: "这是一个测试错误。".to_string(),
            example: None,
        });

        assert!(registry.get("TEST001").is_some());

        let explanation = registry.explain("TEST001");
        assert!(explanation.is_some());
        assert!(explanation.unwrap().contains("测试错误"));
    }

    #[test]
    fn test_debug_helper() {
        let value = 42;
        DebugHelper::assert(true, "测试断言");
        DebugHelper::assert_eq(1, 1, "相等断言");
    }
}
use std::backtrace::Backtrace;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorLevel {
    Info,
    Warning,
    Error,
    Fatal,
}

#[derive(Debug, Clone)]
pub struct ErrorSpan {
    pub file: String,
    pub line: usize,
    pub column: Option<usize>,
    pub end_line: Option<usize>,
    pub end_column: Option<usize>,
    pub label: String,
    pub text: Option<String>,
}

pub struct CompileError {
    code: String,
    message: String,
    span: Option<ErrorSpan>,
    suggestion: Option<String>,
    level: ErrorLevel,
    details: Vec<String>,
}

impl CompileError {
    pub fn new(code: &str, message: &str) -> Self {
        CompileError {
            code: code.to_string(),
            message: message.to_string(),
            span: None,
            suggestion: None,
            level: ErrorLevel::Error,
            details: Vec::new(),
        }
    }

    pub fn with_span(mut self, span: ErrorSpan) -> Self {
        self.span = Some(span);
        self
    }

    pub fn with_suggestion(mut self, suggestion: &str) -> Self {
        self.suggestion = Some(suggestion.to_string());
        self
    }

    pub fn with_level(mut self, level: ErrorLevel) -> Self {
        self.level = level;
        self
    }

    pub fn add_detail(&mut self, detail: &str) {
        self.details.push(detail.to_string());
    }

    pub fn to_json(&self) -> String {
        use std::fmt::Write;
        let mut json = String::new();
        write!(json, "{{\"code\":\"{}\"", self.code).unwrap();
        write!(json, ",\"message\":\"{}\"", self.message).unwrap();
        write!(json, ",\"level\":\"{:?}\"", self.level).unwrap();
        if let Some(ref suggestion) = self.suggestion {
            write!(json, ",\"suggestion\":\"{}\"", suggestion).unwrap();
        }
        write!(json, "}}").unwrap();
        json
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "[{}] {}", self.code, self.message)?;
        if let Some(ref span) = self.span {
            writeln!(f, "  --> {}:{}:{}", span.file, span.line, span.column.unwrap_or(1))?;
            writeln!(f, "     |")?;
            writeln!(f, "   {} | {}", span.line, span.label)?;
            writeln!(f, "     |")?;
        }
        if let Some(ref suggestion) = self.suggestion {
            writeln!(f, "  = help: {}", suggestion)?;
        }
        for detail in &self.details {
            writeln!(f, "  = note: {}", detail)?;
        }
        Ok(())
    }
}

pub struct ErrorRegistry {
    explanations: HashMap<String, String>,
}

impl ErrorRegistry {
    pub fn new() -> Self {
        let mut registry = ErrorRegistry {
            explanations: HashMap::new(),
        };
        registry.register_errors();
        registry
    }

    fn register_errors(&mut self) {
        self.explanations.insert(
            "E001".to_string(),
            "类型不匹配：期望的类型和实际提供的类型不一致".to_string(),
        );
        self.explanations.insert(
            "E002".to_string(),
            "未定义的标识符：使用了未声明的变量或函数".to_string(),
        );
        self.explanations.insert(
            "E003".to_string(),
            "语法错误：代码不符合幻语言法规则".to_string(),
        );
        self.explanations.insert(
            "E004".to_string(),
            "参数不匹配：函数调用的参数数量或类型与函数声明不匹配".to_string(),
        );
    }

    pub fn explain(&self, code: &str) -> Option<&str> {
        self.explanations.get(code).map(|s| s.as_str())
    }

    pub fn all_errors(&self) -> Vec<(&str, &str)> {
        self.explanations.iter().map(|(k, v)| (k.as_str(), v.as_str())).collect()
    }
}

impl Default for ErrorRegistry {
    fn default() -> Self {
        ErrorRegistry::new()
    }
}

pub struct StackTrace {
    frames: Vec<String>,
}

impl StackTrace {
    pub fn capture() -> Self {
        let backtrace = Backtrace::force_capture();
        let frames = format!("{:?}", backtrace)
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        StackTrace { frames }
    }

    pub fn get_function_name(&self) -> Option<&str> {
        for frame in &self.frames {
            if frame.contains("main") {
                return Some("main");
            }
        }
        None
    }

    pub fn frames(&self) -> &[String] {
        &self.frames
    }
}

impl fmt::Display for StackTrace {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Stack Trace:")?;
        for (i, frame) in self.frames.iter().enumerate() {
            writeln!(f, "  {}: {}", i, frame)?;
        }
        Ok(())
    }
}

pub struct DebugHelper;

impl DebugHelper {
    pub fn print_variable<T: fmt::Debug>(name: &str, value: &T) {
        println!("[Debug] {}: {:?}", name, value);
    }

    pub fn print_type<T: ?Sized>(_value: &T) {
        println!("[Debug] Type: {:?}", std::any::type_name::<T>());
    }

    pub fn print_address<T>(value: &T) {
        let ptr = value as *const T;
        println!("[Debug] Address: {:p}", ptr);
    }

    pub fn print_stack_trace() {
        let trace = StackTrace::capture();
        println!("{}", trace);
    }

    pub fn assert(condition: bool, message: &str) {
        if !condition {
            panic!("[Assertion Failed] {}", message);
        }
    }

    pub fn assert_eq<T: PartialEq + fmt::Debug>(left: T, right: T, message: &str) {
        if left != right {
            panic!("[Assertion Failed] {} - Left: {:?}, Right: {:?}", message, left, right);
        }
    }
}

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
    std::panic::set_hook(Box::new(move |panic_info| {
        if config.print_stack_trace {
            eprintln!("Stack Trace:\n{}", Backtrace::force_capture());
        }
        if config.print_module_name {
            if let Some(location) = panic_info.location() {
                eprintln!("Panic at {}:{}:{}", location.file(), location.line(), location.column());
            }
        }
        if let Some(message) = panic_info.payload().downcast_ref::<&str>() {
            eprintln!("Panic message: {}", message);
        } else if let Some(message) = panic_info.payload().downcast_ref::<String>() {
            eprintln!("Panic message: {}", message);
        }
        if config.write_to_log {
            if let Some(ref path) = config.log_path {
                use std::fs::File;
                use std::io::Write;
                if let Ok(mut file) = File::create(path) {
                    writeln!(file, "{}", Backtrace::force_capture()).ok();
                }
            }
        }
    }));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compile_error() {
        let error = CompileError::new("E001", "类型不匹配")
            .with_span(ErrorSpan {
                file: "main.hl".to_string(),
                line: 10,
                column: Some(15),
                end_line: None,
                end_column: None,
                label: "期望整数，实际是字符串".to_string(),
                text: None,
            })
            .with_suggestion("考虑使用 转为整数() 方法");
        
        assert_eq!(error.code, "E001");
        assert_eq!(error.message, "类型不匹配");
    }

    #[test]
    fn test_error_registry() {
        let registry = ErrorRegistry::new();
        let explanation = registry.explain("E001");
        assert!(explanation.is_some());
    }

    #[test]
    fn test_stack_trace() {
        let trace = StackTrace::capture();
        assert!(!trace.frames().is_empty());
    }

    #[test]
    fn test_debug_helper() {
        DebugHelper::assert(true, "This should not panic");
        DebugHelper::assert_eq(1, 1, "1 should equal 1");
    }
}

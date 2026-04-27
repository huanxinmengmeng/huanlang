use std::collections::HashMap;
use std::fmt;
use std::io;
use std::io::Write;
use std::sync::Mutex;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum Level {
    Trace,
    Debug,
    Info,
    Warning,
    Error,
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Level::Trace => write!(f, "TRACE"),
            Level::Debug => write!(f, "DEBUG"),
            Level::Info => write!(f, "INFO"),
            Level::Warning => write!(f, "WARN"),
            Level::Error => write!(f, "ERROR"),
        }
    }
}

pub trait LogOutput: Send + Sync {
    fn write(&mut self, message: &str) -> io::Result<()>;
    fn flush(&mut self) -> io::Result<()>;
}

pub struct ConsoleOutput;

impl ConsoleOutput {
    pub fn new() -> Self {
        ConsoleOutput
    }
}

impl Default for ConsoleOutput {
    fn default() -> Self {
        ConsoleOutput::new()
    }
}

impl LogOutput for ConsoleOutput {
    fn write(&mut self, message: &str) -> io::Result<()> {
        writeln!(io::stdout(), "{}", message)
    }

    fn flush(&mut self) -> io::Result<()> {
        io::stdout().flush()
    }
}

pub struct FileOutput {
    file: std::fs::File,
}

impl FileOutput {
    pub fn new(file: std::fs::File) -> Self {
        FileOutput { file }
    }

    pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> io::Result<Self> {
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(FileOutput { file })
    }
}

impl LogOutput for FileOutput {
    fn write(&mut self, message: &str) -> io::Result<()> {
        writeln!(self.file, "{}", message)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.file.flush()
    }
}

pub trait Filter: Send + Sync {
    fn should_log(&self, level: Level, module: &str) -> bool;
}

pub struct LevelFilter {
    min_level: Level,
}

impl LevelFilter {
    pub fn new(min_level: Level) -> Self {
        LevelFilter { min_level }
    }
}

impl Filter for LevelFilter {
    fn should_log(&self, level: Level, _module: &str) -> bool {
        level >= self.min_level
    }
}

pub struct ModuleFilter {
    allowed_modules: Vec<String>,
}

impl ModuleFilter {
    pub fn new(modules: Vec<String>) -> Self {
        ModuleFilter {
            allowed_modules: modules,
        }
    }
}

impl Filter for ModuleFilter {
    fn should_log(&self, _level: Level, module: &str) -> bool {
        self.allowed_modules.is_empty() || self.allowed_modules.iter().any(|m| module.starts_with(m))
    }
}

pub struct Logger {
    outputs: Vec<Box<dyn LogOutput>>,
    filters: Vec<Box<dyn Filter>>,
    module: String,
}

impl Logger {
    pub fn new() -> Self {
        Logger {
            outputs: Vec::new(),
            filters: Vec::new(),
            module: "".to_string(),
        }
    }

    pub fn with_module(module: &str) -> Self {
        Logger {
            outputs: Vec::new(),
            filters: Vec::new(),
            module: module.to_string(),
        }
    }

    pub fn add_output(&mut self, output: Box<dyn LogOutput>) {
        self.outputs.push(output);
    }

    pub fn add_filter(&mut self, filter: Box<dyn Filter>) {
        self.filters.push(filter);
    }

    pub fn set_filter(&mut self, filter: Box<dyn Filter>) {
        self.filters = vec![filter];
    }

    pub fn log(&mut self, level: Level, message: &str) {
        if !self.filters.iter().all(|f| f.should_log(level, &self.module)) {
            return;
        }

        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let formatted_message = format!("[{}] [{}] [{}] {}", timestamp, level, self.module, message);

        for output in &mut self.outputs {
            if let Err(e) = output.write(&formatted_message) {
                eprintln!("Failed to write log: {}", e);
            }
        }
    }

    pub fn log_with_fields(&mut self, level: Level, message: &str, fields: HashMap<String, String>) {
        if !self.filters.iter().all(|f| f.should_log(level, &self.module)) {
            return;
        }

        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let fields_str: String = fields
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join(", ");

        let formatted_message = format!(
            "[{}] [{}] [{}] {} ({})",
            timestamp, level, self.module, message, fields_str
        );

        for output in &mut self.outputs {
            if let Err(e) = output.write(&formatted_message) {
                eprintln!("Failed to write log: {}", e);
            }
        }
    }

    pub fn trace(&mut self, message: &str) {
        self.log(Level::Trace, message);
    }

    pub fn debug(&mut self, message: &str) {
        self.log(Level::Debug, message);
    }

    pub fn info(&mut self, message: &str) {
        self.log(Level::Info, message);
    }

    pub fn warn(&mut self, message: &str) {
        self.log(Level::Warning, message);
    }

    pub fn error(&mut self, message: &str) {
        self.log(Level::Error, message);
    }

    pub fn flush(&mut self) {
        for output in &mut self.outputs {
            if let Err(e) = output.flush() {
                eprintln!("Failed to flush log: {}", e);
            }
        }
    }
}

impl Default for Logger {
    fn default() -> Self {
        Logger::new()
    }
}

static GLOBAL_LOGGER: Mutex<Option<Logger>> = Mutex::new(None);

pub fn init() {
    let mut logger = Logger::new();
    logger.add_output(Box::new(ConsoleOutput::new()));
    logger.add_filter(Box::new(LevelFilter::new(Level::Info)));
    *GLOBAL_LOGGER.lock().unwrap() = Some(logger);
}

pub fn set_level(level: Level) {
    if let Some(logger) = GLOBAL_LOGGER.lock().unwrap().as_mut() {
        logger.set_filter(Box::new(LevelFilter::new(level)));
    }
}

pub fn get_logger() -> Option<Logger> {
    None
}

pub fn trace(message: &str) {
    if let Some(logger) = GLOBAL_LOGGER.lock().unwrap().as_mut() {
        logger.trace(message);
    }
}

pub fn debug(message: &str) {
    if let Some(logger) = GLOBAL_LOGGER.lock().unwrap().as_mut() {
        logger.debug(message);
    }
}

pub fn info(message: &str) {
    if let Some(logger) = GLOBAL_LOGGER.lock().unwrap().as_mut() {
        logger.info(message);
    }
}

pub fn warn(message: &str) {
    if let Some(logger) = GLOBAL_LOGGER.lock().unwrap().as_mut() {
        logger.warn(message);
    }
}

pub fn error(message: &str) {
    if let Some(logger) = GLOBAL_LOGGER.lock().unwrap().as_mut() {
        logger.error(message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_order() {
        assert!(Level::Debug < Level::Info);
        assert!(Level::Info < Level::Warning);
        assert!(Level::Warning < Level::Error);
    }

    #[test]
    fn test_level_filter() {
        let filter = LevelFilter::new(Level::Info);
        assert!(filter.should_log(Level::Info, "module"));
        assert!(filter.should_log(Level::Warning, "module"));
        assert!(!filter.should_log(Level::Debug, "module"));
    }

    #[test]
    fn test_module_filter() {
        let filter = ModuleFilter::new(vec!["app".to_string()]);
        assert!(filter.should_log(Level::Info, "app"));
        assert!(filter.should_log(Level::Info, "app::module"));
        assert!(!filter.should_log(Level::Info, "other"));
    }
}

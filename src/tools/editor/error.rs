
use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum EditorError {
    #[error("I/O 错误: {0}")]
    Io(#[from] io::Error),
    
    #[error("文件不存在: {0}")]
    FileNotFound(String),
    
    #[error("无效的位置: line={line}, column={column}")]
    InvalidPosition { line: usize, column: usize },
    
    #[error("撤销历史为空")]
    UndoHistoryEmpty,
    
    #[error("重做历史为空")]
    RedoHistoryEmpty,
    
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("解析错误: {0}")]
    ParseError(String),
    
    #[error("未知命令: {0}")]
    UnknownCommand(String),
    
    #[error("内部错误: {0}")]
    InternalError(String),
}

pub type Result<T> = std::result::Result<T, EditorError>;

// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

pub mod cli;
pub mod editor;
pub mod hla;

// 明确导入 cli 模块的内容
pub use cli::commands::*;
pub use cli::config as cli_config;
pub use cli::error as cli_error;
pub use cli::exit_codes::exit_code::*;

// 明确导入 editor 模块的内容
pub use editor::buffer::*;
pub use editor::cursor::*;
pub use editor::input::*;
pub use editor::render::*;
pub use editor::theme::*;
pub use editor::config as editor_config;
pub use editor::editor::*;
pub use editor::error as editor_error;
pub use editor::action::*;
pub use editor::history::*;

// 导入 hla 模块的内容
pub use hla::*;

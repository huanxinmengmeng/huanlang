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

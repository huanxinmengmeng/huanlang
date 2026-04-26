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

pub mod core;
pub mod backend;
pub mod stdlib;
pub mod tools;
pub mod utils;
pub mod interpreter;
pub mod lang_identity;
pub mod file_format;
pub mod test;

#[cfg(feature = "llvm")]
pub mod lsp;

#[cfg(feature = "llvm")]
pub mod package;

pub use core::lexer::Lexer;
pub use core::parser::Parser;
pub use core::sema::SemanticAnalyzer;
pub use utils::error::{HuanError, Result};
pub use interpreter::Interpreter;

pub use lang_identity::*;
pub use file_format::*;

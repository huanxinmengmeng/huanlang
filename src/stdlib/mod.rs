// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

pub mod core;
pub mod io;
pub mod collections;
pub mod string;
pub mod math;
pub mod system;
pub mod time;
pub mod random;
pub mod crypto;
pub mod serialize;
pub mod net;

pub mod prelude {
    pub use super::core::*;
    pub use super::io::*;
    pub use super::collections::*;
    pub use super::string::*;
    pub use super::math::*;
}

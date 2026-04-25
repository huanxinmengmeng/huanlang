// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
//
// 幻语汇编与裸机编程模块
// 本模块实现了规范文档第9章中定义的完整汇编与裸机编程功能

pub mod ast;
pub mod constraints;
pub mod arch;
pub mod registers;
pub mod peripheral;
pub mod linker;

pub use ast::{Asm, AsmOutput, AsmInput, AsmOption, AsmClobber};
pub use constraints::{Constraint, ConstraintType, validate_constraint};
pub use arch::{Arch, get_arch, get_register_name};
pub use registers::{Register, RegisterClass, RegName, RegisterAccess};
pub use peripheral::{Peripheral, PeripheralRegister, PeripheralAccess};
pub use linker::{MemoryRegion, Section, LinkerScript, LayoutType};

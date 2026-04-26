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

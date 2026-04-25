// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

pub mod allocator;
pub mod borrow;
pub mod boxed;
pub mod list;
pub mod map;
pub mod string;
pub mod rc;
pub mod weak;

pub use allocator::{Allocator, GlobalAllocator, FirstFitAllocator, default_allocator};
pub use borrow::{BorrowChecker, BorrowError, LoanState, VarId};
pub use boxed::Box;
pub use list::HuanList;
pub use map::{HuanMap, MapEntry};
pub use string::HuanString;
pub use rc::Rc;
pub use weak::Weak;

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

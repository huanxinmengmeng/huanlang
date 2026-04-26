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
pub mod console;

pub mod prelude {
    pub use super::core::*;
    pub use super::io::*;
    pub use super::collections::*;
    pub use super::string::*;
    pub use super::math::*;
    pub use super::console::*;
}

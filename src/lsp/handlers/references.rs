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

//! 引用请求处理器

use crate::lsp::{Position, Location};

/// 引用请求处理器
pub struct ReferencesHandler;

impl ReferencesHandler {
    /// 处理引用请求
    pub fn handle(
        uri: &str,
        position: Position,
        word: &str,
        include_declaration: bool,
    ) -> Vec<Location> {
        // 这里应该从工作区索引查找所有引用
        // 简化实现：返回空列表
        Vec::new()
    }
}

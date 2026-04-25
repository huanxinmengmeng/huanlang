// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

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

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

//! 依赖管理模块

use serde::{Deserialize, Serialize};

/// 依赖定义
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// 版本约束
    pub version: Option<String>,
    /// 注册表名称
    pub registry: Option<String>,
    /// 依赖路径（本地路径）
    pub path: Option<String>,
    /// 依赖 Git 仓库
    pub git: Option<String>,
    /// Git 分支或标签
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
    /// 可选依赖
    pub optional: Option<bool>,
    /// 特性标志
    pub features: Option<Vec<String>>,
    /// 默认特性
    pub default_features: Option<bool>,
}

/// 版本约束类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum VersionConstraint {
    /// 精确版本
    Exact(String),
    /// 大于等于
    GreaterThanOrEqual(String),
    /// 大于
    GreaterThan(String),
    /// 小于等于
    LessThanOrEqual(String),
    /// 小于
    LessThan(String),
    /// 波浪号范围
    Tilde(String),
    /// 插入号范围
    Caret(String),
    /// 通配符
    Wildcard(usize), // 0: *, 1: x, 2: x.y
    /// 版本范围
    Range(String, String),
    /// 版本集合
    Set(Vec<String>),
}

/// 依赖解析结果
#[derive(Debug, Clone)]
pub struct ResolvedDependency {
    /// 包名称
    pub name: String,
    /// 解析的版本
    pub version: String,
    /// 依赖路径
    pub path: Option<String>,
    /// 依赖来源
    pub source: DependencySource,
    /// 传递依赖
    pub dependencies: Vec<ResolvedDependency>,
}

/// 依赖来源
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencySource {
    /// 注册表
    Registry(String),
    /// 本地路径
    Path(String),
    /// Git 仓库
    Git(String),
    /// 内置依赖
    Builtin,
}

/// 依赖冲突
#[derive(Debug, Clone)]
pub struct DependencyConflict {
    /// 包名称
    pub package: String,
    /// 冲突的版本需求
    pub conflicts: Vec<(String, String)>, // (依赖路径, 版本约束)
}

impl Dependency {
    /// 创建新的依赖
    pub fn new(version: &str) -> Self {
        Self {
            version: Some(version.to_string()),
            registry: None,
            path: None,
            git: None,
            branch: None,
            tag: None,
            rev: None,
            optional: None,
            features: None,
            default_features: None,
        }
    }

    /// 从版本字符串创建依赖
    pub fn from_version(version: &str) -> Self {
        Self::new(version)
    }

    /// 从本地路径创建依赖
    pub fn from_path(path: &str) -> Self {
        Self {
            version: None,
            registry: None,
            path: Some(path.to_string()),
            git: None,
            branch: None,
            tag: None,
            rev: None,
            optional: None,
            features: None,
            default_features: None,
        }
    }

    /// 从 Git 仓库创建依赖
    pub fn from_git(git: &str) -> Self {
        Self {
            version: None,
            registry: None,
            path: None,
            git: Some(git.to_string()),
            branch: None,
            tag: None,
            rev: None,
            optional: None,
            features: None,
            default_features: None,
        }
    }
}

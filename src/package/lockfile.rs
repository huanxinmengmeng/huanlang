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

//! 锁定文件管理模块

use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::package::error::{PackageError, PackageResult};
use crate::package::resolver::ResolutionResult;

/// 锁定文件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageLock {
    /// 版本
    pub version: u32,
    /// 包
    pub packages: HashMap<String, LockPackage>,
    /// 元数据
    pub metadata: Option<LockMetadata>,
}

/// 锁定的包
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockPackage {
    /// 版本
    pub version: String,
    /// 源
    pub source: Option<Source>,
    /// 依赖
    pub dependencies: Option<HashMap<String, String>>,
    /// 开发依赖
    pub dev_dependencies: Option<HashMap<String, String>>,
    /// 构建依赖
    pub build_dependencies: Option<HashMap<String, String>>,
    /// 功能
    pub features: Option<Vec<String>>,
    /// 校验和
    pub checksum: Option<String>,
    /// 版本控制
    pub version_control: Option<VersionControl>,
}

/// 源
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    /// 注册表
    Registry {
        name: String,
        url: String,
    },
    /// 路径
    Path {
        path: String,
    },
    /// Git
    Git {
        url: String,
        rev: String,
        branch: Option<String>,
        tag: Option<String>,
    },
}

/// 版本控制
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControl {
    pub type_field: String, // 使用 type_field 避免关键字冲突
    pub url: String,
    pub rev: String,
    pub branch: Option<String>,
    pub tag: Option<String>,
}

/// 元数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LockMetadata {
    /// 解析器
    pub resolver: String,
    /// 解析日期
    pub resolved_date: String,
    /// 系统信息
    pub system: Option<SystemInfo>,
    /// 编译器信息
    pub compiler: Option<CompilerInfo>,
}

/// 系统信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub version: Option<String>,
}

/// 编译器信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompilerInfo {
    pub name: String,
    pub version: String,
    pub channel: Option<String>,
}

use std::collections::HashMap;

impl PackageLock {
    /// 从文件加载
    pub fn from_file<P: AsRef<Path>>(path: P) -> PackageResult<Self> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(path.to_str().unwrap())))?;
        
        Self::from_str(&content)
    }

    /// 从字符串加载
    pub fn from_str(content: &str) -> PackageResult<Self> {
        toml::from_str(content)
            .map_err(|e| PackageError::config_error(&e.to_string(), None))
    }

    /// 保存到文件
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> PackageResult<()> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| PackageError::config_error(&e.to_string(), None))?;
        
        fs::write(path, content)
            .map_err(|e| PackageError::io_error(&e.to_string(), None))
    }

    /// 从解析结果创建
    pub fn from_resolution(result: &ResolutionResult) -> Self {
        let mut packages = HashMap::new();
        
        for (name, version) in &result.resolved {
            packages.insert(name.clone(), LockPackage {
                version: version.to_string(),
                source: None,
                dependencies: None,
                dev_dependencies: None,
                build_dependencies: None,
                features: None,
                checksum: None,
                version_control: None,
            });
        }
        
        PackageLock {
            version: 1,
            packages,
            metadata: Some(LockMetadata {
                resolver: "pubgrub".to_string(),
                resolved_date: chrono::Utc::now().to_string(),
                system: Some(SystemInfo {
                    os: std::env::consts::OS.to_string(),
                    arch: std::env::consts::ARCH.to_string(),
                    version: None,
                }),
                compiler: Some(CompilerInfo {
                    name: "huanlang".to_string(),
                    version: "1.2".to_string(),
                    channel: Some("stable".to_string()),
                }),
            }),
        }
    }

    /// 验证锁定文件
    pub fn validate(&self) -> PackageResult<()> {
        if self.version != 1 {
            return Err(PackageError::config_error(
                "不支持的锁定文件版本",
                None
            ));
        }
        
        if self.packages.is_empty() {
            return Err(PackageError::config_error(
                "锁定文件中没有包",
                None
            ));
        }
        
        Ok(())
    }

    /// 获取包版本
    pub fn get_package_version(&self, name: &str) -> Option<&String> {
        self.packages.get(name).map(|p| &p.version)
    }

    /// 检查是否包含包
    pub fn contains_package(&self, name: &str) -> bool {
        self.packages.contains_key(name)
    }

    /// 更新包版本
    pub fn update_package(&mut self, name: &str, version: &str) -> PackageResult<()> {
        if let Some(package) = self.packages.get_mut(name) {
            package.version = version.to_string();
            Ok(())
        } else {
            Err(PackageError::package_error(
                &format!("包 {} 不在锁定文件中", name),
                Some(name)
            ))
        }
    }

    /// 移除包
    pub fn remove_package(&mut self, name: &str) -> bool {
        self.packages.remove(name).is_some()
    }

    /// 合并两个锁定文件
    pub fn merge(&mut self, other: &PackageLock) -> PackageResult<()> {
        for (name, package) in &other.packages {
            self.packages.insert(name.clone(), package.clone());
        }
        
        // 更新元数据
        self.metadata = other.metadata.clone();
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::resolver::DependencyResolver;
    use crate::package::resolver::Version;
    use crate::package::resolver::VersionConstraint;

    #[test]
    fn test_lock_file_creation() {
        let mut resolver = DependencyResolver::new();
        
        // 添加包
        resolver.add_package("a", Version::parse("1.0.0").unwrap());
        resolver.add_package("b", Version::parse("1.0.0").unwrap());
        
        // 添加依赖
        resolver.add_dependency(
            "a",
            Version::parse("1.0.0").unwrap(),
            "b",
            VersionConstraint::parse("^1.0.0").unwrap()
        );
        
        // 解析
        let mut root_deps = HashMap::new();
        root_deps.insert("a".to_string(), VersionConstraint::parse("^1.0.0").unwrap());
        
        let result = resolver.resolve(&root_deps).unwrap();
        let lock = PackageLock::from_resolution(&result);
        
        assert_eq!(lock.version, 1);
        assert_eq!(lock.packages.len(), 2);
        assert!(lock.packages.contains_key("a"));
        assert!(lock.packages.contains_key("b"));
    }

    #[test]
    fn test_lock_file_save_load() {
        let mut resolver = DependencyResolver::new();
        
        // 添加包
        resolver.add_package("test", Version::parse("1.0.0").unwrap());
        
        // 解析
        let mut root_deps = HashMap::new();
        root_deps.insert("test".to_string(), VersionConstraint::parse("^1.0.0").unwrap());
        
        let result = resolver.resolve(&root_deps).unwrap();
        let lock = PackageLock::from_resolution(&result);
        
        // 保存到临时文件
        let temp_path = std::env::temp_dir().join("test_lock.toml");
        lock.save_to_file(&temp_path).unwrap();
        
        // 加载
        let loaded_lock = PackageLock::from_file(&temp_path).unwrap();
        
        assert_eq!(loaded_lock.version, lock.version);
        assert_eq!(loaded_lock.packages.len(), lock.packages.len());
        
        // 清理
        std::fs::remove_file(temp_path).unwrap();
    }

    #[test]
    fn test_validate() {
        let mut resolver = DependencyResolver::new();
        
        // 添加包
        resolver.add_package("test", Version::parse("1.0.0").unwrap());
        
        // 解析
        let mut root_deps = HashMap::new();
        root_deps.insert("test".to_string(), VersionConstraint::parse("^1.0.0").unwrap());
        
        let result = resolver.resolve(&root_deps).unwrap();
        let lock = PackageLock::from_resolution(&result);
        
        assert!(lock.validate().is_ok());
    }
}

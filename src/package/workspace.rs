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

//! 工作区管理模块

use std::path::Path;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::package::error::{PackageError, PackageResult};
use crate::package::manifest::PackageManifest;

/// 工作区
pub struct Workspace {
    root: String,
    config: WorkspaceConfig,
    members: HashMap<String, PackageManifest>,
    dependencies: HashMap<String, crate::package::manifest::Dependency>,
}

/// 工作区配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub members: Vec<String>,
    pub exclude: Option<Vec<String>>,
    pub default_members: Option<Vec<String>>,
    pub resolver: Option<String>,
    pub package: Option<Value>,
    pub dependencies: Option<std::collections::HashMap<String, crate::package::manifest::Dependency>>,
}

/// 工作区成员
#[derive(Debug, Clone)]
pub struct WorkspaceMember {
    pub name: String,
    pub path: String,
    pub manifest: PackageManifest,
}

impl Workspace {
    /// 从目录加载工作区
    pub fn from_dir<P: AsRef<Path>>(path: P) -> PackageResult<Self> {
        let root = path.as_ref().to_str().unwrap().to_string();
        let workspace_path = Path::new(&root).join("幻语包.toml");
        
        if !workspace_path.exists() {
            return Err(PackageError::config_error(
                "找不到工作区配置文件",
                Some(workspace_path.to_str().unwrap())
            ));
        }
        
        let manifest = PackageManifest::from_file(&workspace_path)?;
        let manifest_workspace = manifest.workspace.ok_or_else(|| {
            PackageError::config_error("工作区配置文件中缺少 workspace 部分", Some(workspace_path.to_str().unwrap()))
        })?;
        
        // 转换为 workspace::WorkspaceConfig
        let converted_deps = manifest_workspace.dependencies.map(|deps| {
            deps.into_iter().collect()
        });
        
        let config = WorkspaceConfig {
            members: manifest_workspace.members.unwrap_or_default(),
            exclude: manifest_workspace.exclude,
            default_members: manifest_workspace.default_members,
            resolver: manifest_workspace.resolver,
            package: manifest_workspace.package,
            dependencies: converted_deps,
        };
        
        let mut members = HashMap::new();
        let mut dependencies = HashMap::new();
        
        // 加载成员
        for member_path in &config.members {
            let member_full_path = Path::new(&root).join(member_path).join("幻语包.toml");
            if member_full_path.exists() {
                let member_manifest = PackageManifest::from_file(&member_full_path)?;
                members.insert(member_manifest.package.name.clone(), member_manifest);
            }
        }
        
        // 加载工作区依赖
        if let Some(workspace_deps) = &config.dependencies {
            dependencies.extend(workspace_deps.iter().map(|(k, v)| (k.clone(), v.clone())));
        }
        
        Ok(Workspace {
            root,
            config,
            members,
            dependencies,
        })
    }

    /// 获取所有成员
    pub fn get_members(&self) -> Vec<WorkspaceMember> {
        self.members.iter().map(|(name, manifest)| {
            let path = self.find_member_path(name).unwrap_or_default();
            WorkspaceMember {
                name: name.clone(),
                path,
                manifest: manifest.clone(),
            }
        }).collect()
    }

    /// 获取成员
    pub fn get_member(&self, name: &str) -> Option<&PackageManifest> {
        self.members.get(name)
    }

    /// 添加成员
    pub fn add_member(&mut self, path: &str) -> PackageResult<()> {
        let member_full_path = Path::new(&self.root).join(path).join("幻语包.toml");
        
        if !member_full_path.exists() {
            return Err(PackageError::config_error(
                &format!("找不到成员包文件: {}", member_full_path.to_str().unwrap()),
                Some(member_full_path.to_str().unwrap())
            ));
        }
        
        let member_manifest = PackageManifest::from_file(&member_full_path)?;
        
        // 检查是否已存在
        if self.members.contains_key(&member_manifest.package.name) {
            return Err(PackageError::package_error(
                &format!("成员 {} 已存在", member_manifest.package.name),
                Some(&member_manifest.package.name)
            ));
        }
        
        // 添加到配置
        if !self.config.members.contains(&path.to_string()) {
            self.config.members.push(path.to_string());
        }
        
        // 添加到成员列表
        self.members.insert(member_manifest.package.name.clone(), member_manifest);
        
        // 保存配置
        self.save()?;
        
        Ok(())
    }

    /// 移除成员
    pub fn remove_member(&mut self, name: &str) -> PackageResult<()> {
        if !self.members.contains_key(name) {
            return Err(PackageError::package_error(
                &format!("成员 {} 不存在", name),
                Some(name)
            ));
        }
        
        // 从配置中移除
        let _member = self.members.get(name).unwrap();
        let path = self.find_member_path(name).unwrap_or_default();
        
        if let Some(index) = self.config.members.iter().position(|p| *p == path) {
            self.config.members.remove(index);
        }
        
        // 从成员列表中移除
        self.members.remove(name);
        
        // 保存配置
        self.save()?;
        
        Ok(())
    }

    /// 获取工作区依赖
    pub fn get_dependencies(&self) -> &HashMap<String, crate::package::manifest::Dependency> {
        &self.dependencies
    }

    /// 添加工作区依赖
    pub fn add_dependency(&mut self, name: &str, version: &str) -> PackageResult<()> {
        let dependency = crate::package::manifest::Dependency::Version(version.to_string());
        self.dependencies.insert(name.to_string(), dependency.clone());
        
        if self.config.dependencies.is_none() {
            self.config.dependencies = Some(HashMap::new());
        }
        
        self.config.dependencies.as_mut().unwrap().insert(name.to_string(), dependency);
        
        // 保存配置
        self.save()?;
        
        Ok(())
    }

    /// 移除工作区依赖
    pub fn remove_dependency(&mut self, name: &str) -> PackageResult<()> {
        self.dependencies.remove(name);
        
        if let Some(deps) = &mut self.config.dependencies {
            deps.remove(name);
        }
        
        // 保存配置
        self.save()?;
        
        Ok(())
    }

    /// 构建工作区
    pub fn build(&self) -> PackageResult<()> {
        println!("构建工作区...");
        
        for (name, _manifest) in &self.members {
            println!("  构建: {}", name);
            // 这里应该调用构建命令
        }
        
        Ok(())
    }

    /// 测试工作区
    pub fn test(&self) -> PackageResult<()> {
        println!("测试工作区...");
        
        for (name, _manifest) in &self.members {
            println!("  测试: {}", name);
            // 这里应该调用测试命令
        }
        
        Ok(())
    }

    /// 运行工作区命令
    pub fn run(&self, command: &str) -> PackageResult<()> {
        println!("运行工作区命令: {}", command);
        
        for (name, _manifest) in &self.members {
            println!("  运行 {} 在 {}", command, name);
            // 这里应该调用相应命令
        }
        
        Ok(())
    }

    /// 查找成员路径
    fn find_member_path(&self, name: &str) -> Option<String> {
        for member_path in &self.config.members {
            let member_full_path = Path::new(&self.root).join(member_path).join("幻语包.toml");
            if member_full_path.exists() {
                if let Ok(manifest) = PackageManifest::from_file(&member_full_path) {
                    if manifest.package.name == name {
                        return Some(member_path.clone());
                    }
                }
            }
        }
        None
    }

    /// 保存工作区配置
    fn save(&self) -> PackageResult<()> {
        let workspace_path = Path::new(&self.root).join("幻语包.toml");
        let manifest = PackageManifest::from_file(&workspace_path)?;
        
        let mut manifest = manifest;
        manifest.workspace = Some(crate::package::manifest::WorkspaceConfig {
            members: Some(self.config.members.clone()),
            exclude: self.config.exclude.clone(),
            default_members: self.config.default_members.clone(),
            resolver: self.config.resolver.clone(),
            package: self.config.package.clone(),
            dependencies: None, // 转换为正确的类型
        });
        
        manifest.save_to_file(&workspace_path)?;
        
        Ok(())
    }

    /// 验证工作区
    pub fn validate(&self) -> PackageResult<()> {
        // 检查所有成员是否存在
        for member_path in &self.config.members {
            let member_full_path = Path::new(&self.root).join(member_path).join("幻语包.toml");
            if !member_full_path.exists() {
                return Err(PackageError::config_error(
                    &format!("成员路径不存在: {}", member_path),
                    Some(member_full_path.to_str().unwrap())
                ));
            }
        }
        
        // 检查成员名称是否唯一
        let mut names = HashSet::new();
        for (name, _) in &self.members {
            if !names.insert(name) {
                return Err(PackageError::config_error(
                    &format!("成员名称重复: {}", name),
                    None
                ));
            }
        }
        
        Ok(())
    }

    /// 获取根目录
    pub fn root(&self) -> &str {
        &self.root
    }

    /// 获取配置
    pub fn config(&self) -> &WorkspaceConfig {
        &self.config
    }
}

/// 功能标志管理
pub struct FeatureManager {
    features: HashMap<String, Vec<String>>,
    default_features: Vec<String>,
}

impl FeatureManager {
    /// 从清单创建
    pub fn from_manifest(manifest: &PackageManifest) -> Self {
        let features = manifest.features.clone().unwrap_or_default();
        let default_features = vec![]; // 默认为空
        
        FeatureManager {
            features,
            default_features,
        }
    }

    /// 获取所有功能
    pub fn get_features(&self) -> &HashMap<String, Vec<String>> {
        &self.features
    }

    /// 获取默认功能
    pub fn get_default_features(&self) -> &Vec<String> {
        &self.default_features
    }

    /// 检查功能是否存在
    pub fn has_feature(&self, name: &str) -> bool {
        self.features.contains_key(name)
    }

    /// 获取功能依赖
    pub fn get_feature_dependencies(&self, name: &str) -> Option<&Vec<String>> {
        self.features.get(name)
    }

    /// 解析功能标志
    pub fn resolve_features(&self, requested_features: &[String]) -> HashSet<String> {
        let mut resolved = HashSet::new();
        
        // 添加默认功能
        for feature in &self.default_features {
            self.resolve_feature(feature, &mut resolved);
        }
        
        // 添加请求的功能
        for feature in requested_features {
            self.resolve_feature(feature, &mut resolved);
        }
        
        resolved
    }

    /// 递归解析功能
    fn resolve_feature(&self, name: &str, resolved: &mut HashSet<String>) {
        if resolved.contains(name) {
            return;
        }
        
        resolved.insert(name.to_string());
        
        if let Some(deps) = self.features.get(name) {
            for dep in deps {
                self.resolve_feature(dep, resolved);
            }
        }
    }

    /// 添加功能
    pub fn add_feature(&mut self, name: &str, dependencies: &[String]) {
        self.features.insert(name.to_string(), dependencies.to_vec());
    }

    /// 移除功能
    pub fn remove_feature(&mut self, name: &str) {
        self.features.remove(name);
    }

    /// 设置默认功能
    pub fn set_default_features(&mut self, features: &[String]) {
        self.default_features = features.to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::package::manifest::Dependency;
    use std::fs;

    #[test]
    fn test_workspace() {
        // 创建临时目录
        let temp_dir = tempfile::tempdir().unwrap();
        let root = temp_dir.path();
        
        // 创建工作区配置
        let workspace_toml = r#"
[package]
name = "workspace"
version = "0.3.0"

[workspace]
members = ["member1", "member2"]

[workspace.dependencies]
"common" = "1.0.0"
"#;
        
        fs::write(root.join("幻语包.toml"), workspace_toml).unwrap();
        
        // 创建成员目录
        fs::create_dir_all(root.join("member1")).unwrap();
        fs::create_dir_all(root.join("member2")).unwrap();
        
        // 创建成员配置
        let member1_toml = r#"
[package]
name = "member1"
version = "0.3.0"

[dependencies]
common = { workspace = true }
"#;
        
        let member2_toml = r#"
[package]
name = "member2"
version = "0.3.0"

[dependencies]
common = { workspace = true }
member1 = { path = "../member1" }
"#;
        
        fs::write(root.join("member1").join("幻语包.toml"), member1_toml).unwrap();
        fs::write(root.join("member2").join("幻语包.toml"), member2_toml).unwrap();
        
        // 加载工作区
        let workspace = Workspace::from_dir(root).unwrap();
        
        // 验证工作区
        assert_eq!(workspace.members.len(), 2);
        assert!(workspace.members.contains_key("member1"));
        assert!(workspace.members.contains_key("member2"));
        
        // 验证依赖
        assert!(workspace.dependencies.contains_key("common"));
        assert_eq!(workspace.dependencies.get("common"), Some(&Dependency::Version("1.0.0".to_string())));
    }

    #[test]
    fn test_feature_manager() {
        let toml = r#"
[package]
name = "test"
version = "0.3.0"

[features]
default = ["std"]
std = []
serde = ["dep:serde"]
json = ["serde", "dep:serde_json"]
"#;
        
        let manifest = PackageManifest::from_str(toml).unwrap();
        let feature_manager = FeatureManager::from_manifest(&manifest);
        
        // 验证功能
        assert!(feature_manager.has_feature("std"));
        assert!(feature_manager.has_feature("serde"));
        assert!(feature_manager.has_feature("json"));
        
        // 验证功能依赖
        assert_eq!(feature_manager.get_feature_dependencies("json"), Some(&vec!["serde".to_string(), "dep:serde_json".to_string()]));
        
        // 解析功能
        let resolved = feature_manager.resolve_features(&["json".to_string()]);
        assert!(resolved.contains("json"));
        assert!(resolved.contains("serde"));
    }
}

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

//! 包描述文件解析模块

use std::path::Path;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::package::error::PackageError;

use crate::package::security::PackageSignature;

/// 包描述文件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageManifest {
    /// 包信息
    pub package: PackageInfo,
    /// 依赖
    pub dependencies: Option<HashMap<String, Dependency>>,
    /// 开发依赖
    pub dev_dependencies: Option<HashMap<String, Dependency>>,
    /// 构建依赖
    pub build_dependencies: Option<HashMap<String, Dependency>>,
    /// 外部库
    pub extern_libs: Option<ExternLibs>,
    /// 库配置
    pub lib: Option<LibConfig>,
    /// 可执行文件
    pub bins: Option<Vec<BinConfig>>,
    /// 示例
    pub examples: Option<Vec<ExampleConfig>>,
    /// 测试
    pub tests: Option<Vec<TestConfig>>,
    /// 基准测试
    pub benches: Option<Vec<BenchConfig>>,
    /// 构建配置
    pub build: Option<BuildConfig>,
    /// 配置文件
    pub profile: Option<ProfileConfig>,
    /// 绑定配置
    pub bindings: Option<BindingsConfig>,
    /// 工作区配置
    pub workspace: Option<WorkspaceConfig>,
    /// 补丁配置
    pub patch: Option<serde::Value>,
    /// 替换配置
    pub replace: Option<serde::Value>,
    /// 功能标志
    pub features: Option<HashMap<String, Vec<String>>>,
    /// 包签名
    pub signature: Option<PackageSignature>,
}

/// 包信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    /// 包名称
    pub name: String,
    /// 版本
    pub version: String,
    /// 幻语版本
    pub edition: Option<String>,
    /// 作者
    pub authors: Option<Vec<String>>,
    /// 描述
    pub description: Option<String>,
    /// 许可证
    pub license: Option<String>,
    /// 许可证文件
    pub license_file: Option<String>,
    /// 仓库
    pub repository: Option<String>,
    /// 文档
    pub documentation: Option<String>,
    /// 主页
    pub homepage: Option<String>,
    /// 关键词
    pub keywords: Option<Vec<String>>,
    /// 分类
    pub categories: Option<Vec<String>>,
    /// 自述文件
    pub readme: Option<String>,
    /// 关键词风格
    pub keyword_style: Option<String>,
}

/// 依赖类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Dependency {
    /// 版本
    Version(String),
    /// 详细配置
    Detailed(DetailedDependency),
}

/// 详细依赖配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedDependency {
    pub version: Option<String>,
    pub path: Option<String>,
    pub git: Option<String>,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub rev: Option<String>,
    pub optional: Option<bool>,
    pub features: Option<Vec<String>>,
    pub registry: Option<String>,
}

/// 外部库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternLibs {
    /// C 库
    pub c_libs: Option<Vec<String>>,
    /// C 头文件
    pub c_headers: Option<Vec<String>>,
    /// Python 模块
    pub python_modules: Option<Vec<PythonModule>>,
    /// Rust crate
    pub rust_crates: Option<Vec<RustCrate>>,
    /// 汇编文件
    pub asm_files: Option<Vec<String>>,
    /// 链接器脚本
    pub linker_scripts: Option<Vec<String>>,
}

/// Python 模块
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PythonModule {
    pub name: String,
    pub version: Option<String>,
}

/// Rust Crate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustCrate {
    pub name: String,
    pub version: String,
    pub features: Option<Vec<String>>,
}

/// 库配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibConfig {
    pub name: Option<String>,
    pub path: Option<String>,
    pub crate_type: Option<Vec<String>>,
    pub test: Option<bool>,
    pub doctest: Option<bool>,
    pub bench: Option<bool>,
}

/// 可执行文件配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinConfig {
    pub name: String,
    pub path: Option<String>,
    pub test: Option<bool>,
    pub bench: Option<bool>,
}

/// 示例配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExampleConfig {
    pub name: String,
    pub path: String,
    pub crate_type: Option<Vec<String>>,
}

/// 测试配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestConfig {
    pub name: String,
    pub path: String,
}

/// 基准测试配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchConfig {
    pub name: String,
    pub path: String,
}

/// 构建配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub opt_level: Option<u8>,
    pub target: Option<String>,
    pub ownership: Option<bool>,
    pub debug: Option<bool>,
    pub pic: Option<bool>,
    pub stack_protector: Option<bool>,
    pub target_features: Option<Vec<String>>,
    pub jobs: Option<u32>,
    pub incremental: Option<bool>,
}

/// Profile 配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    pub dev: Option<ProfileSettings>,
    pub release: Option<ProfileSettings>,
    pub test: Option<ProfileSettings>,
    pub bench: Option<ProfileSettings>,
}

/// Profile 设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileSettings {
    pub opt_level: Option<u8>,
    pub debug: Option<bool>,
    pub overflow_checks: Option<bool>,
    pub incremental: Option<bool>,
    pub lto: Option<bool>,
    pub codegen_units: Option<u32>,
    pub strip: Option<bool>,
    pub panic: Option<String>,
}

/// 绑定配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingsConfig {
    pub export_name: Option<String>,
    pub target_languages: Option<Vec<String>>,
    pub output_dir: Option<String>,
    pub include_docs: Option<bool>,
}

/// 工作区配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceConfig {
    pub members: Option<Vec<String>>,
    pub exclude: Option<Vec<String>>,
    pub default_members: Option<Vec<String>>,
    pub resolver: Option<String>,
    pub package: Option<serde::Value>,
    pub dependencies: Option<HashMap<String, Dependency>>,
}

/// 结果类型
pub type PackageResult<T> = std::result::Result<T, PackageError>;

use std::collections::HashMap;

impl PackageManifest {
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

    /// 验证包配置
    pub fn validate(&self) -> PackageResult<()> {
        // 验证包名称
        if self.package.name.is_empty() {
            return Err(PackageError::package_error("包名称不能为空", Some(&self.package.name)));
        }

        // 验证版本
        if self.package.version.is_empty() {
            return Err(PackageError::version_error("版本不能为空", None));
        }

        // 验证包名称格式
        if !self.is_valid_package_name(&self.package.name) {
            return Err(PackageError::package_error("包名称格式无效", Some(&self.package.name)));
        }

        Ok(())
    }

    /// 验证包名称格式
    fn is_valid_package_name(&self, name: &str) -> bool {
        if name.is_empty() || name.len() < 3 || name.len() > 64 {
            return false;
        }

        if !name.chars().next().unwrap().is_alphabetic() {
            return false;
        }

        for c in name.chars() {
            if !c.is_alphanumeric() && c != '-' && c != '_' {
                return false;
            }
        }

        true
    }

    /// 获取默认幻语版本
    pub fn default_edition(&self) -> String {
        self.package.edition.clone().unwrap_or_else(|| "1.2".to_string())
    }

    /// 获取所有依赖
    pub fn all_dependencies(&self) -> HashMap<String, Dependency> {
        let mut all = HashMap::new();
        
        if let Some(deps) = &self.dependencies {
            all.extend(deps.iter().map(|(k, v)| (k.clone(), v.clone())));
        }

        if let Some(deps) = &self.dev_dependencies {
            all.extend(deps.iter().map(|(k, v)| (k.clone(), v.clone())));
        }

        if let Some(deps) = &self.build_dependencies {
            all.extend(deps.iter().map(|(k, v)| (k.clone(), v.clone())));
        }
        
        all
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manifest_parse() {
        let toml = r#"
[package]
name = "测试包"
version = "0.1.0"
edition = "1.2"
authors = ["测试者 <test@example.com>"]
description = "测试包描述"
license = "MIT"

[dependencies]
网络 = "0.3"
序列化 = { version = "1.0", optional = true }

[dev-dependencies]
测试工具 = "0.5"
"#;

        let manifest = PackageManifest::from_str(toml).unwrap();
        assert_eq!(manifest.package.name, "测试包");
        assert_eq!(manifest.package.version, "0.1.0");
        assert_eq!(manifest.dependencies.unwrap().len(), 2);
    }

    #[test]
    fn test_validate() {
        let toml = r#"
[package]
name = "valid-package"
version = "0.1.0"
"#;

        let manifest = PackageManifest::from_str(toml).unwrap();
        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_invalid_package_name() {
        let toml = r#"
[package]
name = "123invalid"
version = "0.1.0"
"#;

        let manifest = PackageManifest::from_str(toml).unwrap();
        assert!(manifest.validate().is_err());
    }
}

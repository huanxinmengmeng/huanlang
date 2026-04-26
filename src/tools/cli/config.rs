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

//! 配置文件解析模块

use std::collections::HashMap;
use std::path::Path;
use crate::tools::cli::error::{CliError, CliResult};

/// 包配置
#[derive(Debug, Clone)]
pub struct PackageConfig {
    pub package: PackageInfo,
    pub dependencies: HashMap<String, Dependency>,
    pub dev_dependencies: HashMap<String, Dependency>,
    pub build_dependencies: HashMap<String, Dependency>,
    pub extern_libs: ExternLibs,
    pub lib: Option<LibConfig>,
    pub bins: Vec<BinConfig>,
    pub examples: Vec<ExampleConfig>,
    pub tests: Vec<TestConfig>,
    pub benches: Vec<BenchConfig>,
    pub build: BuildConfig,
    pub profile: ProfileConfig,
    pub bindings: Option<BindingsConfig>,
    pub workspace: Option<WorkspaceConfig>,
}

/// 包信息
#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub edition: String,
    pub authors: Vec<String>,
    pub description: String,
    pub license: String,
    pub repository: Option<String>,
    pub documentation: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Vec<String>,
    pub categories: Vec<String>,
    pub readme: Option<String>,
    pub keyword_style: KeywordStyle,
}

/// 关键词风格
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeywordStyle {
    Chinese,
    Pinyin,
    English,
    Mixed,
}

impl Default for KeywordStyle {
    fn default() -> Self {
        KeywordStyle::Chinese
    }
}

/// 依赖类型
#[derive(Debug, Clone)]
pub enum Dependency {
    Version(String),
    Path(String),
    Git { url: String, branch: Option<String>, rev: Option<String> },
    Optional(Box<Dependency>),
}

impl Dependency {
    /// 解析依赖字符串
    pub fn parse(s: &str) -> CliResult<Self> {
        if s.starts_with("path = ") {
            let path = s.trim_start_matches("path = ")
                .trim_matches('"');
            Ok(Dependency::Path(path.to_string()))
        } else if s.starts_with("git = ") {
            let url = s.trim_start_matches("git = ")
                .trim_matches('"')
                .to_string();
            Ok(Dependency::Git { url, branch: None, rev: None })
        } else {
            Ok(Dependency::Version(s.trim_matches('"').to_string()))
        }
    }
}

/// 外部库配置
#[derive(Debug, Clone)]
pub struct ExternLibs {
    pub c_libs: Vec<String>,
    pub c_headers: Vec<String>,
    pub python_modules: Vec<PythonModule>,
    pub rust_crates: Vec<RustCrate>,
    pub asm_files: Vec<String>,
}

impl Default for ExternLibs {
    fn default() -> Self {
        Self {
            c_libs: Vec::new(),
            c_headers: Vec::new(),
            python_modules: Vec::new(),
            rust_crates: Vec::new(),
            asm_files: Vec::new(),
        }
    }
}

/// Python 模块
#[derive(Debug, Clone)]
pub struct PythonModule {
    pub name: String,
    pub version: Option<String>,
}

/// Rust Crate
#[derive(Debug, Clone)]
pub struct RustCrate {
    pub name: String,
    pub version: String,
    pub features: Vec<String>,
}

/// 库配置
#[derive(Debug, Clone)]
pub struct LibConfig {
    pub name: String,
    pub path: String,
    pub crate_type: Vec<CrateType>,
}

/// Crate 类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CrateType {
    Staticlib,
    Cdylib,
    Rlib,
}

impl Default for CrateType {
    fn default() -> Self {
        CrateType::Rlib
    }
}

/// 可执行文件配置
#[derive(Debug, Clone)]
pub struct BinConfig {
    pub name: String,
    pub path: String,
}

/// 示例配置
#[derive(Debug, Clone)]
pub struct ExampleConfig {
    pub name: String,
    pub path: String,
}

/// 测试配置
#[derive(Debug, Clone)]
pub struct TestConfig {
    pub name: String,
    pub path: String,
}

/// 基准测试配置
#[derive(Debug, Clone)]
pub struct BenchConfig {
    pub name: String,
    pub path: String,
}

/// 构建配置
#[derive(Debug, Clone)]
pub struct BuildConfig {
    pub opt_level: u8,
    pub target: String,
    pub ownership: bool,
    pub debug: bool,
    pub pic: bool,
    pub stack_protector: bool,
    pub target_features: Vec<String>,
    pub jobs: u32,
}

impl Default for BuildConfig {
    fn default() -> Self {
        Self {
            opt_level: 2,
            target: "host".to_string(),
            ownership: false,
            debug: false,
            pic: false,
            stack_protector: false,
            target_features: Vec::new(),
            jobs: num_cpus(),
        }
    }
}

/// Profile 配置
#[derive(Debug, Clone)]
pub struct ProfileConfig {
    pub dev: ProfileSettings,
    pub release: ProfileSettings,
    pub test: ProfileSettings,
    pub bench: ProfileSettings,
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            dev: ProfileSettings {
                opt_level: 0,
                debug: true,
                overflow_checks: true,
                lto: false,
                codegen_units: 16,
                strip: false,
            },
            release: ProfileSettings {
                opt_level: 3,
                debug: false,
                overflow_checks: false,
                lto: true,
                codegen_units: 1,
                strip: true,
            },
            test: ProfileSettings {
                opt_level: 0,
                debug: true,
                overflow_checks: true,
                lto: false,
                codegen_units: 16,
                strip: false,
            },
            bench: ProfileSettings {
                opt_level: 3,
                debug: false,
                overflow_checks: false,
                lto: true,
                codegen_units: 1,
                strip: true,
            },
        }
    }
}

/// Profile 设置
#[derive(Debug, Clone)]
pub struct ProfileSettings {
    pub opt_level: u8,
    pub debug: bool,
    pub overflow_checks: bool,
    pub lto: bool,
    pub codegen_units: u32,
    pub strip: bool,
}

/// 绑定配置
#[derive(Debug, Clone)]
pub struct BindingsConfig {
    pub export_name: String,
    pub target_languages: Vec<String>,
    pub output_dir: String,
}

/// 工作区配置
#[derive(Debug, Clone)]
pub struct WorkspaceConfig {
    pub members: Vec<String>,
    pub exclude: Vec<String>,
}

/// 获取 CPU 核心数
fn num_cpus() -> u32 {
    std::thread::available_parallelism()
        .map(|p| p.get() as u32)
        .unwrap_or(1)
}

impl PackageConfig {
    /// 从文件加载配置
    pub fn from_file<P: AsRef<Path>>(path: P) -> CliResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| CliError::Config(format!("无法读取配置文件：{}", e)))?;
        Self::from_str(&content)
    }

    /// 从字符串解析配置
    pub fn from_str(s: &str) -> CliResult<Self> {
        // 简化实现：使用基本的 TOML 解析
        let mut config = PackageConfig::default();
        
        for line in s.lines() {
            let line = line.trim();
            
            // 解析 package 部分
            if line.starts_with("name = ") {
                config.package.name = line.trim_start_matches("name = ")
                    .trim_matches('"')
                    .to_string();
            } else if line.starts_with("version = ") {
                config.package.version = line.trim_start_matches("version = ")
                    .trim_matches('"')
                    .to_string();
            } else if line.starts_with("edition = ") {
                config.package.edition = line.trim_start_matches("edition = ")
                    .trim_matches('"')
                    .to_string();
            }
            // 继续解析其他字段...
        }

        Ok(config)
    }

    /// 获取默认配置
    pub fn default_config() -> Self {
        Self {
            package: PackageInfo {
                name: String::new(),
                version: "0.1.0".to_string(),
                edition: "1.2".to_string(),
                authors: Vec::new(),
                description: String::new(),
                license: String::new(),
                repository: None,
                documentation: None,
                homepage: None,
                keywords: Vec::new(),
                categories: Vec::new(),
                readme: None,
                keyword_style: KeywordStyle::default(),
            },
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
            build_dependencies: HashMap::new(),
            extern_libs: ExternLibs::default(),
            lib: None,
            bins: Vec::new(),
            examples: Vec::new(),
            tests: Vec::new(),
            benches: Vec::new(),
            build: BuildConfig::default(),
            profile: ProfileConfig::default(),
            bindings: None,
            workspace: None,
        }
    }

    /// 保存配置到文件
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> CliResult<()> {
        let content = self.to_string();
        std::fs::write(path, content)
            .map_err(|e| CliError::Io(format!("无法写入配置文件：{}", e)))?;
        Ok(())
    }

    /// 转换为 TOML 字符串
    pub fn to_string(&self) -> String {
        let mut s = String::new();
        
        s.push_str("[package]\n");
        s.push_str(&format!("name = \"{}\"\n", self.package.name));
        s.push_str(&format!("version = \"{}\"\n", self.package.version));
        s.push_str(&format!("edition = \"{}\"\n", self.package.edition));
        
        for author in &self.package.authors {
            s.push_str(&format!("authors = [\"{}\"]\n", author));
        }
        
        if !self.package.description.is_empty() {
            s.push_str(&format!("description = \"{}\"\n", self.package.description));
        }
        
        if !self.package.license.is_empty() {
            s.push_str(&format!("license = \"{}\"\n", self.package.license));
        }
        
        s.push_str("\n[dependencies]\n");
        for (name, dep) in &self.dependencies {
            match dep {
                Dependency::Version(v) => s.push_str(&format!("{} = \"{}\"\n", name, v)),
                Dependency::Path(p) => s.push_str(&format!("{} = {{ path = \"{}\" }}\n", name, p)),
                Dependency::Git { url, .. } => s.push_str(&format!("{} = {{ git = \"{}\" }}\n", name, url)),
                Dependency::Optional(_) => {}
            }
        }
        
        s
    }
}

impl Default for PackageConfig {
    fn default() -> Self {
        Self::default_config()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = PackageConfig::default_config();
        assert_eq!(config.package.version, "0.1.0");
        assert_eq!(config.build.opt_level, 2);
    }

    #[test]
    fn test_dependency_parsing() {
        let dep = Dependency::parse("\"1.0\"").unwrap();
        assert!(matches!(dep, Dependency::Version(_)));
        
        let dep = Dependency::parse("path = \"../local\"").unwrap();
        assert!(matches!(dep, Dependency::Path(_)));
    }

    #[test]
    fn test_keyword_style() {
        assert_eq!(KeywordStyle::default(), KeywordStyle::Chinese);
    }
}

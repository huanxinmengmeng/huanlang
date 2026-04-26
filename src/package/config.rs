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

//! 包管理器配置模块

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// 包管理器配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageConfig {
    /// 注册表配置
    pub registry: Option<RegistryConfig>,
    /// 缓存配置
    pub cache: Option<CacheConfig>,
    /// 网络配置
    pub network: Option<NetworkConfig>,
    /// 构建配置
    pub build: Option<BuildConfig>,
    /// 安全配置
    pub security: Option<SecurityConfig>,
}

/// 注册表配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
    /// 默认注册表 URL
    pub default: Option<String>,
    /// 注册表映射
    pub registries: Option<std::collections::HashMap<String, RegistryInfo>>,
}

/// 注册表信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryInfo {
    /// 注册表 URL
    pub url: String,
    /// API 版本
    pub api_version: Option<String>,
    /// 身份验证信息
    pub auth: Option<AuthInfo>,
}

/// 身份验证信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthInfo {
    /// 用户名
    pub username: Option<String>,
    /// 密码
    pub password: Option<String>,
    /// 令牌
    pub token: Option<String>,
}

/// 缓存配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheConfig {
    /// 缓存目录
    pub directory: Option<PathBuf>,
    /// 缓存大小限制（MB）
    pub max_size: Option<u64>,
    /// 缓存过期时间（天）
    pub ttl: Option<u64>,
}

/// 网络配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    /// 代理设置
    pub proxy: Option<String>,
    /// 超时设置（秒）
    pub timeout: Option<u64>,
    /// 重试次数
    pub retries: Option<u32>,
    /// 启用 IPv6
    pub ipv6: Option<bool>,
}

/// 构建配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    /// 构建并行度
    pub jobs: Option<u32>,
    /// 构建目录
    pub target_dir: Option<PathBuf>,
    /// 启用增量构建
    pub incremental: Option<bool>,
    /// 启用调试信息
    pub debug: Option<bool>,
}

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    /// 启用包签名验证
    pub verify_signatures: Option<bool>,
    /// 启用安全审计
    pub audit: Option<bool>,
    /// 漏洞数据库 URL
    pub vulnerability_db: Option<String>,
    /// 安全评级阈值
    pub security_threshold: Option<String>,
}

impl PackageConfig {
    /// 创建默认配置
    pub fn default() -> Self {
        Self {
            registry: None,
            cache: None,
            network: None,
            build: None,
            security: None,
        }
    }

    /// 从文件加载配置
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    /// 保存配置到文件
    pub fn save_to_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

//! 注册表API客户端

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use crate::package::error::{PackageError, PackageResult};
use crate::package::manifest::PackageManifest;

/// 注册表客户端
pub struct RegistryClient {
    base_url: String,
    api_key: Option<String>,
    user_agent: String,
}

/// 包信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub documentation: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub dependencies: Option<HashMap<String, String>>,
    pub dev_dependencies: Option<HashMap<String, String>>,
    pub build_dependencies: Option<HashMap<String, String>>,
    pub features: Option<HashMap<String, Vec<String>>>,
    pub downloads: u64,
    pub created_at: String,
    pub updated_at: String,
    pub published_at: String,
}

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub packages: Vec<PackageInfo>,
    pub total: u32,
    pub page: u32,
    pub per_page: u32,
}

/// 版本列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionList {
    pub versions: Vec<String>,
    pub latest: String,
}

/// 上传请求
#[derive(Debug, Clone, Serialize)]
pub struct UploadRequest {
    pub package: PackageManifest,
    pub tarball: String, // base64 encoded
}

/// 认证响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires_at: String,
    pub user: UserInfo,
}

/// 用户信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: String,
    pub username: String,
    pub email: String,
    pub name: Option<String>,
    pub avatar: Option<String>,
}

/// 错误响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    pub details: Option<serde::Value>,
}

impl RegistryClient {
    /// 创建新的客户端
    pub fn new(base_url: &str, api_key: Option<&str>) -> Self {
        RegistryClient {
            base_url: base_url.to_string(),
            api_key: api_key.map(|k| k.to_string()),
            user_agent: format!("huanlang-package-manager/1.0.0"),
        }
    }

    /// 获取包信息
    pub fn get_package(&self, name: &str) -> PackageResult<PackageInfo> {
        let url = format!("{}/api/packages/{}", self.base_url, name);
        self.make_request::<PackageInfo>(&url, "GET")
    }

    /// 获取包版本
    pub fn get_package_version(&self, name: &str, version: &str) -> PackageResult<PackageInfo> {
        let url = format!("{}/api/packages/{}/versions/{}", self.base_url, name, version);
        self.make_request::<PackageInfo>(&url, "GET")
    }

    /// 获取包的所有版本
    pub fn get_versions(&self, name: &str) -> PackageResult<VersionList> {
        let url = format!("{}/api/packages/{}/versions", self.base_url, name);
        self.make_request::<VersionList>(&url, "GET")
    }

    /// 搜索包
    pub fn search(&self, query: &str, page: u32, per_page: u32) -> PackageResult<SearchResult> {
        let url = format!("{}/api/packages/search?q={}&page={}&per_page={}", 
            self.base_url, query, page, per_page);
        self.make_request::<SearchResult>(&url, "GET")
    }

    /// 上传包
    pub fn upload(&self, manifest: &PackageManifest, tarball_path: &str) -> PackageResult<PackageInfo> {
        let url = format!("{}/api/packages", self.base_url);
        
        // 读取tarball文件
        let mut file = File::open(tarball_path)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(tarball_path)))?;
        let mut tarball = Vec::new();
        file.read_to_end(&mut tarball)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(tarball_path)))?;
        
        let tarball_base64 = base64::encode(&tarball);
        
        let request = UploadRequest {
            package: manifest.clone(),
            tarball: tarball_base64,
        };
        
        self.make_request_with_body::<UploadRequest, PackageInfo>(&url, "POST", &request)
    }

    /// 删除包版本
    pub fn delete_version(&self, name: &str, version: &str) -> PackageResult<()> {
        let url = format!("{}/api/packages/{}/versions/{}", self.base_url, name, version);
        self.make_request::<()>(&url, "DELETE")
    }

    /// 认证
    pub fn authenticate(&self, username: &str, password: &str) -> PackageResult<AuthResponse> {
        let url = format!("{}/api/auth/login", self.base_url);
        
        let credentials = serde_json::json!({
            "username": username,
            "password": password
        });
        
        self.make_request_with_json::<AuthResponse>(&url, "POST", &credentials)
    }

    /// 刷新令牌
    pub fn refresh_token(&self, refresh_token: &str) -> PackageResult<AuthResponse> {
        let url = format!("{}/api/auth/refresh", self.base_url);
        
        let request = serde_json::json!({
            "refresh_token": refresh_token
        });
        
        self.make_request_with_json::<AuthResponse>(&url, "POST", &request)
    }

    /// 获取用户信息
    pub fn get_user_info(&self) -> PackageResult<UserInfo> {
        let url = format!("{}/api/user", self.base_url);
        self.make_request::<UserInfo>(&url, "GET")
    }

    /// 列出用户包
    pub fn list_user_packages(&self, username: &str) -> PackageResult<Vec<PackageInfo>> {
        let url = format!("{}/api/users/{}/packages", self.base_url, username);
        self.make_request::<Vec<PackageInfo>>(&url, "GET")
    }

    /// 检查包是否存在
    pub fn package_exists(&self, name: &str) -> PackageResult<bool> {
        let url = format!("{}/api/packages/{}", self.base_url, name);
        match self.make_request::<PackageInfo>(&url, "GET") {
            Ok(_) => Ok(true),
            Err(e) if matches!(e, PackageError::RegistryError { code: Some(404), .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// 检查版本是否存在
    pub fn version_exists(&self, name: &str, version: &str) -> PackageResult<bool> {
        let url = format!("{}/api/packages/{}/versions/{}", self.base_url, name, version);
        match self.make_request::<PackageInfo>(&url, "GET") {
            Ok(_) => Ok(true),
            Err(e) if matches!(e, PackageError::RegistryError { code: Some(404), .. }) => Ok(false),
            Err(e) => Err(e),
        }
    }

    /// 私有方法：发送请求
    fn make_request<T: for<'de> Deserialize<'de>>(&self, url: &str, method: &str) -> PackageResult<T> {
        let client = reqwest::blocking::Client::new();
        let mut request = client.request(reqwest::Method::from_bytes(method.as_bytes()).unwrap(), url);
        
        // 添加请求头
        request = request
            .header("User-Agent", &self.user_agent)
            .header("Accept", "application/json");
        
        // 添加认证头
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        // 发送请求
        let response = request.send()
            .map_err(|e| PackageError::network_error(&e.to_string(), Some(url)))?;
        
        // 检查响应状态
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error: ErrorResponse = response.json()
                .unwrap_or_else(|_| ErrorResponse {
                    code: status,
                    message: "未知错误".to_string(),
                    details: None,
                });
            return Err(PackageError::registry_error(&error.message, Some(status)));
        }
        
        // 解析响应
        let result = response.json()
            .map_err(|e| PackageError::registry_error(&e.to_string(), None))?;
        
        Ok(result)
    }

    /// 私有方法：发送带JSON体的请求
    fn make_request_with_json<T: for<'de> Deserialize<'de>>(&self, url: &str, method: &str, body: &serde_json::Value) -> PackageResult<T> {
        let client = reqwest::blocking::Client::new();
        let mut request = client.request(reqwest::Method::from_bytes(method.as_bytes()).unwrap(), url);
        
        // 添加请求头
        request = request
            .header("User-Agent", &self.user_agent)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");
        
        // 添加认证头
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        // 添加请求体
        request = request.json(body);
        
        // 发送请求
        let response = request.send()
            .map_err(|e| PackageError::network_error(&e.to_string(), Some(url)))?;
        
        // 检查响应状态
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error: ErrorResponse = response.json()
                .unwrap_or_else(|_| ErrorResponse {
                    code: status,
                    message: "未知错误".to_string(),
                    details: None,
                });
            return Err(PackageError::registry_error(&error.message, Some(status)));
        }
        
        // 解析响应
        let result = response.json()
            .map_err(|e| PackageError::registry_error(&e.to_string(), None))?;
        
        Ok(result)
    }

    /// 私有方法：发送带结构体体的请求
    fn make_request_with_body<B: Serialize, T: for<'de> Deserialize<'de>>(&self, url: &str, method: &str, body: &B) -> PackageResult<T> {
        let client = reqwest::blocking::Client::new();
        let mut request = client.request(reqwest::Method::from_bytes(method.as_bytes()).unwrap(), url);
        
        // 添加请求头
        request = request
            .header("User-Agent", &self.user_agent)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json");
        
        // 添加认证头
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        // 添加请求体
        request = request.json(body);
        
        // 发送请求
        let response = request.send()
            .map_err(|e| PackageError::network_error(&e.to_string(), Some(url)))?;
        
        // 检查响应状态
        if !response.status().is_success() {
            let status = response.status().as_u16();
            let error: ErrorResponse = response.json()
                .unwrap_or_else(|_| ErrorResponse {
                    code: status,
                    message: "未知错误".to_string(),
                    details: None,
                });
            return Err(PackageError::registry_error(&error.message, Some(status)));
        }
        
        // 解析响应
        let result = response.json()
            .map_err(|e| PackageError::registry_error(&e.to_string(), None))?;
        
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry_client() {
        // 创建客户端
        let client = RegistryClient::new("https://registry.huanlang.org", None);
        
        // 测试搜索
        let result = client.search("test", 1, 10);
        assert!(result.is_ok());
        
        // 测试包是否存在
        let exists = client.package_exists("test");
        assert!(exists.is_ok());
    }

    #[test]
    fn test_authentication() {
        // 创建客户端
        let client = RegistryClient::new("https://registry.huanlang.org", None);
        
        // 测试认证（模拟）
        let result = client.authenticate("test", "test");
        assert!(result.is_err()); // 应该失败，因为是模拟测试
    }
}

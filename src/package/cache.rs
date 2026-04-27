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

//! 缓存管理模块

use std::path::Path;
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::package::error::{PackageError, PackageResult};
use crate::package::resolver::Version;

/// 缓存管理器
pub struct CacheManager {
    cache_dir: String,
    index_dir: String,
    package_dir: String,
    #[allow(dead_code)]
    temp_dir: String,
}

/// 缓存条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheEntry {
    pub name: String,
    pub version: String,
    pub checksum: String,
    pub size: u64,
    pub timestamp: u64,
    pub source: String,
    pub dependencies: Option<HashMap<String, String>>,
}

/// 索引条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub name: String,
    pub versions: Vec<String>,
    pub latest: String,
    pub description: Option<String>,
    pub authors: Option<Vec<String>>,
    pub license: Option<String>,
    pub repository: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub downloads: u64,
    pub last_updated: u64,
}

impl CacheManager {
    /// 创建新的缓存管理器
    pub fn new() -> PackageResult<Self> {
        let cache_dir = Self::get_cache_dir()?;
        let index_dir = format!("{}/index", cache_dir);
        let package_dir = format!("{}/packages", cache_dir);
        let temp_dir = format!("{}/temp", cache_dir);
        
        // 创建目录结构
        fs::create_dir_all(&index_dir)?;
        fs::create_dir_all(&package_dir)?;
        fs::create_dir_all(&temp_dir)?;
        
        Ok(CacheManager {
            cache_dir,
            index_dir,
            package_dir,
            temp_dir,
        })
    }

    /// 获取缓存目录
    fn get_cache_dir() -> PackageResult<String> {
        let home_dir = dirs::home_dir()
            .ok_or_else(|| PackageError::io_error("无法获取主目录", None))?;
        
        let cache_dir = home_dir.join(".huanlang").join("cache");
        Ok(cache_dir.to_str().unwrap().to_string())
    }

    /// 缓存包
    pub fn cache_package(&self, name: &str, version: &Version, content: &[u8]) -> PackageResult<String> {
        let package_path = self.get_package_path(name, version);
        
        // 创建包目录
        let package_dir = Path::new(&package_path).parent().unwrap();
        fs::create_dir_all(package_dir)?;
        
        // 写入包文件
        fs::write(&package_path, content)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(&package_path)))?;
        
        // 计算校验和
        let checksum = Self::calculate_checksum(content);
        
        // 更新缓存条目
        self.update_cache_entry(name, version, &checksum, content.len() as u64)?;
        
        Ok(package_path)
    }

    /// 从缓存获取包
    pub fn get_package(&self, name: &str, version: &Version) -> PackageResult<Vec<u8>> {
        let package_path = self.get_package_path(name, version);
        
        if !Path::new(&package_path).exists() {
            return Err(PackageError::package_error(
                &format!("包 {} v{} 不在缓存中", name, version.to_string()),
                Some(name)
            ));
        }
        
        fs::read(&package_path)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(&package_path)))
    }

    /// 检查包是否在缓存中
    pub fn has_package(&self, name: &str, version: &Version) -> bool {
        let package_path = self.get_package_path(name, version);
        Path::new(&package_path).exists()
    }

    /// 移除缓存中的包
    pub fn remove_package(&self, name: &str, version: &Version) -> PackageResult<()> {
        let package_path = self.get_package_path(name, version);
        
        if Path::new(&package_path).exists() {
            fs::remove_file(&package_path)
                .map_err(|e| PackageError::io_error(&e.to_string(), Some(&package_path)))?;
        }
        
        // 移除缓存条目
        self.remove_cache_entry(name, version)?;
        
        Ok(())
    }

    /// 清理过期缓存
    pub fn clean(&self, days: u32) -> PackageResult<u32> {
        let cutoff = chrono::Utc::now().timestamp() - (days * 24 * 60 * 60) as i64;
        let mut removed = 0;
        
        // 遍历所有包目录
        if let Ok(entries) = fs::read_dir(&self.package_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        let _package_name = path.file_name().unwrap().to_str().unwrap();
                        
                        // 遍历版本目录
                        if let Ok(version_entries) = fs::read_dir(&path) {
                            for version_entry in version_entries {
                                if let Ok(version_entry) = version_entry {
                                    let version_path = version_entry.path();
                                    if version_path.is_file() {
                                        // 检查修改时间
                                        let metadata = fs::metadata(&version_path)?;
                                        let modified = metadata.modified()?.duration_since(std::time::UNIX_EPOCH)?.as_secs() as i64;
                                        
                                        if modified < cutoff {
                                            fs::remove_file(&version_path)?;
                                            removed += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(removed)
    }

    /// 缓存索引
    pub fn cache_index(&self, entry: &IndexEntry) -> PackageResult<()> {
        let index_path = self.get_index_path(&entry.name);
        
        // 写入索引文件
        let content = serde_json::to_string_pretty(entry)
            .map_err(|e| PackageError::config_error(&e.to_string(), None))?;
        
        fs::write(&index_path, content)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(&index_path)))?;
        
        Ok(())
    }

    /// 获取索引
    pub fn get_index(&self, name: &str) -> PackageResult<Option<IndexEntry>> {
        let index_path = self.get_index_path(name);
        
        if !Path::new(&index_path).exists() {
            return Ok(None);
        }
        
        let content = fs::read_to_string(&index_path)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(&index_path)))?;
        
        let entry: IndexEntry = serde_json::from_str(&content)
            .map_err(|e| PackageError::config_error(&e.to_string(), Some(&index_path)))?;
        
        Ok(Some(entry))
    }

    /// 搜索本地索引
    pub fn search_index(&self, query: &str) -> PackageResult<Vec<IndexEntry>> {
        let mut results = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.index_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        let content = fs::read_to_string(&path)
                            .map_err(|e| PackageError::io_error(&e.to_string(), Some(path.to_str().unwrap())))?;
                        
                        let index_entry: IndexEntry = serde_json::from_str(&content)
                            .map_err(|e| PackageError::config_error(&e.to_string(), Some(path.to_str().unwrap())))?;
                        
                        // 检查是否匹配查询
                        if index_entry.name.contains(query) || 
                           index_entry.description.as_ref().map(|d| d.contains(query)).unwrap_or(false) {
                            results.push(index_entry);
                        }
                    }
                }
            }
        }
        
        Ok(results)
    }

    /// 获取包路径
    fn get_package_path(&self, name: &str, version: &Version) -> String {
        format!("{}/{}/{}.tar.gz", self.package_dir, name, version.to_string())
    }

    /// 获取索引路径
    fn get_index_path(&self, name: &str) -> String {
        format!("{}/{}.json", self.index_dir, name)
    }

    /// 获取缓存条目路径
    fn get_cache_entry_path(&self, name: &str) -> String {
        format!("{}/{}.json", self.cache_dir, name)
    }

    /// 更新缓存条目
    fn update_cache_entry(&self, name: &str, version: &Version, checksum: &str, size: u64) -> PackageResult<()> {
        let entry_path = self.get_cache_entry_path(name);
        
        let mut entries: HashMap<String, CacheEntry> = if Path::new(&entry_path).exists() {
            let content = fs::read_to_string(&entry_path)
                .map_err(|e| PackageError::io_error(&e.to_string(), Some(&entry_path)))?;
            serde_json::from_str(&content)
                .map_err(|e| PackageError::config_error(&e.to_string(), Some(&entry_path)))?
        } else {
            HashMap::new()
        };
        
        entries.insert(version.to_string(), CacheEntry {
            name: name.to_string(),
            version: version.to_string(),
            checksum: checksum.to_string(),
            size,
            timestamp: chrono::Utc::now().timestamp() as u64,
            source: "registry".to_string(),
            dependencies: None,
        });
        
        let content = serde_json::to_string_pretty(&entries)
            .map_err(|e| PackageError::config_error(&e.to_string(), None))?;
        
        fs::write(&entry_path, content)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(&entry_path)))?;
        
        Ok(())
    }

    /// 移除缓存条目
    fn remove_cache_entry(&self, name: &str, version: &Version) -> PackageResult<()> {
        let entry_path = self.get_cache_entry_path(name);
        
        if Path::new(&entry_path).exists() {
            let mut entries: HashMap<String, CacheEntry> = {
                let content = fs::read_to_string(&entry_path)
                    .map_err(|e| PackageError::io_error(&e.to_string(), Some(&entry_path)))?;
                serde_json::from_str(&content)
                    .map_err(|e| PackageError::config_error(&e.to_string(), Some(&entry_path)))?
            };
            
            entries.remove(&version.to_string());
            
            if entries.is_empty() {
                fs::remove_file(&entry_path)
                    .map_err(|e| PackageError::io_error(&e.to_string(), Some(&entry_path)))?;
            } else {
                let content = serde_json::to_string_pretty(&entries)
                    .map_err(|e| PackageError::config_error(&e.to_string(), None))?;
                
                fs::write(&entry_path, content)
                    .map_err(|e| PackageError::io_error(&e.to_string(), Some(&entry_path)))?;
            }
        }
        
        Ok(())
    }

    /// 计算校验和
    fn calculate_checksum(content: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(content);
        let result = hasher.finalize();
        
        format!("{:x}", result)
    }

    /// 获取缓存大小
    pub fn get_cache_size(&self) -> PackageResult<u64> {
        let mut size = 0;
        
        if let Ok(entries) = fs::read_dir(&self.package_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_file() {
                        let metadata = fs::metadata(&path)?;
                        size += metadata.len();
                    }
                }
            }
        }
        
        Ok(size)
    }

    /// 列出缓存中的包
    pub fn list_packages(&self) -> PackageResult<Vec<(String, String)>> {
        let mut packages = Vec::new();
        
        if let Ok(entries) = fs::read_dir(&self.package_dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.is_dir() {
                        let package_name = path.file_name().unwrap().to_str().unwrap();
                        
                        if let Ok(version_entries) = fs::read_dir(&path) {
                            for version_entry in version_entries {
                                if let Ok(version_entry) = version_entry {
                                    let version_path = version_entry.path();
                                    if version_path.is_file() {
                                        let version = version_path.file_name().unwrap().to_str().unwrap()
                                            .trim_end_matches(".tar.gz");
                                        packages.push((package_name.to_string(), version.to_string()));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(packages)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_manager() {
        let cache = CacheManager::new().unwrap();
        
        // 测试缓存包
        let version = Version::parse("1.0.0").unwrap();
        let content = b"test content";
        
        let path = cache.cache_package("test", &version, content).unwrap();
        assert!(Path::new(&path).exists());
        
        // 测试获取包
        let retrieved = cache.get_package("test", &version).unwrap();
        assert_eq!(retrieved, content);
        
        // 测试检查包是否存在
        assert!(cache.has_package("test", &version));
        
        // 测试移除包
        cache.remove_package("test", &version).unwrap();
        assert!(!cache.has_package("test", &version));
    }

    #[test]
    fn test_cache_index() {
        let cache = CacheManager::new().unwrap();
        
        // 测试缓存索引
        let entry = IndexEntry {
            name: "test".to_string(),
            versions: vec!["1.0.0".to_string()],
            latest: "1.0.0".to_string(),
            description: Some("Test package".to_string()),
            authors: Some(vec!["Test Author".to_string()]),
            license: Some("MIT".to_string()),
            repository: Some("https://gitee.com/test/test".to_string()),
            keywords: Some(vec!["test".to_string()]),
            categories: Some(vec!["utilities".to_string()]),
            downloads: 100,
            last_updated: chrono::Utc::now().timestamp() as u64,
        };
        
        cache.cache_index(&entry).unwrap();
        
        // 测试获取索引
        let retrieved = cache.get_index("test").unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name, "test");
    }

    #[test]
    fn test_clean() {
        let cache = CacheManager::new().unwrap();
        
        // 测试清理
        let removed = cache.clean(30).unwrap();
        assert!(removed >= 0);
    }
}

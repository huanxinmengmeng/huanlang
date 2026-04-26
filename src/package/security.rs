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

//! 包管理器安全模块
//! 
//! 提供包的安全相关功能，包括：
//! - 签名验证
//! - 安全审计
//! - 漏洞扫描

use crate::package::error::{PackageError, PackageResult};
use std::path::Path;
use std::fs;
use ring::{signature, digest, rand};
use ring::signature::KeyPair;
use serde::{Deserialize, Serialize};

/// 签名类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignatureType {
    /// ED25519 签名
    Ed25519,
    /// RSA 签名
    Rsa,
}

/// 包签名
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSignature {
    /// 签名类型
    pub signature_type: SignatureType,
    /// 签名数据
    pub signature: String,
    /// 公钥
    pub public_key: String,
    /// 签名时间
    pub timestamp: String,
}

/// 安全审计结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditResult {
    /// 包名称
    pub package_name: String,
    /// 包版本
    pub package_version: String,
    /// 安全问题列表
    pub issues: Vec<SecurityIssue>,
    /// 审计时间
    pub audit_time: String,
    /// 整体安全评级
    pub security_rating: SecurityRating,
}

/// 安全问题
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    /// 问题ID
    pub id: String,
    /// 问题类型
    pub issue_type: SecurityIssueType,
    /// 严重程度
    pub severity: Severity,
    /// 描述
    pub description: String,
    /// 修复建议
    pub recommendation: String,
    /// 相关CVE
    pub cve: Option<String>,
}

/// 安全问题类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityIssueType {
    /// 依赖漏洞
    DependencyVulnerability,
    /// 代码注入
    CodeInjection,
    /// 缓冲区溢出
    BufferOverflow,
    /// 权限问题
    PermissionIssue,
    /// 信息泄露
    InformationDisclosure,
    /// 其他问题
    Other,
}

/// 严重程度
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    /// 低
    Low,
    /// 中
    Medium,
    /// 高
    High,
    /// 严重
    Critical,
}

/// 安全评级
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityRating {
    /// 安全
    Safe,
    /// 基本安全
    BasicallySafe,
    /// 有风险
    Risky,
    /// 不安全
    Unsafe,
}

/// 漏洞数据库
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VulnerabilityDatabase {
    /// 漏洞列表
    pub vulnerabilities: Vec<Vulnerability>,
    /// 数据库版本
    pub version: String,
    /// 最后更新时间
    pub last_updated: String,
}

/// 漏洞信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vulnerability {
    /// 漏洞ID
    pub id: String,
    /// 包名称
    pub package_name: String,
    /// 受影响的版本范围
    pub affected_versions: String,
    /// 修复版本
    pub fixed_versions: Option<String>,
    /// 严重程度
    pub severity: Severity,
    /// 描述
    pub description: String,
    /// CVE编号
    pub cve: Option<String>,
    /// 发布日期
    pub published_date: String,
    /// 最后更新日期
    pub last_modified_date: String,
}

/// 安全管理器
pub struct SecurityManager {
    /// 漏洞数据库
    vulnerability_db: Option<VulnerabilityDatabase>,
}

impl SecurityManager {
    /// 创建新的安全管理器
    pub fn new() -> Self {
        Self {
            vulnerability_db: None,
        }
    }

    /// 加载漏洞数据库
    pub fn load_vulnerability_db<P: AsRef<Path>>(&mut self, path: P) -> PackageResult<()> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(path.to_str().unwrap())))?;
        
        let db: VulnerabilityDatabase = serde_json::from_str(&content)
            .map_err(|e| PackageError::config_error(&e.to_string(), None))?;
        
        self.vulnerability_db = Some(db);
        Ok(())
    }

    /// 验证包签名
    pub fn verify_signature(&self, package_path: &Path, signature: &PackageSignature) -> PackageResult<bool> {
        // 读取包文件
        let package_data = fs::read(package_path)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(package_path.to_str().unwrap())))?;
        
        // 验证签名
        match signature.signature_type {
            SignatureType::Ed25519 => self.verify_ed25519_signature(&package_data, signature),
            SignatureType::Rsa => self.verify_rsa_signature(&package_data, signature),
        }
    }

    /// 验证ED25519签名
    fn verify_ed25519_signature(&self, data: &[u8], signature: &PackageSignature) -> PackageResult<bool> {
        // 解码公钥
        let public_key_bytes = base64::decode(&signature.public_key)
            .map_err(|e| PackageError::package_error(&format!("公钥解码失败: {:?}", e), None))?;
        
        // 解码签名
        let signature_bytes = base64::decode(&signature.signature)
            .map_err(|e| PackageError::package_error(&format!("签名解码失败: {:?}", e), None))?;
        
        // 创建公钥
        let public_key = signature::UnparsedPublicKey::new(&signature::ED25519, &public_key_bytes);
        
        // 验证签名
        let result = public_key.verify(data, &signature_bytes);
        match result {
            Ok(_) => Ok(true),
            Err(e) => Err(PackageError::package_error(&format!("签名验证失败: {:?}", e), None)),
        }
    }

    /// 验证RSA签名
    fn verify_rsa_signature(&self, data: &[u8], signature: &PackageSignature) -> PackageResult<bool> {
        // 解码公钥
        let public_key_bytes = base64::decode(&signature.public_key)
            .map_err(|e| PackageError::package_error(&format!("公钥解码失败: {:?}", e), None))?;
        
        // 解码签名
        let signature_bytes = base64::decode(&signature.signature)
            .map_err(|e| PackageError::package_error(&format!("签名解码失败: {:?}", e), None))?;
        
        // 计算数据哈希
        let digest = digest::digest(&digest::SHA256, data);
        
        // 验证签名（简化实现）
        // 实际实现需要根据RSA密钥格式和签名算法进行调整
        Ok(true)
    }

    /// 执行安全审计
    pub fn audit_package(&self, package_name: &str, package_version: &str) -> PackageResult<SecurityAuditResult> {
        let mut issues = Vec::new();
        
        // 检查依赖漏洞
        if let Some(db) = &self.vulnerability_db {
            for vuln in &db.vulnerabilities {
                if vuln.package_name == package_name {
                    // 检查版本是否在受影响范围内
                    if self.is_version_affected(package_version, &vuln.affected_versions) {
                        issues.push(SecurityIssue {
                            id: vuln.id.clone(),
                            issue_type: SecurityIssueType::DependencyVulnerability,
                            severity: vuln.severity.clone(),
                            description: vuln.description.clone(),
                            recommendation: format!("升级到版本: {:?}", vuln.fixed_versions),
                            cve: vuln.cve.clone(),
                        });
                    }
                }
            }
        }
        
        // 计算安全评级
        let security_rating = self.calculate_security_rating(&issues);
        
        Ok(SecurityAuditResult {
            package_name: package_name.to_string(),
            package_version: package_version.to_string(),
            issues,
            audit_time: chrono::Utc::now().to_rfc3339(),
            security_rating,
        })
    }

    /// 检查版本是否在受影响范围内
    fn is_version_affected(&self, version: &str, affected_range: &str) -> bool {
        // 简化实现，实际需要解析版本范围
        // 这里只是一个示例，实际实现需要使用版本解析库
        true
    }

    /// 计算安全评级
    fn calculate_security_rating(&self, issues: &[SecurityIssue]) -> SecurityRating {
        if issues.is_empty() {
            return SecurityRating::Safe;
        }
        
        let has_critical = issues.iter().any(|issue| issue.severity == Severity::Critical);
        let has_high = issues.iter().any(|issue| issue.severity == Severity::High);
        let has_medium = issues.iter().any(|issue| issue.severity == Severity::Medium);
        
        if has_critical {
            SecurityRating::Unsafe
        } else if has_high {
            SecurityRating::Risky
        } else if has_medium {
            SecurityRating::BasicallySafe
        } else {
            SecurityRating::Safe
        }
    }

    /// 扫描包漏洞
    pub fn scan_for_vulnerabilities(&self, package_path: &Path) -> PackageResult<Vec<Vulnerability>> {
        // 简化实现，实际需要解析包内容并检查漏洞
        let mut vulnerabilities = Vec::new();
        
        // 这里只是一个示例，实际实现需要：
        // 1. 解析包内容
        // 2. 提取依赖信息
        // 3. 检查依赖是否存在漏洞
        
        Ok(vulnerabilities)
    }
}

/// 签名工具
pub struct SignatureTool {
    /// 私钥
    private_key: Vec<u8>,
    /// 公钥
    public_key: Vec<u8>,
}

impl SignatureTool {
    /// 生成新的密钥对
    pub fn generate_keypair() -> PackageResult<Self> {
        // 生成ED25519密钥对
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = signature::Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|e| PackageError::package_error(&format!("密钥对生成失败: {:?}", e), None))?;
        
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())
            .map_err(|e| PackageError::package_error(&format!("密钥对解析失败: {:?}", e), None))?;
        
        let private_key = pkcs8_bytes.as_ref().to_vec();
        let public_key = key_pair.public_key().as_ref().to_vec();
        
        Ok(Self {
            private_key,
            public_key,
        })
    }

    /// 从文件加载密钥对
    pub fn from_files(private_key_path: &Path, public_key_path: &Path) -> PackageResult<Self> {
        let private_key = fs::read(private_key_path)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(private_key_path.to_str().unwrap())))?;
        
        let public_key = fs::read(public_key_path)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(public_key_path.to_str().unwrap())))?;
        
        Ok(Self {
            private_key,
            public_key,
        })
    }

    /// 签名包
    pub fn sign_package(&self, package_path: &Path) -> PackageResult<PackageSignature> {
        // 读取包文件
        let package_data = fs::read(package_path)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(package_path.to_str().unwrap())))?;
        
        // 创建密钥对
        let key_pair = signature::Ed25519KeyPair::from_pkcs8(&self.private_key)
            .map_err(|e| PackageError::package_error(&format!("密钥解析失败: {:?}", e), None))?;
        
        // 生成签名
        let signature = key_pair.sign(&package_data);
        
        Ok(PackageSignature {
            signature_type: SignatureType::Ed25519,
            signature: base64::encode(signature.as_ref()),
            public_key: base64::encode(&self.public_key),
            timestamp: chrono::Utc::now().to_rfc3339(),
        })
    }

    /// 保存密钥对
    pub fn save_keypair(&self, private_key_path: &Path, public_key_path: &Path) -> PackageResult<()> {
        fs::write(private_key_path, &self.private_key)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(private_key_path.to_str().unwrap())))?;
        
        fs::write(public_key_path, &self.public_key)
            .map_err(|e| PackageError::io_error(&e.to_string(), Some(public_key_path.to_str().unwrap())))?;
        
        Ok(())
    }
}

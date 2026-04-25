// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use sha2::{Sha256, Sha512, Digest};
use hmac::{Hmac, Mac};
use rand;
use crate::stdlib::core::HuanResult;

/// 哈希算法枚举
pub enum 哈希算法 {
    SHA256,
    SHA512,
    SHA3_256,
    SHA3_512,
    BLAKE3,
    MD5,
}

/// 哈希器
pub struct 哈希器 {
    算法: 哈希算法,
    sha256: Option<Sha256>,
    sha512: Option<Sha512>,
}

impl 哈希器 {
    /// 创建新的哈希器
    pub fn 新建(算法: 哈希算法) -> Self {
        match 算法 {
            哈希算法::SHA256 => 哈希器 {
                算法: 算法,
                sha256: Some(Sha256::new()),
                sha512: None,
            },
            哈希算法::SHA512 => 哈希器 {
                算法: 算法,
                sha256: None,
                sha512: Some(Sha512::new()),
            },
            _ => 哈希器 {
                算法: 算法,
                sha256: None,
                sha512: None,
            },
        }
    }
    
    /// 写入字节
    pub fn 写入(&mut self, 数据: &[u8]) {
        match self.算法 {
            哈希算法::SHA256 => {
                if let Some(ref mut hasher) = self.sha256 {
                    hasher.update(数据);
                }
            },
            哈希算法::SHA512 => {
                if let Some(ref mut hasher) = self.sha512 {
                    hasher.update(数据);
                }
            },
            _ => {}
        }
    }
    
    /// 写入字符串
    pub fn 写入字符串(&mut self, 数据: &str) {
        self.写入(数据.as_bytes());
    }
    
    /// 完成哈希计算
    pub fn 完成(&mut self) -> Vec<u8> {
        match self.算法 {
            哈希算法::SHA256 => {
                if let Some(hasher) = self.sha256.take() {
                    hasher.finalize().to_vec()
                } else {
                    Vec::new()
                }
            },
            哈希算法::SHA512 => {
                if let Some(hasher) = self.sha512.take() {
                    hasher.finalize().to_vec()
                } else {
                    Vec::new()
                }
            },
            _ => Vec::new()
        }
    }
    
    /// 完成哈希计算并返回十六进制字符串
    pub fn 完成十六进制(&mut self) -> crate::stdlib::string::字符串 {
        let hash = self.完成();
        let hex: String = hash.iter()
            .map(|byte| format!("{:02x}", byte))
            .collect();
        crate::stdlib::string::字符串::从(&hex)
    }
}

/// 便捷函数：计算哈希
pub fn 哈希(算法: 哈希算法, 数据: &[u8]) -> Vec<u8> {
    let mut 哈希器 = 哈希器::新建(算法);
    哈希器.写入(数据);
    哈希器.完成()
}

/// 便捷函数：计算字符串哈希（返回十六进制）
pub fn 哈希字符串(算法: 哈希算法, 数据: &str) -> crate::stdlib::string::字符串 {
    let mut 哈希器 = 哈希器::新建(算法);
    哈希器.写入字符串(数据);
    哈希器.完成十六进制()
}

/// 快捷函数：SHA256
pub fn sha256(数据: &[u8]) -> Vec<u8> {
    哈希(哈希算法::SHA256, 数据)
}

/// 快捷函数：SHA256 字符串
pub fn sha256字符串(数据: &str) -> crate::stdlib::string::字符串 {
    哈希字符串(哈希算法::SHA256, 数据)
}

/// HMAC 计算
pub fn hmac(算法: 哈希算法, 密钥: &[u8], 数据: &[u8]) -> Vec<u8> {
    // 简化实现 - 仅支持 SHA256
    match 算法 {
        哈希算法::SHA256 => {
            let mut mac = <Hmac<Sha256> as Mac>::new_from_slice(密钥).unwrap();
            mac.update(数据);
            mac.finalize().into_bytes().to_vec()
        },
        _ => Vec::new()
    }
}

/// HMAC 验证
pub fn hmac验证(算法: 哈希算法, 密钥: &[u8], 数据: &[u8], 期望的摘要: &[u8]) -> bool {
    let 计算的摘要 = hmac(算法, 密钥, 数据);
    计算的摘要 == 期望的摘要
}

/// 加密算法枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum 加密算法 {
    Aes128Gcm,
    Aes256Gcm,
    Chacha20Poly1305,
}

use aes_gcm::{Aes256Gcm, KeyInit, aead::{Aead, Nonce}};
use argon2::{Argon2, PasswordHash, PasswordVerifier, PasswordHasher, password_hash::SaltString};
use rand::Rng;

/// 加密函数
pub fn 加密(算法: 加密算法, 密钥: &[u8], 明文: &[u8], 随机数: &[u8]) -> HuanResult<(Vec<u8>, Vec<u8>), &'static str> {
    match 算法 {
        加密算法::Aes256Gcm => {
            if 密钥.len() != 32 {
                return HuanResult::Err("AES256-GCM 需要 32 字节密钥");
            }
            if 随机数.len() != 12 {
                return HuanResult::Err("AES256-GCM 需要 12 字节随机数");
            }
            
            let cipher = Aes256Gcm::new_from_slice(密钥).unwrap();
            let nonce = Nonce::<Aes256Gcm>::from_slice(随机数);
            
            match cipher.encrypt(nonce, 明文.as_ref()) {
                Ok(ciphertext) => {
                    // 生成认证标签（在AES-GCM中，密文已经包含认证标签）
                    let tag = &ciphertext[ciphertext.len() - 16..];
                    let ciphertext_without_tag = &ciphertext[..ciphertext.len() - 16];
                    HuanResult::Ok((ciphertext_without_tag.to_vec(), tag.to_vec()))
                },
                Err(_) => HuanResult::Err("加密失败"),
            }
        },
        _ => HuanResult::Err("不支持的加密算法"),
    }
}

/// 解密函数
pub fn 解密(算法: 加密算法, 密钥: &[u8], 密文: &[u8], 随机数: &[u8], 认证标签: &[u8]) -> HuanResult<Vec<u8>, &'static str> {
    match 算法 {
        加密算法::Aes256Gcm => {
            if 密钥.len() != 32 {
                return HuanResult::Err("AES256-GCM 需要 32 字节密钥");
            }
            if 随机数.len() != 12 {
                return HuanResult::Err("AES256-GCM 需要 12 字节随机数");
            }
            if 认证标签.len() != 16 {
                return HuanResult::Err("AES256-GCM 需要 16 字节认证标签");
            }
            
            let cipher = Aes256Gcm::new_from_slice(密钥).unwrap();
            let nonce = Nonce::<Aes256Gcm>::from_slice(随机数);
            
            // 组合密文和认证标签
            let mut combined = Vec::with_capacity(密文.len() + 认证标签.len());
            combined.extend_from_slice(密文);
            combined.extend_from_slice(认证标签);
            
            match cipher.decrypt(nonce, combined.as_ref()) {
                Ok(plaintext) => HuanResult::Ok(plaintext),
                Err(_) => HuanResult::Err("解密失败"),
            }
        },
        _ => HuanResult::Err("不支持的加密算法"),
    }
}

/// 生成密钥
pub fn 生成密钥(算法: 加密算法) -> Vec<u8> {
    match 算法 {
        加密算法::Aes128Gcm => {
            let mut key = vec![0u8; 16];
            rand::thread_rng().fill(&mut key[..]);
            key
        },
        加密算法::Aes256Gcm => {
            let mut key = vec![0u8; 32];
            rand::thread_rng().fill(&mut key[..]);
            key
        },
        加密算法::Chacha20Poly1305 => {
            let mut key = vec![0u8; 32];
            rand::thread_rng().fill(&mut key[..]);
            key
        },
    }
}

/// 生成随机数
pub fn 生成随机数(_算法: 加密算法) -> Vec<u8> {
    let mut nonce = vec![0u8; 12];
    rand::thread_rng().fill(&mut nonce[..]);
    nonce
}

/// 密码哈希算法枚举
pub enum 密码哈希算法 {
    Argon2id,
    Bcrypt,
    Scrypt,
}

/// 密码哈希配置
pub struct 密码哈希配置 {
    pub 内存成本: usize,
    pub 时间成本: usize,
    pub 并行度: usize,
    pub 盐长度: usize,
    pub 输出长度: usize,
}

impl Default for 密码哈希配置 {
    fn default() -> Self {
        密码哈希配置 {
            内存成本: 19456,
            时间成本: 2,
            并行度: 1,
            盐长度: 16,
            输出长度: 32,
        }
    }
}

/// 密码哈希
pub fn 哈希密码(算法: 密码哈希算法, 密码: &str, _配置: 密码哈希配置) -> HuanResult<crate::stdlib::string::字符串, &'static str> {
    match 算法 {
        密码哈希算法::Argon2id => {
            let salt = SaltString::generate(&mut rand::thread_rng());
            let argon2 = Argon2::default();
            
            match argon2.hash_password(密码.as_bytes(), &salt) {
                Ok(hash) => {
                    HuanResult::Ok(crate::stdlib::string::字符串::从(hash.to_string().as_str()))
                },
                Err(_) => HuanResult::Err("密码哈希失败"),
            }
        },
        _ => HuanResult::Err("不支持的密码哈希算法"),
    }
}

/// 密码验证
pub fn 验证密码(密码: &str, 哈希值: &str) -> HuanResult<bool, &'static str> {
    match PasswordHash::new(哈希值) {
        Ok(parsed_hash) => {
            let argon2 = Argon2::default();
            match argon2.verify_password(密码.as_bytes(), &parsed_hash) {
                Ok(_) => HuanResult::Ok(true),
                Err(_) => HuanResult::Ok(false),
            }
        },
        Err(_) => HuanResult::Err("无效的哈希格式"),
    }
}

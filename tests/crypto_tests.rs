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

use huanlang::stdlib::crypto::*;

#[test]
fn test_hash() {
    // 测试哈希功能
    let data = b"Hello World";
    
    let hash = sha256(data);
    assert_eq!(hash.len(), 32);
    
    let hash_str = sha256字符串("Hello World");
    assert_eq!(hash_str.字节长度(), 64); // 32字节 * 2 (十六进制)
}

#[test]
fn test_hmac() {
    // 测试HMAC功能
    let key = b"secret_key";
    let data = b"Hello World";
    
    let hmac = hmac(哈希算法::SHA256, key, data);
    assert_eq!(hmac.len(), 32);
    
    let valid = hmac验证(哈希算法::SHA256, key, data, &hmac);
    assert!(valid);
}

#[test]
fn test_encryption() {
    // 测试对称加密
    let algorithm = 加密算法::Aes256Gcm;
    let key = 生成密钥(algorithm);
    let nonce = 生成随机数(algorithm);
    let plaintext = b"Hello Encryption";
    
    // 加密
    let (ciphertext, tag) = 加密(algorithm, &key, plaintext, &nonce).unwrap();
    assert!(!ciphertext.is_empty());
    assert_eq!(tag.len(), 16);
    
    // 解密
    let decrypted = 解密(algorithm, &key, &ciphertext, &nonce, &tag).unwrap();
    assert_eq!(&decrypted, plaintext);
}

#[test]
fn test_password_hash() {
    // 测试密码哈希
    let password = "password123";
    let config = 密码哈希配置::default();
    
    // 哈希密码
    let hash = 哈希密码(密码哈希算法::Argon2id, password, config).unwrap();
    assert!(!hash.作为字符串().is_empty());
    
    // 验证密码
    let valid = 验证密码(password, hash.作为字符串()).unwrap();
    assert!(valid);
    
    // 验证错误密码
    let invalid = 验证密码("wrong_password", hash.作为字符串()).unwrap();
    assert!(!invalid);
}

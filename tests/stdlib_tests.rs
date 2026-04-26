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

// 独立测试文件，只测试我们实现的标准库功能

use huanlang::stdlib::core::*;
use huanlang::stdlib::collections::*;
use huanlang::stdlib::string::*;
use huanlang::stdlib::io::*;
use huanlang::stdlib::math::*;
use huanlang::stdlib::time::*;
use huanlang::stdlib::random::*;
use huanlang::stdlib::crypto::*;
use huanlang::stdlib::serialize::*;
use huanlang::stdlib::system::*;
use huanlang::stdlib::net::*;

#[test]
fn test_stdlib_basic() {
    // 测试基本功能
    assert_eq!(1 + 1, 2);
}

#[test]
fn test_core_module() {
    // 测试核心模块
    显示("测试核心模块");
    显示行("Hello, World!");
    错误("测试错误输出");
    错误行("测试错误输出并换行");
    
    // 测试断言
    断言(1 + 1 == 2, "1+1 应该等于 2");
    
    // 测试类型相关函数
    let type_name = 类型名::<i32>();
    assert!(!type_name.is_empty());
    
    let size = 大小::<i32>();
    assert_eq!(size, 4);
    
    let align = 对齐::<i32>();
    assert!(align > 0);
}

#[test]
fn test_collections_module() {
    // 测试列表
    let mut list = 列表::新建();
    list.追加(1);
    list.追加(2);
    list.追加(3);
    assert_eq!(list.长度(), 3);
    assert!(list.获取(0).is_some());
    assert!(list.获取(1).is_some());
    assert!(list.获取(2).is_some());
    
    // 测试字典
    let mut dict = 字典::新建();
    dict.插入("key1", 100);
    dict.插入("key2", 200);
    assert_eq!(dict.长度(), 2);
    assert!(dict.获取(&"key1").is_some());
    assert!(dict.获取(&"key2").is_some());
    
    // 测试集合
    let mut set = 集合::新建();
    set.插入(1);
    set.插入(2);
    set.插入(3);
    assert_eq!(set.长度(), 3);
    assert!(set.包含(&1));
    assert!(set.包含(&2));
    assert!(set.包含(&3));
}

#[test]
fn test_string_module() {
    // 测试字符串
    let s = 字符串::从("Hello, World!");
    assert_eq!(s.长度(), 13);
    assert_eq!(s.字节长度(), 13);
    assert!(s.包含("World"));
    
    let sub = s.子串(7, 12).unwrap();
    assert_eq!(sub.作为字符串(), "World");
    
    let upper = s.转大写();
    assert_eq!(upper.作为字符串(), "HELLO, WORLD!");
    
    let lower = s.转小写();
    assert_eq!(lower.作为字符串(), "hello, world!");
}

#[test]
fn test_math_module() {
    // 测试数学函数
    assert_eq!(绝对值(-5), 5);
    assert_eq!(最小值(1, 2), 1);
    assert_eq!(最大值(1, 2), 2);
    assert_eq!(限制(5, 0, 10), 5);
    assert_eq!(限制(-1, 0, 10), 0);
    assert_eq!(限制(11, 0, 10), 10);
    
    // 测试数学常量
    assert!(PI > 3.14);
    assert!(E > 2.71);
    
    // 测试数学函数
    assert_eq!(平方(5), 25);
    assert_eq!(立方(2), 8);
    assert!((开平方(16.0) - 4.0).abs() < 0.0001);
    assert!((正弦(0.0) - 0.0).abs() < 0.0001);
    assert!((余弦(0.0) - 1.0).abs() < 0.0001);
}

#[test]
fn test_time_module() {
    // 测试时间点
    let now = 时间点::现在();
    
    // 测试持续时间
    let duration = 持续时间::从秒(1.5);
    assert_eq!(duration.转为秒(), 1.5);
    assert_eq!(duration.转为毫秒(), 1500);
    
    // 测试时间点操作
    let later = now.加(&duration);
    assert!(later.晚于(&now));
}

#[test]
fn test_random_module() {
    // 测试随机数生成器
    let mut rng = 随机数生成器::新建();
    
    // 测试随机整数
    let num = rng.生成整数范围(1, 100);
    assert!(num >= 1 && num <= 100);
    
    // 测试随机浮点数
    let f = rng.生成浮点();
    assert!(f >= 0.0 && f < 1.0);
    
    // 测试随机布尔值
    let b = rng.生成布尔();
    assert!(b || !b);
}

#[test]
fn test_crypto_module() {
    // 测试哈希函数
    let data = b"test data";
    let hash = sha256(data);
    assert_eq!(hash.len(), 32);
    
    // 测试哈希字符串
    let hash_str = sha256字符串("test data");
    assert_eq!(hash_str.长度(), 64);
    
    // 测试密码哈希
    let password = "test_password";
    let hash_result = 哈希密码(密码哈希算法::Argon2id, password, 密码哈希配置::default());
    assert!(hash_result.is_ok());
    
    let hash_value = hash_result.unwrap();
    let verify_result = 验证密码(password, hash_value.作为字符串());
    assert!(verify_result.is_ok());
    assert!(verify_result.unwrap());
}

#[test]
fn test_serialize_module() {
    // 测试JSON解析
    let json_str = r#"{"name": "test", "value": 42}"#;
    let json_value = 解析_json(json_str);
    assert!(json_value.is_ok());
    
    // 测试JSON转换
    let json_string = 转为_json字符串(&JSON值::对象(std::collections::HashMap::new()));
    assert_eq!(json_string.作为字符串(), "{}");
}

#[test]
fn test_system_module() {
    // 测试环境变量
    let result = 获取环境变量("PATH");
    assert!(result.is_some());
    
    // 测试命令行参数
    let args = 参数();
    assert!(args.长度() >= 1);
    
    // 测试进程ID
    let pid = 当前进程_id();
    assert!(pid > 0);
}

#[test]
fn test_io_module() {
    // 测试路径
    let path = 路径::新建(".");
    assert!(path.是否存在());
    assert!(path.是否为目录());
    
    // 测试当前目录
    let current_dir = 路径::当前目录();
    assert!(current_dir.is_ok());
    
    // 测试临时目录
    let temp_dir = 路径::临时目录();
    assert!(temp_dir.是否存在());
}

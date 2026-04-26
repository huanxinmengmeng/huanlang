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

// 这是一个示例文件，展示了如何使用幻语标准库

fn main() {
    // 示例1：核心模块的输出函数
    println!("=== 示例1：核心模块输出函数 ===");
    println!("Hello, 幻语!");
    println!("数字：{}", 42);
    println!("浮点数：{}", 3.14);
    println!("布尔值：{}", true);
    println!();
    
    // 示例2：数学模块
    println!("=== 示例2：数学模块 ===");
    let x: f64 = 10.0;
    let y: f64 = 20.0;
    
    println!("绝对值：{}", x.abs());
    println!("最大值：{}", x.max(y));
    println!("最小值：{}", x.min(y));
    println!("平方根：{}", x.sqrt());
    println!("幂运算：{}", x.powf(2.0));
    println!();
    
    // 示例3：集合模块
    println!("=== 示例3：集合模块 ===");
    let mut list = Vec::new();
    list.push(1);
    list.push(2);
    list.push(3);
    
    println!("列表长度：{}", list.len());
    println!("列表内容：{:?}", list);
    
    let mut map = std::collections::HashMap::new();
    map.insert("a".to_string(), 1);
    map.insert("b".to_string(), 2);
    println!("映射内容：{:?}", map);
    println!();
    
    // 示例4：字符串模块
    println!("=== 示例4：字符串模块 ===");
    let s = "Hello, 幻语!";
    println!("字符串内容：{}", s);
    println!("字符串长度（字符数）：{}", s.chars().count());
    println!("字符串长度（字节数）：{}", s.len());
    println!("大写：{}", s.to_uppercase());
    println!("小写：{}", s.to_lowercase());
    println!();
    
    // 示例5：文件系统模块
    println!("=== 示例5：文件系统模块 ===");
    let current_dir = std::env::current_dir().unwrap();
    println!("当前目录：{:?}", current_dir);
    
    let temp_file = std::env::temp_dir().join("huanlang_test.txt");
    println!("临时文件路径：{:?}", temp_file);
    println!();
    
    // 示例6：时间模块
    println!("=== 示例6：时间模块 ===");
    let now = std::time::Instant::now();
    
    println!("等待 1 秒钟...");
    std::thread::sleep(std::time::Duration::from_secs(1));
    
    let elapsed = now.elapsed();
    println!("经过时间：{:?}", elapsed);
    println!();
    
    // 示例7：随机数模块
    println!("=== 示例7：随机数模块 ===");
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    println!("随机整数：{}", rng.gen::<i32>());
    println!("随机浮点数：{}", rng.gen::<f64>());
    println!("1-100 的随机数：{}", rng.gen_range(1..=100));
    println!();
    
    // 示例8：加密模块（哈希）
    println!("=== 示例8：加密模块（哈希）===");
    use sha2::{Sha256, Digest};
    
    let message = "Hello, 幻语!";
    let mut hasher = Sha256::new();
    hasher.update(message.as_bytes());
    let result = hasher.finalize();
    
    println!("消息：{}", message);
    println!("SHA256 哈希：{:x}", result);
    println!();
    
    // 示例9：序列化模块（JSON）
    println!("=== 示例9：序列化模块（JSON）===");
    use serde_json;
    
    let data = serde_json::json!({
        "name": "张三",
        "age": 30,
        "city": "北京",
        "hobbies": ["编程", "读书", "运动"]
    });
    
    println!("JSON 数据：{}", data);
    println!("美化的 JSON：{}", serde_json::to_string_pretty(&data).unwrap());
    println!();
    
    // 示例10：系统模块
    println!("=== 示例10：系统模块 ===");
    let env_vars = std::env::vars_os().take(5).collect::<Vec<_>>();
    println!("部分环境变量：");
    for (key, value) in env_vars {
        println!("  {:?} = {:?}", key, value);
    }
    
    println!("\n所有示例已执行完毕！");
}

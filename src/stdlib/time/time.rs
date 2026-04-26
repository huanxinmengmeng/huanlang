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

/// 持续时间类型
pub struct 持续时间 {
    秒: u64,
    纳秒: u32,
}

impl 持续时间 {
    /// 从秒创建持续时间
    pub fn 从秒(秒数: f64) -> Self {
        let 整数秒 = 秒数.floor() as u64;
        let 纳秒部分 = (秒数.fract() * 1_000_000_000.0) as u32;
        持续时间 { 秒: 整数秒, 纳秒: 纳秒部分 }
    }
    
    /// 从毫秒创建持续时间
    pub fn 从毫秒(毫秒数: i64) -> Self {
        let 秒数 = 毫秒数.abs() as u64 / 1000;
        let 纳秒数 = (毫秒数.abs() as u64 % 1000) * 1_000_000;
        持续时间 { 秒: 秒数, 纳秒: 纳秒数 as u32 }
    }
    
    /// 从微秒创建持续时间
    pub fn 从微秒(微秒数: i64) -> Self {
        let 秒数 = 微秒数.abs() as u64 / 1_000_000;
        let 纳秒数 = (微秒数.abs() as u64 % 1_000_000) * 1000;
        持续时间 { 秒: 秒数, 纳秒: 纳秒数 as u32 }
    }
    
    /// 从纳秒创建持续时间
    pub fn 从纳秒(纳秒数: i64) -> Self {
        let 秒数 = 纳秒数.abs() as u64 / 1_000_000_000;
        let 剩余纳秒 = 纳秒数.abs() as u64 % 1_000_000_000;
        持续时间 { 秒: 秒数, 纳秒: 剩余纳秒 as u32 }
    }
    
    /// 转换为秒
    pub fn 转为秒(&self) -> f64 {
        self.秒 as f64 + self.纳秒 as f64 / 1_000_000_000.0
    }
    
    /// 转换为毫秒
    pub fn 转为毫秒(&self) -> i64 {
        (self.秒 as i64 * 1000) + (self.纳秒 as i64 / 1_000_000)
    }
    
    /// 转换为微秒
    pub fn 转为微秒(&self) -> i64 {
        (self.秒 as i64 * 1_000_000) + (self.纳秒 as i64 / 1000)
    }
    
    /// 转换为纳秒
    pub fn 转为纳秒(&self) -> i64 {
        (self.秒 as i64 * 1_000_000_000) + self.纳秒 as i64
    }
    
    /// 加法
    pub fn 加(&self, 其他: &持续时间) -> 持续时间 {
        let 总纳秒 = self.纳秒 as u64 + 其他.纳秒 as u64;
        let 进位秒 = 总纳秒 / 1_000_000_000;
        let 结果纳秒 = (总纳秒 % 1_000_000_000) as u32;
        持续时间 { 秒: self.秒 + 其他.秒 + 进位秒, 纳秒: 结果纳秒 }
    }
    
    /// 减法
    pub fn 减(&self, 其他: &持续时间) -> 持续时间 {
        let 自总纳秒 = self.秒 * 1_000_000_000 + self.纳秒 as u64;
        let 他总纳秒 = 其他.秒 * 1_000_000_000 + 其他.纳秒 as u64;
        let 结果纳秒 = if 自总纳秒 >= 他总纳秒 { 自总纳秒 - 他总纳秒 } else { 0 };
        持续时间 { 秒: 结果纳秒 / 1_000_000_000, 纳秒: (结果纳秒 % 1_000_000_000) as u32 }
    }
    
    /// 乘法
    pub fn 乘(&self, 因子: f64) -> 持续时间 {
        let 总纳秒 = (self.转为纳秒() as f64 * 因子) as i64;
        持续时间::从纳秒(总纳秒)
    }
    
    /// 除法
    pub fn 除(&self, 因子: f64) -> 持续时间 {
        let 总纳秒 = (self.转为纳秒() as f64 / 因子) as i64;
        持续时间::从纳秒(总纳秒)
    }
    
    /// 大于比较
    pub fn 大于(&self, 其他: &持续时间) -> bool {
        self.转为纳秒() > 其他.转为纳秒()
    }
    
    /// 小于比较
    pub fn 小于(&self, 其他: &持续时间) -> bool {
        self.转为纳秒() < 其他.转为纳秒()
    }
    
    /// 等于比较
    pub fn 等于(&self, 其他: &持续时间) -> bool {
        self.转为纳秒() == 其他.转为纳秒()
    }
}

/// 常用持续时间
pub const 零持续时间: 持续时间 = 持续时间 { 秒: 0, 纳秒: 0 };
pub const 纳秒: 持续时间 = 持续时间 { 秒: 0, 纳秒: 1 };
pub const 微秒: 持续时间 = 持续时间 { 秒: 0, 纳秒: 1000 };
pub const 毫秒: 持续时间 = 持续时间 { 秒: 0, 纳秒: 1_000_000 };
pub const 秒: 持续时间 = 持续时间 { 秒: 1, 纳秒: 0 };
pub const 分钟: 持续时间 = 持续时间 { 秒: 60, 纳秒: 0 };
pub const 小时: 持续时间 = 持续时间 { 秒: 3600, 纳秒: 0 };

/// 时间点类型
pub struct 时间点 {
    时间: std::time::Instant,
}

impl 时间点 {
    /// 获取当前时间
    pub fn 现在() -> Self {
        时间点 { 时间: std::time::Instant::now() }
    }
    
    /// 从 Unix 时间戳创建
    pub fn 从_unix时间戳(_秒数: i64, _纳秒数: i32) -> Self {
        // 简化实现
        时间点 { 时间: std::time::Instant::now() }
    }
    
    /// 加法
    pub fn 加(&self, 持续时间: &持续时间) -> 时间点 {
        let std_duration = std::time::Duration::new(持续时间.秒, 持续时间.纳秒);
        时间点 { 时间: self.时间 + std_duration }
    }
    
    /// 减法（减去持续时间）
    pub fn 减持续时间(&self, 持续时间: &持续时间) -> 时间点 {
        let std_duration = std::time::Duration::new(持续时间.秒, 持续时间.纳秒);
        时间点 { 时间: self.时间 - std_duration }
    }
    
    /// 减法（计算两个时间点的间隔）
    pub fn 减时间点(&self, 其他: &时间点) -> 持续时间 {
        let duration = self.时间.duration_since(其他.时间);
        持续时间 { 秒: duration.as_secs(), 纳秒: duration.subsec_nanos() }
    }
    
    /// 早于比较
    pub fn 早于(&self, 其他: &时间点) -> bool {
        self.时间 < 其他.时间
    }
    
    /// 晚于比较
    pub fn 晚于(&self, 其他: &时间点) -> bool {
        self.时间 > 其他.时间
    }
    
    /// 等于比较
    pub fn 等于(&self, 其他: &时间点) -> bool {
        self.时间 == 其他.时间
    }
    
    /// 转换为 Unix 时间戳
    pub fn 转为_unix时间戳(&self) -> (i64, i32) {
        // 简化实现
        (0, 0)
    }
    
    /// 获取经过的时间
    pub fn 经过时间(&self) -> 持续时间 {
        let duration = self.时间.elapsed();
        持续时间 { 秒: duration.as_secs(), 纳秒: duration.subsec_nanos() }
    }
    
    /// 格式化时间
    pub fn 格式化(&self, 格式: &str) -> crate::stdlib::string::字符串 {

        use chrono::offset::Utc;
        
        let now = Utc::now();
        let formatted = now.format(格式).to_string();
        crate::stdlib::string::字符串::从(&formatted)
    }
    
    /// 从字符串解析时间
    pub fn 解析(字符串: &str, 格式: &str) -> crate::stdlib::core::HuanResult<时间点, &'static str> {

        use chrono::NaiveDateTime;

        match NaiveDateTime::parse_from_str(字符串, 格式) {
            Ok(_) => {
                // 创建时间点（简化实现）
                crate::stdlib::core::HuanResult::Ok(时间点::现在())
            },
            Err(_) => crate::stdlib::core::HuanResult::Err("解析时间失败"),
        }
    }
}

/// 阻塞当前线程睡眠
pub fn 睡眠(持续时间: &持续时间) {
    let std_duration = std::time::Duration::new(持续时间.秒, 持续时间.纳秒);
    std::thread::sleep(std_duration);
}

/// 异步睡眠
pub async fn 异步睡眠(持续时间: &持续时间) {
    let std_duration = std::time::Duration::new(持续时间.秒, 持续时间.纳秒);
    tokio::time::sleep(std_duration).await;
}

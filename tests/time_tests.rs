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

use huanlang::stdlib::time::*;

#[test]
fn test_duration() {
    // 测试持续时间创建
    let d1 = 持续时间::从秒(1.5);
    assert_eq!(d1.转为秒(), 1.5);
    assert_eq!(d1.转为毫秒(), 1500);
    
    let d2 = 持续时间::从毫秒(500);
    assert_eq!(d2.转为秒(), 0.5);
    
    // 测试持续时间运算
    let d3 = d1.加(&d2);
    assert_eq!(d3.转为秒(), 2.0);
    
    let d4 = d1.减(&d2);
    assert_eq!(d4.转为秒(), 1.0);
    
    // 测试比较
    assert!(d1.大于(&d2));
    assert!(d2.小于(&d1));
    assert!(d1.等于(&d1));
}

#[test]
fn test_time_point() {
    // 测试时间点创建
    let t1 = 时间点::现在();
    
    // 测试时间点运算
    let d = 持续时间::从秒(0.1);
    let t2 = t1.加(&d);
    assert!(t2.晚于(&t1));
    
    let t3 = t2.减持续时间(&d);
    assert!(t3.早于(&t2));
    
    // 测试经过时间
    let elapsed = t1.经过时间();
    assert!(elapsed.转为秒() >= 0.0);
}

#[test]
fn test_time_formatting() {
    // 测试时间格式化
    let t = 时间点::现在();
    let formatted = t.格式化("%Y-%m-%d %H:%M:%S");
    assert!(!formatted.作为字符串().is_empty());
}

#[test]
fn test_time_parsing() {
    // 测试时间解析
    let result = 时间点::解析("2026-01-01 12:00:00", "%Y-%m-%d %H:%M:%S");
    assert!(result.is_ok());
}

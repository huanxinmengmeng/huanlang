// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use rand::{Rng, RngCore, SeedableRng};
use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use std::option::Option;

/// 随机数生成器
pub struct 随机数生成器 {
    rng: StdRng,
}

impl 随机数生成器 {
    /// 创建新的随机数生成器
    pub fn 新建() -> Self {
        随机数生成器 { rng: StdRng::from_entropy() }
    }
    
    /// 从种子创建随机数生成器
    pub fn 从种子(种子: i64) -> Self {
        let rng = StdRng::seed_from_u64(种子 as u64);
        随机数生成器 { rng }
    }
    
    /// 生成整数
    pub fn 生成整数(&mut self) -> i64 {
        self.rng.gen()
    }
    
    /// 生成指定范围的整数
    pub fn 生成整数范围(&mut self, 最小: i64, 最大: i64) -> i64 {
        self.rng.gen_range(最小..=最大)
    }
    
    /// 生成浮点数 [0, 1)
    pub fn 生成浮点(&mut self) -> f64 {
        self.rng.gen()
    }
    
    /// 生成指定范围的浮点数
    pub fn 生成浮点范围(&mut self, 最小: f64, 最大: f64) -> f64 {
        self.rng.gen_range(最小..最大)
    }
    
    /// 生成布尔值
    pub fn 生成布尔(&mut self) -> bool {
        self.rng.gen()
    }
    
    /// 从切片中随机选择一个元素
    pub fn 选择<'a, T>(&mut self, 切片: &'a [T]) -> Option<&'a T> {
        if 切片.is_empty() {
            Option::None
        } else {
            let 索引 = self.rng.gen_range(0..切片.len());
            Option::Some(&切片[索引])
        }
    }
    
    /// 打乱切片中的元素
    pub fn 打乱<T>(&mut self, 切片: &mut [T]) {
        切片.shuffle(&mut self.rng);
    }
    
    /// 填充字节缓冲区
    pub fn 填充字节(&mut self, 缓冲区: &mut [u8]) {
        self.rng.fill_bytes(缓冲区);
    }
}

/// 全局随机数生成器（线程安全）
use std::sync::Mutex;

static 全局随机数生成器: Mutex<Option<随机数生成器>> = Mutex::new(None);

fn 获取全局生成器() -> &'static Mutex<Option<随机数生成器>> {
    &全局随机数生成器
}

/// 生成随机整数
pub fn 随机整数() -> i64 {
    let mut guard = 获取全局生成器().lock().unwrap();
    if guard.is_none() {
        *guard = Some(随机数生成器::新建());
    }
    match guard.as_mut() {
        Some(rng) => rng.生成整数(),
        None => 0,
    }
}

/// 生成随机浮点数
pub fn 随机浮点() -> f64 {
    let mut guard = 获取全局生成器().lock().unwrap();
    if guard.is_none() {
        *guard = Some(随机数生成器::新建());
    }
    match guard.as_mut() {
        Some(rng) => rng.生成浮点(),
        None => 0.0,
    }
}

/// 生成随机字节
pub fn 随机字节(长度: usize) -> Vec<u8> {
    let mut buffer = vec![0u8; 长度];
    let mut guard = 获取全局生成器().lock().unwrap();
    if guard.is_none() {
        *guard = Some(随机数生成器::新建());
    }
    match guard.as_mut() {
        Some(rng) => {
            rng.填充字节(&mut buffer);
            buffer
        },
        None => buffer,
    }
}

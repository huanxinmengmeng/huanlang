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

use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use std::option::Option;
use std::result::Result;

/// 列表（动态数组）
pub struct 列表<T> {
    data: Vec<T>,
}

impl<T> 列表<T> {
    /// 创建新的空列表
    pub fn 新建() -> Self {
        Self { data: Vec::new() }
    }
    
    /// 创建具有指定容量的空列表
    pub fn 从容量(容量: usize) -> Self {
        Self { data: Vec::with_capacity(容量) }
    }
    
    /// 从数组创建列表
    pub fn 从数组(数组: &[T]) -> Self 
    where
        T: Clone,
    {
        Self { data: 数组.to_vec() }
    }
    
    /// 获取列表长度
    pub fn 长度(&self) -> usize {
        self.data.len()
    }
    
    /// 检查列表是否为空
    pub fn 是否为空(&self) -> bool {
        self.data.is_empty()
    }
    
    /// 获取列表容量
    pub fn 容量(&self) -> usize {
        self.data.capacity()
    }
    
    /// 预留额外容量
    pub fn 预留容量(&mut self, 额外: usize) {
        self.data.reserve(额外);
    }
    
    /// 收缩到适合当前大小
    pub fn 收缩到合适大小(&mut self) {
        self.data.shrink_to_fit();
    }
    
    /// 获取元素（返回引用）
    pub fn 获取(&self, 索引: usize) -> Option<&T> {
        match self.data.get(索引) {
            Some(t) => Option::Some(t),
            None => Option::None,
        }
    }
    
    /// 获取可变引用
    pub fn 获取可变(&mut self, 索引: usize) -> Option<&mut T> {
        self.data.get_mut(索引)
    }
    
    /// 获取第一个元素
    pub fn 首个(&self) -> Option<&T> {
        match self.data.first() {
            Some(t) => Option::Some(t),
            None => Option::None,
        }
    }
    
    /// 获取可变第一个元素
    pub fn 首个可变(&mut self) -> Option<&mut T> {
        self.data.first_mut()
    }
    
    /// 获取最后一个元素
    pub fn 最后(&self) -> Option<&T> {
        match self.data.last() {
            Some(t) => Option::Some(t),
            None => Option::None,
        }
    }
    
    /// 获取可变最后一个元素
    pub fn 最后可变(&mut self) -> Option<&mut T> {
        self.data.last_mut()
    }
    
    /// 在末尾追加元素
    pub fn 追加(&mut self, 元素: T) {
        self.data.push(元素);
    }
    
    /// 在指定位置插入元素
    pub fn 插入(&mut self, 索引: usize, 元素: T) {
        self.data.insert(索引, 元素);
    }
    
    /// 扩展另一个列表
    pub fn 扩展(&mut self, 其他: &列表<T>) 
    where
        T: Clone,
    {
        self.data.extend(其他.data.clone());
    }
    
    /// 移除指定位置的元素
    pub fn 移除(&mut self, 索引: usize) -> T {
        self.data.remove(索引)
    }
    
    /// 移除最后一个元素
    pub fn 弹出(&mut self) -> Option<T> {
        match self.data.pop() {
            Some(t) => Option::Some(t),
            None => Option::None,
        }
    }
    
    /// 移除满足条件的第一个元素
    pub fn 移除首个<F>(&mut self, 谓词: F) -> Option<T> 
    where
        F: Fn(&T) -> bool,
    {
        if let Some(索引) = self.data.iter().position(谓词) {
            Option::Some(self.data.remove(索引))
        } else {
            Option::None
        }
    }
    
    /// 保留满足条件的元素
    pub fn 保留<F>(&mut self, 谓词: F) 
    where
        F: Fn(&T) -> bool,
    {
        self.data.retain(谓词);
    }
    
    /// 清空列表
    pub fn 清空(&mut self) {
        self.data.clear();
    }
    
    /// 检查是否包含元素
    pub fn 包含(&self, 元素: &T) -> bool 
    where
        T: PartialEq,
    {
        self.data.contains(元素)
    }
    
    /// 查找元素位置
    pub fn 查找(&self, 元素: &T) -> Option<usize> 
    where
        T: PartialEq,
    {
        if let Some(索引) = self.data.iter().position(|x| x == 元素) {
            Option::Some(索引)
        } else {
            Option::None
        }
    }
    
    /// 二分查找
    pub fn 二分查找(&self, 元素: &T) -> Result<usize, usize>
    where
        T: Ord,
    {
        match self.data.binary_search(元素) {
            Ok(索引) => Result::Ok(索引),
            Err(索引) => Result::Err(索引),
        }
    }
    
    /// 排序
    pub fn 排序(&mut self) 
    where
        T: Ord,
    {
        self.data.sort();
    }
    
    /// 根据比较函数排序
    pub fn 排序依据<F>(&mut self, 比较函数: F) 
    where
        F: Fn(&T, &T) -> std::cmp::Ordering,
    {
        self.data.sort_by(比较函数);
    }
    
    /// 映射
    pub fn 映射<U, F>(self, 映射函数: F) -> 列表<U> 
    where
        F: Fn(T) -> U,
    {
        列表 { data: self.data.into_iter().map(映射函数).collect() }
    }
    
    /// 过滤
    pub fn 过滤<F>(self, 谓词: F) -> 列表<T> 
    where
        F: Fn(&T) -> bool,
    {
        列表 { data: self.data.into_iter().filter(谓词).collect() }
    }
    
    /// 折叠
    pub fn 折叠<U, F>(self, 初始值: U, 折叠函数: F) -> U 
    where
        F: Fn(U, T) -> U,
    {
        self.data.into_iter().fold(初始值, 折叠函数)
    }
    
    /// 连接为字符串
    pub fn 连接(&self, 分隔符: &str) -> String 
    where
        T: std::fmt::Display,
    {
        self.data.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(分隔符)
    }
    
    /// 获取迭代器
    pub fn 迭代(&self) -> std::slice::Iter<'_, T> {
        self.data.iter()
    }
    
    /// 获取可变迭代器
    pub fn 迭代可变(&mut self) -> std::slice::IterMut<'_, T> {
        self.data.iter_mut()
    }
}

impl<T: Clone> Clone for 列表<T> {
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
}

/// 字典（哈希映射）
pub struct 字典<K, V> {
    data: HashMap<K, V>,
}

impl<K, V> 字典<K, V> 
where
    K: Hash + Eq,
{
    /// 创建新的空字典
    pub fn 新建() -> Self {
        Self { data: HashMap::new() }
    }
    
    /// 创建具有指定容量的空字典
    pub fn 从容量(容量: usize) -> Self {
        Self { data: HashMap::with_capacity(容量) }
    }
    
    /// 获取字典长度
    pub fn 长度(&self) -> usize {
        self.data.len()
    }
    
    /// 检查字典是否为空
    pub fn 是否为空(&self) -> bool {
        self.data.is_empty()
    }
    
    /// 获取字典容量
    pub fn 容量(&self) -> usize {
        self.data.capacity()
    }
    
    /// 获取值（返回引用）
    pub fn 获取(&self, 键: &K) -> Option<&V> {
        match self.data.get(键) {
            Some(v) => Option::Some(v),
            None => Option::None,
        }
    }
    
    /// 获取可变值
    pub fn 获取可变(&mut self, 键: &K) -> Option<&mut V> {
        self.data.get_mut(键)
    }
    
    /// 插入键值对，返回旧值（如果存在）
    pub fn 插入(&mut self, 键: K, 值: V) -> Option<V> {
        match self.data.insert(键, 值) {
            Some(v) => Option::Some(v),
            None => Option::None,
        }
    }
    
    /// 移除键值对
    pub fn 移除(&mut self, 键: &K) -> Option<V> {
        match self.data.remove(键) {
            Some(v) => Option::Some(v),
            None => Option::None,
        }
    }
    
    /// 清空字典
    pub fn 清空(&mut self) {
        self.data.clear();
    }
    
    /// 检查是否包含键
    pub fn 包含键(&self, 键: &K) -> bool {
        self.data.contains_key(键)
    }
    
    /// 获取所有键的迭代器
    pub fn 所有键(&self) -> std::collections::hash_map::Keys<'_, K, V> {
        self.data.keys()
    }
    
    /// 获取所有值的迭代器
    pub fn 所有值(&self) -> std::collections::hash_map::Values<'_, K, V> {
        self.data.values()
    }
    
    /// 获取可变值迭代器
    pub fn 所有值可变(&mut self) -> std::collections::hash_map::ValuesMut<'_, K, V> {
        self.data.values_mut()
    }
    
    /// 获取键值对迭代器
    pub fn 迭代(&self) -> std::collections::hash_map::Iter<'_, K, V> {
        self.data.iter()
    }
    
    /// 获取可变键值对迭代器
    pub fn 迭代可变(&mut self) -> std::collections::hash_map::IterMut<'_, K, V> {
        self.data.iter_mut()
    }
    
    /// 扩展另一个字典
    pub fn 扩展(&mut self, 其他: &字典<K, V>) 
    where
        K: Clone,
        V: Clone,
    {
        for (键, 值) in 其他.data.iter() {
            self.data.insert(键.clone(), 值.clone());
        }
    }
    
    /// 保留满足条件的键值对
    pub fn 保留<F>(&mut self, 谓词: F) 
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.data.retain(谓词);
    }
}

impl<K: Clone + Hash + Eq, V: Clone> Clone for 字典<K, V> {
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
}

/// 集合（哈希集合）
pub struct 集合<T> {
    data: HashSet<T>,
}

impl<T> 集合<T> 
where
    T: Hash + Eq,
{
    /// 创建新的空集合
    pub fn 新建() -> Self {
        Self { data: HashSet::new() }
    }
    
    /// 创建具有指定容量的空集合
    pub fn 从容量(容量: usize) -> Self {
        Self { data: HashSet::with_capacity(容量) }
    }
    
    /// 获取集合长度
    pub fn 长度(&self) -> usize {
        self.data.len()
    }
    
    /// 检查集合是否为空
    pub fn 是否为空(&self) -> bool {
        self.data.is_empty()
    }
    
    /// 插入元素，返回是否是新插入的
    pub fn 插入(&mut self, 元素: T) -> bool {
        self.data.insert(元素)
    }
    
    /// 移除元素，返回是否成功
    pub fn 移除(&mut self, 元素: &T) -> bool {
        self.data.remove(元素)
    }
    
    /// 检查是否包含元素
    pub fn 包含(&self, 元素: &T) -> bool {
        self.data.contains(元素)
    }
    
    /// 清空集合
    pub fn 清空(&mut self) {
        self.data.clear();
    }
    
    /// 取并集
    pub fn 并集(&self, 其他: &集合<T>) -> 集合<T> 
    where
        T: Clone,
    {
        let mut result = 集合::新建();
        for x in self.data.union(&其他.data) {
            result.插入(x.clone());
        }
        result
    }
    
    /// 取交集
    pub fn 交集(&self, 其他: &集合<T>) -> 集合<T> 
    where
        T: Clone,
    {
        let mut result = 集合::新建();
        for x in self.data.intersection(&其他.data) {
            result.插入(x.clone());
        }
        result
    }
    
    /// 取差集
    pub fn 差集(&self, 其他: &集合<T>) -> 集合<T> 
    where
        T: Clone,
    {
        let mut result = 集合::新建();
        for x in self.data.difference(&其他.data) {
            result.插入(x.clone());
        }
        result
    }
    
    /// 取对称差
    pub fn 对称差(&self, 其他: &集合<T>) -> 集合<T> 
    where
        T: Clone,
    {
        let mut result = 集合::新建();
        for x in self.data.symmetric_difference(&其他.data) {
            result.插入(x.clone());
        }
        result
    }
    
    /// 检查是否是子集
    pub fn 是否为子集(&self, 其他: &集合<T>) -> bool {
        self.data.is_subset(&其他.data)
    }
    
    /// 检查是否是超集
    pub fn 是否为超集(&self, 其他: &集合<T>) -> bool {
        self.data.is_superset(&其他.data)
    }
    
    /// 检查是否不相交
    pub fn 是否不相交(&self, 其他: &集合<T>) -> bool {
        self.data.is_disjoint(&其他.data)
    }
    
    /// 获取迭代器
    pub fn 迭代(&self) -> std::collections::hash_set::Iter<'_, T> {
        self.data.iter()
    }
}

impl<T: Clone + Hash + Eq> Clone for 集合<T> {
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
}

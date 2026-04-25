// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 运行时库测试

#[test]
pub fn test_memory_allocation() {
    // 测试内存分配
    let ptr = unsafe {
        std::alloc::alloc(std::alloc::Layout::new::<u8>())
    };
    assert!(!ptr.is_null());
    unsafe {
        std::alloc::dealloc(ptr, std::alloc::Layout::new::<u8>());
    }
}

#[test]
pub fn test_string_operations() {
    // 测试字符串操作
    let mut str1 = String::from("Hello");
    assert_eq!(str1.len(), 5);
    assert_eq!(str1, "Hello");
    
    str1.push_str(" World");
    assert_eq!(str1.len(), 11);
    assert_eq!(str1, "Hello World");
}

#[test]
pub fn test_list_operations() {
    // 测试列表操作
    let mut list = Vec::new();
    assert_eq!(list.len(), 0);
    
    let data1 = 42;
    let data2 = 100;
    
    list.push(data1);
    list.push(data2);
    
    assert_eq!(list.len(), 2);
    assert_eq!(list[0], 42);
    assert_eq!(list[1], 100);
}

#[test]
pub fn test_map_operations() {
    // 测试映射操作
    use std::collections::HashMap;
    let mut map = HashMap::new();
    assert_eq!(map.len(), 0);
    
    let value1 = 42;
    let value2 = 100;
    
    map.insert("key1", value1);
    map.insert("key2", value2);
    
    assert_eq!(map.len(), 2);
    assert_eq!(map.get("key1"), Some(&42));
    assert_eq!(map.get("key2"), Some(&100));
    
    map.remove("key1");
    assert_eq!(map.len(), 1);
    assert!(map.get("key1").is_none());
}

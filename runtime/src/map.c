// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 幻语运行时库映射实现

#include "huanrt.h"
#include <string.h>

// 哈希函数
static size_t huan_map_hash(const char* key) {
    size_t hash = 5381;
    int c;
    while ((c = *key++)) {
        hash = ((hash << 5) + hash) + c; // hash * 33 + c
    }
    return hash;
}

// 创建新映射
huan_map* huan_map_new(void) {
    huan_map* map = huan_malloc(sizeof(huan_map));
    map->capacity = 16;
    map->size = 0;
    map->buckets = huan_calloc(map->capacity, sizeof(huan_map_entry*));
    return map;
}

// 释放映射
void huan_map_free(huan_map* map) {
    if (map) {
        for (size_t i = 0; i < map->capacity; i++) {
            huan_map_entry* current = map->buckets[i];
            while (current) {
                huan_map_entry* next = current->next;
                huan_free(current->key);
                huan_free(current);
                current = next;
            }
        }
        huan_free(map->buckets);
        huan_free(map);
    }
}

// 设置键值对
void huan_map_set(huan_map* map, const char* key, void* value) {
    size_t hash = huan_map_hash(key);
    size_t index = hash % map->capacity;
    
    // 查找是否已存在该键
    huan_map_entry* current = map->buckets[index];
    while (current) {
        if (strcmp(current->key, key) == 0) {
            current->value = value;
            return;
        }
        current = current->next;
    }
    
    // 扩容检查
    if (map->size >= map->capacity * 0.75) {
        size_t new_capacity = map->capacity * 2;
        huan_map_entry** new_buckets = huan_calloc(new_capacity, sizeof(huan_map_entry*));
        
        // 重新哈希所有元素
        for (size_t i = 0; i < map->capacity; i++) {
            current = map->buckets[i];
            while (current) {
                huan_map_entry* next = current->next;
                size_t new_index = huan_map_hash(current->key) % new_capacity;
                current->next = new_buckets[new_index];
                new_buckets[new_index] = current;
                current = next;
            }
        }
        
        huan_free(map->buckets);
        map->buckets = new_buckets;
        map->capacity = new_capacity;
        index = hash % new_capacity;
    }
    
    // 添加新条目
    huan_map_entry* entry = huan_malloc(sizeof(huan_map_entry));
    entry->key = huan_malloc(strlen(key) + 1);
    strcpy(entry->key, key);
    entry->value = value;
    entry->next = map->buckets[index];
    map->buckets[index] = entry;
    map->size++;
}

// 获取值
void* huan_map_get(const huan_map* map, const char* key) {
    size_t hash = huan_map_hash(key);
    size_t index = hash % map->capacity;
    
    huan_map_entry* current = map->buckets[index];
    while (current) {
        if (strcmp(current->key, key) == 0) {
            return current->value;
        }
        current = current->next;
    }
    
    return NULL;
}

// 移除键值对
void huan_map_remove(huan_map* map, const char* key) {
    size_t hash = huan_map_hash(key);
    size_t index = hash % map->capacity;
    
    huan_map_entry* current = map->buckets[index];
    huan_map_entry* prev = NULL;
    
    while (current) {
        if (strcmp(current->key, key) == 0) {
            if (prev) {
                prev->next = current->next;
            } else {
                map->buckets[index] = current->next;
            }
            huan_free(current->key);
            huan_free(current);
            map->size--;
            return;
        }
        prev = current;
        current = current->next;
    }
}

// 获取映射大小
size_t huan_map_size(const huan_map* map) {
    return map->size;
}

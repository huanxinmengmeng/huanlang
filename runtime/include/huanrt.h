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
//
// 幻语运行时库

#ifndef HUANRT_H
#define HUANRT_H

#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>

// 版本信息
#define HUANRT_VERSION "0.0.1"

// 基本类型定义
typedef int8_t  int8;
typedef int16_t int16;
typedef int32_t int32;
typedef int64_t int64;

typedef uint8_t  uint8;
typedef uint16_t uint16;
typedef uint32_t uint32;
typedef uint64_t uint64;

typedef float  float32;
typedef double float64;

// 字符串类型
typedef struct {
    char* data;
    size_t length;
    size_t capacity;
} huan_string;

// 列表类型
typedef struct huan_list_node {
    void* data;
    struct huan_list_node* next;
    struct huan_list_node* prev;
} huan_list_node;

typedef struct {
    huan_list_node* head;
    huan_list_node* tail;
    size_t size;
} huan_list;

// 映射类型
typedef struct huan_map_entry {
    char* key;
    void* value;
    struct huan_map_entry* next;
} huan_map_entry;

typedef struct {
    huan_map_entry** buckets;
    size_t size;
    size_t capacity;
} huan_map;

// 内存分配器函数
extern void* huan_malloc(size_t size);
extern void* huan_calloc(size_t count, size_t size);
extern void* huan_realloc(void* ptr, size_t size);
extern void huan_free(void* ptr);

// 字符串函数
extern huan_string* huan_string_new(const char* str);
extern huan_string* huan_string_new_empty(size_t capacity);
extern void huan_string_free(huan_string* str);
extern huan_string* huan_string_append(huan_string* str, const char* other);
extern huan_string* huan_string_append_char(huan_string* str, char c);
extern size_t huan_string_length(const huan_string* str);
extern const char* huan_string_data(const huan_string* str);

// 列表函数
extern huan_list* huan_list_new(void);
extern void huan_list_free(huan_list* list);
extern void huan_list_push_back(huan_list* list, void* data);
extern void huan_list_push_front(huan_list* list, void* data);
extern void* huan_list_pop_back(huan_list* list);
extern void* huan_list_pop_front(huan_list* list);
extern size_t huan_list_size(const huan_list* list);
extern void* huan_list_get(const huan_list* list, size_t index);

// 映射函数
extern huan_map* huan_map_new(void);
extern void huan_map_free(huan_map* map);
extern void huan_map_set(huan_map* map, const char* key, void* value);
extern void* huan_map_get(const huan_map* map, const char* key);
extern void huan_map_remove(huan_map* map, const char* key);
extern size_t huan_map_size(const huan_map* map);

// 输入输出函数
extern void huan_print(const char* str);
extern void huan_println(const char* str);
extern char* huan_input(const char* prompt);

// 控制台函数
extern void huan_console_clear(void);
extern void huan_console_set_color(int color);
extern void huan_console_reset_color(void);

// 内存管理
extern void huan_gc_collect(void);

// 初始化和清理
extern void huanrt_init(void);
extern void huanrt_cleanup(void);

#endif // HUANRT_H
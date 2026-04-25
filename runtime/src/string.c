// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 幻语运行时库字符串实现

#include "huanrt.h"
#include <string.h>

// 创建新字符串
huan_string* huan_string_new(const char* str) {
    size_t length = strlen(str);
    huan_string* string = huan_malloc(sizeof(huan_string));
    string->length = length;
    string->capacity = length + 1;
    string->data = huan_malloc(string->capacity);
    strcpy(string->data, str);
    return string;
}

// 创建空字符串
huan_string* huan_string_new_empty(size_t capacity) {
    huan_string* string = huan_malloc(sizeof(huan_string));
    string->length = 0;
    string->capacity = capacity > 0 ? capacity : 16;
    string->data = huan_malloc(string->capacity);
    string->data[0] = '\0';
    return string;
}

// 释放字符串
void huan_string_free(huan_string* str) {
    if (str) {
        huan_free(str->data);
        huan_free(str);
    }
}

// 追加字符串
huan_string* huan_string_append(huan_string* str, const char* other) {
    size_t other_length = strlen(other);
    size_t new_length = str->length + other_length;
    
    if (new_length >= str->capacity) {
        str->capacity = new_length * 2;
        str->data = huan_realloc(str->data, str->capacity);
    }
    
    strcpy(str->data + str->length, other);
    str->length = new_length;
    return str;
}

// 追加字符
huan_string* huan_string_append_char(huan_string* str, char c) {
    size_t new_length = str->length + 1;
    
    if (new_length >= str->capacity) {
        str->capacity = new_length * 2;
        str->data = huan_realloc(str->data, str->capacity);
    }
    
    str->data[str->length] = c;
    str->data[new_length] = '\0';
    str->length = new_length;
    return str;
}

// 获取字符串长度
size_t huan_string_length(const huan_string* str) {
    return str->length;
}

// 获取字符串数据
const char* huan_string_data(const huan_string* str) {
    return str->data;
}

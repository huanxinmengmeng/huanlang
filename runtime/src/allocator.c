// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 幻语运行时库内存分配器

#include "huanrt.h"
#include <stdlib.h>

// 内存分配
void* huan_malloc(size_t size) {
    void* ptr = malloc(size);
    if (!ptr) {
        fprintf(stderr, "HuanRT: Out of memory\n");
        exit(1);
    }
    return ptr;
}

// 内存分配并清零
void* huan_calloc(size_t count, size_t size) {
    void* ptr = calloc(count, size);
    if (!ptr) {
        fprintf(stderr, "HuanRT: Out of memory\n");
        exit(1);
    }
    return ptr;
}

// 内存重分配
void* huan_realloc(void* ptr, size_t size) {
    void* new_ptr = realloc(ptr, size);
    if (!new_ptr) {
        fprintf(stderr, "HuanRT: Out of memory\n");
        exit(1);
    }
    return new_ptr;
}

// 内存释放
void huan_free(void* ptr) {
    free(ptr);
}

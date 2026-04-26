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

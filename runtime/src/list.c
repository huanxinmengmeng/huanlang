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
// 幻语运行时库列表实现

#include "huanrt.h"

// 创建新列表
huan_list* huan_list_new(void) {
    huan_list* list = huan_malloc(sizeof(huan_list));
    list->head = NULL;
    list->tail = NULL;
    list->size = 0;
    return list;
}

// 释放列表
void huan_list_free(huan_list* list) {
    if (list) {
        huan_list_node* current = list->head;
        while (current) {
            huan_list_node* next = current->next;
            huan_free(current);
            current = next;
        }
        huan_free(list);
    }
}

// 在列表末尾添加元素
void huan_list_push_back(huan_list* list, void* data) {
    huan_list_node* node = huan_malloc(sizeof(huan_list_node));
    node->data = data;
    node->next = NULL;
    node->prev = list->tail;
    
    if (list->tail) {
        list->tail->next = node;
    } else {
        list->head = node;
    }
    list->tail = node;
    list->size++;
}

// 在列表开头添加元素
void huan_list_push_front(huan_list* list, void* data) {
    huan_list_node* node = huan_malloc(sizeof(huan_list_node));
    node->data = data;
    node->next = list->head;
    node->prev = NULL;
    
    if (list->head) {
        list->head->prev = node;
    } else {
        list->tail = node;
    }
    list->head = node;
    list->size++;
}

// 从列表末尾移除元素
void* huan_list_pop_back(huan_list* list) {
    if (!list->tail) return NULL;
    
    huan_list_node* node = list->tail;
    void* data = node->data;
    
    if (node->prev) {
        node->prev->next = NULL;
        list->tail = node->prev;
    } else {
        list->head = NULL;
        list->tail = NULL;
    }
    
    huan_free(node);
    list->size--;
    return data;
}

// 从列表开头移除元素
void* huan_list_pop_front(huan_list* list) {
    if (!list->head) return NULL;
    
    huan_list_node* node = list->head;
    void* data = node->data;
    
    if (node->next) {
        node->next->prev = NULL;
        list->head = node->next;
    } else {
        list->head = NULL;
        list->tail = NULL;
    }
    
    huan_free(node);
    list->size--;
    return data;
}

// 获取列表大小
size_t huan_list_size(const huan_list* list) {
    return list->size;
}

// 获取列表指定位置的元素
void* huan_list_get(const huan_list* list, size_t index) {
    if (index >= list->size) return NULL;
    
    huan_list_node* current = list->head;
    for (size_t i = 0; i < index; i++) {
        current = current->next;
    }
    return current->data;
}

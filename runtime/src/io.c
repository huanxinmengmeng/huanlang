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
// 幻语运行时库 IO 实现

#include "huanrt.h"
#include <stdio.h>
#include <stdlib.h>

// 打印字符串
void huan_print(const char* str) {
    printf("%s", str);
}

// 打印字符串并换行
void huan_println(const char* str) {
    printf("%s\n", str);
}

// 读取用户输入
char* huan_input(const char* prompt) {
    if (prompt) {
        printf("%s", prompt);
    }
    
    char* buffer = huan_malloc(256);
    if (fgets(buffer, 256, stdin)) {
        // 移除换行符
        size_t length = strlen(buffer);
        if (length > 0 && buffer[length - 1] == '\n') {
            buffer[length - 1] = '\0';
        }
        return buffer;
    }
    
    buffer[0] = '\0';
    return buffer;
}

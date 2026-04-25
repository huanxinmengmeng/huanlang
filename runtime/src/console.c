// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 幻语运行时库控制台实现

#include "huanrt.h"
#include <stdio.h>

// 清屏
void huan_console_clear(void) {
#ifdef _WIN32
    system("cls");
#else
    system("clear");
#endif
}

// 设置控制台颜色
void huan_console_set_color(int color) {
#ifdef _WIN32
    // Windows 颜色代码
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(hConsole, color);
#else
    // ANSI 颜色代码
    switch (color) {
        case 0: printf("\033[30m"); break; // 黑色
        case 1: printf("\033[31m"); break; // 红色
        case 2: printf("\033[32m"); break; // 绿色
        case 3: printf("\033[33m"); break; // 黄色
        case 4: printf("\033[34m"); break; // 蓝色
        case 5: printf("\033[35m"); break; // 洋红色
        case 6: printf("\033[36m"); break; // 青色
        case 7: printf("\033[37m"); break; // 白色
        default: printf("\033[37m"); break; // 默认白色
    }
#endif
}

// 重置控制台颜色
void huan_console_reset_color(void) {
#ifdef _WIN32
    HANDLE hConsole = GetStdHandle(STD_OUTPUT_HANDLE);
    SetConsoleTextAttribute(hConsole, 7); // 默认白色
#else
    printf("\033[0m"); // 重置颜色
#endif
}

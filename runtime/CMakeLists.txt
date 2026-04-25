cmake_minimum_required(VERSION 3.20)
project(huanrt C)

set(CMAKE_C_STANDARD 11)
set(CMAKE_C_STANDARD_REQUIRED ON)

include_directories(include)

# 运行时库源码
set(SOURCES
    src/huanrt.c
    src/allocator.c
    src/string.c
    src/list.c
    src/map.c
    src/io.c
    src/console.c
)

# 编译为静态库
add_library(huanrt STATIC ${SOURCES})

# 编译为动态库（可选）
if(NOT WIN32)
    add_library(huanrt_shared SHARED ${SOURCES})
    set_target_properties(huanrt_shared PROPERTIES OUTPUT_NAME huanrt)
endif()

# 安装
install(TARGETS huanrt
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
)

if(NOT WIN32)
    install(TARGETS huanrt_shared
        LIBRARY DESTINATION lib
    )
endif()

install(DIRECTORY include/
    DESTINATION include
)
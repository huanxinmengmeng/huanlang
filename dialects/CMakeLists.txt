cmake_minimum_required(VERSION 3.20)
project(HuanDialect CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(MLIR REQUIRED CONFIG)

message(STATUS "Using MLIR configuration: ${MLIR_DIR}")

include_directories(${MLIR_INCLUDE_DIRS})
add_definitions(${MLIR_DEFINITIONS})

# 方言定义
add_library(HuanDialect
    lib/HuanDialect.cpp
    lib/HuanOps.cpp
    lib/HuanTypes.cpp
)

# 降级 Pass
add_library(LowerHuanToLLVM
    lib/LowerHuanToLLVM.cpp
    lib/LowerHuanToSCF.cpp
    lib/LowerHuanToArith.cpp
    lib/LowerHuanToFunc.cpp
)

target_include_directories(HuanDialect PRIVATE include)
target_include_directories(LowerHuanToLLVM PRIVATE include)

target_link_libraries(HuanDialect
    MLIRIR
    MLIRDialect
    MLIRSupport
)

target_link_libraries(LowerHuanToLLVM
    HuanDialect
    MLIRIR
    MLIRDialect
    MLIRSupport
    MLIRTransforms
    MLIRLLVMIR
    MLIRSCF
    MLIRArith
    MLIRFunc
)

# 安装
sinstall(TARGETS HuanDialect LowerHuanToLLVM
    LIBRARY DESTINATION lib
    ARCHIVE DESTINATION lib
)

install(DIRECTORY include/huan
    DESTINATION include
)
// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 幻语 MLIR 方言实现

#include "huan/HuanDialect.h"

using namespace mlir;
using namespace huan;

// 方言构造函数
HuanDialect::HuanDialect(MLIRContext *context) : Dialect(getDialectNamespace(), context) {
    // 注册操作
    addOperations<
#define GET_OP_LIST
#include "huan/HuanOps.cpp.inc"
    >();
    
    // 注册类型
    addTypes<
#define GET_TYPEDEF_LIST
#include "huan/HuanTypes.cpp.inc"
    >();
}

// 注册方言
#include "mlir/InitAllDialects.h"

void registerHuanDialect(DialectRegistry &registry) {
    registry.insert<HuanDialect>();
}

// 注册方言到上下文
void registerHuanDialect(MLIRContext &context) {
    DialectRegistry registry;
    registerHuanDialect(registry);
    context.appendDialectRegistry(registry);
}

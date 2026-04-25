// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 幻语 MLIR 方言类型实现

#include "huan/HuanDialect.h"

using namespace mlir;
using namespace huan;

// 生成类型实现
#define GET_TYPEDEF_IMPL
#include "huan/HuanTypes.cpp.inc"

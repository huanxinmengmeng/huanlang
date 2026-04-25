// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 幻语 MLIR 方言操作实现

#include "huan/HuanDialect.h"

using namespace mlir;
using namespace huan;

// 生成操作实现
#define GET_OP_IMPL
#include "huan/HuanOps.cpp.inc"

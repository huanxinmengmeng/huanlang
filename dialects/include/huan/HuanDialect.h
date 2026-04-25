// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 幻语 MLIR 方言定义

#ifndef HUAN_DIALECT_H
#define HUAN_DIALECT_H

#include "mlir/IR/Dialect.h"
#include "mlir/IR/OpDefinition.h"
#include "mlir/IR/Types.h"

namespace huan {

class HuanDialect : public mlir::Dialect {
public:
    explicit HuanDialect(mlir::MLIRContext *context);
    static constexpr llvm::StringLiteral getDialectNamespace() { return "huan"; }
};

} // namespace huan

#define GET_OP_CLASSES
#include "huan/HuanOps.h.inc"

#define GET_TYPEDEF_CLASSES
#include "huan/HuanTypes.h.inc"

#endif // HUAN_DIALECT_H
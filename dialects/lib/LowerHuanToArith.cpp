// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 幻语 MLIR 方言到 Arith 方言的降级

#include "huan/HuanDialect.h"
#include "mlir/Transforms/DialectConversion.h"
#include "mlir/Dialect/Arith/IR/Arith.h"

using namespace mlir;
using namespace huan;

namespace {

// 操作降级模式
class LowerHuanToArithPattern : public ConversionPattern {
public:
    LowerHuanToArithPattern(MLIRContext *context) : ConversionPattern(PatternBenefit::High, context) {}
};

// 二元操作降级到 Arith
class LowerHLBinOpToArith : public LowerHuanToArithPattern {
public:
    LowerHLBinOpToArith(StringRef opName, MLIRContext *context) 
        : LowerHuanToArithPattern(context), opName(opName) {}
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        if (op->getName().getStringRef() != opName) return failure();
        
        auto resultType = op->getResult(0).getType();
        
        if (opName == "huan.add") {
            rewriter.replaceOpWithNewOp<arith::AddIOp>(op, resultType, operands[0], operands[1]);
        } else if (opName == "huan.sub") {
            rewriter.replaceOpWithNewOp<arith::SubIOp>(op, resultType, operands[0], operands[1]);
        } else if (opName == "huan.mul") {
            rewriter.replaceOpWithNewOp<arith::MulIOp>(op, resultType, operands[0], operands[1]);
        } else if (opName == "huan.div") {
            rewriter.replaceOpWithNewOp<arith::DivSIOp>(op, resultType, operands[0], operands[1]);
        }
        
        return success();
    }
    
private:
    StringRef opName;
};

// 比较操作降级到 Arith
class LowerHLCompareOpToArith : public LowerHuanToArithPattern {
public:
    LowerHLCompareOpToArith(StringRef opName, MLIRContext *context) 
        : LowerHuanToArithPattern(context), opName(opName) {}
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        if (op->getName().getStringRef() != opName) return failure();
        
        auto resultType = op->getResult(0).getType();
        arith::CmpIPredicate predicate;
        
        if (opName == "huan.eq") {
            predicate = arith::CmpIPredicate::eq;
        } else if (opName == "huan.ne") {
            predicate = arith::CmpIPredicate::ne;
        } else if (opName == "huan.lt") {
            predicate = arith::CmpIPredicate::slt;
        } else if (opName == "huan.le") {
            predicate = arith::CmpIPredicate::sle;
        } else if (opName == "huan.ge") {
            predicate = arith::CmpIPredicate::sge;
        } else if (opName == "huan.gr") {
            predicate = arith::CmpIPredicate::sgt;
        } else {
            return failure();
        }
        
        rewriter.replaceOpWithNewOp<arith::CmpIOp>(op, predicate, operands[0], operands[1]);
        return success();
    }
    
private:
    StringRef opName;
};

// 逻辑操作降级到 Arith
class LowerHLLogicalOpToArith : public LowerHuanToArithPattern {
public:
    LowerHLLogicalOpToArith(StringRef opName, MLIRContext *context) 
        : LowerHuanToArithPattern(context), opName(opName) {}
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        if (op->getName().getStringRef() != opName) return failure();
        
        auto resultType = op->getResult(0).getType();
        
        if (opName == "huan.and") {
            rewriter.replaceOpWithNewOp<arith::AndIOp>(op, resultType, operands[0], operands[1]);
        } else if (opName == "huan.or") {
            rewriter.replaceOpWithNewOp<arith::OrIOp>(op, resultType, operands[0], operands[1]);
        } else if (opName == "huan.not") {
            rewriter.replaceOpWithNewOp<arith::XOrIOp>(op, resultType, operands[0], 
                rewriter.create<arith::ConstantOp>(op->getLoc(), resultType, rewriter.getIntegerAttr(resultType, 1)));
        }
        
        return success();
    }
    
private:
    StringRef opName;
};

// 常量降级到 Arith
class LowerHLConstOpToArith : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto constOp = dyn_cast<HLConstOp>(op);
        if (!constOp) return failure();
        
        auto value = constOp.getValue();
        
        if (auto intAttr = value.dyn_cast<IntegerAttr>()) {
            rewriter.replaceOpWithNewOp<arith::ConstantOp>(op, intAttr);
            return success();
        }
        if (auto floatAttr = value.dyn_cast<FloatAttr>()) {
            rewriter.replaceOpWithNewOp<arith::ConstantOp>(op, floatAttr);
            return success();
        }
        if (auto boolAttr = value.dyn_cast<BoolAttr>()) {
            rewriter.replaceOpWithNewOp<arith::ConstantOp>(op, boolAttr);
            return success();
        }
        
        return failure();
    }
};

// 模块降级 Pass
class LowerHuanToArithPass : public PassWrapper<LowerHuanToArithPass, OperationPass<ModuleOp>> {
public:
    MLIR_DEFINE_EXPLICIT_INTERNAL_INLINE_TYPE_ID(LowerHuanToArithPass)
    
    void runOnOperation() override {
        auto module = getOperation();
        auto context = module.getContext();
        
        RewritePatternSet patterns(context);
        
        // 添加降级模式
        patterns.add<LowerHLConstOpToArith>(context);
        patterns.add<LowerHLBinOpToArith>("huan.add", context);
        patterns.add<LowerHLBinOpToArith>("huan.sub", context);
        patterns.add<LowerHLBinOpToArith>("huan.mul", context);
        patterns.add<LowerHLBinOpToArith>("huan.div", context);
        patterns.add<LowerHLCompareOpToArith>("huan.eq", context);
        patterns.add<LowerHLCompareOpToArith>("huan.ne", context);
        patterns.add<LowerHLCompareOpToArith>("huan.lt", context);
        patterns.add<LowerHLCompareOpToArith>("huan.le", context);
        patterns.add<LowerHLCompareOpToArith>("huan.ge", context);
        patterns.add<LowerHLCompareOpToArith>("huan.gr", context);
        patterns.add<LowerHLLogicalOpToArith>("huan.and", context);
        patterns.add<LowerHLLogicalOpToArith>("huan.or", context);
        patterns.add<LowerHLLogicalOpToArith>("huan.not", context);
        
        ConversionTarget target(*context);
        target.addLegalDialect<arith::ArithDialect>();
        target.addIllegalOp<HLConstOp>();
        target.addIllegalOp<HLAddOp>();
        target.addIllegalOp<HLSubOp>();
        target.addIllegalOp<HLMulOp>();
        target.addIllegalOp<HLDivOp>();
        target.addIllegalOp<HLEqOp>();
        target.addIllegalOp<HLNeOp>();
        target.addIllegalOp<HLLtOp>();
        target.addIllegalOp<HLLeOp>();
        target.addIllegalOp<HLGeOp>();
        target.addIllegalOp<HLGrOp>();
        target.addIllegalOp<HLAndOp>();
        target.addIllegalOp<HLOrOp>();
        target.addIllegalOp<HLNotOp>();
        
        if (failed(applyPartialConversion(module, target, std::move(patterns)))) {
            signalPassFailure();
        }
    }
};

} // namespace

// 注册 Pass
std::unique_ptr<Pass> createLowerHuanToArithPass() {
    return std::make_unique<LowerHuanToArithPass>();
}

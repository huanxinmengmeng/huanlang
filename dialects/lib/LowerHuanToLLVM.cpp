// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 幻语 MLIR 方言到 LLVM 方言的降级

#include "huan/HuanDialect.h"
#include "mlir/Transforms/DialectConversion.h"
#include "mlir/Conversion/LLVMCommon/TypeConverter.h"
#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"

using namespace mlir;
using namespace huan;

namespace {

// 类型转换器
class HuanToLLVMTypeConverter : public LLVMTypeConverter {
public:
    using LLVMTypeConverter::LLVMTypeConverter;
    
    Type convertType(Type type) override {
        if (auto intType = type.dyn_cast<HLIntType>()) {
            return IntegerType::get(getContext(), intType.getWidth());
        }
        if (auto floatType = type.dyn_cast<HLFloatType>()) {
            switch (floatType.getWidth()) {
                case 32: return Float32Type::get(getContext());
                case 64: return Float64Type::get(getContext());
                default: return type;
            }
        }
        if (type.isa<HLBoolType>()) {
            return IntegerType::get(getContext(), 1);
        }
        if (type.isa<HLStringType>()) {
            return LLVM::LLVMPointerType::get(IntegerType::get(getContext(), 8));
        }
        return LLVMTypeConverter::convertType(type);
    }
};

// 操作降级模式
class LowerHuanToLLVMPattern : public ConvertToLLVMPattern {
public:
    LowerHuanToLLVMPattern(HuanToLLVMTypeConverter &converter, MLIRContext *context)
        : ConvertToLLVMPattern(converter, context) {}
};

// 常量降级
class LowerHLConstOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto constOp = cast<HLConstOp>(op);
        auto value = constOp.getValue();
        
        if (auto intAttr = value.dyn_cast<IntegerAttr>()) {
            rewriter.replaceOpWithNewOp<LLVM::ConstantOp>(op, 
                typeConverter->convertType(constOp.getType()), 
                intAttr);
            return success();
        }
        if (auto floatAttr = value.dyn_cast<FloatAttr>()) {
            rewriter.replaceOpWithNewOp<LLVM::ConstantOp>(op, 
                typeConverter->convertType(constOp.getType()), 
                floatAttr);
            return success();
        }
        if (auto stringAttr = value.dyn_cast<StringAttr>()) {
            // 处理字符串常量
            rewriter.replaceOpWithNewOp<LLVM::ConstantOp>(op, 
                typeConverter->convertType(constOp.getType()), 
                stringAttr);
            return success();
        }
        return failure();
    }
};

// 二元操作降级
class LowerHLBinOp : public LowerHuanToLLVMPattern {
public:
    LowerHLBinOp(StringRef opName, HuanToLLVMTypeConverter &converter, MLIRContext *context)
        : LowerHuanToLLVMPattern(converter, context), opName(opName) {}
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        if (op->getName().getStringRef() != opName) return failure();
        
        auto resultType = typeConverter->convertType(op->getResult(0).getType());
        
        if (opName == "huan.add") {
            rewriter.replaceOpWithNewOp<LLVM::AddOp>(op, resultType, operands[0], operands[1]);
        } else if (opName == "huan.sub") {
            rewriter.replaceOpWithNewOp<LLVM::SubOp>(op, resultType, operands[0], operands[1]);
        } else if (opName == "huan.mul") {
            rewriter.replaceOpWithNewOp<LLVM::MulOp>(op, resultType, operands[0], operands[1]);
        } else if (opName == "huan.div") {
            rewriter.replaceOpWithNewOp<LLVM::SDivOp>(op, resultType, operands[0], operands[1]);
        }
        
        return success();
    }
    
private:
    StringRef opName;
};

// 比较操作降级
class LowerHLCompareOp : public LowerHuanToLLVMPattern {
public:
    LowerHLCompareOp(StringRef opName, HuanToLLVMTypeConverter &converter, MLIRContext *context)
        : LowerHuanToLLVMPattern(converter, context), opName(opName) {}
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        if (op->getName().getStringRef() != opName) return failure();
        
        auto resultType = typeConverter->convertType(op->getResult(0).getType());
        LLVM::ICmpPredicate predicate;
        
        if (opName == "huan.eq") {
            predicate = LLVM::ICmpPredicate::eq;
        } else if (opName == "huan.ne") {
            predicate = LLVM::ICmpPredicate::ne;
        } else if (opName == "huan.lt") {
            predicate = LLVM::ICmpPredicate::slt;
        } else if (opName == "huan.le") {
            predicate = LLVM::ICmpPredicate::sle;
        } else if (opName == "huan.ge") {
            predicate = LLVM::ICmpPredicate::sge;
        } else if (opName == "huan.gr") {
            predicate = LLVM::ICmpPredicate::sgt;
        } else {
            return failure();
        }
        
        rewriter.replaceOpWithNewOp<LLVM::ICmpOp>(op, resultType, predicate, operands[0], operands[1]);
        return success();
    }
    
private:
    StringRef opName;
};

// 逻辑操作降级
class LowerHLLogicalOp : public LowerHuanToLLVMPattern {
public:
    LowerHLLogicalOp(StringRef opName, HuanToLLVMTypeConverter &converter, MLIRContext *context)
        : LowerHuanToLLVMPattern(converter, context), opName(opName) {}
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        if (op->getName().getStringRef() != opName) return failure();
        
        auto resultType = typeConverter->convertType(op->getResult(0).getType());
        
        if (opName == "huan.and") {
            rewriter.replaceOpWithNewOp<LLVM::AndOp>(op, resultType, operands[0], operands[1]);
        } else if (opName == "huan.or") {
            rewriter.replaceOpWithNewOp<LLVM::OrOp>(op, resultType, operands[0], operands[1]);
        } else if (opName == "huan.not") {
            rewriter.replaceOpWithNewOp<LLVM::XorOp>(op, resultType, operands[0], 
                rewriter.create<LLVM::ConstantOp>(op->getLoc(), resultType, rewriter.getIntegerAttr(resultType, 1)));
        }
        
        return success();
    }
    
private:
    StringRef opName;
};

// 模块降级 Pass
class LowerHuanToLLVMPass : public PassWrapper<LowerHuanToLLVMPass, OperationPass<ModuleOp>> {
public:
    MLIR_DEFINE_EXPLICIT_INTERNAL_INLINE_TYPE_ID(LowerHuanToLLVMPass)
    
    void runOnOperation() override {
        auto module = getOperation();
        auto context = module.getContext();
        
        HuanToLLVMTypeConverter typeConverter(context);
        RewritePatternSet patterns(context);
        
        // 添加降级模式
        patterns.add<LowerHLConstOp>(typeConverter, context);
        patterns.add<LowerHLBinOp>("huan.add", typeConverter, context);
        patterns.add<LowerHLBinOp>("huan.sub", typeConverter, context);
        patterns.add<LowerHLBinOp>("huan.mul", typeConverter, context);
        patterns.add<LowerHLBinOp>("huan.div", typeConverter, context);
        patterns.add<LowerHLCompareOp>("huan.eq", typeConverter, context);
        patterns.add<LowerHLCompareOp>("huan.ne", typeConverter, context);
        patterns.add<LowerHLCompareOp>("huan.lt", typeConverter, context);
        patterns.add<LowerHLCompareOp>("huan.le", typeConverter, context);
        patterns.add<LowerHLCompareOp>("huan.ge", typeConverter, context);
        patterns.add<LowerHLCompareOp>("huan.gr", typeConverter, context);
        patterns.add<LowerHLLogicalOp>("huan.and", typeConverter, context);
        patterns.add<LowerHLLogicalOp>("huan.or", typeConverter, context);
        patterns.add<LowerHLLogicalOp>("huan.not", typeConverter, context);
        
        ConversionTarget target(*context);
        target.addLegalDialect<LLVM::LLVMDialect>();
        target.addIllegalDialect<HuanDialect>();
        
        if (failed(applyPartialConversion(module, target, std::move(patterns)))) {
            signalPassFailure();
        }
    }
};

} // namespace

// 注册 Pass
std::unique_ptr<Pass> createLowerHuanToLLVMPass() {
    return std::make_unique<LowerHuanToLLVMPass>();
}

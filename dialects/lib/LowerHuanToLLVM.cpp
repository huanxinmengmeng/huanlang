// Copyright © 2026 幻心梦梦 (huanxinmengmeng)
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
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
// 幻语 MLIR 方言到 LLVM 方言的降级

#include "huan/HuanDialect.h"
#include "mlir/Transforms/DialectConversion.h"
#include "mlir/Conversion/LLVMCommon/TypeConverter.h"
#include "mlir/Conversion/LLVMCommon/Pattern.h"
#include "mlir/Dialect/LLVMIR/LLVMDialect.h"
#include "mlir/Dialect/Func/IR/FuncOps.h"

using namespace mlir;
using namespace huan;

namespace {

// 类型转换器
class HuanToLLVMTypeConverter : public LLVMTypeConverter {
public:
    using LLVMTypeConverter::LLVMTypeConverter;
    
    Type convertType(Type type) override {
        if (type.isa<HLIntType>()) {
            if (auto intType = type.dyn_cast<HLIntType>()) {
                return IntegerType::get(getContext(), intType.getWidth());
            }
        }
        if (type.isa<HLFloatType>()) {
            if (auto floatType = type.dyn_cast<HLFloatType>()) {
                switch (floatType.getWidth()) {
                    case 32: return Float32Type::get(getContext());
                    case 64: return Float64Type::get(getContext());
                    default: break;
                }
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

// 操作降级模式基类
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
        auto constOp = dyn_cast<HLConstOp>(op);
        if (!constOp) return failure();
        
        auto value = constOp.getValue();
        auto llvmType = typeConverter->convertType(constOp.getResult().getType());
        
        if (auto intAttr = value.dyn_cast<IntegerAttr>()) {
            rewriter.replaceOpWithNewOp<LLVM::ConstantOp>(op, llvmType, intAttr);
            return success();
        }
        if (auto floatAttr = value.dyn_cast<FloatAttr>()) {
            rewriter.replaceOpWithNewOp<LLVM::ConstantOp>(op, llvmType, floatAttr);
            return success();
        }
        
        return failure();
    }
};

// 变量声明降级
class LowerHLVarOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto varOp = dyn_cast<HLVarOp>(op);
        if (!varOp) return failure();
        
        auto name = varOp.getName();
        auto llvmType = typeConverter->convertType(varOp.getType());
        
        auto alloc = rewriter.create<LLVM::AllocaOp>(op->getLoc(),
            LLVM::LLVMPointerType::get(llvmType),
            llvmType,
            rewriter.create<LLVM::ConstantOp>(op->getLoc(),
                rewriter.getI32Type(),
                rewriter.getIntegerAttr(rewriter.getI32Type(), 1)));
        
        if (operands.size() == 1 && operands[0]) {
            rewriter.create<LLVM::StoreOp>(op->getLoc(), operands[0], alloc);
        }
        
        rewriter.eraseOp(op);
        return success();
    }
};

// 赋值操作降级
class LowerHLAssignOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto assignOp = dyn_cast<HLAssignOp>(op);
        if (!assignOp) return failure();
        
        auto name = assignOp.getName();
        
        auto load = rewriter.create<LLVM::LoadOp>(op->getLoc(),
            LLVM::LLVMPointerType::get(operands[0].getType()),
            operands[0]);
        
        rewriter.create<LLVM::StoreOp>(op->getLoc(), operands[1], load);
        rewriter.eraseOp(op);
        return success();
    }
};

// 二元操作降级
class LowerHLAddOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto addOp = dyn_cast<HLAddOp>(op);
        if (!addOp) return failure();
        
        auto resultType = typeConverter->convertType(addOp.getResult().getType());
        rewriter.replaceOpWithNewOp<LLVM::AddOp>(op, resultType, operands[0], operands[1]);
        return success();
    }
};

class LowerHLSubOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto subOp = dyn_cast<HLSubOp>(op);
        if (!subOp) return failure();
        
        auto resultType = typeConverter->convertType(subOp.getResult().getType());
        rewriter.replaceOpWithNewOp<LLVM::SubOp>(op, resultType, operands[0], operands[1]);
        return success();
    }
};

class LowerHLMulOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto mulOp = dyn_cast<HLMulOp>(op);
        if (!mulOp) return failure();
        
        auto resultType = typeConverter->convertType(mulOp.getResult().getType());
        rewriter.replaceOpWithNewOp<LLVM::MulOp>(op, resultType, operands[0], operands[1]);
        return success();
    }
};

class LowerHLDivOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto divOp = dyn_cast<HLDivOp>(op);
        if (!divOp) return failure();
        
        auto resultType = typeConverter->convertType(divOp.getResult().getType());
        rewriter.replaceOpWithNewOp<LLVM::SDivOp>(op, resultType, operands[0], operands[1]);
        return success();
    }
};

// 比较操作降级
class LowerHLEqOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto eqOp = dyn_cast<HLEqOp>(op);
        if (!eqOp) return failure();
        
        auto resultType = typeConverter->convertType(eqOp.getResult().getType());
        rewriter.replaceOpWithNewOp<LLVM::ICmpOp>(op, resultType, LLVM::ICmpPredicate::eq, operands[0], operands[1]);
        return success();
    }
};

class LowerHLNeOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto neOp = dyn_cast<HLNeOp>(op);
        if (!neOp) return failure();
        
        auto resultType = typeConverter->convertType(neOp.getResult().getType());
        rewriter.replaceOpWithNewOp<LLVM::ICmpOp>(op, resultType, LLVM::ICmpPredicate::ne, operands[0], operands[1]);
        return success();
    }
};

class LowerHLLtOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto ltOp = dyn_cast<HLLtOp>(op);
        if (!ltOp) return failure();
        
        auto resultType = typeConverter->convertType(ltOp.getResult().getType());
        rewriter.replaceOpWithNewOp<LLVM::ICmpOp>(op, resultType, LLVM::ICmpPredicate::slt, operands[0], operands[1]);
        return success();
    }
};

class LowerHLLeOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto leOp = dyn_cast<HLLeOp>(op);
        if (!leOp) return failure();
        
        auto resultType = typeConverter->convertType(leOp.getResult().getType());
        rewriter.replaceOpWithNewOp<LLVM::ICmpOp>(op, resultType, LLVM::ICmpPredicate::sle, operands[0], operands[1]);
        return success();
    }
};

class LowerHLGeOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto geOp = dyn_cast<HLGeOp>(op);
        if (!geOp) return failure();
        
        auto resultType = typeConverter->convertType(geOp.getResult().getType());
        rewriter.replaceOpWithNewOp<LLVM::ICmpOp>(op, resultType, LLVM::ICmpPredicate::sge, operands[0], operands[1]);
        return success();
    }
};

class LowerHLGrOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto grOp = dyn_cast<HLGrOp>(op);
        if (!grOp) return failure();
        
        auto resultType = typeConverter->convertType(grOp.getResult().getType());
        rewriter.replaceOpWithNewOp<LLVM::ICmpOp>(op, resultType, LLVM::ICmpPredicate::sgt, operands[0], operands[1]);
        return success();
    }
};

// 逻辑操作降级
class LowerHLAndOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto andOp = dyn_cast<HLAndOp>(op);
        if (!andOp) return failure();
        
        auto resultType = typeConverter->convertType(andOp.getResult().getType());
        rewriter.replaceOpWithNewOp<LLVM::AndOp>(op, resultType, operands[0], operands[1]);
        return success();
    }
};

class LowerHLOrOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto orOp = dyn_cast<HLOrOp>(op);
        if (!orOp) return failure();
        
        auto resultType = typeConverter->convertType(orOp.getResult().getType());
        rewriter.replaceOpWithNewOp<LLVM::OrOp>(op, resultType, operands[0], operands[1]);
        return success();
    }
};

class LowerHLNotOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto notOp = dyn_cast<HLNotOp>(op);
        if (!notOp) return failure();
        
        auto resultType = typeConverter->convertType(notOp.getResult().getType());
        auto one = rewriter.create<LLVM::ConstantOp>(op->getLoc(), resultType, rewriter.getIntegerAttr(resultType, 1));
        rewriter.replaceOpWithNewOp<LLVM::XorOp>(op, resultType, operands[0], one);
        return success();
    }
};

// 函数定义降级
class LowerHLFuncOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto funcOp = dyn_cast<HLFuncOp>(op);
        if (!funcOp) return failure();
        
        auto name = funcOp.getName();
        auto type = typeConverter->convertType(funcOp.getType());
        
        auto llvmFuncType = type.dyn_cast<LLVM::LLVMFunctionType>();
        if (!llvmFuncType) return failure();
        
        auto newFuncOp = rewriter.create<LLVM::LLVMFuncOp>(op->getLoc(), name, llvmFuncType);
        
        if (!funcOp.getBodyRegion().empty()) {
            auto &bodyRegion = funcOp.getBodyRegion();
            auto &newBodyRegion = newFuncOp.getBody();
            rewriter.inlineRegionBefore(bodyRegion, newBodyRegion, newBodyRegion.end());
        }
        
        rewriter.eraseOp(op);
        return success();
    }
};

// 函数调用降级
class LowerHLCallOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto callOp = dyn_cast<HLCallOp>(op);
        if (!callOp) return failure();
        
        auto name = callOp.getName();
        auto resultType = typeConverter->convertType(callOp.getResult().getType());
        
        SmallVector<Value> args(operands.begin(), operands.end());
        auto callee = rewriter.create<LLVM::LLVMFuncOp>(op->getLoc(), name,
            LLVM::LLVMPointerType::get(LLVM::LLVMFunctionType::get(resultType, {})));
        
        rewriter.replaceOpWithNewOp<LLVM::CallOp>(op, resultType, callee, args);
        return success();
    }
};

// 返回操作降级
class LowerHLReturnOp : public LowerHuanToLLVMPattern {
public:
    using LowerHuanToLLVMPattern::LowerHuanToLLVMPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto returnOp = dyn_cast<HLReturnOp>(op);
        if (!returnOp) return failure();
        
        if (operands.empty()) {
            rewriter.replaceOpWithNewOp<LLVM::ReturnOp>(op, ValueRange{});
        } else {
            rewriter.replaceOpWithNewOp<LLVM::ReturnOp>(op, operands);
        }
        
        return success();
    }
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
        patterns.add<LowerHLVarOp>(typeConverter, context);
        patterns.add<LowerHLAssignOp>(typeConverter, context);
        patterns.add<LowerHLAddOp>(typeConverter, context);
        patterns.add<LowerHLSubOp>(typeConverter, context);
        patterns.add<LowerHLMulOp>(typeConverter, context);
        patterns.add<LowerHLDivOp>(typeConverter, context);
        patterns.add<LowerHLEqOp>(typeConverter, context);
        patterns.add<LowerHLNeOp>(typeConverter, context);
        patterns.add<LowerHLLtOp>(typeConverter, context);
        patterns.add<LowerHLLeOp>(typeConverter, context);
        patterns.add<LowerHLGeOp>(typeConverter, context);
        patterns.add<LowerHLGrOp>(typeConverter, context);
        patterns.add<LowerHLAndOp>(typeConverter, context);
        patterns.add<LowerHLOrOp>(typeConverter, context);
        patterns.add<LowerHLNotOp>(typeConverter, context);
        patterns.add<LowerHLFuncOp>(typeConverter, context);
        patterns.add<LowerHLCallOp>(typeConverter, context);
        patterns.add<LowerHLReturnOp>(typeConverter, context);
        
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

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
// 幻语 MLIR 方言到 Arith 方言的降级

#include "huan/HuanDialect.h"
#include "mlir/Transforms/DialectConversion.h"
#include "mlir/Dialect/Arith/IR/Arith.h"

using namespace mlir;
using namespace huan;

namespace {

// 操作降级模式基类
class LowerHuanToArithPattern : public ConversionPattern {
public:
    LowerHuanToArithPattern(MLIRContext *context) 
        : ConversionPattern(PatternBenefit::High, context) {}
};

// 常量降级到 Arith
class LowerHLConstOp : public LowerHuanToArithPattern {
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

// 二元操作降级到 Arith
class LowerHLAddOp : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto addOp = dyn_cast<HLAddOp>(op);
        if (!addOp) return failure();
        
        auto resultType = addOp.getResult().getType();
        rewriter.replaceOpWithNewOp<arith::AddIOp>(op, resultType, operands[0], operands[1]);
        return success();
    }
};

class LowerHLSubOp : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto subOp = dyn_cast<HLSubOp>(op);
        if (!subOp) return failure();
        
        auto resultType = subOp.getResult().getType();
        rewriter.replaceOpWithNewOp<arith::SubIOp>(op, resultType, operands[0], operands[1]);
        return success();
    }
};

class LowerHLMulOp : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto mulOp = dyn_cast<HLMulOp>(op);
        if (!mulOp) return failure();
        
        auto resultType = mulOp.getResult().getType();
        rewriter.replaceOpWithNewOp<arith::MulIOp>(op, resultType, operands[0], operands[1]);
        return success();
    }
};

class LowerHLDivOp : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto divOp = dyn_cast<HLDivOp>(op);
        if (!divOp) return failure();
        
        auto resultType = divOp.getResult().getType();
        rewriter.replaceOpWithNewOp<arith::DivSIOp>(op, resultType, operands[0], operands[1]);
        return success();
    }
};

// 比较操作降级到 Arith
class LowerHLEqOp : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto eqOp = dyn_cast<HLEqOp>(op);
        if (!eqOp) return failure();
        
        auto resultType = eqOp.getResult().getType();
        rewriter.replaceOpWithNewOp<arith::CmpIOp>(op, arith::CmpIPredicate::eq, operands[0], operands[1]);
        return success();
    }
};

class LowerHLNeOp : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto neOp = dyn_cast<HLNeOp>(op);
        if (!neOp) return failure();
        
        auto resultType = neOp.getResult().getType();
        rewriter.replaceOpWithNewOp<arith::CmpIOp>(op, arith::CmpIPredicate::ne, operands[0], operands[1]);
        return success();
    }
};

class LowerHLLtOp : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto ltOp = dyn_cast<HLLtOp>(op);
        if (!ltOp) return failure();
        
        auto resultType = ltOp.getResult().getType();
        rewriter.replaceOpWithNewOp<arith::CmpIOp>(op, arith::CmpIPredicate::slt, operands[0], operands[1]);
        return success();
    }
};

class LowerHLLeOp : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto leOp = dyn_cast<HLLeOp>(op);
        if (!leOp) return failure();
        
        auto resultType = leOp.getResult().getType();
        rewriter.replaceOpWithNewOp<arith::CmpIOp>(op, arith::CmpIPredicate::sle, operands[0], operands[1]);
        return success();
    }
};

class LowerHLGeOp : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto geOp = dyn_cast<HLGeOp>(op);
        if (!geOp) return failure();
        
        auto resultType = geOp.getResult().getType();
        rewriter.replaceOpWithNewOp<arith::CmpIOp>(op, arith::CmpIPredicate::sge, operands[0], operands[1]);
        return success();
    }
};

class LowerHLGrOp : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto grOp = dyn_cast<HLGrOp>(op);
        if (!grOp) return failure();
        
        auto resultType = grOp.getResult().getType();
        rewriter.replaceOpWithNewOp<arith::CmpIOp>(op, arith::CmpIPredicate::sgt, operands[0], operands[1]);
        return success();
    }
};

// 逻辑操作降级到 Arith
class LowerHLAndOp : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto andOp = dyn_cast<HLAndOp>(op);
        if (!andOp) return failure();
        
        auto resultType = andOp.getResult().getType();
        rewriter.replaceOpWithNewOp<arith::AndIOp>(op, resultType, operands[0], operands[1]);
        return success();
    }
};

class LowerHLOrOp : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto orOp = dyn_cast<HLOrOp>(op);
        if (!orOp) return failure();
        
        auto resultType = orOp.getResult().getType();
        rewriter.replaceOpWithNewOp<arith::OrIOp>(op, resultType, operands[0], operands[1]);
        return success();
    }
};

class LowerHLNotOp : public LowerHuanToArithPattern {
public:
    using LowerHuanToArithPattern::LowerHuanToArithPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto notOp = dyn_cast<HLNotOp>(op);
        if (!notOp) return failure();
        
        auto resultType = notOp.getResult().getType();
        auto one = rewriter.create<arith::ConstantOp>(op->getLoc(), resultType, rewriter.getIntegerAttr(resultType, 1));
        rewriter.replaceOpWithNewOp<arith::XOrIOp>(op, resultType, operands[0], one);
        return success();
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
        patterns.add<LowerHLConstOp>(context);
        patterns.add<LowerHLAddOp>(context);
        patterns.add<LowerHLSubOp>(context);
        patterns.add<LowerHLMulOp>(context);
        patterns.add<LowerHLDivOp>(context);
        patterns.add<LowerHLEqOp>(context);
        patterns.add<LowerHLNeOp>(context);
        patterns.add<LowerHLLtOp>(context);
        patterns.add<LowerHLLeOp>(context);
        patterns.add<LowerHLGeOp>(context);
        patterns.add<LowerHLGrOp>(context);
        patterns.add<LowerHLAndOp>(context);
        patterns.add<LowerHLOrOp>(context);
        patterns.add<LowerHLNotOp>(context);
        
        ConversionTarget target(*context);
        target.addLegalDialect<arith::ArithDialect>();
        target.addLegalDialect<scf::SCFDialect>();
        target.addLegalDialect<func::FuncDialect>();
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

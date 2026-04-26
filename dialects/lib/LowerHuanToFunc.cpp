// Copyright © 2026 幻心梦梦 (huanxinmengmeng)
// Licensed under the Apache License, Version 2.0 (the "License");
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
// 幻语 MLIR 方言到 Func 方言的降级

#include "huan/HuanDialect.h"
#include "mlir/Transforms/DialectConversion.h"
#include "mlir/Dialect/Func/IR/FuncOps.h"

using namespace mlir;
using namespace huan;

namespace {

// 操作降级模式
class LowerHuanToFuncPattern : public ConversionPattern {
public:
    LowerHuanToFuncPattern(MLIRContext *context) : ConversionPattern(PatternBenefit::High, context) {}
};

// 函数定义降级到 Func
class LowerHLFuncOpToFunc : public LowerHuanToFuncPattern {
public:
    using LowerHuanToFuncPattern::LowerHuanToFuncPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto funcOp = dyn_cast<HLFuncOp>(op);
        if (!funcOp) return failure();
        
        auto name = funcOp.getName();
        auto type = funcOp.getType();
        
        // 创建 Func 函数
        auto funcType = type.cast<FunctionType>();
        auto newFuncOp = rewriter.create<func::FuncOp>(op->getLoc(), name, funcType);
        
        // 复制函数体
        if (funcOp.getBody()) {
            auto &body = funcOp.getBody();
            auto &newBody = newFuncOp.getBody();
            
            // 复制函数体内容
            for (auto &innerOp : body.front().getOperations()) {
                rewriter.clone(innerOp);
            }
        }
        
        rewriter.eraseOp(op);
        return success();
    }
};

// 函数调用降级到 Func
class LowerHLCallOpToFunc : public LowerHuanToFuncPattern {
public:
    using LowerHuanToFuncPattern::LowerHuanToFuncPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto callOp = dyn_cast<HLCallOp>(op);
        if (!callOp) return failure();
        
        auto name = callOp.getName();
        auto args = operands;
        
        // 创建 Func 调用
        auto resultType = callOp.getResult().getType();
        auto newCallOp = rewriter.create<func::CallOp>(op->getLoc(), name, resultType, args);
        
        rewriter.replaceOp(op, newCallOp.getResults());
        return success();
    }
};

// 返回操作降级到 Func
class LowerHLReturnOpToFunc : public LowerHuanToFuncPattern {
public:
    using LowerHuanToFuncPattern::LowerHuanToFuncPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto returnOp = dyn_cast<HLReturnOp>(op);
        if (!returnOp) return failure();
        
        // 创建 Func 返回操作
        if (returnOp.getValue()) {
            rewriter.replaceOpWithNewOp<func::ReturnOp>(op, operands[0]);
        } else {
            rewriter.replaceOpWithNewOp<func::ReturnOp>(op);
        }
        
        return success();
    }
};

// 模块降级 Pass
class LowerHuanToFuncPass : public PassWrapper<LowerHuanToFuncPass, OperationPass<ModuleOp>> {
public:
    MLIR_DEFINE_EXPLICIT_INTERNAL_INLINE_TYPE_ID(LowerHuanToFuncPass)
    
    void runOnOperation() override {
        auto module = getOperation();
        auto context = module.getContext();
        
        RewritePatternSet patterns(context);
        
        // 添加降级模式
        patterns.add<LowerHLFuncOpToFunc>(context);
        patterns.add<LowerHLCallOpToFunc>(context);
        patterns.add<LowerHLReturnOpToFunc>(context);
        
        ConversionTarget target(*context);
        target.addLegalDialect<func::FuncDialect>();
        target.addIllegalOp<HLFuncOp>();
        target.addIllegalOp<HLCallOp>();
        target.addIllegalOp<HLReturnOp>();
        
        if (failed(applyPartialConversion(module, target, std::move(patterns)))) {
            signalPassFailure();
        }
    }
};

} // namespace

// 注册 Pass
std::unique_ptr<Pass> createLowerHuanToFuncPass() {
    return std::make_unique<LowerHuanToFuncPass>();
}

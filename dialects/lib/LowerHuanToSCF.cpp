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
// 幻语 MLIR 方言到 SCF 方言的降级

#include "huan/HuanDialect.h"
#include "mlir/Transforms/DialectConversion.h"
#include "mlir/Dialect/SCF/IR/SCF.h"

using namespace mlir;
using namespace huan;

namespace {

// 类型转换器
class HuanToSCFTypeConverter : public TypeConverter {
public:
    HuanToSCFTypeConverter() {
        // 添加类型转换回调
        addConversion([](Type type) -> Type {
            return type;
        });
    }
};

// 操作降级模式基类
class LowerHuanToSCFPattern : public ConversionPattern {
public:
    LowerHuanToSCFPattern(MLIRContext *context) 
        : ConversionPattern(PatternBenefit::High, context) {}
};

// 条件判断降级
class LowerHLIfOp : public LowerHuanToSCFPattern {
public:
    using LowerHuanToSCFPattern::LowerHuanToSCFPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto ifOp = dyn_cast<HLIfOp>(op);
        if (!ifOp) return failure();
        
        auto condition = operands[0];
        
        // 创建 SCF if 操作
        auto scfIfOp = rewriter.create<scf::IfOp>(op->getLoc(), 
            TypeRange{}, 
            condition, 
            /*hasElse*/ !ifOp.getElseRegion().empty());
        
        // 构建 then 分支
        auto &thenRegion = scfIfOp.getThenRegion();
        if (!thenRegion.empty()) {
            rewriter.eraseBlock(&thenRegion.front());
        }
        Block *thenBlock = rewriter.createBlock(&thenRegion);
        rewriter.setInsertionPointToStart(thenBlock);
        
        // 复制原 then 分支内容
        if (!ifOp.getThenRegion().empty()) {
            auto &srcThenRegion = ifOp.getThenRegion();
            if (!srcThenRegion.empty()) {
                for (auto &innerOp : srcThenRegion.front().getOperations()) {
                    if (!innerOp.hasTrait<OpTrait::IsTerminator>()) {
                        rewriter.clone(innerOp);
                    }
                }
            }
        }
        
        rewriter.create<scf::YieldOp>(op->getLoc(), ValueRange{});
        
        // 构建 else 分支（如果存在）
        if (!ifOp.getElseRegion().empty()) {
            auto &elseRegion = scfIfOp.getElseRegion();
            if (!elseRegion.empty()) {
                rewriter.eraseBlock(&elseRegion.front());
            }
            Block *elseBlock = rewriter.createBlock(&elseRegion);
            rewriter.setInsertionPointToStart(elseBlock);
            
            // 复制原 else 分支内容
            auto &srcElseRegion = ifOp.getElseRegion();
            if (!srcElseRegion.empty()) {
                for (auto &innerOp : srcElseRegion.front().getOperations()) {
                    if (!innerOp.hasTrait<OpTrait::IsTerminator>()) {
                        rewriter.clone(innerOp);
                    }
                }
            }
            
            rewriter.create<scf::YieldOp>(op->getLoc(), ValueRange{});
        }
        
        rewriter.eraseOp(op);
        return success();
    }
};

// 循环降级
class LowerHLLoopOp : public LowerHuanToSCFPattern {
public:
    using LowerHuanToSCFPattern::LowerHuanToSCFPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto loopOp = dyn_cast<HLLoopOp>(op);
        if (!loopOp) return failure();
        
        auto condition = operands[0];
        
        // 创建 SCF while 循环
        auto whileOp = rewriter.create<scf::WhileOp>(op->getLoc(), 
            TypeRange{}, 
            ValueRange{});
        
        // 构建 before 区域（条件检查）
        auto &beforeRegion = whileOp.getBefore();
        Block *beforeBlock = rewriter.createBlock(&beforeRegion);
        rewriter.setInsertionPointToStart(beforeBlock);
        
        rewriter.create<scf::ConditionOp>(op->getLoc(), condition, ValueRange{});
        
        // 构建 after 区域（循环体）
        auto &afterRegion = whileOp.getAfter();
        Block *afterBlock = rewriter.createBlock(&afterRegion);
        rewriter.setInsertionPointToStart(afterBlock);
        
        // 复制原循环体内容
        if (!loopOp.getBody().empty()) {
            auto &srcBody = loopOp.getBody();
            if (!srcBody.empty()) {
                for (auto &innerOp : srcBody.front().getOperations()) {
                    if (!innerOp.hasTrait<OpTrait::IsTerminator>()) {
                        rewriter.clone(innerOp);
                    }
                }
            }
        }
        
        rewriter.create<scf::YieldOp>(op->getLoc(), ValueRange{});
        
        rewriter.eraseOp(op);
        return success();
    }
};

// 模块降级 Pass
class LowerHuanToSCFPass : public PassWrapper<LowerHuanToSCFPass, OperationPass<ModuleOp>> {
public:
    MLIR_DEFINE_EXPLICIT_INTERNAL_INLINE_TYPE_ID(LowerHuanToSCFPass)
    
    void runOnOperation() override {
        auto module = getOperation();
        auto context = module.getContext();
        
        HuanToSCFTypeConverter typeConverter;
        RewritePatternSet patterns(context);
        
        // 添加降级模式
        patterns.add<LowerHLIfOp>(context);
        patterns.add<LowerHLLoopOp>(context);
        
        ConversionTarget target(*context);
        target.addLegalDialect<scf::SCFDialect>();
        target.addLegalDialect<arith::ArithDialect>();
        target.addLegalDialect<func::FuncDialect>();
        target.addIllegalOp<HLIfOp>();
        target.addIllegalOp<HLLoopOp>();
        
        if (failed(applyPartialConversion(module, target, std::move(patterns)))) {
            signalPassFailure();
        }
    }
};

} // namespace

// 注册 Pass
std::unique_ptr<Pass> createLowerHuanToSCFPass() {
    return std::make_unique<LowerHuanToSCFPass>();
}

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
// 幻语 MLIR 方言到 SCF 方言的降级

#include "huan/HuanDialect.h"
#include "mlir/Transforms/DialectConversion.h"
#include "mlir/Dialect/SCF/SCF.h"

using namespace mlir;
using namespace huan;

namespace {

// 类型转换器
class HuanToSCFTypeConverter : public TypeConverter {
public:
    HuanToSCFTypeConverter() {}
};

// 操作降级模式
class LowerHuanToSCFPattern : public ConversionPattern {
public:
    LowerHuanToSCFPattern(MLIRContext *context) : ConversionPattern(PatternBenefit::High, context) {}
};

// 循环降级
class LowerHLLoopOp : public LowerHuanToSCFPattern {
public:
    using LowerHuanToSCFPattern::LowerHuanToSCFPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto loopOp = cast<HLLoopOp>(op);
        auto condition = operands[0];
        
        // 创建 SCF while 循环
        auto whileOp = rewriter.create<scf::WhileOp>(op->getLoc(), 
            loopOp.getCondition().getType(), 
            ValueRange{}, 
            [&](OpBuilder &builder, Location loc, ValueRange args) {
                // 条件分支
                auto condition = builder.create<mlir::Value>(loc, loopOp.getCondition().getType());
                builder.create<scf::ConditionOp>(loc, condition, ValueRange{});
            },
            [&](OpBuilder &builder, Location loc, ValueRange args) {
                // 循环体
                auto bodyBuilder = rewriter.saveInsertionPoint();
                rewriter.setInsertionPointToStart(&loopOp.getBody().front());
                
                // 复制循环体内容
                for (auto &innerOp : loopOp.getBody().front().getOperations()) {
                    rewriter.clone(innerOp);
                }
                
                rewriter.restoreInsertionPoint(bodyBuilder);
                builder.create<scf::YieldOp>(loc, ValueRange{});
            });
        
        rewriter.eraseOp(op);
        return success();
    }
};

// 条件判断降级
class LowerHLIfOp : public LowerHuanToSCFPattern {
public:
    using LowerHuanToSCFPattern::LowerHuanToSCFPattern;
    
    LogicalResult matchAndRewrite(Operation *op, ArrayRef<Value> operands, ConversionPatternRewriter &rewriter) const override {
        auto ifOp = cast<HLIfOp>(op);
        auto condition = operands[0];
        
        // 创建 SCF if 操作
        auto scfIfOp = rewriter.create<scf::IfOp>(op->getLoc(), 
            TypeRange{}, 
            condition, 
            [&](OpBuilder &builder, Location loc) {
                // then 分支
                auto thenBuilder = rewriter.saveInsertionPoint();
                rewriter.setInsertionPointToStart(&ifOp.getThenRegion().front());
                
                // 复制 then 分支内容
                for (auto &innerOp : ifOp.getThenRegion().front().getOperations()) {
                    rewriter.clone(innerOp);
                }
                
                rewriter.restoreInsertionPoint(thenBuilder);
                builder.create<scf::YieldOp>(loc, ValueRange{});
            },
            [&](OpBuilder &builder, Location loc) {
                // else 分支（如果存在）
                if (ifOp.getElseRegion()) {
                    auto elseBuilder = rewriter.saveInsertionPoint();
                    rewriter.setInsertionPointToStart(&ifOp.getElseRegion()->front());
                    
                    // 复制 else 分支内容
                    for (auto &innerOp : ifOp.getElseRegion()->front().getOperations()) {
                        rewriter.clone(innerOp);
                    }
                    
                    rewriter.restoreInsertionPoint(elseBuilder);
                }
                builder.create<scf::YieldOp>(loc, ValueRange{});
            });
        
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
        patterns.add<LowerHLLoopOp>(context);
        patterns.add<LowerHLIfOp>(context);
        
        ConversionTarget target(*context);
        target.addLegalDialect<scf::SCFDialect>();
        target.addIllegalOp<HLLoopOp>();
        target.addIllegalOp<HLIfOp>();
        
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

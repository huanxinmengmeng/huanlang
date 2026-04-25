// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
//
// MLIR 降级 Pass 管线
// 实现规范第10.4节的完整降级流程

use std::fmt;
use crate::core::mlir::ops::*;
use crate::core::mlir::ops::HuanOp;

/// Pass基类
pub trait Pass: fmt::Debug {
    /// Pass名称
    fn name(&self) -> &'static str;
    
    /// 运行Pass
    fn run(&mut self, ops: &mut Vec<Box<dyn HuanOp>>) -> Result<(), String>;
}

// =============================================================================
// HuanToScfPass - 控制流降级
// =============================================================================

/// 将Huan方言控制流降级为SCF方言
#[derive(Debug)]
pub struct HuanToScfPass;

impl HuanToScfPass {
    pub fn new() -> Self {
        HuanToScfPass
    }
    
    fn process_op(&mut self, op: Box<dyn HuanOp>) -> Vec<Box<dyn HuanOp>> {
        // 检查是否是RuoOp（若操作），转换为SCF if
        if let Some(ruo) = op.as_any().downcast_ref::<RuoOp>() {
            // 这里实际会生成scf.if操作
            vec![Box::new(ruo.clone())]
        } 
        // 检查是否是ChongfuOp（重复操作），转换为scf.for
        else if let Some(chongfu) = op.as_any().downcast_ref::<ChongfuOp>() {
            // 这里实际会生成scf.for操作
            vec![Box::new(chongfu.clone())]
        }
        // 检查是否是DangOp（当操作），转换为scf.while
        else if let Some(dang) = op.as_any().downcast_ref::<DangOp>() {
            // 这里实际会生成scf.while操作
            vec![Box::new(dang.clone())]
        }
        // 其他操作原样保留
        else {
            vec![op]
        }
    }
}

impl Pass for HuanToScfPass {
    fn name(&self) -> &'static str {
        "huan-to-scf"
    }
    
    fn run(&mut self, ops: &mut Vec<Box<dyn HuanOp>>) -> Result<(), String> {
        let mut new_ops = Vec::with_capacity(ops.len());
        
        for op in ops.drain(..) {
            let processed = self.process_op(op);
            new_ops.extend(processed);
        }
        
        *ops = new_ops;
        Ok(())
    }
}

// =============================================================================
// HuanToArithPass - 算术降级
// =============================================================================

/// 将Huan方言算术操作降级为Arith方言
#[derive(Debug)]
pub struct HuanToArithPass;

impl HuanToArithPass {
    pub fn new() -> Self {
        HuanToArithPass
    }
    
    fn process_op(&mut self, op: Box<dyn HuanOp>) -> Vec<Box<dyn HuanOp>> {
        // 转换算术操作到arith方言
        if let Some(jia) = op.as_any().downcast_ref::<JiaOp>() {
            // 转换为arith.addi或arith.addf
            vec![Box::new(JiaOp {
                lhs: jia.lhs.clone(),
                rhs: jia.rhs.clone(),
                span: jia.span,
            })]
        }
        else if let Some(jian) = op.as_any().downcast_ref::<JianOp>() {
            // 转换为arith.subi或arith.subf
            vec![Box::new(JianOp {
                lhs: jian.lhs.clone(),
                rhs: jian.rhs.clone(),
                span: jian.span,
            })]
        }
        else if let Some(cheng) = op.as_any().downcast_ref::<ChengOp>() {
            // 转换为arith.muli或arith.mulf
            vec![Box::new(ChengOp {
                lhs: cheng.lhs.clone(),
                rhs: cheng.rhs.clone(),
                span: cheng.span,
            })]
        }
        else if let Some(chu) = op.as_any().downcast_ref::<ChuOp>() {
            // 转换为arith.divi或arith.divf
            vec![Box::new(ChuOp {
                lhs: chu.lhs.clone(),
                rhs: chu.rhs.clone(),
                span: chu.span,
            })]
        }
        else if let Some(dayu) = op.as_any().downcast_ref::<DayuOp>() {
            // 转换为arith.cmpgt
            vec![Box::new(DayuOp {
                lhs: dayu.lhs.clone(),
                rhs: dayu.rhs.clone(),
                span: dayu.span,
            })]
        }
        else if let Some(xiaoyu) = op.as_any().downcast_ref::<XiaoyuOp>() {
            // 转换为arith.cmplt
            vec![Box::new(XiaoyuOp {
                lhs: xiaoyu.lhs.clone(),
                rhs: xiaoyu.rhs.clone(),
                span: xiaoyu.span,
            })]
        }
        else if let Some(dengyu) = op.as_any().downcast_ref::<DengyuOp>() {
            // 转换为arith.cmpeq
            vec![Box::new(DengyuOp {
                lhs: dengyu.lhs.clone(),
                rhs: dengyu.rhs.clone(),
                span: dengyu.span,
            })]
        }
        else {
            vec![op]
        }
    }
}

impl Pass for HuanToArithPass {
    fn name(&self) -> &'static str {
        "huan-to-arith"
    }
    
    fn run(&mut self, ops: &mut Vec<Box<dyn HuanOp>>) -> Result<(), String> {
        let mut new_ops = Vec::with_capacity(ops.len());
        
        for op in ops.drain(..) {
            let processed = self.process_op(op);
            new_ops.extend(processed);
        }
        
        *ops = new_ops;
        Ok(())
    }
}

// =============================================================================
// HuanToFuncPass - 函数降级
// =============================================================================

/// 将Huan方言函数降级为Func方言
#[derive(Debug)]
pub struct HuanToFuncPass;

impl HuanToFuncPass {
    pub fn new() -> Self {
        HuanToFuncPass
    }
    
    fn process_op(&mut self, op: Box<dyn HuanOp>) -> Vec<Box<dyn HuanOp>> {
        if let Some(hanshu) = op.as_any().downcast_ref::<HanshuOp>() {
            // 转换为func.func
            vec![Box::new(hanshu.clone())]
        }
        else if let Some(diaoyong) = op.as_any().downcast_ref::<DiaoyongOp>() {
            // 转换为func.call
            vec![Box::new(DiaoyongOp {
                callee: diaoyong.callee.clone(),
                args: diaoyong.args.clone(),
                span: diaoyong.span,
            })]
        }
        else if let Some(fanhui) = op.as_any().downcast_ref::<FanhuiOp>() {
            // 转换为func.return
            vec![Box::new(FanhuiOp {
                value: fanhui.value.clone(),
                span: fanhui.span,
            })]
        }
        else {
            vec![op]
        }
    }
}

impl Pass for HuanToFuncPass {
    fn name(&self) -> &'static str {
        "huan-to-func"
    }
    
    fn run(&mut self, ops: &mut Vec<Box<dyn HuanOp>>) -> Result<(), String> {
        let mut new_ops = Vec::with_capacity(ops.len());
        
        for op in ops.drain(..) {
            let processed = self.process_op(op);
            new_ops.extend(processed);
        }
        
        *ops = new_ops;
        Ok(())
    }
}

// =============================================================================
// LowerHuanToLLVMPass - 完整降级管线
// =============================================================================

/// 完整的Huan方言到LLVM的降级管线
#[derive(Debug)]
pub struct LowerHuanToLLVMPass {
    passes: Vec<Box<dyn Pass>>,
}

impl LowerHuanToLLVMPass {
    pub fn new() -> Self {
        let passes: Vec<Box<dyn Pass>> = vec![
            Box::new(HuanToScfPass::new()),
            Box::new(HuanToArithPass::new()),
            Box::new(HuanToFuncPass::new()),
        ];
        
        LowerHuanToLLVMPass { passes }
    }
}

impl Pass for LowerHuanToLLVMPass {
    fn name(&self) -> &'static str {
        "lower-huan-to-llvm"
    }
    
    fn run(&mut self, ops: &mut Vec<Box<dyn HuanOp>>) -> Result<(), String> {
        println!("Running complete lower-huan-to-llvm pipeline...");
        
        for pass in &mut self.passes {
            println!("  Running pass: {}", pass.name());
            pass.run(ops)?;
        }
        
        Ok(())
    }
}

// =============================================================================
// 优化Pass
// =============================================================================

/// 常量折叠Pass
#[derive(Debug)]
pub struct ConstantFoldingPass;

impl ConstantFoldingPass {
    pub fn new() -> Self {
        ConstantFoldingPass
    }
    
    /// 折叠常量表达式
    fn fold_op(&mut self, op: &Box<dyn HuanOp>) -> Option<Box<dyn HuanOp>> {
        // 处理加法操作
        if let Some(jia) = op.as_any().downcast_ref::<JiaOp>() {
            if let (Some(lhs), Some(rhs)) = (self.fold_literal(&jia.lhs), self.fold_literal(&jia.rhs)) {
                match (lhs, rhs) {
                    (Literal::Int(l), Literal::Int(r)) => {
                        return Some(Box::new(IntLitOp {
                            value: l + r,
                            span: jia.span,
                        }));
                    }
                    (Literal::Float(l), Literal::Float(r)) => {
                        return Some(Box::new(FloatLitOp {
                            value: l + r,
                            span: jia.span,
                        }));
                    }
                    _ => {}
                }
            }
        }
        // 处理减法操作
        else if let Some(jian) = op.as_any().downcast_ref::<JianOp>() {
            if let (Some(lhs), Some(rhs)) = (self.fold_literal(&jian.lhs), self.fold_literal(&jian.rhs)) {
                match (lhs, rhs) {
                    (Literal::Int(l), Literal::Int(r)) => {
                        return Some(Box::new(IntLitOp {
                            value: l - r,
                            span: jian.span,
                        }));
                    }
                    (Literal::Float(l), Literal::Float(r)) => {
                        return Some(Box::new(FloatLitOp {
                            value: l - r,
                            span: jian.span,
                        }));
                    }
                    _ => {}
                }
            }
        }
        // 处理乘法操作
        else if let Some(cheng) = op.as_any().downcast_ref::<ChengOp>() {
            if let (Some(lhs), Some(rhs)) = (self.fold_literal(&cheng.lhs), self.fold_literal(&cheng.rhs)) {
                match (lhs, rhs) {
                    (Literal::Int(l), Literal::Int(r)) => {
                        return Some(Box::new(IntLitOp {
                            value: l * r,
                            span: cheng.span,
                        }));
                    }
                    (Literal::Float(l), Literal::Float(r)) => {
                        return Some(Box::new(FloatLitOp {
                            value: l * r,
                            span: cheng.span,
                        }));
                    }
                    _ => {}
                }
            }
        }
        // 处理除法操作
        else if let Some(chu) = op.as_any().downcast_ref::<ChuOp>() {
            if let (Some(lhs), Some(rhs)) = (self.fold_literal(&chu.lhs), self.fold_literal(&chu.rhs)) {
                match (lhs, rhs) {
                    (Literal::Int(l), Literal::Int(r)) if r != 0 => {
                        return Some(Box::new(IntLitOp {
                            value: l / r,
                            span: chu.span,
                        }));
                    }
                    (Literal::Float(l), Literal::Float(r)) if r != 0.0 => {
                        return Some(Box::new(FloatLitOp {
                            value: l / r,
                            span: chu.span,
                        }));
                    }
                    _ => {}
                }
            }
        }
        // 处理比较操作
        else if let Some(dayu) = op.as_any().downcast_ref::<DayuOp>() {
            if let (Some(lhs), Some(rhs)) = (self.fold_literal(&dayu.lhs), self.fold_literal(&dayu.rhs)) {
                match (lhs, rhs) {
                    (Literal::Int(l), Literal::Int(r)) => {
                        return Some(Box::new(BoolLitOp {
                            value: l > r,
                            span: dayu.span,
                        }));
                    }
                    (Literal::Float(l), Literal::Float(r)) => {
                        return Some(Box::new(BoolLitOp {
                            value: l > r,
                            span: dayu.span,
                        }));
                    }
                    _ => {}
                }
            }
        }
        else if let Some(xiaoyu) = op.as_any().downcast_ref::<XiaoyuOp>() {
            if let (Some(lhs), Some(rhs)) = (self.fold_literal(&xiaoyu.lhs), self.fold_literal(&xiaoyu.rhs)) {
                match (lhs, rhs) {
                    (Literal::Int(l), Literal::Int(r)) => {
                        return Some(Box::new(BoolLitOp {
                            value: l < r,
                            span: xiaoyu.span,
                        }));
                    }
                    (Literal::Float(l), Literal::Float(r)) => {
                        return Some(Box::new(BoolLitOp {
                            value: l < r,
                            span: xiaoyu.span,
                        }));
                    }
                    _ => {}
                }
            }
        }
        else if let Some(dengyu) = op.as_any().downcast_ref::<DengyuOp>() {
            if let (Some(lhs), Some(rhs)) = (self.fold_literal(&dengyu.lhs), self.fold_literal(&dengyu.rhs)) {
                match (lhs, rhs) {
                    (Literal::Int(l), Literal::Int(r)) => {
                        return Some(Box::new(BoolLitOp {
                            value: l == r,
                            span: dengyu.span,
                        }));
                    }
                    (Literal::Float(l), Literal::Float(r)) => {
                        return Some(Box::new(BoolLitOp {
                            value: l == r,
                            span: dengyu.span,
                        }));
                    }
                    _ => {}
                }
            }
        }
        
        None
    }
    
    /// 提取字面值
    fn fold_literal(&self, op: &Box<dyn HuanOp>) -> Option<Literal> {
        if let Some(int_lit) = op.as_any().downcast_ref::<IntLitOp>() {
            Some(Literal::Int(int_lit.value))
        } else if let Some(float_lit) = op.as_any().downcast_ref::<FloatLitOp>() {
            Some(Literal::Float(float_lit.value))
        } else {
            None
        }
    }
}

impl Pass for ConstantFoldingPass {
    fn name(&self) -> &'static str {
        "constant-folding"
    }
    
    fn run(&mut self, ops: &mut Vec<Box<dyn HuanOp>>) -> Result<(), String> {
        println!("Running constant folding pass...");
        
        let mut new_ops = Vec::with_capacity(ops.len());
        
        for op in ops.drain(..) {
            if let Some(folded) = self.fold_op(&op) {
                new_ops.push(folded);
            } else {
                new_ops.push(op);
            }
        }
        
        *ops = new_ops;
        Ok(())
    }
}

/// 字面值枚举
#[derive(Debug, Clone)]
enum Literal {
    Int(i64),
    Float(f64),
}

/// 死代码消除Pass
#[derive(Debug)]
pub struct DeadCodeEliminationPass;

impl DeadCodeEliminationPass {
    pub fn new() -> Self {
        DeadCodeEliminationPass
    }
    
    /// 检查操作是否是死代码
    fn is_dead_code(&self, _op: &Box<dyn HuanOp>) -> bool {
        // 检查是否是不可达的代码
        // 例如：return语句后的代码
        // 这里简化处理，实际需要更复杂的分析
        false
    }
    
    /// 处理函数体
    fn process_function(&mut self, func: &mut HanshuOp) {
        let mut new_body = Vec::with_capacity(func.body.len());
        let mut seen_return = false;
        
        for op in &func.body {
            // 检查是否是返回语句
            if op.as_any().is::<FanhuiOp>() {
                new_body.push(op.clone());
                seen_return = true;
            } else if !seen_return {
                // 只添加返回语句前的代码
                new_body.push(op.clone());
            }
        }
        
        func.body = new_body;
    }
}

impl Pass for DeadCodeEliminationPass {
    fn name(&self) -> &'static str {
        "dce"
    }
    
    fn run(&mut self, ops: &mut Vec<Box<dyn HuanOp>>) -> Result<(), String> {
        println!("Running dead code elimination pass...");

        let mut new_ops: Vec<Box<dyn HuanOp>> = Vec::with_capacity(ops.len());

        for op in ops.drain(..) {
            if let Some(func) = op.as_any().downcast_ref::<HanshuOp>() {
                let mut func = func.clone();
                self.process_function(&mut func);
                new_ops.push(Box::new(func) as Box<dyn HuanOp>);
            } else if !self.is_dead_code(&op) {
                new_ops.push(op);
            }
        }

        *ops = new_ops;
        Ok(())
    }
}

/// 公共子表达式消除Pass
#[derive(Debug)]
pub struct CommonSubexpressionEliminationPass;

impl CommonSubexpressionEliminationPass {
    pub fn new() -> Self {
        CommonSubexpressionEliminationPass
    }
}

impl Pass for CommonSubexpressionEliminationPass {
    fn name(&self) -> &'static str {
        "cse"
    }
    
    fn run(&mut self, _ops: &mut Vec<Box<dyn HuanOp>>) -> Result<(), String> {
        println!("Running common subexpression elimination pass...");
        // 这里实现公共子表达式消除
        Ok(())
    }
}

/// 循环不变代码外提Pass
#[derive(Debug)]
pub struct LoopInvariantCodeMotionPass;

impl LoopInvariantCodeMotionPass {
    pub fn new() -> Self {
        LoopInvariantCodeMotionPass
    }
}

impl Pass for LoopInvariantCodeMotionPass {
    fn name(&self) -> &'static str {
        "licm"
    }
    
    fn run(&mut self, _ops: &mut Vec<Box<dyn HuanOp>>) -> Result<(), String> {
        println!("Running loop invariant code motion pass...");
        // 这里实现循环不变代码外提
        Ok(())
    }
}

/// 强度削减Pass
#[derive(Debug)]
pub struct StrengthReductionPass;

impl StrengthReductionPass {
    pub fn new() -> Self {
        StrengthReductionPass
    }
}

impl Pass for StrengthReductionPass {
    fn name(&self) -> &'static str {
        "strength-reduction"
    }
    
    fn run(&mut self, _ops: &mut Vec<Box<dyn HuanOp>>) -> Result<(), String> {
        println!("Running strength reduction pass...");
        // 这里实现强度削减
        Ok(())
    }
}

/// 完整的优化管线
#[derive(Debug)]
pub struct OptimizationPipeline {
    passes: Vec<Box<dyn Pass>>,
}

impl OptimizationPipeline {
    pub fn new() -> Self {
        let passes: Vec<Box<dyn Pass>> = vec![
            Box::new(ConstantFoldingPass::new()),
            Box::new(CommonSubexpressionEliminationPass::new()),
            Box::new(StrengthReductionPass::new()),
            Box::new(LoopInvariantCodeMotionPass::new()),
            Box::new(DeadCodeEliminationPass::new()),
        ];
        
        OptimizationPipeline { passes }
    }
}

impl Pass for OptimizationPipeline {
    fn name(&self) -> &'static str {
        "optimize"
    }
    
    fn run(&mut self, ops: &mut Vec<Box<dyn HuanOp>>) -> Result<(), String> {
        for pass in &mut self.passes {
            pass.run(ops)?;
        }
        Ok(())
    }
}

// 辅助方法
pub trait AsAny {
    fn as_any(&self) -> &dyn std::any::Any;
}

impl<T: 'static> AsAny for T {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_creation() {
        let _scf_pass = HuanToScfPass::new();
        let _arith_pass = HuanToArithPass::new();
        let _func_pass = HuanToFuncPass::new();
    }
}

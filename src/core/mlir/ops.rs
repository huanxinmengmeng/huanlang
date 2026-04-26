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
// 幻语 MLIR 操作定义
// 实现规范第10.2.1节的完整操作定义

use std::fmt;
use crate::core::mlir::types::HuanType;
use crate::core::mlir::passes::AsAny;
use crate::core::lexer::token::SourceSpan;

/// 幻语MLIR模块操作
#[derive(Debug, Clone)]
pub struct ModuleOp {
    pub name: String,
    pub ops: Vec<Box<dyn HuanOp>>,
    pub span: SourceSpan,
}

impl ModuleOp {
    pub fn dummy() -> Self {
        let dummy_func = HanshuOp {
            name: "main".to_string(),
            return_type: Box::new(crate::core::mlir::types::I32Type),
            params: vec![],
            body: vec![],
            span: SourceSpan::default(),
        };
        Self {
            name: "dummy".to_string(),
            ops: vec![Box::new(dummy_func)],
            span: SourceSpan::default(),
        }
    }
}

/// 幻语操作基类特性
pub trait HuanOp: fmt::Debug + AsAny {
    /// 操作名称
    fn mnemonic(&self) -> &'static str;
    
    /// 操作位置
    fn span(&self) -> &SourceSpan;
    
    /// 验证操作
    fn verify(&self) -> Result<(), String>;
    
    /// 获取操作描述
    fn summary(&self) -> &'static str;
    
    /// 克隆操作
    fn clone_box(&self) -> Box<dyn HuanOp>;
}

/// 为 Box<dyn HuanOp> 实现 Clone 特征
impl Clone for Box<dyn HuanOp> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

// =============================================================================
// 变量与常量操作
// =============================================================================

/// 变量声明操作：令 名称 类型 类型 为 值
#[derive(Debug, Clone)]
pub struct LingOp {
    pub sym_name: String,
    pub var_type: Box<dyn HuanType>,
    pub value: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for LingOp {
    fn mnemonic(&self) -> &'static str { "ling" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> {
        // 验证值类型与声明类型是否匹配
        Ok(())
    }
    fn summary(&self) -> &'static str { "变量声明：令 名称 类型 类型 为 值" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(LingOp {
            sym_name: self.sym_name.clone(),
            var_type: self.var_type.clone(),
            value: self.value.clone(),
            span: self.span,
        })
    }
}

/// 常量声明操作：定 名称 类型 类型 为 值
#[derive(Debug, Clone)]
pub struct DingOp {
    pub sym_name: String,
    pub const_type: Box<dyn HuanType>,
    pub value: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for DingOp {
    fn mnemonic(&self) -> &'static str { "ding" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "常量声明：定 名称 类型 类型 为 值" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(DingOp {
            sym_name: self.sym_name.clone(),
            const_type: self.const_type.clone(),
            value: self.value.clone(),
            span: self.span,
        })
    }
}

/// 赋值操作：目标 设为 值
#[derive(Debug, Clone)]
pub struct SheweiOp {
    pub target: Box<dyn HuanOp>,
    pub value: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for SheweiOp {
    fn mnemonic(&self) -> &'static str { "shewei" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "赋值：目标 设为 值" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(SheweiOp {
            target: self.target.clone(),
            value: self.value.clone(),
            span: self.span,
        })
    }
}

// =============================================================================
// 控制流操作
// =============================================================================

/// 条件分支操作：若 条件 则 真块 否则 假块 结束
#[derive(Debug, Clone)]
pub struct RuoOp {
    pub condition: Box<dyn HuanOp>,
    pub then_block: Vec<Box<dyn HuanOp>>,
    pub else_block: Option<Vec<Box<dyn HuanOp>>>,
    pub span: SourceSpan,
}

impl HuanOp for RuoOp {
    fn mnemonic(&self) -> &'static str { "ruo" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> {
        // 验证条件为布尔类型
        Ok(())
    }
    fn summary(&self) -> &'static str { "条件分支：若 条件 则 真块 否则 假块 结束" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(RuoOp {
            condition: self.condition.clone(),
            then_block: self.then_block.clone(),
            else_block: self.else_block.clone(),
            span: self.span,
        })
    }
}

/// 模式匹配操作：匹配 表达式 { 当 模式 => 块 } 结束
#[derive(Debug, Clone)]
pub struct PipeiOp {
    pub value: Box<dyn HuanOp>,
    pub arms: Vec<(Box<dyn HuanOp>, Vec<Box<dyn HuanOp>>)>,
    pub default_arm: Option<Vec<Box<dyn HuanOp>>>,
    pub span: SourceSpan,
}

impl HuanOp for PipeiOp {
    fn mnemonic(&self) -> &'static str { "pipei" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "模式匹配：匹配 表达式 { 当 模式 => 块 } 结束" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(PipeiOp {
            value: self.value.clone(),
            arms: self.arms.clone(),
            default_arm: self.default_arm.clone(),
            span: self.span,
        })
    }
}

/// 固定次数循环操作：重复 n 次 块 结束
#[derive(Debug, Clone)]
pub struct ChongfuOp {
    pub count: Box<dyn HuanOp>,
    pub body: Vec<Box<dyn HuanOp>>,
    pub span: SourceSpan,
}

impl HuanOp for ChongfuOp {
    fn mnemonic(&self) -> &'static str { "chongfu" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "固定次数循环：重复 n 次 块 结束" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(ChongfuOp {
            count: self.count.clone(),
            body: self.body.clone(),
            span: self.span,
        })
    }
}

/// 条件循环操作：当 条件 循环 块 结束
#[derive(Debug, Clone)]
pub struct DangOp {
    pub condition: Box<dyn HuanOp>,
    pub body: Vec<Box<dyn HuanOp>>,
    pub span: SourceSpan,
}

impl HuanOp for DangOp {
    fn mnemonic(&self) -> &'static str { "dang" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "条件循环：当 条件 循环 块 结束" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(DangOp {
            condition: self.condition.clone(),
            body: self.body.clone(),
            span: self.span,
        })
    }
}

/// 遍历循环操作：对于 每个 变量 在 容器 中 块 结束
#[derive(Debug, Clone)]
pub struct DuiyuOp {
    pub iterable: Box<dyn HuanOp>,
    pub loop_var: String,
    pub body: Vec<Box<dyn HuanOp>>,
    pub span: SourceSpan,
}

impl HuanOp for DuiyuOp {
    fn mnemonic(&self) -> &'static str { "duiyu" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "遍历循环：对于 每个 变量 在 容器 中 块 结束" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(DuiyuOp {
            iterable: self.iterable.clone(),
            loop_var: self.loop_var.clone(),
            body: self.body.clone(),
            span: self.span,
        })
    }
}

// =============================================================================
// 函数操作
// =============================================================================

/// 函数定义操作：函数 名称(参数) 返回 类型 块 结束
#[derive(Debug, Clone)]
pub struct HanshuOp {
    pub name: String,
    pub params: Vec<(String, Box<dyn HuanType>)>,
    pub return_type: Box<dyn HuanType>,
    pub body: Vec<Box<dyn HuanOp>>,
    pub span: SourceSpan,
}

impl HuanOp for HanshuOp {
    fn mnemonic(&self) -> &'static str { "hanshu" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "函数定义：函数 名称(参数) 返回 类型 块 结束" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(HanshuOp {
            name: self.name.clone(),
            params: self.params.clone(),
            return_type: self.return_type.clone(),
            body: self.body.clone(),
            span: self.span,
        })
    }
}

/// 返回语句操作：返回 值
#[derive(Debug)]
pub struct FanhuiOp {
    pub value: Option<Box<dyn HuanOp>>,
    pub span: SourceSpan,
}

impl HuanOp for FanhuiOp {
    fn mnemonic(&self) -> &'static str { "fanhui" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "返回语句：返回 值" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(FanhuiOp {
            value: self.value.clone(),
            span: self.span,
        })
    }
}

/// 函数调用操作：调用 函数(参数)
#[derive(Debug)]
pub struct DiaoyongOp {
    pub callee: String,
    pub args: Vec<Box<dyn HuanOp>>,
    pub span: SourceSpan,
}

impl HuanOp for DiaoyongOp {
    fn mnemonic(&self) -> &'static str { "diaoyong" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "函数调用：调用 函数(参数)" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(DiaoyongOp {
            callee: self.callee.clone(),
            args: self.args.clone(),
            span: self.span,
        })
    }
}

// =============================================================================
// 算术与逻辑操作
// =============================================================================

/// 加法操作
#[derive(Debug)]
pub struct JiaOp {
    pub lhs: Box<dyn HuanOp>,
    pub rhs: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for JiaOp {
    fn mnemonic(&self) -> &'static str { "jia" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "加法" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(JiaOp {
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            span: self.span,
        })
    }
}

/// 减法操作
#[derive(Debug)]
pub struct JianOp {
    pub lhs: Box<dyn HuanOp>,
    pub rhs: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for JianOp {
    fn mnemonic(&self) -> &'static str { "jian" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "减法" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(JianOp {
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            span: self.span,
        })
    }
}

/// 乘法操作
#[derive(Debug)]
pub struct ChengOp {
    pub lhs: Box<dyn HuanOp>,
    pub rhs: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for ChengOp {
    fn mnemonic(&self) -> &'static str { "cheng" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "乘法" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(ChengOp {
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            span: self.span,
        })
    }
}

/// 除法操作
#[derive(Debug)]
pub struct ChuOp {
    pub lhs: Box<dyn HuanOp>,
    pub rhs: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for ChuOp {
    fn mnemonic(&self) -> &'static str { "chu" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "除法" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(ChuOp {
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            span: self.span,
        })
    }
}

/// 取余操作
#[derive(Debug)]
pub struct QuyuOp {
    pub lhs: Box<dyn HuanOp>,
    pub rhs: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for QuyuOp {
    fn mnemonic(&self) -> &'static str { "quyu" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "取余" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(QuyuOp {
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            span: self.span,
        })
    }
}

/// 大于比较操作
#[derive(Debug)]
pub struct DayuOp {
    pub lhs: Box<dyn HuanOp>,
    pub rhs: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for DayuOp {
    fn mnemonic(&self) -> &'static str { "dayu" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "大于比较" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(DayuOp {
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            span: self.span,
        })
    }
}

/// 小于比较操作
#[derive(Debug)]
pub struct XiaoyuOp {
    pub lhs: Box<dyn HuanOp>,
    pub rhs: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for XiaoyuOp {
    fn mnemonic(&self) -> &'static str { "xiaoyu" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "小于比较" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(XiaoyuOp {
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            span: self.span,
        })
    }
}

/// 等于比较操作
#[derive(Debug)]
pub struct DengyuOp {
    pub lhs: Box<dyn HuanOp>,
    pub rhs: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for DengyuOp {
    fn mnemonic(&self) -> &'static str { "dengyu" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "等于比较" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(DengyuOp {
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            span: self.span,
        })
    }
}

/// 逻辑与操作
#[derive(Debug)]
pub struct QieOp {
    pub lhs: Box<dyn HuanOp>,
    pub rhs: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for QieOp {
    fn mnemonic(&self) -> &'static str { "qie" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "逻辑与" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(QieOp {
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            span: self.span,
        })
    }
}

/// 逻辑或操作
#[derive(Debug)]
pub struct HuoOp {
    pub lhs: Box<dyn HuanOp>,
    pub rhs: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for HuoOp {
    fn mnemonic(&self) -> &'static str { "huo" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "逻辑或" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(HuoOp {
            lhs: self.lhs.clone(),
            rhs: self.rhs.clone(),
            span: self.span,
        })
    }
}

/// 逻辑非操作
#[derive(Debug)]
pub struct FeiOp {
    pub operand: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for FeiOp {
    fn mnemonic(&self) -> &'static str { "fei" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "逻辑非" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(FeiOp {
            operand: self.operand.clone(),
            span: self.span,
        })
    }
}

// =============================================================================
// 复合类型操作
// =============================================================================

/// 创建列表操作
#[derive(Debug)]
pub struct LiebiaoOp {
    pub elements: Vec<Box<dyn HuanOp>>,
    pub span: SourceSpan,
}

impl HuanOp for LiebiaoOp {
    fn mnemonic(&self) -> &'static str { "liebiao" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "创建列表" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(LiebiaoOp {
            elements: self.elements.clone(),
            span: self.span,
        })
    }
}

/// 创建字典操作
#[derive(Debug)]
pub struct ZidianOp {
    pub keys: Vec<Box<dyn HuanOp>>,
    pub values: Vec<Box<dyn HuanOp>>,
    pub span: SourceSpan,
}

impl HuanOp for ZidianOp {
    fn mnemonic(&self) -> &'static str { "zidian" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "创建字典" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(ZidianOp {
            keys: self.keys.clone(),
            values: self.values.clone(),
            span: self.span,
        })
    }
}

/// 列表追加操作
#[derive(Debug)]
pub struct ZhuizhuiOp {
    pub list: Box<dyn HuanOp>,
    pub element: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for ZhuizhuiOp {
    fn mnemonic(&self) -> &'static str { "zhuizhui" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "列表追加" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(ZhuizhuiOp {
            list: self.list.clone(),
            element: self.element.clone(),
            span: self.span,
        })
    }
}

/// 索引访问操作
#[derive(Debug)]
pub struct SuoyinOp {
    pub container: Box<dyn HuanOp>,
    pub index: Box<dyn HuanOp>,
    pub span: SourceSpan,
}

impl HuanOp for SuoyinOp {
    fn mnemonic(&self) -> &'static str { "suoyin" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "索引访问" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(SuoyinOp {
            container: self.container.clone(),
            index: self.index.clone(),
            span: self.span,
        })
    }
}

/// 字段访问操作
#[derive(Debug)]
pub struct ZiduanOp {
    pub object: Box<dyn HuanOp>,
    pub field: String,
    pub span: SourceSpan,
}

impl HuanOp for ZiduanOp {
    fn mnemonic(&self) -> &'static str { "ziduan" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "字段访问" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(ZiduanOp {
            object: self.object.clone(),
            field: self.field.clone(),
            span: self.span,
        })
    }
}

// =============================================================================
// 内联汇编操作
// =============================================================================

/// 内联汇编操作
#[derive(Debug, Clone)]
pub struct AsmOp {
    pub asm_string: String,
    pub constraints: String,
    pub outputs: Vec<String>,
    pub inputs: Vec<String>,
    pub clobbers: Vec<String>,
    pub options: Vec<String>,
    pub span: SourceSpan,
}

impl HuanOp for AsmOp {
    fn mnemonic(&self) -> &'static str { "asm" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "内联汇编" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(AsmOp {
            asm_string: self.asm_string.clone(),
            constraints: self.constraints.clone(),
            outputs: self.outputs.clone(),
            inputs: self.inputs.clone(),
            clobbers: self.clobbers.clone(),
            options: self.options.clone(),
            span: self.span,
        })
    }
}

// =============================================================================
// 字面值操作
// =============================================================================

/// 整数字面值
#[derive(Debug, Clone)]
pub struct IntLitOp {
    pub value: i64,
    pub span: SourceSpan,
}

impl HuanOp for IntLitOp {
    fn mnemonic(&self) -> &'static str { "constant" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "整数字面值" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(IntLitOp {
            value: self.value,
            span: self.span,
        })
    }
}

/// 浮点数字面值
#[derive(Debug, Clone)]
pub struct FloatLitOp {
    pub value: f64,
    pub span: SourceSpan,
}

impl HuanOp for FloatLitOp {
    fn mnemonic(&self) -> &'static str { "constant" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "浮点数字面值" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(FloatLitOp {
            value: self.value,
            span: self.span,
        })
    }
}

/// 字符串字面值
#[derive(Debug, Clone)]
pub struct StringLitOp {
    pub value: String,
    pub span: SourceSpan,
}

impl HuanOp for StringLitOp {
    fn mnemonic(&self) -> &'static str { "constant" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "字符串字面值" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(StringLitOp {
            value: self.value.clone(),
            span: self.span,
        })
    }
}

/// 布尔字面值
#[derive(Debug, Clone)]
pub struct BoolLitOp {
    pub value: bool,
    pub span: SourceSpan,
}

impl HuanOp for BoolLitOp {
    fn mnemonic(&self) -> &'static str { "constant" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "布尔字面值" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(BoolLitOp {
            value: self.value,
            span: self.span,
        })
    }
}

/// 字符字面值
#[derive(Debug, Clone)]
pub struct CharLitOp {
    pub value: char,
    pub span: SourceSpan,
}

impl HuanOp for CharLitOp {
    fn mnemonic(&self) -> &'static str { "constant" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "字符字面值" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(CharLitOp {
            value: self.value,
            span: self.span,
        })
    }
}

/// 空值操作
#[derive(Debug, Clone)]
pub struct NullOp {
    pub span: SourceSpan,
}

impl HuanOp for NullOp {
    fn mnemonic(&self) -> &'static str { "null" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "空值" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(NullOp {
            span: self.span,
        })
    }
}

/// 标识符（变量引用）
#[derive(Debug, Clone)]
pub struct IdentOp {
    pub name: String,
    pub span: SourceSpan,
}

impl HuanOp for IdentOp {
    fn mnemonic(&self) -> &'static str { "ident" }
    fn span(&self) -> &SourceSpan { &self.span }
    fn verify(&self) -> Result<(), String> { Ok(()) }
    fn summary(&self) -> &'static str { "标识符" }
    fn clone_box(&self) -> Box<dyn HuanOp> {
        Box::new(IdentOp {
            name: self.name.clone(),
            span: self.span,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::lexer::token::SourceSpan;

    #[test]
    fn test_basic_op() {
        let span = SourceSpan::dummy();
        
        let int_lit = IntLitOp {
            value: 42,
            span: span.clone(),
        };
        
        assert_eq!(int_lit.mnemonic(), "constant");
        assert_eq!(int_lit.value, 42);
    }

    #[test]
    fn test_add_op() {
        let span = SourceSpan::dummy();
        
        let a = IntLitOp { value: 2, span: span.clone() };
        let b = IntLitOp { value: 3, span: span.clone() };
        
        let add = JiaOp {
            lhs: Box::new(a),
            rhs: Box::new(b),
            span,
        };
        
        assert_eq!(add.mnemonic(), "jia");
    }
}

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
// 幻语 MLIR 方言与降级系统

//! MLIR方言与降级系统实现规范第10章的完整功能

//! MLIR方言与降级系统
//! 
//! 本模块实现了规范第10章的完整功能：
//! - `huan` 方言定义
//! - AST到MLIR的转换
//! - 降级Pass管线
//! - 类型系统

pub mod dialect;
pub mod types;
pub mod ops;
pub mod passes;
pub mod conversion;
pub mod lowering;

pub use dialect::HuanDialect;
pub use types::{
    IntType, I8Type, I16Type, I32Type, I64Type,
    U8Type, U16Type, U32Type, U64Type,
    F32Type, F64Type, BoolType, CharType, StringType, UnitType,
    ListType, ArrayType, MapType, PtrType, OptionType, FuncType,
    HuanType,
};
pub use ops::{
    ModuleOp,
    LingOp, DingOp, SheweiOp,
    RuoOp, PipeiOp, ChongfuOp, DangOp, DuiyuOp,
    HanshuOp, FanhuiOp, DiaoyongOp,
    JiaOp, JianOp, ChengOp, ChuOp, QuyuOp,
    DayuOp, XiaoyuOp, DengyuOp,
    QieOp, HuoOp, FeiOp,
    LiebiaoOp, ZidianOp, ZhuizhuiOp, SuoyinOp, ZiduanOp,
    AsmOp, IntLitOp, FloatLitOp, StringLitOp, BoolLitOp, IdentOp,
    HuanOp,
};
pub use passes::{
    HuanToScfPass, HuanToArithPass, HuanToFuncPass,
    LowerHuanToLLVMPass, ConstantFoldingPass, DeadCodeEliminationPass,
    OptimizationPipeline, Pass,
};
pub use conversion::AstToMlirConverter;
pub use lowering::{HuanToLLVMTypeConverter, LLVMType};

// 测试模块
#[cfg(test)]
mod tests;

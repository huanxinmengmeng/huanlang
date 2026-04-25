// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。

use std::path::PathBuf;
use crate::core::mlir;

/// 代码生成器统一接口
pub trait CodeGenerator {
    /// 初始化代码生成器
    fn new(target: super::target::TargetTriple, options: CodeGenOptions) -> Self;

    /// 从 MLIR 模块生成目标代码
    fn generate(&mut self, mlir_module: &mlir::ModuleOp) -> Result<Vec<u8>, super::error::CodeGenError>;

    /// 生成汇编代码
    fn emit_assembly(&mut self, mlir_module: &mlir::ModuleOp) -> Result<String, super::error::CodeGenError>;

    /// 生成 LLVM IR 文本（仅 LLVM 后端支持）
    fn emit_llvm_ir(&mut self, _mlir_module: &mlir::ModuleOp) -> Result<String, super::error::CodeGenError> {
        Err(super::error::CodeGenError::Unsupported("LLVM IR 文本生成不支持此目标".to_string()))
    }

    /// 链接目标文件
    fn link(&self, objects: Vec<PathBuf>, output: PathBuf) -> Result<(), super::error::LinkError>;
}

/// 代码生成选项
#[derive(Debug, Clone)]
pub struct CodeGenOptions {
    /// 优化级别：0-3, s（尺寸）, z（激进尺寸）
    pub opt_level: OptLevel,
    /// 是否生成调试信息
    pub debug_info: bool,
    /// 是否生成位置无关代码（PIC/PIE）
    pub pic: bool,
    /// 是否启用栈保护
    pub stack_protector: bool,
    /// 目标 CPU 特性（如 "+avx2,+fma"）
    pub target_features: String,
    /// 调用约定
    pub calling_convention: CallingConvention,
    /// 是否保留内联汇编
    pub keep_asm: bool,
}

impl Default for CodeGenOptions {
    fn default() -> Self {
        Self {
            opt_level: OptLevel::Default,
            debug_info: false,
            pic: false,
            stack_protector: false,
            target_features: String::new(),
            calling_convention: CallingConvention::HuanDefault,
            keep_asm: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OptLevel {
    None = 0,
    Less = 1,
    Default = 2,
    Aggressive = 3,
    Size = b's' as isize,
    SizeAggressive = b'z' as isize,
}

impl OptLevel {
    pub fn as_u8(&self) -> u8 {
        match self {
            OptLevel::None => 0,
            OptLevel::Less => 1,
            OptLevel::Default => 2,
            OptLevel::Aggressive => 3,
            OptLevel::Size => b's',
            OptLevel::SizeAggressive => b'z',
        }
    }
}

#[derive(Debug, Clone)]
pub enum CallingConvention {
    HuanDefault,
    C,
    Fast,
    Interrupt,
}

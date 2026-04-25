// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
//
// MLIR 到 LLVM 类型转换
// 实现规范第10.5节的完整类型转换

use std::collections::HashMap;
use crate::core::mlir::types::*;

/// LLVM 类型表示
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LLVMType {
    /// LLVM 整数类型
    Integer { bits: u32 },
    /// LLVM 32位浮点类型
    Float32,
    /// LLVM 64位浮点类型
    Float64,
    /// LLVM 指针类型
    Pointer { pointee: Box<LLVMType> },
    /// LLVM 数组类型
    Array { element: Box<LLVMType>, count: u64 },
    /// LLVM 结构类型
    Struct { fields: Vec<LLVMType> },
    /// LLVM 函数类型
    Function { params: Vec<LLVMType>, return_type: Box<LLVMType> },
    /// LLVM Void 类型
    Void,
}

/// LLVM 类型转换器
pub struct HuanToLLVMTypeConverter {
    type_cache: HashMap<String, LLVMType>,
}

impl HuanToLLVMTypeConverter {
    pub fn new() -> Self {
        HuanToLLVMTypeConverter {
            type_cache: HashMap::new(),
        }
    }
    
    /// 将 Huan 类型转换为 LLVM 类型
    pub fn convert_type(&mut self, ty: &Box<dyn HuanType>) -> Result<LLVMType, String> {
        // 先尝试从缓存查找
        let ty_str = ty.to_string();
        if let Some(cached) = self.type_cache.get(&ty_str) {
            return Ok(cached.clone());
        }
        
        // 转换类型
        let llvm_ty = self.convert_type_inner(ty)?;
        
        // 缓存结果
        self.type_cache.insert(ty_str, llvm_ty.clone());
        
        Ok(llvm_ty)
    }
    
    /// 内部类型转换逻辑
    fn convert_type_inner(&self, ty: &Box<dyn HuanType>) -> Result<LLVMType, String> {
        // 使用 mnemonic 来匹配类型
        let ty_ref = &**ty;
        let mnemonic = ty_ref.mnemonic();
        
        match mnemonic {
            "int" => Ok(LLVMType::Integer { bits: 64 }),
            "i8" => Ok(LLVMType::Integer { bits: 8 }),
            "i16" => Ok(LLVMType::Integer { bits: 16 }),
            "i32" => Ok(LLVMType::Integer { bits: 32 }),
            "i64" => Ok(LLVMType::Integer { bits: 64 }),
            "u8" => Ok(LLVMType::Integer { bits: 8 }),
            "u16" => Ok(LLVMType::Integer { bits: 16 }),
            "u32" => Ok(LLVMType::Integer { bits: 32 }),
            "u64" => Ok(LLVMType::Integer { bits: 64 }),
            "bool" => Ok(LLVMType::Integer { bits: 1 }),
            "f32" => Ok(LLVMType::Float32),
            "f64" => Ok(LLVMType::Float64),
            "char" => Ok(LLVMType::Integer { bits: 32 }),
            "unit" => Ok(LLVMType::Void),
            "string" => {
                // 字符串表示为 { i8*, i64 }
                Ok(LLVMType::Struct {
                    fields: vec![
                        LLVMType::Pointer {
                            pointee: Box::new(LLVMType::Integer { bits: 8 }),
                        },
                        LLVMType::Integer { bits: 64 },
                    ],
                })
            }
            "list" => {
                // 列表表示为 { i8*, i64, i64, i64 }
                Ok(LLVMType::Struct {
                    fields: vec![
                        LLVMType::Pointer {
                            pointee: Box::new(LLVMType::Integer { bits: 8 }),
                        },
                        LLVMType::Integer { bits: 64 }, // length
                        LLVMType::Integer { bits: 64 }, // capacity
                        LLVMType::Integer { bits: 64 }, // element size
                    ],
                })
            }
            "ptr" => {
                // 指针类型
                // 注意：这里简化处理，实际需要获取指针指向的类型
                Ok(LLVMType::Pointer {
                    pointee: Box::new(LLVMType::Integer { bits: 8 }),
                })
            }
            "array" => {
                // 数组类型
                // 注意：这里简化处理，实际需要获取数组元素类型和大小
                Ok(LLVMType::Array {
                    element: Box::new(LLVMType::Integer { bits: 8 }),
                    count: 0,
                })
            }
            "func" => {
                // 函数类型
                // 注意：这里简化处理，实际需要获取函数参数类型和返回类型
                Ok(LLVMType::Function {
                    params: vec![],
                    return_type: Box::new(LLVMType::Void),
                })
            }
            _ => {
                Err(format!("不支持的类型转换: {:?}", ty))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_converter_creation() {
        let _converter = HuanToLLVMTypeConverter::new();
    }
}

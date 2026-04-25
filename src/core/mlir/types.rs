// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
//
// 幻语 MLIR 类型系统
// 实现规范第10.2.1节的完整类型定义

use std::fmt;
use std::hash::Hash;
use std::any::TypeId;
use crate::core::ast::Type as AstType;
use crate::core::mlir::passes::AsAny;

/// 幻语类型基础特性
pub trait HuanType: fmt::Debug + AsAny {
    /// 获取类型助记符
    fn mnemonic(&self) -> &'static str;
    
    /// 转换为字符串表示
    fn to_string(&self) -> String;
    
    /// 类型是否兼容
    fn is_compatible(&self, other: &dyn HuanType) -> bool;
    
    /// 类型相等性检查
    fn equals(&self, other: &dyn HuanType) -> bool {
        self.mnemonic() == other.mnemonic()
    }
    
    /// 克隆类型
    fn clone_box(&self) -> Box<dyn HuanType>;
}

/// 为 Box<dyn HuanType> 实现 Clone 特征
impl Clone for Box<dyn HuanType> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// 为 Box<dyn HuanType> 实现 PartialEq 特征
impl PartialEq for Box<dyn HuanType> {
    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }
}

/// 为 Box<dyn HuanType> 实现 Eq 特征
impl Eq for Box<dyn HuanType> {}

/// 为 Box<dyn HuanType> 实现 Hash 特征
impl std::hash::Hash for Box<dyn HuanType> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.mnemonic().hash(state);
    }
}

/// 为 Box<dyn HuanType> 实现 HuanType 特征
impl HuanType for Box<dyn HuanType> {
    fn mnemonic(&self) -> &'static str {
        (**self).mnemonic()
    }
    
    fn to_string(&self) -> String {
        (**self).to_string()
    }
    
    fn is_compatible(&self, other: &dyn HuanType) -> bool {
        (**self).is_compatible(other)
    }
    
    fn equals(&self, other: &dyn HuanType) -> bool {
        (**self).equals(other)
    }
    
    fn clone_box(&self) -> Box<dyn HuanType> {
        self.clone()
    }
}

// 基础类型定义

/// 平台相关整数类型（64位）
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IntType;

/// 8位有符号整数
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct I8Type;

/// 16位有符号整数
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct I16Type;

/// 32位有符号整数
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct I32Type;

/// 64位有符号整数
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct I64Type;

/// 8位无符号整数
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct U8Type;

/// 16位无符号整数
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct U16Type;

/// 32位无符号整数
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct U32Type;

/// 64位无符号整数
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct U64Type;

/// 32位浮点数
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct F32Type;

/// 64位浮点数
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct F64Type;

/// 布尔类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BoolType;

/// 字符类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CharType;

/// 字符串类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StringType;

/// 单元类型
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UnitType;

// 复合类型定义

/// 列表类型
#[derive(Debug, Clone)]
pub struct ListType {
    /// 元素类型
    pub element_type: Box<dyn HuanType>,
}

impl PartialEq for ListType {
    fn eq(&self, other: &Self) -> bool {
        &self.element_type == &other.element_type
    }
}

impl Eq for ListType {}

impl std::hash::Hash for ListType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.element_type.hash(state);
    }
}

/// 数组类型（固定大小）
#[derive(Debug, Clone)]
pub struct ArrayType {
    /// 元素类型
    pub element_type: Box<dyn HuanType>,
    /// 数组大小
    pub size: u64,
}

impl PartialEq for ArrayType {
    fn eq(&self, other: &Self) -> bool {
        &self.element_type == &other.element_type && self.size == other.size
    }
}

impl Eq for ArrayType {}

impl std::hash::Hash for ArrayType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.element_type.hash(state);
        self.size.hash(state);
    }
}

/// 映射类型（字典）
#[derive(Debug, Clone)]
pub struct MapType {
    /// 键类型
    pub key_type: Box<dyn HuanType>,
    /// 值类型
    pub value_type: Box<dyn HuanType>,
}

impl PartialEq for MapType {
    fn eq(&self, other: &Self) -> bool {
        &self.key_type == &other.key_type && &self.value_type == &other.value_type
    }
}

impl Eq for MapType {}

impl std::hash::Hash for MapType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.key_type.hash(state);
        self.value_type.hash(state);
    }
}

/// 指针类型
#[derive(Debug, Clone)]
pub struct PtrType {
    /// 指向的类型
    pub pointee_type: Box<dyn HuanType>,
}

impl PartialEq for PtrType {
    fn eq(&self, other: &Self) -> bool {
        &self.pointee_type == &other.pointee_type
    }
}

impl Eq for PtrType {}

impl std::hash::Hash for PtrType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pointee_type.hash(state);
    }
}

/// 选项类型
#[derive(Debug, Clone)]
pub struct OptionType {
    /// 内部类型
    pub inner_type: Box<dyn HuanType>,
}

impl PartialEq for OptionType {
    fn eq(&self, other: &Self) -> bool {
        &self.inner_type == &other.inner_type
    }
}

impl Eq for OptionType {}

impl std::hash::Hash for OptionType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.inner_type.hash(state);
    }
}

/// 函数类型
#[derive(Debug, Clone)]
pub struct FuncType {
    /// 参数类型列表
    pub input_types: Vec<Box<dyn HuanType>>,
    /// 返回类型
    pub output_type: Box<dyn HuanType>,
}

impl PartialEq for FuncType {
    fn eq(&self, other: &Self) -> bool {
        self.input_types == other.input_types && &self.output_type == &other.output_type
    }
}

impl Eq for FuncType {}

impl std::hash::Hash for FuncType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.input_types.hash(state);
        self.output_type.hash(state);
    }
}

// 实现HuanType特性

macro_rules! impl_huan_type_simple {
    ($type:ty, $mnemonic:expr) => {
        impl HuanType for $type {
            fn mnemonic(&self) -> &'static str {
                $mnemonic
            }
            
            fn to_string(&self) -> String {
                $mnemonic.to_string()
            }
            
            fn is_compatible(&self, other: &dyn HuanType) -> bool {
                other.as_any().type_id() == TypeId::of::<Self>()
            }
            
            fn clone_box(&self) -> Box<dyn HuanType> {
                Box::new(<$type>::default())
            }
        }
        
        impl Default for $type {
            fn default() -> Self {
                Self
            }
        }
    };
}

impl_huan_type_simple!(IntType, "int");
impl_huan_type_simple!(I8Type, "i8");
impl_huan_type_simple!(I16Type, "i16");
impl_huan_type_simple!(I32Type, "i32");
impl_huan_type_simple!(I64Type, "i64");
impl_huan_type_simple!(U8Type, "u8");
impl_huan_type_simple!(U16Type, "u16");
impl_huan_type_simple!(U32Type, "u32");
impl_huan_type_simple!(U64Type, "u64");
impl_huan_type_simple!(F32Type, "f32");
impl_huan_type_simple!(F64Type, "f64");
impl_huan_type_simple!(BoolType, "bool");
impl_huan_type_simple!(CharType, "char");
impl_huan_type_simple!(StringType, "string");
impl_huan_type_simple!(UnitType, "unit");

impl HuanType for ListType {
    fn mnemonic(&self) -> &'static str {
        "list"
    }
    
    fn to_string(&self) -> String {
        format!("list<{}>", self.element_type.to_string())
    }
    
    fn is_compatible(&self, other: &dyn HuanType) -> bool {
        if let Some(other_list) = other.as_any().downcast_ref::<Self>() {
            self.element_type.is_compatible(&*other_list.element_type)
        } else {
            false
        }
    }
    
    fn clone_box(&self) -> Box<dyn HuanType> {
        Box::new(ListType {
            element_type: self.element_type.clone(),
        })
    }
}

impl HuanType for ArrayType {
    fn mnemonic(&self) -> &'static str {
        "array"
    }
    
    fn to_string(&self) -> String {
        format!("array<{}, {}>", self.element_type.to_string(), self.size)
    }
    
    fn is_compatible(&self, other: &dyn HuanType) -> bool {
        if let Some(other_arr) = other.as_any().downcast_ref::<Self>() {
            self.element_type.is_compatible(&*other_arr.element_type) &&
            self.size == other_arr.size
        } else {
            false
        }
    }
    
    fn clone_box(&self) -> Box<dyn HuanType> {
        Box::new(ArrayType {
            element_type: self.element_type.clone(),
            size: self.size,
        })
    }
}

impl HuanType for MapType {
    fn mnemonic(&self) -> &'static str {
        "map"
    }
    
    fn to_string(&self) -> String {
        format!("map<{}, {}>", self.key_type.to_string(), self.value_type.to_string())
    }
    
    fn is_compatible(&self, other: &dyn HuanType) -> bool {
        if let Some(other_map) = other.as_any().downcast_ref::<Self>() {
            self.key_type.is_compatible(&*other_map.key_type) &&
            self.value_type.is_compatible(&*other_map.value_type)
        } else {
            false
        }
    }
    
    fn clone_box(&self) -> Box<dyn HuanType> {
        Box::new(MapType {
            key_type: self.key_type.clone(),
            value_type: self.value_type.clone(),
        })
    }
}

impl HuanType for PtrType {
    fn mnemonic(&self) -> &'static str {
        "ptr"
    }
    
    fn to_string(&self) -> String {
        format!("ptr<{}>", self.pointee_type.to_string())
    }
    
    fn is_compatible(&self, other: &dyn HuanType) -> bool {
        if let Some(other_ptr) = other.as_any().downcast_ref::<Self>() {
            self.pointee_type.is_compatible(&*other_ptr.pointee_type)
        } else {
            false
        }
    }
    
    fn clone_box(&self) -> Box<dyn HuanType> {
        Box::new(PtrType {
            pointee_type: self.pointee_type.clone(),
        })
    }
}

impl HuanType for OptionType {
    fn mnemonic(&self) -> &'static str {
        "option"
    }
    
    fn to_string(&self) -> String {
        format!("option<{}>", self.inner_type.to_string())
    }
    
    fn is_compatible(&self, other: &dyn HuanType) -> bool {
        if let Some(other_opt) = other.as_any().downcast_ref::<Self>() {
            self.inner_type.is_compatible(&*other_opt.inner_type)
        } else {
            false
        }
    }
    
    fn clone_box(&self) -> Box<dyn HuanType> {
        Box::new(OptionType {
            inner_type: self.inner_type.clone(),
        })
    }
}

impl HuanType for FuncType {
    fn mnemonic(&self) -> &'static str {
        "func"
    }
    
    fn to_string(&self) -> String {
        let inputs = self.input_types
            .iter()
            .map(|t| t.to_string())
            .collect::<Vec<_>>()
            .join(", ");
        format!("func<({}) -> {}>", inputs, self.output_type.to_string())
    }
    
    fn is_compatible(&self, other: &dyn HuanType) -> bool {
        if let Some(other_func) = other.as_any().downcast_ref::<Self>() {
            if self.input_types.len() != other_func.input_types.len() {
                return false;
            }
            self.input_types.iter().zip(&other_func.input_types)
                .all(|(a, b)| a.is_compatible(&**b)) &&
            self.output_type.is_compatible(&*other_func.output_type)
        } else {
            false
        }
    }
    
    fn clone_box(&self) -> Box<dyn HuanType> {
        Box::new(FuncType {
            input_types: self.input_types.clone(),
            output_type: self.output_type.clone(),
        })
    }
}





// 从AstType转换为HuanType
pub fn from_ast_type(ast_ty: &AstType) -> Result<Box<dyn HuanType>, String> {
    match ast_ty {
        AstType::Int => Ok(Box::new(IntType)),
        AstType::I8 => Ok(Box::new(I8Type)),
        AstType::I16 => Ok(Box::new(I16Type)),
        AstType::I32 => Ok(Box::new(I32Type)),
        AstType::I64 => Ok(Box::new(I64Type)),
        AstType::U8 => Ok(Box::new(U8Type)),
        AstType::U16 => Ok(Box::new(U16Type)),
        AstType::U32 => Ok(Box::new(U32Type)),
        AstType::U64 => Ok(Box::new(U64Type)),
        AstType::F32 => Ok(Box::new(F32Type)),
        AstType::F64 => Ok(Box::new(F64Type)),
        AstType::Bool => Ok(Box::new(BoolType)),
        AstType::Char => Ok(Box::new(CharType)),
        AstType::String => Ok(Box::new(StringType)),
        AstType::Unit => Ok(Box::new(UnitType)),
        AstType::List(inner) => {
            let inner_ty = from_ast_type(inner)?;
            Ok(Box::new(ListType { element_type: inner_ty }))
        }
        AstType::Ptr(inner) => {
            let inner_ty = from_ast_type(inner)?;
            Ok(Box::new(PtrType { pointee_type: inner_ty }))
        }
        AstType::Func(params, ret) => {
            let param_tys = params.iter()
                .map(|p| from_ast_type(p))
                .collect::<Result<Vec<_>, _>>()?;
            let ret_ty = from_ast_type(ret)?;
            Ok(Box::new(FuncType {
                input_types: param_tys,
                output_type: ret_ty,
            }))
        }
        _ => Err(format!("不支持的类型转换: {:?}", ast_ty)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_types() {
        assert_eq!(IntType.mnemonic(), "int");
        assert_eq!(I32Type.mnemonic(), "i32");
        assert_eq!(BoolType.mnemonic(), "bool");
        assert_eq!(F64Type.mnemonic(), "f64");
    }

    #[test]
    fn test_list_type() {
        let list_ty = ListType {
            element_type: Box::new(I32Type),
        };
        assert_eq!(list_ty.to_string(), "list<i32>");
    }

    #[test]
    fn test_func_type() {
        let func_ty = FuncType {
            input_types: vec![Box::new(I32Type), Box::new(StringType)],
            output_type: Box::new(F64Type),
        };
        assert_eq!(func_ty.to_string(), "func<(i32, string) -> f64>");
    }
}

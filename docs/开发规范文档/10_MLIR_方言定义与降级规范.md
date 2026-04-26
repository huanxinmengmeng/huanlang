# 幻语编程语言 - 完整详细的开发规范文档

## 许可证

本项目采用 [Apache License 2.0](../LICENSE) 开源协议。
版权所有 © 2026 幻心梦梦 (huanxinmengmeng)

**更新日期：2026年4月**  
**适用对象：编译器开发者、工具链开发者**  

> **说明**：本文档为幻语（HuanLang）编程语言的**完整开发规范**，旨在提供精确的实现指导。所有接口定义均采用 Rust 风格伪代码或 TableGen/IDL 描述，可直接转换为实际代码。  

---


# 10. MLIR 方言定义与降级规范

## 10.1 MLIR 方言概述

MLIR（Multi-Level Intermediate Representation）是 LLVM 项目中的多层中间表示框架。幻语编译器使用 MLIR 作为核心 IR，定义了 `huan` 方言来表示幻语的高级语义，然后通过一系列降级 Pass 逐步转换为标准 MLIR 方言（如 `scf`、`arith`、`func`、`llvm`），最终生成 LLVM IR 或目标代码。

**选择 MLIR 的理由**：
- **多层抽象**：可在不同层级进行优化，如高级循环优化、内存布局优化。
- **可扩展方言**：可定义与幻语语义直接对应的操作，保留高级信息以利于优化。
- **复用生态**：可直接使用 MLIR 内置的优化 Pass 和转换基础设施。
- **多目标支持**：通过 `llvm` 方言可无缝生成 LLVM IR，进而支持多种 CPU/GPU 架构。

## 10.2 幻语方言（`huan`）设计

### 10.2.1 方言定义（TableGen）

使用 MLIR 的 ODS（Operation Definition Specification）框架定义 `huan` 方言。以下是核心 TableGen 文件 `HuanDialect.td` 的内容。

```tablegen
//===-- HuanDialect.td - 幻语方言定义 -------------------*- tablegen -*--===//

#ifndef HUAN_DIALECT_TD
#define HUAN_DIALECT_TD

include "mlir/IR/OpBase.td"
include "mlir/IR/AttrTypeBase.td"
include "mlir/Interfaces/SideEffectInterfaces.td"
include "mlir/Interfaces/ControlFlowInterfaces.td"
include "mlir/Interfaces/FunctionInterfaces.td"
include "mlir/Interfaces/InferTypeOpInterface.td"

//===----------------------------------------------------------------------===//
// 幻语方言
//===----------------------------------------------------------------------===//

def Huan_Dialect : Dialect {
    let name = "huan";
    let summary = "幻语（HuanLang）高级编程语言方言";
    let description = [{
        幻语方言表示了幻语言的高级语义，包括变量声明、控制流、
        函数定义、复合类型操作等。它是从 AST 直接生成的初始 IR。
    }];
    let cppNamespace = "::mlir::huan";
    let usePropertiesForAttributes = 1;
}

//===----------------------------------------------------------------------===//
// 幻语类型
//===----------------------------------------------------------------------===//

class Huan_Type<string name, string typeMnemonic>
    : TypeDef<Huan_Dialect, name> {
    let mnemonic = typeMnemonic;
}

def Huan_IntType : Huan_Type<"Int", "int"> {
    let summary = "平台相关整数类型（64位）";
}

def Huan_I8Type  : Huan_Type<"I8", "i8">;
def Huan_I16Type : Huan_Type<"I16", "i16">;
def Huan_I32Type : Huan_Type<"I32", "i32">;
def Huan_I64Type : Huan_Type<"I64", "i64">;
def Huan_U8Type  : Huan_Type<"U8", "u8">;
def Huan_U16Type : Huan_Type<"U16", "u16">;
def Huan_U32Type : Huan_Type<"U32", "u32">;
def Huan_U64Type : Huan_Type<"U64", "u64">;
def Huan_F32Type : Huan_Type<"F32", "f32">;
def Huan_F64Type : Huan_Type<"F64", "f64">;
def Huan_BoolType : Huan_Type<"Bool", "bool">;
def Huan_CharType : Huan_Type<"Char", "char">;
def Huan_StringType : Huan_Type<"String", "string">;
def Huan_UnitType : Huan_Type<"Unit", "unit">;

def Huan_ListType : Huan_Type<"List", "list"> {
    let parameters = (ins "mlir::Type":$elementType);
    let assemblyFormat = "`<` $elementType `>`";
}

def Huan_ArrayType : Huan_Type<"Array", "array"> {
    let parameters = (ins "mlir::Type":$elementType, "uint64_t":$size);
    let assemblyFormat = "`<` $elementType `,` $size `>`";
}

def Huan_MapType : Huan_Type<"Map", "map"> {
    let parameters = (ins "mlir::Type":$keyType, "mlir::Type":$valueType);
    let assemblyFormat = "`<` $keyType `,` $valueType `>`";
}

def Huan_PtrType : Huan_Type<"Ptr", "ptr"> {
    let parameters = (ins "mlir::Type":$pointeeType);
    let assemblyFormat = "`<` $pointeeType `>`";
}

def Huan_OptionType : Huan_Type<"Option", "option"> {
    let parameters = (ins "mlir::Type":$innerType);
    let assemblyFormat = "`<` $innerType `>`";
}

def Huan_FuncType : Huan_Type<"Func", "func"> {
    let parameters = (ins "ArrayRef<mlir::Type>":$inputs, "mlir::Type":$output);
    let assemblyFormat = "`<` `(` $inputs `)` `->` $output `>`";
}

//===----------------------------------------------------------------------===//
// 幻语操作基类
//===----------------------------------------------------------------------===//

class Huan_Op<string mnemonic, list<Trait> traits = []>
    : Op<Huan_Dialect, mnemonic, traits>;

//===----------------------------------------------------------------------===//
// 变量与常量操作
//===----------------------------------------------------------------------===//

def Huan_LingOp : Huan_Op<"ling"> {
    let summary = "变量声明：令 名称 类型 类型 为 值";
    let arguments = (ins StrAttr:$sym_name, TypeAttr:$type, AnyType:$value);
    let results = (outs);
    let assemblyFormat = "$sym_name `:` $type `为` $value attr-dict";
    let hasVerifier = 1;
}

def Huan_DingOp : Huan_Op<"ding"> {
    let summary = "常量声明：定 名称 类型 类型 为 值";
    let arguments = (ins StrAttr:$sym_name, TypeAttr:$type, AnyType:$value);
    let results = (outs);
    let assemblyFormat = "$sym_name `:` $type `为` $value attr-dict";
    let hasVerifier = 1;
}

def Huan_SheweiOp : Huan_Op<"shewei"> {
    let summary = "赋值：目标 设为 值";
    let arguments = (ins AnyType:$target, AnyType:$value);
    let results = (outs);
    let assemblyFormat = "$target `设为` $value attr-dict";
}

//===----------------------------------------------------------------------===//
// 控制流操作
//===----------------------------------------------------------------------===//

def Huan_RuoOp : Huan_Op<"ruo", [
    DeclareOpInterfaceMethods<RegionBranchOpInterface>
]> {
    let summary = "条件分支：若 条件 则 真块 否则 假块 结束";
    let arguments = (ins I1:$condition);
    let regions = (region SizedRegion<1>:$thenRegion, AnyRegion:$elseRegion);
    let assemblyFormat = "$condition `则` $thenRegion (`否则` $elseRegion^)? `结束` attr-dict";
    let hasVerifier = 1;
}

def Huan_PipeiOp : Huan_Op<"pipei", [
    DeclareOpInterfaceMethods<RegionBranchOpInterface>
]> {
    let summary = "模式匹配：匹配 表达式 { 当 模式 => 块 } 结束";
    let arguments = (ins AnyType:$value);
    let regions = (region VariadicRegion<SizedRegion<1>>:$arms, AnyRegion:$default);
    let assemblyFormat = "$value $arms (`默认` $default^)? `结束` attr-dict";
    let hasVerifier = 1;
}

def Huan_ChongfuOp : Huan_Op<"chongfu"> {
    let summary = "固定次数循环：重复 n 次 块 结束";
    let arguments = (ins I64:$count);
    let regions = (region SizedRegion<1>:$body);
    let assemblyFormat = "$count `次` $body `结束` attr-dict";
}

def Huan_DangOp : Huan_Op<"dang"> {
    let summary = "条件循环：当 条件 循环 块 结束";
    let arguments = (ins I1:$condition);
    let regions = (region SizedRegion<1>:$body);
    let assemblyFormat = "$condition `循环` $body `结束` attr-dict";
}

def Huan_DuiyuOp : Huan_Op<"duiyu"> {
    let summary = "遍历循环：对于 每个 变量 在 容器 中 块 结束";
    let arguments = (ins AnyType:$iterable);
    let regions = (region SizedRegion<1>:$body);
    let assemblyFormat = "`每个` $iterable `中` $body `结束` attr-dict";
}

//===----------------------------------------------------------------------===//
// 函数操作
//===----------------------------------------------------------------------===//

def Huan_HanshuOp : Huan_Op<"hanshu", [
    IsolatedFromAbove,
    FunctionOpInterface
]> {
    let summary = "函数定义：函数 名称(参数) 返回 类型 块 结束";
    let arguments = (ins StrAttr:$sym_name, TypeAttr:$function_type);
    let regions = (region AnyRegion:$body);
    let assemblyFormat = "$sym_name $function_type $body `结束` attr-dict";
    let hasVerifier = 1;
}

def Huan_FanhuiOp : Huan_Op<"fanhui", [
    ReturnLike, Terminator,
    HasParent<"Huan_HanshuOp">
]> {
    let summary = "返回语句：返回 值";
    let arguments = (ins Optional<AnyType>:$value);
    let assemblyFormat = "attr-dict ($value^)?";
    let hasVerifier = 1;
}

def Huan_DiaoyongOp : Huan_Op<"diaoyong"> {
    let summary = "函数调用：调用 函数(参数)";
    let arguments = (ins FlatSymbolRefAttr:$callee, Variadic<AnyType>:$args);
    let results = (outs Variadic<AnyType>:$results);
    let assemblyFormat = "$callee `(` $args `)` attr-dict";
}

//===----------------------------------------------------------------------===//
// 算术与逻辑操作
//===----------------------------------------------------------------------===//

def Huan_JiaOp : Huan_Op<"jia"> {
    let summary = "加法";
    let arguments = (ins AnyType:$lhs, AnyType:$rhs);
    let results = (outs AnyType:$result);
    let assemblyFormat = "$lhs `加` $rhs attr-dict";
    let hasVerifier = 1;
}

def Huan_JianOp : Huan_Op<"jian"> {
    let summary = "减法";
    let arguments = (ins AnyType:$lhs, AnyType:$rhs);
    let results = (outs AnyType:$result);
    let assemblyFormat = "$lhs `减` $rhs attr-dict";
}

def Huan_ChengOp : Huan_Op<"cheng"> {
    let summary = "乘法";
    let arguments = (ins AnyType:$lhs, AnyType:$rhs);
    let results = (outs AnyType:$result);
    let assemblyFormat = "$lhs `乘` $rhs attr-dict";
}

def Huan_ChuOp : Huan_Op<"chu"> {
    let summary = "除法";
    let arguments = (ins AnyType:$lhs, AnyType:$rhs);
    let results = (outs AnyType:$result);
    let assemblyFormat = "$lhs `除` $rhs attr-dict";
}

def Huan_QuyuOp : Huan_Op<"quyu"> {
    let summary = "取余";
    let arguments = (ins AnyType:$lhs, AnyType:$rhs);
    let results = (outs AnyType:$result);
    let assemblyFormat = "$lhs `取余` $rhs attr-dict";
}

def Huan_DayuOp : Huan_Op<"dayu"> {
    let summary = "大于比较";
    let arguments = (ins AnyType:$lhs, AnyType:$rhs);
    let results = (outs I1:$result);
    let assemblyFormat = "$lhs `大于` $rhs attr-dict";
}

def Huan_XiaoyuOp : Huan_Op<"xiaoyu"> {
    let summary = "小于比较";
    let arguments = (ins AnyType:$lhs, AnyType:$rhs);
    let results = (outs I1:$result);
    let assemblyFormat = "$lhs `小于` $rhs attr-dict";
}

def Huan_DengyuOp : Huan_Op<"dengyu"> {
    let summary = "等于比较";
    let arguments = (ins AnyType:$lhs, AnyType:$rhs);
    let results = (outs I1:$result);
    let assemblyFormat = "$lhs `等于` $rhs attr-dict";
}

def Huan_QieOp : Huan_Op<"qie"> {
    let summary = "逻辑与";
    let arguments = (ins I1:$lhs, I1:$rhs);
    let results = (outs I1:$result);
    let assemblyFormat = "$lhs `且` $rhs attr-dict";
}

def Huan_HuoOp : Huan_Op<"huo"> {
    let summary = "逻辑或";
    let arguments = (ins I1:$lhs, I1:$rhs);
    let results = (outs I1:$result);
    let assemblyFormat = "$lhs `或` $rhs attr-dict";
}

def Huan_FeiOp : Huan_Op<"fei"> {
    let summary = "逻辑非";
    let arguments = (ins I1:$operand);
    let results = (outs I1:$result);
    let assemblyFormat = "`非` $operand attr-dict";
}

//===----------------------------------------------------------------------===//
// 复合类型操作
//===----------------------------------------------------------------------===//

def Huan_LiebiaoOp : Huan_Op<"liebiao"> {
    let summary = "创建列表";
    let arguments = (ins Variadic<AnyType>:$elements);
    let results = (outs Huan_ListType:$result);
    let assemblyFormat = "`[` $elements `]` attr-dict";
}

def Huan_ZidianOp : Huan_Op<"zidian"> {
    let summary = "创建字典";
    let arguments = (ins Variadic<AnyType>:$keys, Variadic<AnyType>:$values);
    let results = (outs Huan_MapType:$result);
    let assemblyFormat = "`{` $keys `:` $values `}` attr-dict";
}

def Huan_ZhuizhuiOp : Huan_Op<"zhuizhui"> {
    let summary = "列表追加元素";
    let arguments = (ins Huan_ListType:$list, AnyType:$element);
    let results = (outs);
    let assemblyFormat = "$list `追加` $element attr-dict";
}

def Huan_SuoyinOp : Huan_Op<"suoyin"> {
    let summary = "索引访问";
    let arguments = (ins AnyType:$container, AnyType:$index);
    let results = (outs AnyType:$result);
    let assemblyFormat = "$container `[` $index `]` attr-dict";
}

def Huan_ZiduanOp : Huan_Op<"ziduan"> {
    let summary = "字段访问";
    let arguments = (ins AnyType:$object, StrAttr:$field);
    let results = (outs AnyType:$result);
    let assemblyFormat = "$object `.` $field attr-dict";
}

//===----------------------------------------------------------------------===//
// 内联汇编
//===----------------------------------------------------------------------===//

def Huan_AsmOp : Huan_Op<"asm"> {
    let summary = "内联汇编";
    let arguments = (ins StrAttr:$asm_string, StrAttr:$constraints);
    let results = (outs Variadic<AnyType>:$results);
    let assemblyFormat = "$asm_string $constraints attr-dict";
    let hasVerifier = 1;
}

#endif // HUAN_DIALECT_TD
```

### 10.2.2 方言接口实现（C++）

```cpp
// HuanDialect.h
#ifndef HUAN_DIALECT_H
#define HUAN_DIALECT_H

#include "mlir/IR/Dialect.h"
#include "mlir/IR/BuiltinTypes.h"
#include "mlir/Interfaces/ControlFlowInterfaces.h"
#include "mlir/Interfaces/FunctionInterfaces.h"

// 包含 TableGen 生成的头文件
#include "HuanDialect.h.inc"

#endif // HUAN_DIALECT_H
```

```cpp
// HuanDialect.cpp
#include "HuanDialect.h"
#include "mlir/IR/Builders.h"
#include "mlir/IR/OpImplementation.h"

using namespace mlir;
using namespace mlir::huan;

// 方言初始化
void HuanDialect::initialize() {
    addOperations<
#define GET_OP_LIST
#include "HuanOps.cpp.inc"
    >();
    
    addTypes<
#define GET_TYPEDEF_LIST
#include "HuanTypes.cpp.inc"
    >();
}

//===----------------------------------------------------------------------===//
// 类型实现
//===----------------------------------------------------------------------===//

#define GET_TYPEDEF_CLASSES
#include "HuanTypes.cpp.inc"

//===----------------------------------------------------------------------===//
// 操作验证器实现示例
//===----------------------------------------------------------------------===//

LogicalResult Huan_LingOp::verify() {
    // 验证值的类型与声明的类型是否匹配
    auto valueType = getValue().getType();
    auto declaredType = getType();
    if (valueType != declaredType) {
        return emitOpError("值的类型与声明类型不匹配");
    }
    return success();
}

LogicalResult Huan_RuoOp::verify() {
    // 验证条件是否为布尔类型
    if (!getCondition().getType().isInteger(1)) {
        return emitOpError("条件必须是布尔类型");
    }
    return success();
}

//===----------------------------------------------------------------------===//
// RegionBranchOpInterface 实现（用于控制流）
//===----------------------------------------------------------------------===//

void Huan_RuoOp::getSuccessorRegions(
    mlir::RegionBranchPoint point, 
    SmallVectorImpl<mlir::RegionSuccessor> &regions) {
    // 条件分支的后继区域：then 区域和 else 区域
    if (point.isParent()) {
        regions.push_back(RegionSuccessor(&getThenRegion()));
        if (!getElseRegion().empty()) {
            regions.push_back(RegionSuccessor(&getElseRegion()));
        }
        return;
    }
    // 从 then/else 区域离开后，返回到父操作之后
    regions.push_back(RegionSuccessor(getResults()));
}

void Huan_RuoOp::getEntrySuccessorRegions(
    ArrayRef<Attribute> operands,
    SmallVectorImpl<RegionSuccessor> &regions) {
    getSuccessorRegions(RegionBranchPoint::parent(), regions);
}

//===----------------------------------------------------------------------===//
// FunctionOpInterface 实现（用于函数）
//===----------------------------------------------------------------------===//

Region *Huan_HanshuOp::getCallableRegion() {
    return &getBody();
}

ArrayRef<Type> Huan_HanshuOp::getArgumentTypes() {
    auto funcType = getFunctionType().cast<Huan_FuncType>();
    return funcType.getInputs();
}

ArrayRef<Type> Huan_HanshuOp::getResultTypes() {
    auto funcType = getFunctionType().cast<Huan_FuncType>();
    return {funcType.getOutput()};
}

// 包含 TableGen 生成的实现
#define GET_OP_CLASSES
#include "HuanOps.cpp.inc"
```

## 10.3 AST 到 MLIR 的转换

### 10.3.1 转换器结构

```cpp
// ASTToMLIR.h
class ASTToMLIRConverter {
public:
    ASTToMLIRConverter(MLIRContext *context, ModuleOp module);
    
    // 转换整个程序
    LogicalResult convertProgram(const Program &program);
    
private:
    MLIRContext *ctx;
    ModuleOp module;
    OpBuilder builder;
    SymbolTable symbolTable;
    
    // 环境管理
    llvm::StringMap<Value> variables;  // 当前作用域的变量映射
    llvm::SmallVector<llvm::StringMap<Value>> scopeStack;
    
    // 转换函数
    LogicalResult convertItem(const Item &item);
    LogicalResult convertFunction(const Function &func);
    LogicalResult convertStruct(const Struct &s);
    LogicalResult convertStmt(const Stmt &stmt);
    Value convertExpr(const Expr &expr);
    Type convertType(const Type &type);
    
    // 作用域管理
    void enterScope();
    void exitScope();
    void defineVariable(StringRef name, Value value);
    Value lookupVariable(StringRef name);
};
```

### 10.3.2 表达式转换

```cpp
Value ASTToMLIRConverter::convertExpr(const Expr &expr) {
    return match(expr,
        [&](const IntLit &lit) -> Value {
            return builder.create<arith::ConstantIntOp>(
                loc, lit.value, builder.getIntegerType(64));
        },
        [&](const FloatLit &lit) -> Value {
            return builder.create<arith::ConstantFloatOp>(
                loc, APFloat(lit.value), builder.getF64Type());
        },
        [&](const StringLit &lit) -> Value {
            return builder.create<huan::StringLitOp>(loc, lit.value);
        },
        [&](const Ident &ident) -> Value {
            return lookupVariable(ident.name);
        },
        [&](const BinaryOp &binop) -> Value {
            Value lhs = convertExpr(*binop.left);
            Value rhs = convertExpr(*binop.right);
            switch (binop.op) {
                case BinaryOp::Add:
                    return builder.create<huan::JiaOp>(loc, lhs, rhs);
                case BinaryOp::Sub:
                    return builder.create<huan::JianOp>(loc, lhs, rhs);
                case BinaryOp::Mul:
                    return builder.create<huan::ChengOp>(loc, lhs, rhs);
                case BinaryOp::Div:
                    return builder.create<huan::ChuOp>(loc, lhs, rhs);
                // ... 其他运算符
            }
        },
        [&](const Call &call) -> Value {
            SmallVector<Value> args;
            for (const auto &arg : call.args) {
                args.push_back(convertExpr(arg));
            }
            return builder.create<huan::DiaoyongOp>(
                loc, call.func.name, args);
        },
        // ... 其他表达式类型
    );
}
```

### 10.3.3 语句转换

```cpp
LogicalResult ASTToMLIRConverter::convertStmt(const Stmt &stmt) {
    return match(stmt,
        [&](const Let &let) -> LogicalResult {
            Value value = convertExpr(*let.value);
            defineVariable(let.name.name, value);
            builder.create<huan::LingOp>(loc, let.name.name, 
                                         convertType(let.ty), value);
            return success();
        },
        [&](const If &ifStmt) -> LogicalResult {
            Value cond = convertExpr(*ifStmt.cond);
            auto ruoOp = builder.create<huan::RuoOp>(loc, cond);
            
            // 构建 then 区域
            Region &thenRegion = ruoOp.getThenRegion();
            Block *thenBlock = builder.createBlock(&thenRegion);
            builder.setInsertionPointToStart(thenBlock);
            for (const auto &s : ifStmt.then_block) {
                if (failed(convertStmt(s))) return failure();
            }
            
            // 构建 else 区域
            if (ifStmt.else_block) {
                Region &elseRegion = ruoOp.getElseRegion();
                Block *elseBlock = builder.createBlock(&elseRegion);
                builder.setInsertionPointToStart(elseBlock);
                for (const auto &s : *ifStmt.else_block) {
                    if (failed(convertStmt(s))) return failure();
                }
            }
            
            builder.setInsertionPointAfter(ruoOp);
            return success();
        },
        [&](const While &whileStmt) -> LogicalResult {
            Value cond = convertExpr(*whileStmt.cond);
            auto dangOp = builder.create<huan::DangOp>(loc, cond);
            
            Region &body = dangOp.getBody();
            Block *bodyBlock = builder.createBlock(&body);
            builder.setInsertionPointToStart(bodyBlock);
            for (const auto &s : whileStmt.body) {
                if (failed(convertStmt(s))) return failure();
            }
            builder.setInsertionPointAfter(dangOp);
            return success();
        },
        [&](const Return &ret) -> LogicalResult {
            if (ret.value) {
                Value val = convertExpr(*ret.value);
                builder.create<huan::FanhuiOp>(loc, val);
            } else {
                builder.create<huan::FanhuiOp>(loc);
            }
            return success();
        },
        // ... 其他语句类型
    );
}
```

## 10.4 降级 Pass 管线

### 10.4.1 降级流程概览

幻语 MLIR 降级到 LLVM IR 的完整流程：

```
huan 方言
    │
    ├── HuanToScfPass      (控制流降级：huan.ruo → scf.if)
    ├── HuanToArithPass    (算术降级：huan.jia → arith.addi)
    ├── HuanToFuncPass     (函数降级：huan.hanshu → func.func)
    ├── HuanToMemRefPass   (复合类型降级：列表/字典 → memref/结构体)
    │
    ▼
scf + arith + func + memref 方言
    │
    ├── ConvertSCFToCFPass
    ├── ConvertFuncToLLVMPass
    ├── ConvertArithToLLVMPass
    ├── ConvertMemRefToLLVMPass
    │
    ▼
llvm 方言
    │
    ▼
LLVM IR
```

### 10.4.2 Pass 声明（TableGen）

```tablegen
// HuanPasses.td
include "mlir/Pass/PassBase.td"

def HuanToScfPass : Pass<"huan-to-scf", "ModuleOp"> {
    let summary = "将幻语控制流降级为 SCF 方言";
    let constructor = "mlir::huan::createHuanToScfPass()";
    let dependentDialects = ["scf::SCFDialect", "arith::ArithDialect"];
}

def HuanToArithPass : Pass<"huan-to-arith", "ModuleOp"> {
    let summary = "将幻语算术操作降级为 Arith 方言";
    let constructor = "mlir::huan::createHuanToArithPass()";
    let dependentDialects = ["arith::ArithDialect"];
}

def HuanToFuncPass : Pass<"huan-to-func", "ModuleOp"> {
    let summary = "将幻语函数降级为 Func 方言";
    let constructor = "mlir::huan::createHuanToFuncPass()";
    let dependentDialects = ["func::FuncDialect"];
}

def LowerHuanToLLVMPass : Pass<"lower-huan-to-llvm", "ModuleOp"> {
    let summary = "完整的幻语到 LLVM 降级管线";
    let constructor = "mlir::huan::createLowerHuanToLLVMPass()";
    let dependentDialects = ["llvm::LLVMDialect"];
}
```

### 10.4.3 HuanToScfPass 实现

```cpp
// HuanToScfPass.cpp
#include "mlir/Dialect/SCF/IR/SCF.h"
#include "mlir/Dialect/Arith/IR/Arith.h"
#include "mlir/Transforms/DialectConversion.h"
#include "HuanDialect.h"

using namespace mlir;
using namespace mlir::huan;

namespace {

// 将 huan.ruo 转换为 scf.if
struct RuoOpLowering : public OpConversionPattern<Huan_RuoOp> {
    using OpConversionPattern::OpConversionPattern;
    
    LogicalResult matchAndRewrite(
        Huan_RuoOp op, OpAdaptor adaptor,
        ConversionPatternRewriter &rewriter) const override {
        
        auto loc = op.getLoc();
        auto cond = adaptor.getCondition();
        
        // 创建 scf.if 操作
        auto ifOp = rewriter.create<scf::IfOp>(
            loc, cond, /*withElseRegion=*/!op.getElseRegion().empty());
        
        // 填充 then 区域
        rewriter.inlineRegionBefore(op.getThenRegion(), 
                                    ifOp.getThenRegion(), 
                                    ifOp.getThenRegion().begin());
        
        // 填充 else 区域
        if (!op.getElseRegion().empty()) {
            rewriter.inlineRegionBefore(op.getElseRegion(),
                                        ifOp.getElseRegion(),
                                        ifOp.getElseRegion().begin());
        }
        
        rewriter.eraseOp(op);
        return success();
    }
};

// 将 huan.chongfu 转换为 scf.for
struct ChongfuOpLowering : public OpConversionPattern<Huan_ChongfuOp> {
    using OpConversionPattern::OpConversionPattern;
    
    LogicalResult matchAndRewrite(
        Huan_ChongfuOp op, OpAdaptor adaptor,
        ConversionPatternRewriter &rewriter) const override {
        
        auto loc = op.getLoc();
        auto count = adaptor.getCount();
        
        Value zero = rewriter.create<arith::ConstantIndexOp>(loc, 0);
        Value one = rewriter.create<arith::ConstantIndexOp>(loc, 1);
        Value upperBound = rewriter.create<arith::IndexCastOp>(
            loc, rewriter.getIndexType(), count);
        
        auto forOp = rewriter.create<scf::ForOp>(
            loc, zero, upperBound, one);
        
        rewriter.inlineRegionBefore(op.getBody(), forOp.getRegion(),
                                    forOp.getRegion().begin());
        
        rewriter.eraseOp(op);
        return success();
    }
};

struct HuanToScfPass : public PassWrapper<HuanToScfPass, OperationPass<ModuleOp>> {
    MLIR_DEFINE_EXPLICIT_INTERNAL_INLINE_TYPE_ID(HuanToScfPass)
    
    void runOnOperation() override {
        ModuleOp module = getOperation();
        MLIRContext *ctx = &getContext();
        
        RewritePatternSet patterns(ctx);
        patterns.add<RuoOpLowering, ChongfuOpLowering>(ctx);
        
        ConversionTarget target(*ctx);
        target.addLegalDialect<scf::SCFDialect, arith::ArithDialect>();
        target.addIllegalOp<Huan_RuoOp, Huan_ChongfuOp>();
        
        if (failed(applyPartialConversion(module, target, std::move(patterns)))) {
            signalPassFailure();
        }
    }
};

} // namespace

namespace mlir::huan {
std::unique_ptr<Pass> createHuanToScfPass() {
    return std::make_unique<HuanToScfPass>();
}
} // namespace mlir::huan
```

### 10.4.4 HuanToArithPass 实现

```cpp
// HuanToArithPass.cpp
struct JiaOpLowering : public OpConversionPattern<Huan_JiaOp> {
    using OpConversionPattern::OpConversionPattern;
    
    LogicalResult matchAndRewrite(
        Huan_JiaOp op, OpAdaptor adaptor,
        ConversionPatternRewriter &rewriter) const override {
        
        auto loc = op.getLoc();
        auto lhs = adaptor.getLhs();
        auto rhs = adaptor.getRhs();
        
        Value result;
        if (lhs.getType().isInteger(64)) {
            result = rewriter.create<arith::AddIOp>(loc, lhs, rhs);
        } else if (lhs.getType().isF64()) {
            result = rewriter.create<arith::AddFOp>(loc, lhs, rhs);
        } else {
            return failure();
        }
        
        rewriter.replaceOp(op, result);
        return success();
    }
};

// 类似的模式：JianOp, ChengOp, ChuOp, DayuOp, XiaoyuOp...
```

### 10.4.5 完整的 LowerHuanToLLVMPass

```cpp
void LowerHuanToLLVMPass::runOnOperation() {
    ModuleOp module = getOperation();
    MLIRContext *ctx = &getContext();
    
    PassManager pm(ctx);
    
    // 第一阶段：幻语方言内部优化
    pm.addPass(createCSEPass());                // 公共子表达式消除
    pm.addPass(createCanonicalizerPass());      // 规范化
    
    // 第二阶段：降级到标准方言
    pm.addPass(createHuanToScfPass());
    pm.addPass(createHuanToArithPass());
    pm.addPass(createHuanToFuncPass());
    
    // 第三阶段：标准方言降级到 LLVM
    pm.addPass(createConvertSCFToCFPass());
    pm.addPass(createConvertFuncToLLVMPass());
    pm.addPass(createArithToLLVMConversionPass());
    pm.addPass(createConvertMathToLLVMPass());
    pm.addPass(createConvertControlFlowToLLVMPass());
    
    // 第四阶段：LLVM 方言优化
    pm.addPass(createCSEPass());
    pm.addPass(createCanonicalizerPass());
    
    if (failed(pm.run(module))) {
        signalPassFailure();
    }
}
```

## 10.5 类型转换器

```cpp
class HuanToLLVMTypeConverter : public LLVMTypeConverter {
public:
    HuanToLLVMTypeConverter(MLIRContext *ctx) : LLVMTypeConverter(ctx) {
        addConversion([&](Huan_IntType type) {
            return IntegerType::get(ctx, 64);
        });
        addConversion([&](Huan_I32Type type) {
            return IntegerType::get(ctx, 32);
        });
        addConversion([&](Huan_F64Type type) {
            return Float64Type::get(ctx);
        });
        addConversion([&](Huan_BoolType type) {
            return IntegerType::get(ctx, 1);
        });
        addConversion([&](Huan_StringType type) {
            // 字符串表示为 { ptr, i64 }
            auto ptrTy = LLVM::LLVMPointerType::get(IntegerType::get(ctx, 8));
            auto lenTy = IntegerType::get(ctx, 64);
            return LLVM::LLVMStructType::getLiteral(ctx, {ptrTy, lenTy});
        });
        addConversion([&](Huan_ListType type) {
            // 列表表示为 { ptr, i64, i64, i64 } (数据, 长度, 容量, 元素大小)
            auto ptrTy = LLVM::LLVMPointerType::get(IntegerType::get(ctx, 8));
            auto intTy = IntegerType::get(ctx, 64);
            return LLVM::LLVMStructType::getLiteral(ctx, 
                {ptrTy, intTy, intTy, intTy});
        });
        // 其他类型转换...
    }
};
```

## 10.6 方言内部优化 Pass

### 10.6.1 常量折叠

```cpp
struct ConstantFolding : public OpRewritePattern<Huan_JiaOp> {
    using OpRewritePattern::OpRewritePattern;
    
    LogicalResult matchAndRewrite(Huan_JiaOp op,
                                  PatternRewriter &rewriter) const override {
        auto lhs = op.getLhs().getDefiningOp<arith::ConstantOp>();
        auto rhs = op.getRhs().getDefiningOp<arith::ConstantOp>();
        
        if (lhs && rhs) {
            if (auto lhsInt = lhs.getValue().dyn_cast<IntegerAttr>()) {
                if (auto rhsInt = rhs.getValue().dyn_cast<IntegerAttr>()) {
                    auto result = lhsInt.getInt() + rhsInt.getInt();
                    auto newConst = rewriter.create<arith::ConstantOp>(
                        op.getLoc(), rewriter.getIntegerAttr(op.getType(), result));
                    rewriter.replaceOp(op, newConst);
                    return success();
                }
            }
        }
        return failure();
    }
};
```

### 10.6.2 死代码消除

```cpp
struct DeadCodeElimination : public PassWrapper<DeadCodeElimination, OperationPass<ModuleOp>> {
    void runOnOperation() override {
        // 使用标准的 MLIR 死代码消除 Pass
        OpPassManager pm;
        pm.addPass(createCSEPass());
        pm.addPass(createCanonicalizerPass());
    }
};
```

## 10.7 调试信息生成

幻语编译器通过 LLVM 的 DIBuilder 生成 DWARF 调试信息。

```cpp
class DebugInfoBuilder {
public:
    DebugInfoBuilder(MLIRContext *ctx, ModuleOp module);
    
    void setLocation(OpBuilder &builder, const SourceSpan &span);
    void finalize();
    
private:
    mlir::LLVM::DIBuilder dibuilder;
    mlir::LLVM::DIFile file;
    mlir::LLVM::DICompileUnit compileUnit;
};
```

## 10.8 测试用例

```mlir
// 测试幻语方言的基本操作
module {
  func.func @test_add(%arg0: i64, %arg1: i64) -> i64 {
    %0 = huan.jia %arg0 加 %arg1 : i64
    return %0 : i64
  }
  
  func.func @test_if(%arg0: i1) {
    huan.ruo %arg0 则 {
      // then block
      %c = arith.constant 42 : i64
    } 否则 {
      // else block
      %c = arith.constant 0 : i64
    } 结束
    return
  }
}
```

---

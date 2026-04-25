// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 幻语 MLIR 方言类型定义

include "mlir/IR/TypeBase.td"

// 方言定义
define_dialect "huan" HLDialect {
  let cppNamespace = "::huan";
}

// 基本类型
define_type HLIntType "int" {
  let parameters = (ins "unsigned":$width);
  let assemblyFormat = "`<` $width `>`";
  let cppNamespace = "::huan";
}

define_type HLFloatType "float" {
  let parameters = (ins "unsigned":$width);
  let assemblyFormat = "`<` $width `>`";
  let cppNamespace = "::huan";
}

define_type HLBoolType "bool" {
  let parameters = (ins);
  let assemblyFormat = "`()";
  let cppNamespace = "::huan";
}

define_type HLStringType "string" {
  let parameters = (ins);
  let assemblyFormat = "`()";
  let cppNamespace = "::huan";
}

define_type HLListType "list" {
  let parameters = (ins Type:$elementType);
  let assemblyFormat = "`<` $elementType `>`";
  let cppNamespace = "::huan";
}

define_type HLMapType "map" {
  let parameters = (ins Type:$keyType, Type:$valueType);
  let assemblyFormat = "`<` $keyType `,` $valueType `>`";
  let cppNamespace = "::huan";
}

define_type HLFuncType "func" {
  let parameters = (ins Variadic<Type>:$inputTypes, Variadic<Type>:$resultTypes);
  let assemblyFormat = "`<` $inputTypes `->` $resultTypes `>`";
  let cppNamespace = "::huan";
}

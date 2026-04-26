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
// 幻语 MLIR 方言操作定义

include "mlir/IR/OpBase.td"
include "mlir/IR/FunctionInterfaces.td"

// 方言定义
define_dialect "huan" HLDialect {
  let cppNamespace = "::huan";
}

// 基本类型定义
define_type HLLitType "lit" {
  let parameters = (ins "StringRef":$value);
  let assemblyFormat = "`<` $value `>`";
  let cppNamespace = "::huan";
}

// 操作基类
class HLOp<string mnemonic, list<OpTrait> traits = []> :
  Op<HLDialect, mnemonic, traits> {
  let cppNamespace = "::huan";
}

// 基本操作

// 变量声明
def HLVarOp : HLOp<"var"> {
  let arguments = (ins StrAttr:$name, TypeAttr:$type, Optional<AnyType>:$value);
  let results = (outs);
  let assemblyFormat = "`(` $name `:` $type `,` $value? `)`";
}

// 赋值操作
def HLAssignOp : HLOp<"assign"> {
  let arguments = (ins StrAttr:$name, AnyType:$value);
  let results = (outs);
  let assemblyFormat = "`(` $name `,` $value `)`";
}

// 函数定义
def HLFuncOp : HLOp<"func", [FunctionOpInterface]> {
  let arguments = (ins StrAttr:$name, TypeAttr:$type, Optional<Region>:$body);
  let results = (outs);
  let assemblyFormat = "`(` $name `:` $type `)` attr-dict-with-keyword $body?";
}

// 函数调用
def HLCallOp : HLOp<"call"> {
  let arguments = (ins StrAttr:$name, Variadic<AnyType>:$args);
  let results = (outs AnyType:$result);
  let assemblyFormat = "`(` $name `,` $args `)`";
}

// 条件判断
def HLIfOp : HLOp<"if"> {
  let arguments = (ins AnyType:$condition);
  let regions = (region AnyRegion:$thenRegion, AnyRegion:$elseRegion?);
  let assemblyFormat = "`(` $condition `)` attr-dict-with-keyword $thenRegion (`else` $elseRegion)?";
}

// 循环
def HLLoopOp : HLOp<"loop"> {
  let arguments = (ins AnyType:$condition);
  let regions = (region AnyRegion:$body);
  let assemblyFormat = "`(` $condition `)` attr-dict-with-keyword $body";
}

// 二元操作
def HLBinOp<string mnemonic> : HLOp<mnemonic> {
  let arguments = (ins AnyType:$lhs, AnyType:$rhs);
  let results = (outs AnyType:$result);
  let assemblyFormat = "`(` $lhs `,` $rhs `)`";
}

def HLAddOp : HLBinOp<"add">;
def HLSubOp : HLBinOp<"sub">;
def HLMulOp : HLBinOp<"mul">;
def HLDivOp : HLBinOp<"div">;

// 比较操作
def HLCompareOp<string mnemonic> : HLOp<mnemonic> {
  let arguments = (ins AnyType:$lhs, AnyType:$rhs);
  let results = (outs AnyType:$result);
  let assemblyFormat = "`(` $lhs `,` $rhs `)`";
}

def HLEqOp : HLCompareOp<"eq">;
def HLNeOp : HLCompareOp<"ne">;
def HLLtOp : HLCompareOp<"lt">;
def HLLeOp : HLCompareOp<"le">;
def HLGeOp : HLCompareOp<"ge">;
def HLGrOp : HLCompareOp<"gr">;

// 逻辑操作
def HLLogicalOp<string mnemonic> : HLOp<mnemonic> {
  let arguments = (ins AnyType:$lhs, AnyType:$rhs);
  let results = (outs AnyType:$result);
  let assemblyFormat = "`(` $lhs `,` $rhs `)`";
}

def HLAndOp : HLLogicalOp<"and">;
def HLOrOp : HLLogicalOp<"or">;
def HLNotOp : HLOp<"not"> {
  let arguments = (ins AnyType:$value);
  let results = (outs AnyType:$result);
  let assemblyFormat = "`(` $value `)`";
}

// 常量
def HLConstOp : HLOp<"const"> {
  let arguments = (ins AnyAttr:$value);
  let results = (outs AnyType:$result);
  let assemblyFormat = "`(` $value `)`";
}

// 返回操作
def HLReturnOp : HLOp<"return"> {
  let arguments = (ins Optional<AnyType>:$value);
  let results = (outs);
  let assemblyFormat = "`(` $value? `)`";
}

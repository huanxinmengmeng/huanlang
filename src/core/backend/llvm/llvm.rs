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

use super::super::{CodeGenerator, CodeGenOptions, TargetTriple};
use super::super::error::{CodeGenError, LinkError};
use crate::core::mlir::ModuleOp;
use crate::core::mlir::ops::{HanshuOp, FanhuiOp, JiaOp, JianOp, ChengOp, ChuOp, IntLitOp, IdentOp, FloatLitOp, HuanOp};
use crate::core::ast::Program;
use std::path::PathBuf;
use std::collections::HashMap;

use super::ast_to_llvm::{AstToLlvmCodeGen, validate_llvm_ir};

pub struct LLVMBackend {
    target: TargetTriple,
    #[allow(dead_code)]
    options: CodeGenOptions,
    symbols: HashMap<String, String>,
    #[allow(dead_code)]
    string_constants: Vec<String>,
}

impl LLVMBackend {
    pub fn new(target: TargetTriple, options: CodeGenOptions) -> Self {
        Self {
            target,
            options,
            symbols: HashMap::new(),
            string_constants: Vec::new(),
        }
    }

    /// 直接从 AST 生成 LLVM IR（新的完整功能）
    pub fn generate_from_ast(&mut self, program: &Program) -> Result<String, CodeGenError> {
        let mut codegen = AstToLlvmCodeGen::new();
        let ir = codegen.generate_program(program, self.get_target_triple())?;
        
        validate_llvm_ir(&ir)?;
        
        self.symbols.insert("main".to_string(), ir.clone());
        Ok(ir)
    }

    fn generate_type(&self, type_name: &str) -> String {
        match type_name {
            "i32" | "整数" | "int" => "i32".to_string(),
            "i64" | "长整数" | "long" => "i64".to_string(),
            "i8" | "短整数" | "short" => "i8".to_string(),
            "f32" | "浮点数" | "float" => "float".to_string(),
            "f64" | "双精度浮点数" | "double" => "double".to_string(),
            "void" | "空" => "void".to_string(),
            _ => "i32".to_string(),
        }
    }

    fn generate_function_header(&self, func: &HanshuOp) -> String {
        let return_type = self.generate_type(&func.return_type.to_string());
        let params = if func.params.is_empty() {
            "".to_string()
        } else {
            func.params.iter()
                .map(|(name, ty)| format!("{} {}", self.generate_type(&ty.to_string()), name))
                .collect::<Vec<_>>()
                .join(", ")
        };
        format!("{} @{} ({})", return_type, func.name, params)
    }

    fn generate_operation(&self, op: &Box<dyn HuanOp>) -> String {
        if let Some(jia) = op.as_any().downcast_ref::<JiaOp>() {
            let lhs = self.generate_operand(&jia.lhs);
            let rhs = self.generate_operand(&jia.rhs);
            format!("add i32 {}, {}", lhs, rhs)
        } else if let Some(jian) = op.as_any().downcast_ref::<JianOp>() {
            let lhs = self.generate_operand(&jian.lhs);
            let rhs = self.generate_operand(&jian.rhs);
            format!("sub i32 {}, {}", lhs, rhs)
        } else if let Some(cheng) = op.as_any().downcast_ref::<ChengOp>() {
            let lhs = self.generate_operand(&cheng.lhs);
            let rhs = self.generate_operand(&cheng.rhs);
            format!("mul i32 {}, {}", lhs, rhs)
        } else if let Some(chu) = op.as_any().downcast_ref::<ChuOp>() {
            let lhs = self.generate_operand(&chu.lhs);
            let rhs = self.generate_operand(&chu.rhs);
            format!("sdiv i32 {}, {}", lhs, rhs)
        } else if let Some(ret) = op.as_any().downcast_ref::<FanhuiOp>() {
            if let Some(value) = &ret.value {
                let val = self.generate_operand(value);
                format!("ret i32 {}", val)
            } else {
                "ret void".to_string()
            }
        } else {
            "".to_string()
        }
    }

    fn generate_operand(&self, op: &Box<dyn HuanOp>) -> String {
        if let Some(int_lit) = op.as_any().downcast_ref::<IntLitOp>() {
            int_lit.value.to_string()
        } else if let Some(ident) = op.as_any().downcast_ref::<IdentOp>() {
            format!("%{}", ident.name)
        } else if let Some(float_lit) = op.as_any().downcast_ref::<FloatLitOp>() {
            format!("{:e}", float_lit.value)
        } else {
            "0".to_string()
        }
    }

    fn generate_function_body(&self, func: &HanshuOp) -> String {
        let mut body = String::new();
        let mut local_vars = HashMap::new();
        let mut local_counter = 0;

        for op in &func.body {
            let asm = self.generate_operation(op);
            if !asm.is_empty() {
                if asm.starts_with("%") && !asm.contains("ret") {
                    let local_name = format!("%l{}", local_counter);
                    local_vars.insert(local_counter, asm.clone());
                    local_counter += 1;
                    body.push_str(&format!("  {} = {}\n", local_name, asm));
                } else {
                    body.push_str(&format!("  {}\n", asm));
                }
            }
        }

        body
    }

    fn generate_module(&self, module: &ModuleOp) -> String {
        let mut ir = String::new();
        ir.push_str("; ModuleID = 'huanlang_module'\n");
        ir.push_str(&format!("source_filename = \"{}\"\n", module.name));
        ir.push_str("target datalayout = \"e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128\"\n");
        ir.push_str(&format!("target triple = \"{}\"\n\n", self.get_target_triple()));

        for (_i, op) in module.ops.iter().enumerate() {
            if let Some(func) = op.as_any().downcast_ref::<HanshuOp>() {
                ir.push_str("define ");
                ir.push_str(&self.generate_function_header(func));
                ir.push_str(" {\n");
                ir.push_str(&self.generate_function_body(func));
                ir.push_str("}\n\n");
            }
        }

        ir
    }

    fn get_target_triple(&self) -> &str {
        match self.target.as_str() {
            s if s.contains("windows") && s.contains("x86_64") => "x86_64-pc-windows-msvc",
            s if s.contains("linux") && s.contains("x86_64") => "x86_64-unknown-linux-gnu",
            s if s.contains("darwin") && s.contains("x86_64") => "x86_64-apple-darwin",
            s if s.contains("darwin") && s.contains("aarch64") => "arm64-apple-darwin",
            _ => "x86_64-unknown-linux-gnu",
        }
    }

    fn generate_assembly(&self, module: &ModuleOp) -> String {
        let mut asm = String::new();
        asm.push_str("    .file   \"main.huan\"\n");
        asm.push_str("    .text\n");
        asm.push_str("    .globl  main\n");
        asm.push_str("    .type   main, @function\n");
        asm.push_str("main:\n");
        asm.push_str("    .cfi_startproc\n");

        for op in &module.ops {
            if let Some(func) = op.as_any().downcast_ref::<HanshuOp>() {
                asm.push_str(&format!("{}:\n", func.name));
                asm.push_str("    pushq   %rbp\n");
                asm.push_str("    .cfi_def_cfa_offset 16\n");
                asm.push_str("    .cfi_offset 6, -16\n");
                asm.push_str("    movq    %rsp, %rbp\n");
                asm.push_str("    .cfi_def_cfa_register 6\n");

                for op in &func.body {
                    if let Some(ret) = op.as_any().downcast_ref::<FanhuiOp>() {
                        if let Some(int_lit) = ret.value.as_ref().and_then(|v| v.as_any().downcast_ref::<IntLitOp>()) {
                            asm.push_str(&format!("    movl    ${}, %eax\n", int_lit.value));
                        } else {
                            asm.push_str("    movl    $0, %eax\n");
                        }
                    } else if let Some(jia) = op.as_any().downcast_ref::<JiaOp>() {
                        let lhs = self.get_immediate(&jia.lhs);
                        let rhs = self.get_immediate(&jia.rhs);
                        if lhs.is_some() && rhs.is_some() {
                            asm.push_str(&format!("    movl    ${}, %eax\n", lhs.unwrap()));
                            asm.push_str(&format!("    addl    ${}, %eax\n", rhs.unwrap()));
                        }
                    }
                }

                asm.push_str("    popq    %rbp\n");
                asm.push_str("    ret\n");
            }
        }

        asm.push_str("    .cfi_endproc\n");
        asm.push_str("    .size   main, .-main\n");
        asm
    }

    fn get_immediate(&self, op: &Box<dyn HuanOp>) -> Option<i64> {
        if let Some(int_lit) = op.as_any().downcast_ref::<IntLitOp>() {
            Some(int_lit.value)
        } else {
            None
        }
    }
}

impl CodeGenerator for LLVMBackend {
    fn new(target: TargetTriple, options: CodeGenOptions) -> Self {
        Self {
            target,
            options,
            symbols: HashMap::new(),
            string_constants: Vec::new(),
        }
    }

    fn generate(&mut self, mlir_module: &ModuleOp) -> Result<Vec<u8>, CodeGenError> {
        let ir = self.generate_module(mlir_module);
        self.symbols.insert("main".to_string(), ir.clone());
        Ok(ir.into_bytes())
    }

    fn emit_assembly(&mut self, mlir_module: &ModuleOp) -> Result<String, CodeGenError> {
        Ok(self.generate_assembly(mlir_module))
    }

    fn emit_llvm_ir(&mut self, mlir_module: &ModuleOp) -> Result<String, CodeGenError> {
        Ok(self.generate_module(mlir_module))
    }

    fn link(&self, objects: Vec<PathBuf>, output: PathBuf) -> Result<(), LinkError> {
        if objects.is_empty() {
            return Ok(());
        }

        let mut cmd = std::process::Command::new("ld");
        cmd.arg("-o").arg(&output);

        for obj in &objects {
            cmd.arg(obj);
        }

        cmd.arg("-lc");
        cmd.arg("-lm");

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    Ok(())
                } else {
                    Err(LinkError::LinkFailed(
                        String::from_utf8_lossy(&output.stderr).to_string(),
                    ))
                }
            }
            Err(e) => Err(LinkError::LinkFailed(e.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LLVMOptions {
    pub loop_vectorize: bool,
    pub slp_vectorize: bool,
    pub inline: bool,
    pub gvn: bool,
    pub dce: bool,
    pub optimization_level: u32,
}

impl Default for LLVMOptions {
    fn default() -> Self {
        Self {
            loop_vectorize: true,
            slp_vectorize: true,
            inline: true,
            gvn: true,
            dce: true,
            optimization_level: 2,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::mlir::ops::{HanshuOp, IntLitOp, JiaOp, FanhuiOp};

    #[test]
    fn test_llvm_backend_basic() {
        let target = TargetTriple::x86_64_linux();
        let options = CodeGenOptions::default();
        let backend = LLVMBackend::new(target, options);

        assert_eq!(backend.get_target_triple(), "x86_64-unknown-linux-gnu");
    }

    #[test]
    fn test_llvm_backend_type_generation() {
        let target = TargetTriple::x86_64_linux();
        let options = CodeGenOptions::default();
        let backend = LLVMBackend::new(target, options);

        assert_eq!(backend.generate_type("i32"), "i32");
        assert_eq!(backend.generate_type("整数"), "i32");
        assert_eq!(backend.generate_type("float"), "float");
        assert_eq!(backend.generate_type("f64"), "double");
    }

    #[test]
    fn test_get_immediate() {
        let target = TargetTriple::x86_64_linux();
        let options = CodeGenOptions::default();
        let backend = LLVMBackend::new(target, options);

        let int_lit: Box<dyn HuanOp> = Box::new(IntLitOp { value: 42, span: Default::default() });
        assert_eq!(backend.get_immediate(&int_lit), Some(42));
    }
}

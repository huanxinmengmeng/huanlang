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

pub mod traits;
pub mod llvm;
pub mod wasm;
pub mod gpu;
pub mod target;
pub mod error;
pub mod linker;

pub use traits::{CodeGenerator, CodeGenOptions, OptLevel, CallingConvention};
pub use target::{TargetTriple, Arch};
pub use error::{CodeGenError, LinkError};
pub use llvm::LLVMBackend;
pub use wasm::WasmBackend;
pub use gpu::{GpuBackend, GpuTarget};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::mlir::*;

    #[test]
    fn test_target_triple_host() {
        let triple = TargetTriple::host();
        println!("Host triple: {}", triple.as_str());
        assert!(!triple.as_str().is_empty());
    }

    #[test]
    fn test_target_triple_x86_64() {
        let triple = TargetTriple::x86_64_linux();
        assert_eq!(triple.as_str(), "x86_64-unknown-linux-gnu");
        assert_eq!(triple.arch(), Arch::X86_64);
    }

    #[test]
    fn test_target_triple_wasm() {
        let triple = TargetTriple::wasm32_unknown_unknown();
        assert_eq!(triple.arch(), Arch::Wasm32);
    }

    #[test]
    fn test_arch_pointer_width() {
        assert_eq!(Arch::X86.pointer_width(), 4);
        assert_eq!(Arch::X86_64.pointer_width(), 8);
        assert_eq!(Arch::Wasm32.pointer_width(), 4);
        assert_eq!(Arch::AArch64.pointer_width(), 8);
    }

    #[test]
    fn test_llvm_backend_new() {
        let triple = TargetTriple::host();
        let options = CodeGenOptions::default();
        let _backend = LLVMBackend::new(triple, options);
        // 成功创建
    }

    #[test]
    fn test_wasm_backend_new() {
        let triple = TargetTriple::wasm32_unknown_unknown();
        let options = CodeGenOptions::default();
        let _backend = WasmBackend::new(triple, options);
        // 成功创建
    }

    #[test]
    fn test_gpu_backend_new() {
        let triple = TargetTriple::host();
        let options = CodeGenOptions::default();
        let _backend = GpuBackend::new(triple, options);
        // 成功创建
    }

    #[test]
    fn test_codegen_options_default() {
        let opts = CodeGenOptions::default();
        assert_eq!(opts.opt_level, OptLevel::Default);
        assert_eq!(opts.debug_info, false);
    }

    #[test]
    fn test_opt_level_as_u8() {
        assert_eq!(OptLevel::None.as_u8(), 0);
        assert_eq!(OptLevel::Default.as_u8(), 2);
        assert_eq!(OptLevel::Aggressive.as_u8(), 3);
    }

    #[test]
    fn test_llvm_emit_assembly() {
        let triple = TargetTriple::x86_64_linux();
        let options = CodeGenOptions::default();
        let mut backend = LLVMBackend::new(triple, options);
        
        let mlir_module = ModuleOp::dummy();
        let asm = backend.emit_assembly(&mlir_module).unwrap();
        assert!(asm.contains("main"));
    }

    #[test]
    fn test_llvm_emit_llvm_ir() {
        let triple = TargetTriple::x86_64_windows();
        let options = CodeGenOptions::default();
        let mut backend = LLVMBackend::new(triple, options);
        
        let mlir_module = ModuleOp::dummy();
        let ir = backend.emit_llvm_ir(&mlir_module).unwrap();
        assert!(ir.contains("define"));
    }

    #[test]
    fn test_wasm_generate() {
        let triple = TargetTriple::wasm32_unknown_unknown();
        let options = CodeGenOptions::default();
        let mut backend = WasmBackend::new(triple, options);
        
        let mlir_module = ModuleOp::dummy();
        let wasm = backend.generate(&mlir_module).unwrap();
        
        // 检查Wasm魔数
        assert_eq!(&wasm[0..4], &[0x00, 0x61, 0x73, 0x6d]);
    }

    #[test]
    fn test_wasm_options_default() {
        let opts = wasm::WasmOptions::default();
        assert_eq!(opts.export_memory, true);
        assert_eq!(opts.initial_memory, 10);
        assert_eq!(opts.wasi, true);
    }

    #[test]
    fn test_arch_display() {
        assert_eq!(Arch::X86_64.as_str(), "x86_64");
        assert_eq!(Arch::AArch64.as_str(), "aarch64");
        assert_eq!(Arch::RiscV64.as_str(), "riscv64");
    }
}

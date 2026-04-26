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
// 架构适配层
// 实现规范第9.5节中的多架构指令集支持



/// 目标架构枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Arch {
    /// x86 32位
    X86,
    /// x86_64 64位
    X86_64,
    /// ARM 32位
    ARM,
    /// AArch64
    AArch64,
    /// RISC-V 32位
    RISCV,
    /// RISC-V 64位
    RISCV64,
    /// AVR
    AVR,
    /// MSP430
    MSP430,
    /// Xtensa (ESP芯片)
    Xtensa,
}

impl Arch {
    /// 获取架构的默认字长（字节）
    pub fn word_size(&self) -> usize {
        match self {
            Arch::X86 | Arch::ARM | Arch::RISCV => 4,
            Arch::X86_64 | Arch::AArch64 | Arch::RISCV64 => 8,
            Arch::AVR => 2,
            Arch::MSP430 => 2,
            Arch::Xtensa => 4,
        }
    }

    /// 获取架构名称字符串
    pub fn to_name(&self) -> &'static str {
        match self {
            Arch::X86 => "x86",
            Arch::X86_64 => "x86_64",
            Arch::ARM => "arm",
            Arch::AArch64 => "aarch64",
            Arch::RISCV => "riscv",
            Arch::RISCV64 => "riscv64",
            Arch::AVR => "avr",
            Arch::MSP430 => "msp430",
            Arch::Xtensa => "xtensa",
        }
    }

    /// 从字符串解析架构
    pub fn from_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "x86" | "i386" | "i686" => Some(Arch::X86),
            "x86_64" | "amd64" | "x86-64" => Some(Arch::X86_64),
            "arm" | "armv7" => Some(Arch::ARM),
            "aarch64" | "arm64" => Some(Arch::AArch64),
            "riscv" | "riscv32" => Some(Arch::RISCV),
            "riscv64" => Some(Arch::RISCV64),
            "avr" => Some(Arch::AVR),
            "msp430" => Some(Arch::MSP430),
            "xtensa" => Some(Arch::Xtensa),
            _ => None,
        }
    }
}

/// 获取当前编译目标的架构
pub fn get_arch() -> Arch {
    #[cfg(target_arch = "x86_64")]
    return Arch::X86_64;
    
    #[cfg(target_arch = "x86")]
    return Arch::X86;
    
    #[cfg(target_arch = "aarch64")]
    return Arch::AArch64;
    
    #[cfg(target_arch = "arm")]
    return Arch::ARM;
    
    #[cfg(target_arch = "riscv64")]
    return Arch::RISCV64;
    
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    return Arch::RISCV;
    
    // 默认回退到主机架构
    #[cfg(not(any(target_arch = "x86_64", target_arch = "x86", target_arch = "aarch64", 
                  target_arch = "arm", target_arch = "riscv64")))]
    return Arch::X86_64;
}

/// 获取通用寄存器名（多架构支持）
pub fn get_register_name(arch: Arch, reg_idx: usize) -> Option<&'static str> {
    match arch {
        Arch::X86 => get_x86_register(reg_idx),
        Arch::X86_64 => _get_x86_64_register(reg_idx),
        Arch::ARM | Arch::AArch64 => get_arm_register(reg_idx),
        Arch::RISCV | Arch::RISCV64 => get_riscv_register(reg_idx),
        _ => None,
    }
}

fn get_x86_register(reg_idx: usize) -> Option<&'static str> {
    let regs = ["eax", "ebx", "ecx", "edx", "esi", "edi", "ebp", "esp"];
    regs.get(reg_idx).copied()
}

fn _get_x86_64_register(reg_idx: usize) -> Option<&'static str> {
    let regs = ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rbp", "rsp",
                "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"];
    regs.get(reg_idx).copied()
}

fn get_arm_register(reg_idx: usize) -> Option<&'static str> {
    let regs = ["r0", "r1", "r2", "r3", "r4", "r5", "r6", "r7",
                "r8", "r9", "r10", "r11", "r12", "sp", "lr", "pc"];
    regs.get(reg_idx).copied()
}

fn get_riscv_register(reg_idx: usize) -> Option<&'static str> {
    let regs = ["zero", "ra", "sp", "gp", "tp", "t0", "t1", "t2",
                "s0", "s1", "a0", "a1", "a2", "a3", "a4", "a5",
                "a6", "a7", "s2", "s3", "s4", "s5", "s6", "s7",
                "s8", "s9", "s10", "s11", "t3", "t4", "t5", "t6"];
    regs.get(reg_idx).copied()
}

/// 预定义宏
#[derive(Debug, Clone)]
pub struct CompilerMacro {
    pub name: &'static str,
    pub value: &'static str,
}

/// 获取架构相关的预定义宏
pub fn get_arch_macros(arch: Arch) -> Vec<CompilerMacro> {
    let mut macros = Vec::new();
    
    match arch {
        Arch::X86 => {
            macros.push(CompilerMacro { name: "ARCH_X86", value: "1" });
        },
        Arch::X86_64 => {
            macros.push(CompilerMacro { name: "ARCH_X86_64", value: "1" });
        },
        Arch::ARM => {
            macros.push(CompilerMacro { name: "ARCH_ARM", value: "1" });
        },
        Arch::AArch64 => {
            macros.push(CompilerMacro { name: "ARCH_AARCH64", value: "1" });
        },
        Arch::RISCV => {
            macros.push(CompilerMacro { name: "ARCH_RISCV32", value: "1" });
        },
        Arch::RISCV64 => {
            macros.push(CompilerMacro { name: "ARCH_RISCV64", value: "1" });
        },
        Arch::AVR => {
            macros.push(CompilerMacro { name: "ARCH_AVR", value: "1" });
        },
        _ => {}
    }
    
    macros
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arch_from_name() {
        assert_eq!(Arch::from_name("x86_64"), Some(Arch::X86_64));
        assert_eq!(Arch::from_name("amd64"), Some(Arch::X86_64));
        assert_eq!(Arch::from_name("arm"), Some(Arch::ARM));
        assert_eq!(Arch::from_name("riscv64"), Some(Arch::RISCV64));
        assert_eq!(Arch::from_name("unknown"), None);
    }

    #[test]
    fn test_arch_word_size() {
        assert_eq!(Arch::X86.word_size(), 4);
        assert_eq!(Arch::X86_64.word_size(), 8);
        assert_eq!(Arch::ARM.word_size(), 4);
        assert_eq!(Arch::AArch64.word_size(), 8);
    }

    #[test]
    fn test_get_register_name() {
        assert_eq!(get_register_name(Arch::X86_64, 0), Some("rax"));
        assert_eq!(get_register_name(Arch::X86_64, 5), Some("rdi"));
        assert_eq!(get_register_name(Arch::X86_64, 100), None);
        
        assert_eq!(get_register_name(Arch::ARM, 0), Some("r0"));
        assert_eq!(get_register_name(Arch::ARM, 14), Some("lr"));
    }

    #[test]
    fn test_arch_macros() {
        let x86_64_macros = get_arch_macros(Arch::X86_64);
        assert!(!x86_64_macros.is_empty());
        
        let arm_macros = get_arch_macros(Arch::ARM);
        assert!(!arm_macros.is_empty());
    }
}

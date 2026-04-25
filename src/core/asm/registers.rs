// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
//
// 寄存器操作与约束
// 实现规范第9.6节中的寄存器操作与约束系统

/// 寄存器分类
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterClass {
    /// 通用寄存器
    GeneralPurpose,
    /// 浮点寄存器
    Float,
    /// 向量寄存器
    Vector,
    /// 系统寄存器
    System,
}

/// 寄存器访问模式
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegisterAccess {
    /// 只读
    Read,
    /// 只写
    Write,
    /// 读写
    ReadWrite,
}

/// 统一寄存器名枚举
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RegName {
    /// 通用寄存器 0
    Gpr0,
    /// 通用寄存器 1
    Gpr1,
    /// 通用寄存器 2
    Gpr2,
    /// 通用寄存器 3
    Gpr3,
    /// 通用寄存器 4
    Gpr4,
    /// 通用寄存器 5
    Gpr5,
    /// 通用寄存器 6
    Gpr6,
    /// 通用寄存器 7
    Gpr7,
    /// 栈指针
    StackPtr,
    /// 帧指针
    FramePtr,
    /// 链接寄存器（返回地址）
    LinkRegister,
    /// 程序计数器
    ProgramCounter,
}

/// 寄存器描述
#[derive(Debug, Clone)]
pub struct Register {
    /// 架构独立名称
    pub name: RegName,
    /// 寄存器分类
    pub class: RegisterClass,
    /// 默认访问模式
    pub access: RegisterAccess,
    /// 是否在函数调用间需要保存
    pub callee_saved: bool,
}

impl Register {
    /// 创建一个通用寄存器
    pub fn gpr(name: RegName, callee_saved: bool) -> Self {
        Register {
            name,
            class: RegisterClass::GeneralPurpose,
            access: RegisterAccess::ReadWrite,
            callee_saved,
        }
    }
    
    /// 创建一个系统寄存器
    pub fn system(name: RegName, access: RegisterAccess) -> Self {
        Register {
            name,
            class: RegisterClass::System,
            access,
            callee_saved: false,
        }
    }

    /// 检查寄存器是否需要保存
    pub fn should_save(&self) -> bool {
        self.callee_saved
    }
}

/// 系统寄存器读写辅助
pub mod sysregs {

    use std::arch::asm;

    /// 读取系统寄存器
    /// # Safety
    /// 这是一个危险操作，需要对硬件有深入了解
    pub unsafe fn read(reg_name: &str) -> u64 {
        // 实际实现会根据架构生成相应的汇编代码
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        match reg_name {
            "cr0" => {
                // 使用内联汇编读取控制寄存器
                let value: u64;
                unsafe {
                    asm!("mov {0}, cr0", out(reg) value, options(nomem, nostack));
                };
                value
            }
            "cr2" => {
                // 使用内联汇编读取控制寄存器
                let value: u64;
                unsafe {
                    asm!("mov {0}, cr2", out(reg) value, options(nomem, nostack));
                };
                value
            }
            "cr3" => {
                // 使用内联汇编读取控制寄存器
                let value: u64;
                unsafe {
                    asm!("mov {0}, cr3", out(reg) value, options(nomem, nostack));
                };
                value
            }
            "cr4" => {
                // 使用内联汇编读取控制寄存器
                let value: u64;
                unsafe {
                    asm!("mov {0}, cr4", out(reg) value, options(nomem, nostack));
                };
                value
            }
            "cr8" => {
                // 使用内联汇编读取控制寄存器
                let value: u64;
                unsafe {
                    asm!("mov {0}, cr8", out(reg) value, options(nomem, nostack));
                };
                value
            }
            _ => unimplemented!("Register {} not supported for reading on this architecture", reg_name),
        }

        #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
        match reg_name {
            "sp" => {
                // ARM栈指针读取
                let value: u64;
                unsafe {
                    asm!("mov {0}, sp", out(reg) value, options(volatile));
                };
                value
            }
            _ => unimplemented!("Register {} not supported", reg_name),
        }

        #[cfg(not(any(target_arch = "x86", target_arch = "x86_64",
                      target_arch = "aarch64", target_arch = "arm")))]
        unimplemented!("Register access not supported on this architecture")
    }

    /// 写入系统寄存器
    /// # Safety
    /// 这是一个危险操作，可能导致系统崩溃
    pub unsafe fn write(reg_name: &str, value: u64) {
        // 实际实现会根据架构生成相应的汇编代码
        #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
        match reg_name {
            "cr0" => {
                // 使用内联汇编写入控制寄存器
                unsafe {
                    asm!("mov cr0, {0}", in(reg) value, options(nomem, nostack));
                };
            }
            "cr2" => {
                // 使用内联汇编写入控制寄存器
                unsafe {
                    asm!("mov cr2, {0}", in(reg) value, options(nomem, nostack));
                };
            }
            "cr3" => {
                // 使用内联汇编写入控制寄存器
                unsafe {
                    asm!("mov cr3, {0}", in(reg) value, options(nomem, nostack));
                };
            }
            "cr4" => {
                // 使用内联汇编写入控制寄存器
                unsafe {
                    asm!("mov cr4, {0}", in(reg) value, options(nomem, nostack));
                };
            }
            "cr8" => {
                // 使用内联汇编写入控制寄存器
                unsafe {
                    asm!("mov cr8, {0}", in(reg) value, options(nomem, nostack));
                };
            }
            _ => unimplemented!("Register {} not supported for writing", reg_name),
        }

        #[cfg(any(target_arch = "aarch64", target_arch = "arm"))]
        unimplemented!("Register write not supported on this architecture")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_create() {
        let gpr0 = Register::gpr(RegName::Gpr0, false);
        assert_eq!(gpr0.class, RegisterClass::GeneralPurpose);
        assert_eq!(gpr0.access, RegisterAccess::ReadWrite);
        assert!(!gpr0.should_save());

        let sp = Register::system(RegName::StackPtr, RegisterAccess::ReadWrite);
        assert_eq!(sp.class, RegisterClass::System);
    }
}

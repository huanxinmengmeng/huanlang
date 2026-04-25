// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
//
// 外设寄存器定义
// 实现规范第9.8节中的外设寄存器访问功能

use std::collections::HashMap;

use crate::core::ast::Type;


/// 外设访问权限
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeripheralAccess {
    /// 只读
    ReadOnly,
    /// 只写
    WriteOnly,
    /// 读写
    ReadWrite,
}

/// 寄存器描述
#[derive(Debug, Clone)]
pub struct PeripheralRegister {
    /// 寄存器名称
    pub name: String,
    /// 相对于外设基址的偏移
    pub offset: u64,
    /// 寄存器类型（宽度）
    pub ty: Type,
    /// 访问权限
    pub access: PeripheralAccess,
    /// 位域定义（可选）
    pub bit_fields: HashMap<String, (u32, u32)>, // (start, end)
}

/// 外设定义
#[derive(Debug, Clone)]
pub struct Peripheral {
    /// 外设名称
    pub name: String,
    /// 外设基地址
    pub base_addr: u64,
    /// 寄存器列表
    pub registers: HashMap<String, PeripheralRegister>,
}

impl Peripheral {
    /// 创建新的外设
    pub fn new(name: String, base_addr: u64) -> Self {
        Peripheral {
            name,
            base_addr,
            registers: HashMap::new(),
        }
    }

    /// 添加寄存器
    pub fn add_register(&mut self, name: String, offset: u64, ty: Type, access: PeripheralAccess) {
        let reg = PeripheralRegister {
            name: name.clone(),
            offset,
            ty,
            access,
            bit_fields: HashMap::new(),
        };
        self.registers.insert(name, reg);
    }

    /// 添加位域
    pub fn add_bitfield(&mut self, reg_name: &str, field_name: String, start: u32, end: u32) -> Result<(), String> {
        if let Some(reg) = self.registers.get_mut(reg_name) {
            reg.bit_fields.insert(field_name, (start, end));
            Ok(())
        } else {
            Err(format!("寄存器 '{}' 未在 '{}' 中找到", reg_name, self.name))
        }
    }

    /// 获取寄存器地址
    pub fn get_register_addr(&self, reg_name: &str) -> Option<u64> {
        self.registers.get(reg_name).map(|r| self.base_addr + r.offset)
    }
}

/// 寄存器访问助手（编译器会将其转换为volatile指针操作）
pub mod register_access {


    /// 读取寄存器
    /// # Safety
    /// 这是危险的硬件操作
    pub unsafe fn read<T: Copy>(addr: *const T) -> T {
        // 这会被编译为volatile读取
        std::ptr::read_volatile(addr)
    }

    /// 写入寄存器
    /// # Safety
    /// 这是危险的硬件操作
    pub unsafe fn write<T: Copy>(addr: *mut T, value: T) {
        // 这会被编译为volatile写入
        std::ptr::write_volatile(addr, value);
    }

    /// 修改寄存器位
    /// # Safety
    /// 这是危险的硬件操作
    pub unsafe fn modify<T: Copy + std::ops::BitAnd<Output=T> + std::ops::BitOr<Output=T> + 
                        std::ops::Not<Output=T> + From<u32>>(
        addr: *mut T,
        mask: T,
        value: T
    ) {
        let current = read(addr);
        let new_val = (current & !mask) | (value & mask);
        write(addr, new_val);
    }

    /// 设置单个位
    /// # Safety
    /// 这是危险的硬件操作
    pub unsafe fn set_bit(addr: *mut u32, bit: u32) {
        modify(addr, 1 << bit, 1 << bit);
    }

    /// 清除单个位
    /// # Safety
    /// 这是危险的硬件操作
    pub unsafe fn clear_bit(addr: *mut u32, bit: u32) {
        modify(addr, 1 << bit, 0);
    }
}

/// 预定义的通用外设
pub mod predefined {

    use super::Peripheral;
    use super::PeripheralAccess;
    use crate::core::ast::Type;

    /// 创建一个通用GPIO外设定义
    pub fn create_gpio(name: String, base_addr: u64) -> Peripheral {
        let mut gpio = Peripheral::new(name, base_addr);
        
        gpio.add_register("CRL".into(), 0x00, Type::U32, PeripheralAccess::ReadWrite);
        gpio.add_register("CRH".into(), 0x04, Type::U32, PeripheralAccess::ReadWrite);
        gpio.add_register("IDR".into(), 0x08, Type::U32, PeripheralAccess::ReadOnly);
        gpio.add_register("ODR".into(), 0x0C, Type::U32, PeripheralAccess::ReadWrite);
        gpio.add_register("BSRR".into(), 0x10, Type::U32, PeripheralAccess::WriteOnly);
        gpio.add_register("BRR".into(), 0x14, Type::U32, PeripheralAccess::WriteOnly);
        gpio.add_register("LCKR".into(), 0x18, Type::U32, PeripheralAccess::ReadWrite);
        
        // 添加常用位域
        for i in 0..8 {
            let field_name = format!("MODE{}", i);
            gpio.add_bitfield("CRL", field_name, i * 4, (i * 4) + 3).ok();
        }
        for i in 8..16 {
            let field_name = format!("MODE{}", i);
            gpio.add_bitfield("CRH", field_name, (i - 8) * 4, ((i - 8) * 4) + 3).ok();
        }
        
        gpio
    }

    /// 创建一个通用RCC（时钟控制）外设定义
    pub fn create_rcc(name: String, base_addr: u64) -> Peripheral {
        let mut rcc = Peripheral::new(name, base_addr);
        
        rcc.add_register("CR".into(), 0x00, Type::U32, PeripheralAccess::ReadWrite);
        rcc.add_register("CFGR".into(), 0x04, Type::U32, PeripheralAccess::ReadWrite);
        rcc.add_register("CIR".into(), 0x08, Type::U32, PeripheralAccess::ReadWrite);
        rcc.add_register("APB2RSTR".into(), 0x0C, Type::U32, PeripheralAccess::ReadWrite);
        rcc.add_register("APB1RSTR".into(), 0x10, Type::U32, PeripheralAccess::ReadWrite);
        rcc.add_register("AHBENR".into(), 0x14, Type::U32, PeripheralAccess::ReadWrite);
        rcc.add_register("APB2ENR".into(), 0x18, Type::U32, PeripheralAccess::ReadWrite);
        rcc.add_register("APB1ENR".into(), 0x1C, Type::U32, PeripheralAccess::ReadWrite);
        
        rcc
    }
}

#[cfg(test)]
mod tests {

    use super::Peripheral;
    use super::PeripheralAccess;
    use super::predefined;
    use crate::core::ast::Type;

    #[test]
    fn test_peripheral_new() {
        let gpio = Peripheral::new("GPIOA".into(), 0x40010800);
        assert_eq!(gpio.name, "GPIOA");
        assert_eq!(gpio.base_addr, 0x40010800);
        assert!(gpio.registers.is_empty());
    }

    #[test]
    fn test_add_register() {
        let mut gpio = Peripheral::new("GPIOA".into(), 0x40010800);
        gpio.add_register("CRL".into(), 0x00, Type::U32, PeripheralAccess::ReadWrite);
        assert!(gpio.registers.contains_key("CRL"));
        assert_eq!(gpio.get_register_addr("CRL"), Some(0x40010800));
    }

    #[test]
    fn test_create_predefined_gpio() {
        let gpio = predefined::create_gpio("GPIOA".into(), 0x40010800);
        assert!(gpio.registers.contains_key("CRL"));
        assert!(gpio.registers.contains_key("ODR"));
        assert!(gpio.registers.contains_key("BSRR"));
    }
}

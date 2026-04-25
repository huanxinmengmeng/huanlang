// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
//
// 链接器脚本支持
// 实现规范第9.9节中的链接器脚本功能



/// 内存区域属性
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemoryAttrs {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

impl MemoryAttrs {
    pub fn new(readable: bool, writable: bool, executable: bool) -> Self {
        MemoryAttrs {
            readable,
            writable,
            executable,
        }
    }

    pub fn to_ld_attrs(&self) -> String {
        let mut s = String::new();
        if self.readable { s.push('r'); }
        if self.writable { s.push('w'); }
        if self.executable { s.push('x'); }
        s
    }
}

impl Default for MemoryAttrs {
    fn default() -> Self {
        MemoryAttrs::new(true, true, false)
    }
}

/// 内存区域定义
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    /// 区域名称
    pub name: String,
    /// 起始地址
    pub start: u64,
    /// 长度
    pub length: u64,
    /// 属性
    pub attrs: MemoryAttrs,
}

impl MemoryRegion {
    pub fn new(name: String, start: u64, length: u64, attrs: MemoryAttrs) -> Self {
        MemoryRegion { name, start, length, attrs }
    }

    pub fn end(&self) -> u64 {
        self.start + self.length
    }
}

/// 段属性
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LayoutType {
    /// 普通段
    Regular,
    /// 零初始化段
    Bss,
    /// 加载到ROM，初始化数据
    Load,
}

/// 段定义
#[derive(Debug, Clone)]
pub struct Section {
    /// 段名
    pub name: String,
    /// 放置位置
    pub region: String,
    /// 对齐
    pub align: u32,
    /// 加载位置
    pub layout: LayoutType,
    /// 包含内容
    pub includes: Vec<String>,
}

impl Section {
    pub fn new(name: String, region: String) -> Self {
        Section {
            name,
            region,
            align: 1,
            layout: LayoutType::Regular,
            includes: Vec::new(),
        }
    }
}

/// 幻语链接器脚本
#[derive(Debug, Clone)]
pub struct LinkerScript {
    /// 内存区域
    pub memory_regions: Vec<MemoryRegion>,
    /// 段定义
    pub sections: Vec<Section>,
}

impl LinkerScript {
    pub fn new() -> Self {
        LinkerScript {
            memory_regions: Vec::new(),
            sections: Vec::new(),
        }
    }

    /// 添加内存区域
    pub fn add_region(&mut self, region: MemoryRegion) {
        self.memory_regions.push(region);
    }

    /// 添加段
    pub fn add_section(&mut self, section: Section) {
        self.sections.push(section);
    }

    /// 导出为GNU LD格式
    pub fn to_ld_string(&self) -> String {
        let mut out = String::new();
        
        // 内存区域部分
        out.push_str("/* 幻语链接器脚本 - 自动生成\n");
        out.push_str("MEMORY\n");
        out.push_str("{\n");
        
        for region in &self.memory_regions {
            out.push_str(&format!(
                "  {} ({}): ORIGIN = {:#x}, LENGTH = {:#x}\n",
                region.name,
                region.attrs.to_ld_attrs(),
                region.start,
                region.length
            ));
        }
        
        out.push_str("}\n\n");
        
        // 段部分
        out.push_str("SECTIONS\n");
        out.push_str("{\n");
        
        // 导出地址别名
        out.push_str("  /* 向量表段 - 来自幻语程序启动\n");
        
        for section in &self.sections {
            out.push_str(&format!("  {}\n", section.name));
            out.push_str("  {\n");
            section.includes.iter().for_each(|incl| {
                out.push_str(&format!("    {}\n", incl));
            });
            out.push_str("  }\n");
        }
        
        // 默认符号
        out.push_str("\n  /* 符号定义\n");
        
        // 栈顶符号
        if let Some(ram_region) = self.memory_regions.iter().find(|r| r.attrs.writable) {
            out.push_str(&format!("  _stack_top = ORIGIN({}) + LENGTH({});\n",
                                ram_region.name, ram_region.name));
        }
        
        out.push_str("}\n");
        
        out
    }
}

/// 预定义链接脚本模板
pub mod templates {
    use super::*;

    /// 生成ARM Cortex-M3/4的链接脚本
    pub fn create_stm32(flash_kb: u64, ram_kb: u64) -> LinkerScript {
        let mut script = LinkerScript::new();

        // 添加Flash区域
        let flash_attrs = MemoryAttrs::new(true, false, true);
        script.add_region(MemoryRegion::new("FLASH".into(), 0x08000000, flash_kb * 1024, flash_attrs));

        // 添加RAM区域
        let ram_attrs = MemoryAttrs::new(true, true, true);
        script.add_region(MemoryRegion::new("RAM".into(), 0x20000000, ram_kb * 1024, ram_attrs));

        // 添加向量表段
        let mut vector_table = Section::new(".vector_table".into(), "FLASH".into());
        vector_table.align = 256;
        vector_table.includes.push("KEEP(*(.vector_table))".into());
        script.add_section(vector_table);

        // 添加代码段
        let mut text = Section::new(".text".into(), "FLASH".into());
        text.includes.push("*(.text .text.*)".into());
        text.includes.push("*(.rodata .rodata.*)".into());
        script.add_section(text);

        // 添加数据段
        let mut data = Section::new(".data".into(), "RAM".into());
        data.layout = LayoutType::Load;
        data.includes.push("*(.data .data.*)".into());
        script.add_section(data);

        // 添加BSS段
        let mut bss = Section::new(".bss".into(), "RAM".into());
        bss.layout = LayoutType::Bss;
        bss.includes.push("*(.bss .bss.*)".into());
        script.add_section(bss);

        script
    }

    /// 通用RISC-V 32位
    pub fn create_riscv32(rom_kb: u64, ram_kb: u64) -> LinkerScript {
        let mut script = LinkerScript::new();

        let flash_attrs = MemoryAttrs::new(true, false, true);
        script.add_region(MemoryRegion::new("ROM".into(), 0x00000000, rom_kb * 1024, flash_attrs));

        let ram_attrs = MemoryAttrs::new(true, true, true);
        script.add_region(MemoryRegion::new("RAM".into(), 0x80000000, ram_kb * 1024, ram_attrs));

        let mut text = Section::new(".text".into(), "ROM".into());
        text.includes.push("*(.text .text.*)".into());
        text.includes.push("*(.rodata .rodata.*)".into());
        script.add_section(text);

        script
    }

    /// x86_64的通用脚本
    pub fn create_x86_64() -> LinkerScript {
        let mut script = LinkerScript::new();
        
        let text_attrs = MemoryAttrs::new(true, false, true);
        script.add_region(MemoryRegion::new("TEXT".into(), 0x00100000, 0x00f00000, text_attrs));

        let data_attrs = MemoryAttrs::new(true, true, false);
        script.add_region(MemoryRegion::new("DATA".into(), 0x01000000, 0x00f00000, data_attrs));

        script
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linker_script_new() {
        let script = LinkerScript::new();
        assert!(script.memory_regions.is_empty());
        assert!(script.sections.is_empty());
    }

    #[test]
    fn test_add_region() {
        let mut script = LinkerScript::new();
        let flash_attrs = MemoryAttrs::new(true, false, true);
        script.add_region(MemoryRegion::new("FLASH".into(), 0x08000000, 0x10000, flash_attrs));
        
        assert_eq!(script.memory_regions.len(), 1);
        assert_eq!(script.memory_regions[0].name, "FLASH");
        assert_eq!(script.memory_regions[0].start, 0x08000000);
    }

    #[test]
    fn test_stm32_template() {
        let script = templates::create_stm32(64, 20);
        assert_eq!(script.memory_regions.len(), 2);
        assert_eq!(script.memory_regions[0].name, "FLASH");
        assert_eq!(script.memory_regions[0].length, 64 * 1024);
    }

    #[test]
    fn test_memory_attrs() {
        let read_only = MemoryAttrs::new(true, false, false);
        assert_eq!(read_only.to_ld_attrs(), "r");

        let all_attrs = MemoryAttrs::new(true, true, true);
        assert_eq!(all_attrs.to_ld_attrs(), "rwx");
    }
}

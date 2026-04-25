// Copyright © 2026 幻心梦梦（huanxinmengmeng）
// 本项目依据项目根目录的 LICENSE 文件中的幻语许可证进行许可。
//
// 幻语 MLIR 方言定义
// 实现规范第10.2.1节的完整方言定义

use std::fmt;

/// 幻语方言
/// 
/// 对应规范中的 HuanDialect
pub struct HuanDialect;

impl fmt::Debug for HuanDialect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("HuanDialect")
    }
}

// 方言特性
impl HuanDialect {
    /// 方言名称
    pub const NAME: &'static str = "huan";
    
    /// 方言命名空间
    pub const NAMESPACE: &'static str = "mlir::huan";
    
    /// 方言摘要
    pub const SUMMARY: &'static str = "幻语（HuanLang）高级编程语言方言";
    
    /// 方言描述
    pub const DESCRIPTION: &'static str = 
        "幻语方言表示了幻语言的高级语义，包括变量声明、控制流、函数定义、复合类型操作等。
         它是从AST直接生成的初始IR。";
}

/// 方言注册
pub struct HuanDialectRegistry;

impl HuanDialectRegistry {
    /// 初始化方言（概念上的，因为我们是用Rust模拟MLIR）
    pub fn initialize() {
        // 在真实MLIR实现中会注册操作和类型
        // 这里我们用类型系统模拟
    }
    
    /// 获取支持的操作列表
    pub fn supported_ops() -> Vec<&'static str> {
        vec![
            "ling", "ding", "shewei",
            "ruo", "pipei", "chongfu", "dang", "duiyu",
            "hanshu", "fanhui", "diaoyong",
            "jia", "jian", "cheng", "chu", "quyu",
            "dayu", "xiaoyu", "dengyu",
            "qie", "huo", "fei",
            "liebiao", "zidian", "zhuizhui", "suoyin", "ziduan",
            "asm",
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dialect_name() {
        assert_eq!(HuanDialect::NAME, "huan");
    }

    #[test]
    fn test_supported_ops() {
        let ops = HuanDialectRegistry::supported_ops();
        assert!(ops.contains(&"ling"));
        assert!(ops.contains(&"jia"));
        assert!(ops.contains(&"hanshu"));
    }
}

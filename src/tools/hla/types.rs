
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

use std::collections::HashMap;

/// HLA 元数据
#[derive(Debug, Clone, PartialEq)]
pub struct HlaMetadata {
    pub version: String,
    pub source: String,
    pub keyword_style: String,
    pub target: Option<String>,
    pub encoding: String,
    pub timestamp: Option<String>,
}

impl Default for HlaMetadata {
    fn default() -> Self {
        Self {
            version: "1.2".to_string(),
            source: "huan-compiler".to_string(),
            keyword_style: "中文".to_string(),
            target: None,
            encoding: "UTF-8".to_string(),
            timestamp: None,
        }
    }
}

/// HLA 操作码
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Opcode {
    // 变量与常量操作
    Ling,
    Ding,
    Shewei,

    // 算术与逻辑操作
    Jia,
    Jian,
    Cheng,
    Chu,
    Quyu,
    Dayu,
    Xiaoyu,
    Dengyu,
    Buxiaoyu,
    Budayu,
    Budengyu,
    Qie,
    Huo,
    Fei,
    Zuoyi,
    Youyi,
    Anweiyu,
    Anweihuo,
    Anweiyihuo,

    // 控制流操作
    Ruo,
    Ruotiao,
    Tiao,
    Fouze,
    Jieshu,
    Biaoqian,
    Chongfu,
    Dang,
    Duiyu,

    // 函数操作
    Hanshu,
    Fanhui,
    Diaoyong,
    Fangfa,

    // 复合类型操作
    Liebiao,
    Zhujia,
    Charu,
    Shanchu,
    Qude,
    Sheshe,
    Changdu,
    Zidian,
    Charudian,
    Qudezhi,
    Jiegou,
    Ziduan,
    Suoyin,

    // 内存与指针操作
    Fenpei,
    Shifang,
    Duxie,
    Xieru,

    // 内联汇编
    Huibian,

    // 导入与外部声明
    Daoru,
    Waibu,
    WaibuJs,

    // 未知操作码
    Unknown,
}

impl Opcode {
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "LING" => Opcode::Ling,
            "DING" => Opcode::Ding,
            "SHEWEI" => Opcode::Shewei,

            "JIA" => Opcode::Jia,
            "JIAN" => Opcode::Jian,
            "CHENG" => Opcode::Cheng,
            "CHU" => Opcode::Chu,
            "QUYU" => Opcode::Quyu,
            "DAYU" => Opcode::Dayu,
            "XIAOYU" => Opcode::Xiaoyu,
            "DENGYU" => Opcode::Dengyu,
            "BUXIAOYU" => Opcode::Buxiaoyu,
            "BUDAYU" => Opcode::Budayu,
            "BUDENGYU" => Opcode::Budengyu,
            "QIE" => Opcode::Qie,
            "HUO" => Opcode::Huo,
            "FEI" => Opcode::Fei,
            "ZUOYI" => Opcode::Zuoyi,
            "YOUYI" => Opcode::Youyi,
            "ANWEIYU" => Opcode::Anweiyu,
            "ANWEIHUO" => Opcode::Anweihuo,
            "ANWEIYIHUO" => Opcode::Anweiyihuo,

            "RUO" => Opcode::Ruo,
            "RUOTIAO" => Opcode::Ruotiao,
            "TIAO" => Opcode::Tiao,
            "FOUZE" => Opcode::Fouze,
            "JIESHU" => Opcode::Jieshu,
            "BIAOQIAN" => Opcode::Biaoqian,
            "CHONGFU" => Opcode::Chongfu,
            "DANG" => Opcode::Dang,
            "DUIYU" => Opcode::Duiyu,

            "HANSHU" => Opcode::Hanshu,
            "FANHUI" => Opcode::Fanhui,
            "DIAOYONG" => Opcode::Diaoyong,
            "FANGFA" => Opcode::Fangfa,

            "LIEBIAO" => Opcode::Liebiao,
            "ZHUJIA" => Opcode::Zhujia,
            "CHARU" => Opcode::Charu,
            "SHANCHU" => Opcode::Shanchu,
            "QUDE" => Opcode::Qude,
            "SHESHE" => Opcode::Sheshe,
            "CHANGDU" => Opcode::Changdu,
            "ZIDIAN" => Opcode::Zidian,
            "CHARUDIAN" => Opcode::Charudian,
            "QUDEZHI" => Opcode::Qudezhi,
            "JIEGOU" => Opcode::Jiegou,
            "ZIDUAN" => Opcode::Ziduan,
            "SUOYIN" => Opcode::Suoyin,

            "FENPEI" => Opcode::Fenpei,
            "SHIFANG" => Opcode::Shifang,
            "DUXIE" => Opcode::Duxie,
            "XIERU" => Opcode::Xieru,

            "HUIBIAN" => Opcode::Huibian,

            "DAORU" => Opcode::Daoru,
            "WAIBU" => Opcode::Waibu,
            "WAIBUJS" => Opcode::WaibuJs,

            _ => Opcode::Unknown,
        }
    }

    pub fn to_str(self) -> &'static str {
        match self {
            Opcode::Ling => "LING",
            Opcode::Ding => "DING",
            Opcode::Shewei => "SHEWEI",

            Opcode::Jia => "JIA",
            Opcode::Jian => "JIAN",
            Opcode::Cheng => "CHENG",
            Opcode::Chu => "CHU",
            Opcode::Quyu => "QUYU",
            Opcode::Dayu => "DAYU",
            Opcode::Xiaoyu => "XIAOYU",
            Opcode::Dengyu => "DENGYU",
            Opcode::Buxiaoyu => "BUXIAOYU",
            Opcode::Budayu => "BUDAYU",
            Opcode::Budengyu => "BUDENGYU",
            Opcode::Qie => "QIE",
            Opcode::Huo => "HUO",
            Opcode::Fei => "FEI",
            Opcode::Zuoyi => "ZUOYI",
            Opcode::Youyi => "YOUYI",
            Opcode::Anweiyu => "ANWEIYU",
            Opcode::Anweihuo => "ANWEIHUO",
            Opcode::Anweiyihuo => "ANWEIYIHUO",

            Opcode::Ruo => "RUO",
            Opcode::Ruotiao => "RUOTIAO",
            Opcode::Tiao => "TIAO",
            Opcode::Fouze => "FOUZE",
            Opcode::Jieshu => "JIESHU",
            Opcode::Biaoqian => "BIAOQIAN",
            Opcode::Chongfu => "CHONGFU",
            Opcode::Dang => "DANG",
            Opcode::Duiyu => "DUIYU",

            Opcode::Hanshu => "HANSHU",
            Opcode::Fanhui => "FANHUI",
            Opcode::Diaoyong => "DIAOYONG",
            Opcode::Fangfa => "FANGFA",

            Opcode::Liebiao => "LIEBIAO",
            Opcode::Zhujia => "ZHUJIA",
            Opcode::Charu => "CHARU",
            Opcode::Shanchu => "SHANCHU",
            Opcode::Qude => "QUDE",
            Opcode::Sheshe => "SHESHE",
            Opcode::Changdu => "CHANGDU",
            Opcode::Zidian => "ZIDIAN",
            Opcode::Charudian => "CHARUDIAN",
            Opcode::Qudezhi => "QUDEZHI",
            Opcode::Jiegou => "JIEGOU",
            Opcode::Ziduan => "ZIDUAN",
            Opcode::Suoyin => "SUOYIN",

            Opcode::Fenpei => "FENPEI",
            Opcode::Shifang => "SHIFANG",
            Opcode::Duxie => "DUXIE",
            Opcode::Xieru => "XIERU",

            Opcode::Huibian => "HUIBIAN",

            Opcode::Daoru => "DAORU",
            Opcode::Waibu => "WAIBU",
            Opcode::WaibuJs => "WAIBUJS",

            Opcode::Unknown => "UNKNOWN",
        }
    }
}

/// HLA 操作
#[derive(Debug, Clone, PartialEq)]
pub struct HlaOperation {
    pub label: Option<String>,
    pub opcode: Opcode,
    pub operands: Vec<String>,
    pub line_number: usize,
}

/// HLA 程序
#[derive(Debug, Clone, PartialEq)]
pub struct HlaProgram {
    pub metadata: HlaMetadata,
    pub operations: Vec<HlaOperation>,
    pub labels: HashMap<String, usize>, // 标签名到操作索引
}

impl Default for HlaProgram {
    fn default() -> Self {
        Self {
            metadata: HlaMetadata::default(),
            operations: Vec::new(),
            labels: HashMap::new(),
        }
    }
}

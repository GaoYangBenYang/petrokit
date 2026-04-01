#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct CurveInfo {
    pub mnemonic: String,          // 曲线助记符：GR, SP, RT
    pub unit: String,              // 单位：API, OHMM
    pub api_code: Option<String>,  // API 编码
    pub description: String,       // 描述
    pub value_type: ValueType,     // 数据类型
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ValueType {
    Float,
    Integer,
    String,
}

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// 井头信息
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WellInfo {
    pub name: String,
    pub uwi: String,               // 唯一井标识
    pub field: String,             // 油田
    pub country: String,
    pub operator: String,          // 作业者
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub elevation: Option<f64>,    // 海拔
    pub kb_elevation: Option<f64>, // 补心海拔
    pub total_depth: Option<f64>,  // 完钻井深
}

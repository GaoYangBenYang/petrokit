use std::fmt;

// 验证错误
#[derive(Debug)]
pub enum ValidationError {
    /// 井名为空
    EmptyWellName,

    /// 深度范围无效
    InvalidDepthBounds { start: f64, stop: f64 },

    /// 采样间隔无效
    InvalidStep { step: f64 },

    /// 无曲线定义
    NoCurvesDefined,

    /// 曲线名重复
    DuplicateCurveName { name: String },

    /// 曲线数量与数据列数不匹配
    CurveColumnMismatch { curves: usize, columns: usize },

    /// 数据行数为零
    NoDataRows,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ValidationError::EmptyWellName => write!(f, "Well name is empty"),
            ValidationError::InvalidDepthBounds { start, stop } => {
                write!(f, "Invalid depth bounds: start={}, stop={}", start, stop)
            }
            ValidationError::InvalidStep { step } => {
                write!(f, "Invalid step: {}", step)
            }
            ValidationError::NoCurvesDefined => write!(f, "No curves defined"),
            ValidationError::DuplicateCurveName { name } => {
                write!(f, "Duplicate curve name: '{}'", name)
            }
            ValidationError::CurveColumnMismatch { curves, columns } => {
                write!(f, "Curve count ({}) != data columns ({})", curves, columns)
            }
            ValidationError::NoDataRows => write!(f, "No data rows"),
        }
    }
}

impl std::error::Error for ValidationError {}

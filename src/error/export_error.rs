use std::fmt;

// 导出错误
#[derive(Debug)]
pub enum ExportError {
    /// 格式不支持导出
    UnsupportedExportFormat(String),

    /// 数据类型不兼容
    IncompatibleType {
        curve: String,
        target_format: String,
    },

    /// 超出格式限制
    ExceedsLimit {
        format: String,
        limit: String,
        value: String,
    },

    /// 序列化失败
    SerializationFailed { detail: String },
}

impl fmt::Display for ExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExportError::UnsupportedExportFormat(fmt) => {
                write!(f, "Cannot export to format: {}", fmt)
            }
            ExportError::IncompatibleType {
                curve,
                target_format,
            } => {
                write!(
                    f,
                    "Curve '{}' has incompatible type for {}",
                    curve, target_format
                )
            }
            ExportError::ExceedsLimit {
                format,
                limit,
                value,
            } => {
                write!(f, "{} export limit exceeded: {} ({})", format, limit, value)
            }
            ExportError::SerializationFailed { detail } => {
                write!(f, "Serialization failed: {}", detail)
            }
        }
    }
}

impl std::error::Error for ExportError {}

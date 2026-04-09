use std::fmt;

#[derive(Debug)]
pub enum ConversionError {
    /// 单位转换失败
    UnitConversion {
        from: String,
        to: String,
        detail: String,
    },

    /// 坐标转换失败
    CoordinateConversion {
        from_system: String,
        to_system: String,
        detail: String,
    },

    /// 深度转换失败
    DepthConversion {
        from_type: String,
        to_type: String,
        detail: String,
    },
}

impl fmt::Display for ConversionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionError::UnitConversion { from, to, detail } => {
                write!(f, "Cannot convert '{}' to '{}': {}", from, to, detail)
            }
            ConversionError::CoordinateConversion {
                from_system,
                to_system,
                detail,
            } => {
                write!(
                    f,
                    "Cannot convert '{}' to '{}': {}",
                    from_system, to_system, detail
                )
            }
            ConversionError::DepthConversion {
                from_type,
                to_type,
                detail,
            } => {
                write!(
                    f,
                    "Cannot convert '{}' to '{}': {}",
                    from_type, to_type, detail
                )
            }
        }
    }
}

impl std::error::Error for ConversionError {}

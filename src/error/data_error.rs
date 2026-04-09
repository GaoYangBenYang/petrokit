use std::fmt;

// 数据错误
#[derive(Debug)]
pub enum DataError {
    /// 曲线不存在
    CurveNotFound {
        name: String,
        available: Vec<String>,
    },

    /// 深度范围无效
    InvalidDepthRange {
        from: f64,
        to: f64,
        valid_from: f64,
        valid_to: f64,
    },

    /// 索引越界
    IndexOutOfBounds { index: usize, len: usize },

    /// 数据为空
    EmptyData { context: String },

    /// 列数不匹配
    ColumnMismatch { expected: usize, actual: usize },

    /// 空值过多
    ExcessiveNullValues {
        curve: String,
        null_count: usize,
        total: usize,
    },

    /// 深度不单调
    NonMonotonicDepth {
        index: usize,
        prev_depth: f64,
        curr_depth: f64,
    },

    /// 采样间隔不一致
    IrregularStep {
        expected: f64,
        actual: f64,
        depth: f64,
    },
}

impl fmt::Display for DataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataError::CurveNotFound { name, available } => {
                write!(f, "Curve '{}' not found. Available: {:?}", name, available)
            }
            DataError::InvalidDepthRange {
                from,
                to,
                valid_from,
                valid_to,
            } => {
                write!(
                    f,
                    "Depth range [{}, {}] outside valid range [{}, {}]",
                    from, to, valid_from, valid_to
                )
            }
            DataError::IndexOutOfBounds { index, len } => {
                write!(f, "Index {} out of bounds (len: {})", index, len)
            }
            DataError::EmptyData { context } => {
                write!(f, "Empty data: {}", context)
            }
            DataError::ColumnMismatch { expected, actual } => {
                write!(f, "Column mismatch: expected {}, got {}", expected, actual)
            }
            DataError::ExcessiveNullValues {
                curve,
                null_count,
                total,
            } => {
                write!(
                    f,
                    "Curve '{}': {} / {} null values",
                    curve, null_count, total
                )
            }
            DataError::NonMonotonicDepth {
                index,
                prev_depth,
                curr_depth,
            } => {
                write!(
                    f,
                    "Non-monotonic depth at index {}: {} -> {}",
                    index, prev_depth, curr_depth
                )
            }
            DataError::IrregularStep {
                expected,
                actual,
                depth,
            } => {
                write!(
                    f,
                    "Irregular step at depth {}: expected {}, got {}",
                    depth, expected, actual
                )
            }
        }
    }
}

impl std::error::Error for DataError {}

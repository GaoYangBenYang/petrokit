use std::fmt;

// 解析错误
#[derive(Debug)]
pub enum ParseError {
    /// 文件头损坏
    InvalidHeader { format: String, detail: String },
    /// 段解析失败
    InvalidSection {
        section: String,
        line: usize,
        detail: String,
    },
    /// 曲线定义错误
    InvalidCurveDefinition {
        line: usize,
        raw: String,
        reason: String,
    },
    /// 数据行解析失败
    InvalidDataLine {
        line: usize,
        raw: String,
        expected_columns: usize,
        actual_columns: usize,
    },
    /// 数值解析失败
    InvalidNumber {
        line: usize,
        column: usize,
        raw: String,
    },
    /// 字符串解析失败
    InvalidString { line: usize, detail: String },
    /// 二进制数据损坏
    CorruptedBinary { offset: usize, detail: String },
    /// 版本不支持
    UnsupportedVersion { format: String, version: String },
    /// 缺少必要字段
    MissingField { section: String, field: String },
    /// 文件为空
    EmptyFile,
    /// 截断的文件
    TruncatedFile {
        expected_bytes: usize,
        actual_bytes: usize,
    },
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::InvalidHeader { format, detail } => {
                write!(f, "Invalid {} header: {}", format, detail)
            }
            ParseError::InvalidSection {
                section,
                line,
                detail,
            } => {
                write!(
                    f,
                    "Invalid section '{}' at line {}: {}",
                    section, line, detail
                )
            }
            ParseError::InvalidCurveDefinition { line, raw, reason } => {
                write!(
                    f,
                    "Invalid curve definition at line '{}': {} ({})",
                    line, raw, reason
                )
            }
            ParseError::InvalidDataLine {
                line,
                raw: _,
                expected_columns,
                actual_columns,
            } => {
                write!(
                    f,
                    "Invalid data at line {}: expected {} columns, got {}",
                    line, expected_columns, actual_columns
                )
            }
            ParseError::InvalidNumber { line, column, raw } => {
                write!(
                    f,
                    "Invalid number at line {} column {}: '{}'",
                    line, column, raw
                )
            }
            ParseError::InvalidString { line, detail } => {
                write!(f, "Invalid string at line {}: {}", line, detail)
            }
            ParseError::CorruptedBinary { offset, detail } => {
                write!(
                    f,
                    "Corrupted binary data at offset 0x{:X}: {}",
                    offset, detail
                )
            }
            ParseError::UnsupportedVersion { format, version } => {
                write!(f, "Unsupported {} version: {}", format, version)
            }
            ParseError::MissingField { section, field } => {
                write!(f, "Missing field '{}' in section '{}'", field, section)
            }
            ParseError::EmptyFile => write!(f, "File is empty"),
            ParseError::TruncatedFile {
                expected_bytes,
                actual_bytes,
            } => {
                write!(
                    f,
                    "Truncated file: expected {} bytes, got {}",
                    expected_bytes, actual_bytes
                )
            }
        }
    }
}

impl std::error::Error for ParseError {}

use crate::error::conversion_error::ConversionError;
use crate::error::data_error::DataError;
use crate::error::export_error::ExportError;
use crate::error::parse_error::ParseError;
use crate::error::validation_error::ValidationError;
use std::{fmt, io};

// 全局错误枚举
#[derive(Debug)]
pub enum GlobalError {
    /// IO 错误
    Io(io::Error),
    /// 文件格式不支持
    UnsupportedFormat(String),
    /// 文件编码错误
    EncodingError(String),
    /// 解析错误
    Parse(ParseError),
    /// 数据错误
    Data(DataError),
    /// 导出错误
    Export(ExportError),
    /// 坐标/单位转换错误
    Conversion(ConversionError),
    /// 验证错误
    Validation(ValidationError),
    /// 其他错误
    Other(String),
}

impl fmt::Display for GlobalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GlobalError::Io(e) => write!(f, "IO error: {}", e),
            GlobalError::UnsupportedFormat(fmt) => write!(f, "Unsupported format: {}", fmt),
            GlobalError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
            GlobalError::Parse(e) => write!(f, "Parse error: {}", e),
            GlobalError::Data(e) => write!(f, "Data error: {}", e),
            GlobalError::Export(e) => write!(f, "Export error: {}", e),
            GlobalError::Conversion(e) => write!(f, "Conversion error: {}", e),
            GlobalError::Validation(e) => write!(f, "Validation error: {}", e),
            GlobalError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for GlobalError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            GlobalError::Io(e) => Some(e),
            _ => None,
        }
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for GlobalError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl GlobalError {
    /// 文件不存在
    pub fn file_not_found(path: &str) -> Self {
        GlobalError::Io(io::Error::new(
            io::ErrorKind::NotFound,
            format!("File not found: {}", path),
        ))
    }

    /// 不支持的格式
    pub fn unsupported_format(ext: &str) -> Self {
        GlobalError::UnsupportedFormat(ext.to_string())
    }

    /// 曲线不存在
    pub fn curve_not_found(name: &str, available: &[String]) -> Self {
        GlobalError::Data(DataError::CurveNotFound {
            name: name.to_string(),
            available: available.to_vec(),
        })
    }

    /// 解析行失败
    pub fn parse_error(line: usize, detail: &str) -> Self {
        GlobalError::Parse(ParseError::InvalidString {
            line,
            detail: detail.to_string(),
        })
    }

    /// 通用错误
    pub fn other(msg: &str) -> Self {
        GlobalError::Other(msg.to_string())
    }
}

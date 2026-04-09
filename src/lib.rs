mod error;
#[cfg(feature = "las")]
mod formats;
mod models;
mod types;

use error::global_error::GlobalError;
use formats::las::parse;
use std::path::Path;
use types::PetroResult;

// 自动识别测井数据格式并解析
pub fn parse_file<WellLog>(path: &str) -> PetroResult<WellLog> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "las" => parse::parse_file(path),
        _ => Err(GlobalError::UnsupportedFormat(ext)),
    }
}

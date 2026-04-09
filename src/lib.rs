mod error;
mod models;
mod types;
#[cfg(feature = "las")]
mod formats;

use error::global_error::GlobalError;
use types::PetroResult;
use std::path::Path;
use formats::las;

// 自动识别测井数据格式并解析
pub fn parse_file<WellLog>(path: &str) -> PetroResult<WellLog> {
    let ext = Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();
    match ext.as_str() {
        "las" => las::parse::parse_file(path),
        _ => Err(GlobalError::UnsupportedFormat(ext)),
    }
}

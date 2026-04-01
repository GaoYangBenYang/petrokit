pub mod error;
pub mod models;

#[cfg(feature = "las")]
pub mod formats::las;

pub use error::{PetroError, PetroResult};
pub use models::*;

/// 自动识别测井数据格式并解析
pub fn parse_file(path: &str) -> PetroResult<WellLog> {
    let ext = std::path::Path::new(path)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_lowercase();

    match ext.as_str() {
        "las" => formats::las::parse_file(path),
        _ => Err(PetroError::UnsupportedFormat(ext)),
    }
}
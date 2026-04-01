// use std::fmt;
// use std::io;
//
// // ============================================================
// // 核心错误枚举
// // ============================================================
//
// #[derive(Debug)]
// pub enum PetroError {
//     /// IO 错误
//     Io(io::Error),
//
//     /// 文件格式不支持
//     UnsupportedFormat(String),
//
//     /// 文件编码错误
//     EncodingError(String),
//
//     /// 解析错误
//     Parse(ParseError),
//
//     /// 数据错误
//     Data(DataError),
//
//     /// 导出错误
//     Export(ExportError),
//
//     /// 坐标/单位转换错误
//     Conversion(ConversionError),
//
//     /// 验证错误
//     Validation(ValidationError),
//
//     /// 其他错误
//     Other(String),
// }
//
// // ============================================================
// // 解析错误
// // ============================================================
//
// #[derive(Debug)]
// pub enum ParseError {
//     /// 文件头损坏
//     InvalidHeader {
//         format: String,
//         detail: String,
//     },
//
//     /// 段解析失败
//     InvalidSection {
//         section: String,
//         line: usize,
//         detail: String,
//     },
//
//     /// 曲线定义错误
//     InvalidCurveDefinition {
//         line: usize,
//         raw: String,
//         reason: String,
//     },
//
//     /// 数据行解析失败
//     InvalidDataLine {
//         line: usize,
//         raw: String,
//         expected_columns: usize,
//         actual_columns: usize,
//     },
//
//     /// 数值解析失败
//     InvalidNumber {
//         line: usize,
//         column: usize,
//         raw: String,
//     },
//
//     /// 字符串解析失败
//     InvalidString {
//         line: usize,
//         detail: String,
//     },
//
//     /// 二进制数据损坏
//     CorruptedBinary {
//         offset: usize,
//         detail: String,
//     },
//
//     /// 版本不支持
//     UnsupportedVersion {
//         format: String,
//         version: String,
//     },
//
//     /// 缺少必要字段
//     MissingField {
//         section: String,
//         field: String,
//     },
//
//     /// DLIS 特定错误
//     DlisError(DlisParseError),
//
//     /// LIS 特定错误
//     LisError(LisParseError),
//
//     /// SEG-Y 特定错误
//     SegyError(SegyParseError),
//
//     /// WITSML 特定错误
//     WitsmlError(WitsmlParseError),
//
//     /// 文件为空
//     EmptyFile,
//
//     /// 截断的文件
//     TruncatedFile {
//         expected_bytes: usize,
//         actual_bytes: usize,
//     },
// }
//
// // ============================================================
// // 各格式特定解析错误
// // ============================================================
//
// #[derive(Debug)]
// pub enum DlisParseError {
//     /// Logical File 结构错误
//     InvalidLogicalFile {
//         index: usize,
//         detail: String,
//     },
//
//     /// Component 解析失败
//     InvalidComponent {
//         detail: String,
//     },
//
//     /// 属性类型未知
//     UnknownAttributeType {
//         code: u8,
//     },
//
//     /// Set 结构损坏
//     CorruptedSet {
//         name: String,
//         detail: String,
//     },
//
//     /// Channel 数据不一致
//     ChannelMismatch {
//         channel: String,
//         expected_samples: usize,
//         actual_samples: usize,
//     },
//
//     /// 压缩格式不支持
//     UnsupportedRepresentation {
//         code: u8,
//         detail: String,
//     },
// }
//
// #[derive(Debug)]
// pub enum LisParseError {
//     /// 物理记录损坏
//     CorruptedPhysicalRecord {
//         index: usize条目类型未知
//         UnknownEntryType {
//         code: u8,
//     },
//
//     /// 数据格式描述错误
//     InvalidDataFormat {
//         detail: String,
//     },
//
//     /// ,
//     detail: String,
// },
// /// 帧结构不一致
// FrameMismatch {
// expected_size: usize,
// actual_size: usize,
// },
// }
//
// #[derive(Debug)]
// pub enum SegyParseError {
//     /// 卷头损坏
//     CorruptedVolumeHeader {
//         detail: String,
//     },
//
//     /// 二进制文件头损坏
//     CorruptedBinaryHeader {
//         detail: String,
//     },
//
//     /// 道头损坏
//     CorruptedTraceHeader {
//         trace_index: usize,
//         detail: String,
//     },
//
//     /// 道数据长度不一致
//     TraceLengthMismatch {
//         trace_index: usize,
//         expected: usize,
//         actual: usize,
//     },
//
//     /// 采样格式不支持
//     UnsupportedSampleFormat {
//         code: i16,
//     },
//
//     /// 坐标系统未知
//     UnknownCoordinateSystem {
//         code: i16,
//     },
// }
//
// #[derive(Debug)]
// pub enum WitsmlParseError {
//     /// XML 结构错误
//     InvalidXml {
//         line: usize,
//         column: usize,
//         detail: String,
//     },
//
//     /// 命名空间错误
//     InvalidNamespace {
//         expected: String,
//         actual: String,
//     },
//
//     /// Schema 验证失败
//     SchemaValidation {
//         detail: String,
//     },
//
//     /// 缺少必要元素
//     MissingElement {
//         path: String,
//     },
//
//     /// 属性类型错误
//     InvalidAttribute {
//         element: String,
//         attribute: String,
//         detail: String,
//     },
// }
//
// // ============================================================
// // 数据错误
// // ============================================================
//
// #[derive(Debug)]
// pub enum DataError {
//     /// 曲线不存在
//     CurveNotFound {
//         name: String,
//         available: Vec<String>,
//     },
//
//     /// 深度范围无效
//     InvalidDepthRange {
//         from: f64,
//         to: f64,
//         valid_from: f64,
//         valid_to: f64,
//     },
//
//     /// 索引越界
//     IndexOutOfBounds {
//         index: usize,
//         len: usize,
//     },
//
//     /// 数据为空
//     EmptyData {
//         context: String,
//     },
//
//     /// 列数不匹配
//     ColumnMismatch {
//         expected: usize,
//         actual: usize,
//     },
//
//     /// 空值过多
//     ExcessiveNullValues {
//         curve: String,
//         null_count: usize,
//         total: usize,
//     },
//
//     /// 深度不单调
//     NonMonotonicDepth {
//         index: usize,
//         prev_depth: f64,
//         curr_depth: f64,
//     },
//
//     /// 采样间隔不一致
//     IrregularStep {
//         expected: f64,
//         actual: f64,
//         depth: f64,
//     },
// }
//
// // ============================================================
// // 导出错误
// // ============================================================
//
// #[derive(Debug)]
// pub enum ExportError {
//     /// 格式不支持导出
//     UnsupportedExportFormat(String),
//
//     /// 数据类型不兼容
//     IncompatibleType {
//         curve: String,
//         target_format: String,
//     },
//
//     /// 超出格式限制
//     ExceedsLimit {
//         format: String,
//         limit: String,
//         value: String,
//     },
//
//     /// 序列化失败
//     SerializationFailed {
//         detail: String,
//     },
// }
//
// // ============================================================
// // 转换错误
// // ============================================================
//
// #[derive(Debug)]
// pub enum ConversionError {
//     /// 单位转换失败
//     UnitConversion {
//         from: String,
//         to: String,
//         detail: String,
//     },
//
//     /// 坐标转换失败
//     CoordinateConversion {
//         from_system: String,
//         to_system: String,
//         detail: String,
//     },
//
//     /// 深度转换失败
//     DepthConversion {
//         from_type: String,
//         to_type: String,
//         detail: String,
//     },
// }
//
// // ============================================================
// // 验证错误
// // ============================================================
//
// #[derive(Debug)]
// pub enum ValidationError {
//     /// 井名为空
//     EmptyWellName,
//
//     /// 深度范围无效
//     InvalidDepthBounds {
//         start: f64,
//         stop: f64,
//     },
//
//     /// 采样间隔无效
//     InvalidStep {
//         step: f64,
//     },
//
//     /// 无曲线定义
//     NoCurvesDefined,
//
//     /// 曲线名重复
//     DuplicateCurveName {
//         name: String,
//     },
//
//     /// 曲线数量与数据列数不匹配
//     CurveColumnMismatch {
//         curves: usize,
//         columns: usize,
//     },
//
//     /// 数据行数为零
//     NoDataRows,
// }
//
// // ============================================================
// // Display 实现
// // ============================================================
//
// impl fmt::Display for PetroError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             PetroError::Io(e) => write!(f, "IO error: {}", e),
//             PetroError::UnsupportedFormat(fmt) => write!(f, "Unsupported format: {}", fmt),
//             PetroError::EncodingError(msg) => write!(f, "Encoding error: {}", msg),
//             PetroError::Parse(e) => write!(f, "Parse error: {}", e),
//             PetroError::Data(e) => write!(f, "Data error: {}", e),
//             PetroError::Export(e) => write!(f, "Export error: {}", e),
//             PetroError::Conversion(e) => write!(f, "Conversion error: {}", e),
//             PetroError::Validation(e) => write!(f, "Validation error: {}", e),
//             PetroError::Other(msg) => write!(f, "Error: {}", msg),
//         }
//     }
// }
//
// impl fmt::Display for ParseError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ParseError::InvalidHeader { format, detail } => {
//                 write!(f, "Invalid {} header: {}", format, detail)
//             }
//             ParseError::InvalidSection { section, line, detail } => {
//                 write!(f, "Invalid section '{}' at line {}: {}", section, line, detail)
//             }
//             ParseError::InvalidCurveDefinition { line, raw, reason } => {
//                 write!(f, "Invalid curve definition at line '{}': {} ({})", line, raw, reason)
//             }
//             ParseError::InvalidDataLine { line, raw: _, expected_columns, actual_columns } => {
//                 write!(f, "Invalid data at line {}: expected {} columns, got {}", line, expected_columns, actual_columns)
//             }
//             ParseError::InvalidNumber { line, column, raw } => {
//                 write!(f, "Invalid number at line {} column {}: '{}'", line, column, raw)
//             }
//             ParseError::InvalidString { line, detail } => {
//                 write!(f, "Invalid string at line {}: {}", line, detail)
//             }
//             ParseError::CorruptedBinary { offset, detail } => {
//                 write!(f, "Corrupted binary data at offset 0x{:X}: {}", offset, detail)
//             }
//             ParseError::UnsupportedVersion { format, version } => {
//                 write!(f, "Unsupported {} version: {}", format, version)
//             }
//             ParseError::MissingField { section, field } => {
//                 write!(f, "Missing field '{}' in section '{}'", field, section)
//             }
//             ParseError::DlisError(e) => write!(f, "DLIS error: {:?}", e),
//             ParseError::LisError(e) => write!(f, "LIS error: {:?}", e),
//             ParseError::SegyError(e) => write!(f, "SEG-Y error: {:?}", e),
//             ParseError::WitsmlError(e) => write!(f, "WITSML error: {:?}", e),
//             ParseError::EmptyFile => write!(f, "File is empty"),
//             ParseError::TruncatedFile { expected_bytes, actual_bytes } => {
//                 write!(f, "Truncated file: expected {} bytes, got {}", expected_bytes, actual_bytes)
//             }
//         }
//     }
// }
//
// impl fmt::Display for DataError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             DataError::CurveNotFound { name, available } => {
//                 write!(f, "Curve '{}' not found. Available: {:?}", name, available)
//             }
//             DataError::InvalidDepthRange { from, to, valid_from, valid_to } => {
//                 write!(f, "Depth range [{}, {}] outside valid range [{}, {}]", from, to, valid_from, valid_to)
//             }
//             DataError::IndexOutOfBounds { index, len } => {
//                 write!(f, "Index {} out of bounds (len: {})", index, len)
//             }
//             DataError::EmptyData { context } => {
//                 write!(f, "Empty data: {}", context)
//             }
//             DataError::ColumnMismatch { expected, actual } => {
//                 write!(f, "Column mismatch: expected {}, got {}", expected, actual)
//             }
//             DataError::ExcessiveNullValues { curve, null_count, total } => {
//                 write!(f, "Curve '{}': {} / {} null values", curve, null_count, total)
//             }
//             DataError::NonMonotonicDepth { index, prev_depth, curr_depth } => {
//                 write!(f, "Non-monotonic depth at index {}: {} -> {}", index, prev_depth, curr_depth)
//             }
//             DataError::IrregularStep { expected, actual, depth } => {
//                 write!(f, "Irregular step at depth {}: expected {}, got {}", depth, expected, actual)
//             }
//         }
//     }
// }
//
// impl fmt::Display for ExportError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ExportError::UnsupportedExportFormat(fmt) => {
//                 write!(f, "Cannot export to format: {}", fmt)
//             }
//             ExportError::IncompatibleType { curve, target_format } => {
//                 write!(f, "Curve '{}' has incompatible type for {}", curve, target_format)
//             }
//             ExportError::ExceedsLimit { format, limit, value } => {
//                 write!(f, "{} export limit exceeded: {} ({})", format, limit, value)
//             }
//             ExportError::SerializationFailed { detail } => {
//                 write!(f, "Serialization failed: {}", detail)
//             }
//         }
//     }
// }
//
// impl fmt::Display for ConversionError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ConversionError::UnitConversion { from, to, detail } => {
//                 write!(f, "Cannot convert '{}' to '{}': {}", from, to, detail)
//             }
//             ConversionError::CoordinateConversion { from_system, to_system, detail } => {
//                 write!(f, "Cannot convert '{}' to '{}': {}", from_system, to_system, detail)
//             }
//             ConversionError::DepthConversion { from_type, to_type, detail } => {
//                 write!(f, "Cannot convert '{}' to '{}': {}", from_type, to_type, detail)
//             }
//         }
//     }
// }
//
// impl fmt::Display for ValidationError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ValidationError::EmptyWellName => write!(f, "Well name is empty"),
//             ValidationError::InvalidDepthBounds { start, stop } => {
//                 write!(f, "Invalid depth bounds: start={}, stop={}", start, stop)
//             }
//             ValidationError::InvalidStep { step } => {
//                 write!(f, "Invalid step: {}", step)
//             }
//             ValidationError::NoCurvesDefined => write!(f, "No curves defined"),
//             ValidationError::DuplicateCurveName { name } => {
//                 write!(f, "Duplicate curve name: '{}'", name)
//             }
//             ValidationError::CurveColumnMismatch { curves, columns } => {
//                 write!(f, "Curve count ({}) != data columns ({})", curves, columns)
//             }
//             ValidationError::NoDataRows => write!(f, "No data rows"),
//         }
//     }
// }
//
// // ============================================================
// // Error trait 实现
// // ============================================================
//
// impl std::error::Error for PetroError {
//     fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
//         match self {
//             PetroError::Io(e) => Some(e),
//             _ => None,
//         }
//     }
// }
//
// impl std::error::Error for ParseError {}
// impl std::error::Error for DataError {}
// impl std::error::Error for ExportError {}
// impl std::error::Error for ConversionError {}
// impl std::error::Error for ValidationError {}
//
// // ============================================================
// // From 转换（自动 ? 操作符支持）
// // ============================================================
//
// impl From<io::Error> for PetroError {
//     fn from(e: io::Error) -> Self {
//         PetroError::Io(e)
//     }
// }
//
// impl From<ParseError> for PetroError {
//     fn from(e: ParseError) -> Self {
//         PetroError::Parse(e)
//     }
// }
//
// impl From<DataError> for PetroError {
//     fn from(e: DataError) -> Self {
//         PetroError::Data(e)
//     }
// }
//
// impl From<ExportError> for PetroError {
//     fn from(e: ExportError) -> Self {
//         PetroError::Export(e)
//     }
// }
//
// impl From<ConversionError> for PetroError {
//     fn from(e: ConversionError) -> Self {
//         PetroError::Conversion(e)
//     }
// }
//
// impl From<ValidationError> for PetroError {
//     fn from(e: ValidationError) -> Self {
//         PetroError::Validation(e)
//     }
// }
//
// impl From<String> for PetroError {
//     fn from(s: String) -> Self {
//         PetroError::Other(s)
//     }
// }
//
// impl From<&str> for PetroError {
//     fn from(s: &str) -> Self {
//         PetroError::Other(s.to_string())
//     }
// }
//
// // ============================================================
// // serde 序列化支持
// // ============================================================
//
// #[cfg(feature = "serde")]
// impl serde::Serialize for PetroError {
//     fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
//         serializer.serialize_str(&self.to_string())
//     }
// }
//
// // ============================================================
// // Result 类型别名
// // ============================================================
//
// pub type PetroResult<T> = Result<T, PetroError>;
//
// // ============================================================
// // 便捷构造函数
// // ============================================================
//
// impl PetroError {
//     /// 文件不存在
//     pub fn file_not_found(path: &str) -> Self {
//         PetroError::Io(io::Error::new(
//             io::ErrorKind::NotFound,
//             format!("File not found: {}", path),
//         ))
//     }
//
//     /// 不支持的格式
//     pub fn unsupported_format(ext: &str) -> Self {
//         PetroError::UnsupportedFormat(ext.to_string())
//     }
//
//     /// 曲线不存在
//     pub fn curve_not_found(name: &str, available: &[String]) -> Self {
//         PetroError::Data(DataError::CurveNotFound {
//             name: name.to_string(),
//             available: available.to_vec(),
//         })
//     }
//
//     /// 解析行失败
//     pub fn parse_error(line: usize, detail: &str) -> Self {
//         PetroError::Parse(ParseError::InvalidString {
//             line,
//             detail: detail.to_string(),
//         })
//     }
//
//     /// 通用错误
//     pub fn other(msg: &str) -> Self {
//         PetroError::Other(msg.to_string())
//     }
// }

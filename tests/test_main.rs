//
// // ============================================================
// // 测试
// // ============================================================
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn test_error_display() {
//         let err = PetroError::unsupported_format("xyz");
//         assert_eq!(err.to_string(), "Unsupported format: xyz");
//     }
//
//     #[test]
//     fn test_curve_not_found() {
//         let available = vec!["GR".into(), "SP".into()];
//         let err = PetroError::curve_not_found("RT", &available);
//         assert!(err.to_string().contains("RT"));
//         assert!(err.to_string().contains("GR"));
//     }
//
//     #[test]
//     fn test_question_mark_operator() {
//         fn inner() -> PetroResult<()> {
//             let _content = std::fs::read_to_string("nonexistent.las")?;
//             Ok(())
//         }
//
//         let result = inner();
//         assert!(result.is_err());
//         match result.unwrap_err() {
//             PetroError::Io(_) => {}
//             other => panic!("Expected Io error, got: {}", other),
//         }
//     }
//
//     #[test]
//     fn test_from_string() {
//         let err: PetroError = "something went wrong".into();
//         assert_eq!(err.to_string(), "Error: something went wrong");
//     }
//
//     #[test]
//     fn test_parse_errors() {
//         let err = PetroError::Parse(ParseError::InvalidDataLine {
//             line: 42,
//             raw: "1.2.3 4.5".into(),
//             expected_columns: 5,
//             actual_columns: 2,
//         });
//         assert!(err.to_string().contains("42"));
//     }
//
//     #[test]
//     fn test_depth_range_error() {
//         let err = PetroError::Data(DataError::InvalidDepthRange {
//             from: 5000.0,
//             to: 6000.0,
//             valid_from: 100.0,
//             valid_to: 4000.0,
//         });
//         assert!(err.to_string().contains("5000"));
//     }
// }

pub struct VersionInformation {
    pub mnem: String,
    pub data: StringOrNumber,
    pub description: String,
}

enum StringOrNumber {
    String(String),
    Number(i32),
}

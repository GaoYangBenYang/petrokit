use crate::error::global_error::GlobalError;

// Result 类型别名
pub type PetroResult<T> = Result<T, GlobalError>;

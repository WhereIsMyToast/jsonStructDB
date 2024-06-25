pub use crate::error::AppError;

pub type Result<T> = core::result::Result<T, AppError>;

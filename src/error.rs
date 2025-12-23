use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("数据库错误: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error("内部错误: {0}")]
    Anyhow(#[from] anyhow::Error),
}

impl AppError {
    // 对齐 Java 的 Result.code 为字符串
    pub fn code_msg(&self) -> (String, String) {
        match self {
            AppError::Sqlx(e) => ("500".to_string(), format!("数据库错误: {}", e)),
            AppError::Anyhow(e) => ("500".to_string(), format!("内部错误: {}", e)),
        }
    }
}

pub type AppResult<T> = Result<T, AppError>;

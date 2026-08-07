use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum ApiError {
    #[error("数据库操作失败")]
    Database(#[from] sqlx::Error),
    #[error("请求参数不合法: {0}")]
    BadRequest(String),
    #[error("未授权")]
    Unauthorized,
    #[error("资源不存在")]
    NotFound,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::Database(error) => {
                tracing::error!(%error, "database request failed");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "服务暂时不可用".to_string(),
                )
            }
            Self::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED, "管理凭据无效".to_string()),
            Self::NotFound => (StatusCode::NOT_FOUND, "资源不存在".to_string()),
        };
        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}

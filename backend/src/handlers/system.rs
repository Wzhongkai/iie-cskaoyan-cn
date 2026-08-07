use crate::{error::ApiError, state::AppState};
use axum::{extract::State, Json};

pub(crate) async fn health(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    sqlx::query("SELECT 1").execute(&state.pool).await?;
    Ok(Json(serde_json::json!({ "status": "ok" })))
}

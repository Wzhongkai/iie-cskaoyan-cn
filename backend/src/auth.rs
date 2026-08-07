use crate::{error::ApiError, state::AppState};
use axum::http::HeaderMap;

pub(crate) fn authorized(headers: &HeaderMap, state: &AppState) -> Result<(), ApiError> {
    let provided = headers
        .get("x-admin-token")
        .and_then(|value| value.to_str().ok());
    if provided == Some(state.admin_token.as_str()) {
        Ok(())
    } else {
        Err(ApiError::Unauthorized)
    }
}

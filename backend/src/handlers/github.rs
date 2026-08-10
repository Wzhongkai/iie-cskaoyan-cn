use crate::{
    error::ApiError,
    handlers::comments::github_profile_from_headers,
    models::{GithubAuthQuery, GithubCallbackQuery, GithubProfile},
    state::{AppState, GithubOAuthConfig},
};
use axum::{
    extract::{Query, State},
    http::{header::SET_COOKIE, HeaderMap, HeaderValue},
    response::{IntoResponse, Redirect, Response},
    Json,
};
use chrono::{Duration, Utc};
use reqwest::Client;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use url::Url;
use uuid::Uuid;

const SESSION_COOKIE: &str = "iie_comment_session";

#[derive(Deserialize)]
struct GithubTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct GithubUserResponse {
    id: i64,
    login: String,
    avatar_url: Option<String>,
}

fn oauth_config(state: &AppState) -> Result<&GithubOAuthConfig, ApiError> {
    state.github_oauth.as_ref().ok_or(ApiError::Unavailable)
}

fn return_path(path: Option<String>) -> String {
    let candidate = path.unwrap_or_else(|| "/articles".into());
    if candidate.starts_with("/articles/")
        && !candidate.contains("//")
        && !candidate.contains('\n')
        && !candidate.contains('\r')
    {
        candidate
    } else {
        "/articles".into()
    }
}

pub(crate) async fn start_github_login(
    State(state): State<AppState>,
    Query(query): Query<GithubAuthQuery>,
) -> Result<Redirect, ApiError> {
    let config = oauth_config(&state)?;
    let state_token = Uuid::new_v4();
    sqlx::query("DELETE FROM github_oauth_states WHERE expires_at <= now()")
        .execute(&state.pool)
        .await?;
    sqlx::query(
        "INSERT INTO github_oauth_states (state, return_path, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(state_token)
    .bind(return_path(query.return_to))
    .bind(Utc::now() + Duration::minutes(10))
    .execute(&state.pool)
    .await?;
    let mut url = Url::parse("https://github.com/login/oauth/authorize")
        .map_err(|_| ApiError::Unavailable)?;
    url.query_pairs_mut()
        .append_pair("client_id", &config.client_id)
        .append_pair("redirect_uri", &config.redirect_uri)
        .append_pair("scope", "read:user")
        .append_pair("state", &state_token.to_string());
    Ok(Redirect::temporary(url.as_str()))
}

pub(crate) async fn github_callback(
    State(state): State<AppState>,
    Query(query): Query<GithubCallbackQuery>,
) -> Result<Response, ApiError> {
    let config = oauth_config(&state)?.clone();
    let return_path = sqlx::query_scalar::<_, String>(
        "DELETE FROM github_oauth_states WHERE state = $1 AND expires_at > now() RETURNING return_path",
    )
    .bind(query.state)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::AccessDenied)?;
    let client = Client::new();
    let token = client
        .post("https://github.com/login/oauth/access_token")
        .header("Accept", "application/json")
        .form(&[
            ("client_id", config.client_id.as_str()),
            ("client_secret", config.client_secret.as_str()),
            ("code", query.code.as_str()),
            ("redirect_uri", config.redirect_uri.as_str()),
        ])
        .send()
        .await
        .map_err(|_| ApiError::Unavailable)?
        .error_for_status()
        .map_err(|_| ApiError::AccessDenied)?
        .json::<GithubTokenResponse>()
        .await
        .map_err(|_| ApiError::AccessDenied)?;
    let user = client
        .get("https://api.github.com/user")
        .bearer_auth(token.access_token)
        .header("User-Agent", "iie-cskaoyan-comments")
        .send()
        .await
        .map_err(|_| ApiError::Unavailable)?
        .error_for_status()
        .map_err(|_| ApiError::AccessDenied)?
        .json::<GithubUserResponse>()
        .await
        .map_err(|_| ApiError::AccessDenied)?;
    sqlx::query(
        "INSERT INTO github_users (github_id, login, avatar_url) VALUES ($1, $2, $3)
         ON CONFLICT (github_id) DO UPDATE SET login = EXCLUDED.login, avatar_url = EXCLUDED.avatar_url, updated_at = now()",
    )
    .bind(user.id)
    .bind(user.login)
    .bind(user.avatar_url)
    .execute(&state.pool)
    .await?;
    let session = Uuid::new_v4().simple().to_string();
    let session_hash = Sha256::digest(session.as_bytes()).to_vec();
    sqlx::query("DELETE FROM github_sessions WHERE expires_at <= now()")
        .execute(&state.pool)
        .await?;
    sqlx::query(
        "INSERT INTO github_sessions (token_hash, github_id, expires_at) VALUES ($1, $2, $3)",
    )
    .bind(session_hash)
    .bind(user.id)
    .bind(Utc::now() + Duration::days(30))
    .execute(&state.pool)
    .await?;
    let mut response = Redirect::to(&return_path).into_response();
    let secure = config
        .redirect_uri
        .starts_with("https://")
        .then_some("; Secure")
        .unwrap_or("");
    let cookie = format!(
        "{SESSION_COOKIE}={session}; Path=/; Max-Age={}; HttpOnly; SameSite=Lax{secure}",
        30 * 24 * 60 * 60,
    );
    response.headers_mut().append(
        SET_COOKIE,
        HeaderValue::from_str(&cookie).map_err(|_| ApiError::Unavailable)?,
    );
    Ok(response)
}

pub(crate) async fn github_me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<GithubProfile>, ApiError> {
    let (_, profile) = github_profile_from_headers(&state, &headers).await?;
    Ok(Json(profile))
}

use crate::{
    auth::authorized,
    error::ApiError,
    models::{ArticleComment, ArticleCommentInput, GithubProfile},
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::{header::COOKIE, HeaderMap, StatusCode},
    Json,
};
use sha2::{Digest, Sha256};
use uuid::Uuid;

const SESSION_COOKIE: &str = "iie_comment_session";

pub(crate) async fn github_profile_from_headers(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<(i64, GithubProfile), ApiError> {
    let token = headers
        .get(COOKIE)
        .and_then(|value| value.to_str().ok())
        .and_then(|cookie_header| {
            cookie_header.split(';').find_map(|item| {
                let (name, value) = item.trim().split_once('=')?;
                (name == SESSION_COOKIE).then_some(value)
            })
        })
        .ok_or(ApiError::AuthenticationRequired)?;
    let token_hash = Sha256::digest(token.as_bytes()).to_vec();
    let profile = sqlx::query_as::<_, (i64, String, Option<String>)>(
        "SELECT u.github_id, u.login, u.avatar_url
         FROM github_sessions s JOIN github_users u ON u.github_id = s.github_id
         WHERE s.token_hash = $1 AND s.expires_at > now()",
    )
    .bind(token_hash)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::AuthenticationRequired)?;
    Ok((
        profile.0,
        GithubProfile {
            login: profile.1,
            avatar_url: profile.2,
        },
    ))
}

pub(crate) async fn list_comments(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<ArticleComment>>, ApiError> {
    let comments = sqlx::query_as::<_, ArticleComment>(
        "SELECT c.id, c.parent_id, c.body, c.created_at, u.login AS author_login, u.avatar_url AS author_avatar_url
         FROM article_comments c
         JOIN articles a ON a.id = c.article_id
         JOIN github_users u ON u.github_id = c.github_id
         WHERE a.slug = $1 AND a.status = 'published'
         ORDER BY c.created_at ASC",
    )
    .bind(slug)
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(comments))
}

pub(crate) async fn list_admin_comments(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(article_id): Path<Uuid>,
) -> Result<Json<Vec<ArticleComment>>, ApiError> {
    authorized(&headers, &state)?;
    let comments = sqlx::query_as::<_, ArticleComment>(
        "SELECT c.id, c.parent_id, c.body, c.created_at,
                u.login AS author_login, u.avatar_url AS author_avatar_url
         FROM article_comments c
         JOIN github_users u ON u.github_id = c.github_id
         WHERE c.article_id = $1
         ORDER BY c.created_at ASC",
    )
    .bind(article_id)
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(comments))
}

pub(crate) async fn delete_admin_comment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((article_id, comment_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, ApiError> {
    authorized(&headers, &state)?;
    let deleted = sqlx::query("DELETE FROM article_comments WHERE id = $1 AND article_id = $2")
        .bind(comment_id)
        .bind(article_id)
        .execute(&state.pool)
        .await?;
    if deleted.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

pub(crate) async fn create_comment(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(slug): Path<String>,
    Json(input): Json<ArticleCommentInput>,
) -> Result<(StatusCode, Json<ArticleComment>), ApiError> {
    let (github_id, _) = github_profile_from_headers(&state, &headers).await?;
    let body = input.body.trim();
    if !(1..=2_000).contains(&body.chars().count()) {
        return Err(ApiError::BadRequest("评论应为 1-2000 个字符".into()));
    }
    let article_id = sqlx::query_scalar::<_, Uuid>(
        "SELECT id FROM articles WHERE slug = $1 AND status = 'published'",
    )
    .bind(&slug)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::NotFound)?;
    if let Some(parent_id) = input.parent_id {
        let parent_exists = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM article_comments WHERE id = $1 AND article_id = $2 AND parent_id IS NULL)",
        )
        .bind(parent_id)
        .bind(article_id)
        .fetch_one(&state.pool)
        .await?;
        if !parent_exists {
            return Err(ApiError::BadRequest("回复目标不存在或不是一级评论".into()));
        }
    }
    let comment = sqlx::query_as::<_, ArticleComment>(
        "WITH inserted AS (
             INSERT INTO article_comments (id, article_id, parent_id, github_id, body)
             VALUES ($1, $2, $3, $4, $5)
             RETURNING id, parent_id, body, created_at, github_id
         )
         SELECT inserted.id, inserted.parent_id, inserted.body, inserted.created_at,
                u.login AS author_login, u.avatar_url AS author_avatar_url
         FROM inserted JOIN github_users u ON u.github_id = inserted.github_id",
    )
    .bind(Uuid::new_v4())
    .bind(article_id)
    .bind(input.parent_id)
    .bind(github_id)
    .bind(body)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::NotFound)?;
    Ok((StatusCode::CREATED, Json(comment)))
}

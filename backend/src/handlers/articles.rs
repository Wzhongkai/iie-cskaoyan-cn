use crate::{
    auth::authorized,
    error::ApiError,
    models::{Article, ArticleInput, ArticleQuery},
    state::AppState,
};
use axum::{
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use chrono::{Datelike, Utc};
use uuid::Uuid;

pub(crate) async fn list_articles(
    State(state): State<AppState>,
    Query(params): Query<ArticleQuery>,
) -> Result<Json<Vec<Article>>, ApiError> {
    let limit = params.limit.unwrap_or(30).clamp(1, 100);
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, slug, title, excerpt, body_markdown, category, year, status, is_pinned, created_at, updated_at, published_at
         FROM articles
         WHERE status = 'published'
           AND ($1::text IS NULL OR category = $1)
           AND ($2::text IS NULL OR title ILIKE '%' || $2 || '%' OR COALESCE(excerpt, '') ILIKE '%' || $2 || '%' OR body_markdown ILIKE '%' || $2 || '%')
         ORDER BY is_pinned DESC, COALESCE(published_at, updated_at) DESC LIMIT $3",
    )
    .bind(params.category)
    .bind(params.q)
    .bind(limit)
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(articles))
}

pub(crate) async fn get_article(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Article>, ApiError> {
    let article = sqlx::query_as::<_, Article>(
        "SELECT id, slug, title, excerpt, body_markdown, category, year, status, is_pinned, created_at, updated_at, published_at
         FROM articles WHERE slug = $1 AND status = 'published'",
    )
    .bind(slug)
    .fetch_optional(&state.pool)
    .await?
    .ok_or(ApiError::NotFound)?;
    Ok(Json(article))
}

pub(crate) fn validate_article(input: &ArticleInput) -> Result<(), ApiError> {
    let slug = input.slug.trim();
    if slug.len() < 3
        || slug.len() > 120
        || !slug.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
    {
        return Err(ApiError::BadRequest(
            "Slug 只能使用小写字母、数字和连字符".into(),
        ));
    }
    if !(2..=120).contains(&input.title.trim().chars().count()) {
        return Err(ApiError::BadRequest("标题长度应为 2-120 个字符".into()));
    }
    if !(20..=500_000).contains(&input.body_markdown.trim().chars().count()) {
        return Err(ApiError::BadRequest("正文长度应为 20-500000 个字符".into()));
    }
    if !["initial", "reexam", "career", "policy", "data"].contains(&input.category.as_str()) {
        return Err(ApiError::BadRequest("内容分类不合法".into()));
    }
    if !["draft", "published", "archived"].contains(&input.status.as_str()) {
        return Err(ApiError::BadRequest("文章状态不合法".into()));
    }
    if input
        .year
        .is_some_and(|year| !(2010..=2100).contains(&year))
    {
        return Err(ApiError::BadRequest("文章年份不合法".into()));
    }
    Ok(())
}

pub(crate) async fn list_admin_articles(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<Article>>, ApiError> {
    authorized(&headers, &state)?;
    let articles = sqlx::query_as::<_, Article>(
        "SELECT id, slug, title, excerpt, body_markdown, category, year, status, is_pinned, created_at, updated_at, published_at
         FROM articles ORDER BY is_pinned DESC, updated_at DESC LIMIT 200",
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(articles))
}

pub(crate) async fn create_article(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<ArticleInput>,
) -> Result<(StatusCode, Json<Article>), ApiError> {
    authorized(&headers, &state)?;
    validate_article(&input)?;
    let article = sqlx::query_as::<_, Article>(
        "INSERT INTO articles (id, slug, title, excerpt, body_markdown, category, year, status, is_pinned, published_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, CASE WHEN $8 = 'published' THEN now() ELSE NULL END)
         RETURNING id, slug, title, excerpt, body_markdown, category, year, status, is_pinned, created_at, updated_at, published_at",
    )
    .bind(Uuid::new_v4())
    .bind(input.slug.trim())
    .bind(input.title.trim())
    .bind(input.excerpt.map(|value| value.trim().to_owned()).filter(|value| !value.is_empty()))
    .bind(input.body_markdown.trim())
    .bind(input.category)
    .bind(input.year.unwrap_or_else(|| Utc::now().year()))
    .bind(input.status)
    .bind(input.is_pinned)
    .fetch_one(&state.pool)
    .await?;
    Ok((StatusCode::CREATED, Json(article)))
}

pub(crate) async fn update_article(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    Json(input): Json<ArticleInput>,
) -> Result<Json<Article>, ApiError> {
    authorized(&headers, &state)?;
    validate_article(&input)?;
    let mut transaction = state.pool.begin().await?;
    let status = input.status.clone();
    let article = sqlx::query_as::<_, Article>(
        "UPDATE articles
         SET slug = $2, title = $3, excerpt = $4, body_markdown = $5, category = $6, year = $7,
             status = $8, is_pinned = $9, updated_at = now(),
             published_at = CASE WHEN $8 = 'published' THEN COALESCE(published_at, now()) ELSE published_at END
         WHERE id = $1
         RETURNING id, slug, title, excerpt, body_markdown, category, year, status, is_pinned, created_at, updated_at, published_at",
    )
    .bind(id)
    .bind(input.slug.trim())
    .bind(input.title.trim())
    .bind(input.excerpt.map(|value| value.trim().to_owned()).filter(|value| !value.is_empty()))
    .bind(input.body_markdown.trim())
    .bind(input.category)
    .bind(input.year.unwrap_or_else(|| Utc::now().year()))
    .bind(input.status)
    .bind(input.is_pinned)
    .fetch_optional(&mut *transaction)
    .await?
    .ok_or(ApiError::NotFound)?;

    match status.as_str() {
        "draft" => {
            sqlx::query(
                "UPDATE submissions SET status = 'pending', reviewed_at = NULL WHERE published_article_id = $1",
            )
            .bind(id)
            .execute(&mut *transaction)
            .await?;
        }
        "published" => {
            sqlx::query(
                "UPDATE submissions SET status = 'approved', reviewed_at = now() WHERE published_article_id = $1",
            )
            .bind(id)
            .execute(&mut *transaction)
            .await?;
        }
        _ => {}
    }

    transaction.commit().await?;
    Ok(Json(article))
}

pub(crate) async fn delete_article(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    authorized(&headers, &state)?;
    let mut transaction = state.pool.begin().await?;
    sqlx::query(
        "UPDATE submissions SET published_article_id = NULL WHERE published_article_id = $1",
    )
    .bind(id)
    .execute(&mut *transaction)
    .await?;
    let result = sqlx::query("DELETE FROM articles WHERE id = $1")
        .bind(id)
        .execute(&mut *transaction)
        .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    transaction.commit().await?;
    Ok(StatusCode::NO_CONTENT)
}

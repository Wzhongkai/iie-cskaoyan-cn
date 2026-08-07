use crate::{
    auth::authorized,
    error::ApiError,
    models::{AdmissionStat, AdmissionStatInput},
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};

pub(crate) async fn list_stats(
    State(state): State<AppState>,
) -> Result<Json<Vec<AdmissionStat>>, ApiError> {
    let stats = sqlx::query_as::<_, AdmissionStat>(
        "SELECT year, program, applicants, cutoff, interviewed, admitted, rate, source_note
         FROM admission_stats ORDER BY year DESC, program ASC",
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(stats))
}

pub(crate) async fn list_admin_stats(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<AdmissionStat>>, ApiError> {
    authorized(&headers, &state)?;
    list_stats(State(state)).await
}

pub(crate) async fn upsert_stat(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<AdmissionStatInput>,
) -> Result<Json<AdmissionStat>, ApiError> {
    authorized(&headers, &state)?;
    if !(2010..=2100).contains(&input.year)
        || input.program.trim().is_empty()
        || input.program.trim().chars().count() > 80
        || input.cutoff < 0
        || input.interviewed < 0
        || input.admitted < 0
        || input.applicants.is_some_and(|value| value < 0)
        || !(0.0..=100.0).contains(&input.rate)
    {
        return Err(ApiError::BadRequest("招生数据字段不合法".into()));
    }
    let stat = sqlx::query_as::<_, AdmissionStat>(
        "INSERT INTO admission_stats (year, program, applicants, cutoff, interviewed, admitted, rate, source_note)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
         ON CONFLICT (year, program) DO UPDATE SET applicants = EXCLUDED.applicants, cutoff = EXCLUDED.cutoff,
             interviewed = EXCLUDED.interviewed, admitted = EXCLUDED.admitted, rate = EXCLUDED.rate, source_note = EXCLUDED.source_note
         RETURNING year, program, applicants, cutoff, interviewed, admitted, rate, source_note",
    )
    .bind(input.year)
    .bind(input.program.trim())
    .bind(input.applicants)
    .bind(input.cutoff)
    .bind(input.interviewed)
    .bind(input.admitted)
    .bind(input.rate)
    .bind(input.source_note.map(|value| value.trim().to_owned()).filter(|value| !value.is_empty()))
    .fetch_one(&state.pool)
    .await?;
    Ok(Json(stat))
}

pub(crate) async fn delete_stat(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path((year, program)): Path<(i32, String)>,
) -> Result<StatusCode, ApiError> {
    authorized(&headers, &state)?;
    let result = sqlx::query("DELETE FROM admission_stats WHERE year = $1 AND program = $2")
        .bind(year)
        .bind(program)
        .execute(&state.pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

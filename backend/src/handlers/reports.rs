use crate::{
    auth::authorized,
    error::ApiError,
    models::{
        AnnualReport, AnnualReportDetail, AnnualReportInput, LabInput, LabStat, SchoolInput,
        SchoolStat, SchoolTierInput, SchoolTierStat, ScoreBandInput, ScoreBandStat, SubjectInput,
        SubjectStat,
    },
    state::AppState,
};
use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use sqlx::PgPool;

pub(crate) async fn list_reports(
    State(state): State<AppState>,
) -> Result<Json<Vec<AnnualReport>>, ApiError> {
    let reports = sqlx::query_as::<_, AnnualReport>(
        "SELECT year, title, exam_applicants_min, applicants_note, national_total_cutoff,
                national_politics_english_cutoff, national_subject_cutoff, academic_cutoff,
                professional_cutoff, interviewed_total, admitted_total, academic_admitted,
                professional_admitted, recommendation_total, direct_phd,
                recommendation_academic, recommendation_professional, exam_source_sample,
                exam_source_coverage, score_formula, source_file, source_note, updated_at
         FROM annual_reports ORDER BY year DESC",
    )
    .fetch_all(&state.pool)
    .await?;
    Ok(Json(reports))
}

pub(crate) async fn fetch_report(
    pool: &PgPool,
    year: Option<i32>,
) -> Result<AnnualReportDetail, ApiError> {
    let overview = sqlx::query_as::<_, AnnualReport>(
        "SELECT year, title, exam_applicants_min, applicants_note, national_total_cutoff,
                national_politics_english_cutoff, national_subject_cutoff, academic_cutoff,
                professional_cutoff, interviewed_total, admitted_total, academic_admitted,
                professional_admitted, recommendation_total, direct_phd,
                recommendation_academic, recommendation_professional, exam_source_sample,
                exam_source_coverage, score_formula, source_file, source_note, updated_at
         FROM annual_reports WHERE ($1::int IS NULL OR year = $1)
         ORDER BY year DESC LIMIT 1",
    )
    .bind(year)
    .fetch_optional(pool)
    .await?
    .ok_or(ApiError::NotFound)?;

    let selected_year = overview.year;
    let (school_tiers, schools, subjects, score_bands, labs) = tokio::try_join!(
        sqlx::query_as::<_, SchoolTierStat>(
            "SELECT track, tier, admitted, percentage FROM report_school_tiers
             WHERE year = $1 ORDER BY track DESC, CASE tier WHEN '985' THEN 1 WHEN '211' THEN 2 ELSE 3 END",
        ).bind(selected_year).fetch_all(pool),
        sqlx::query_as::<_, SchoolStat>(
            "SELECT track, tier, school, admitted FROM report_schools
             WHERE year = $1 ORDER BY track DESC, CASE tier WHEN '985' THEN 1 WHEN '211' THEN 2 ELSE 3 END, admitted DESC, school",
        ).bind(selected_year).fetch_all(pool),
        sqlx::query_as::<_, SubjectStat>(
            "SELECT program, phase, subject, highest, lowest, average, median FROM report_subject_stats
             WHERE year = $1 ORDER BY phase, program, subject",
        ).bind(selected_year).fetch_all(pool),
        sqlx::query_as::<_, ScoreBandStat>(
            "SELECT program, band, band_order, interviewed, admitted, cumulative_interviewed, cumulative_admitted, note
             FROM report_score_bands WHERE year = $1 ORDER BY program, band_order",
        ).bind(selected_year).fetch_all(pool),
        sqlx::query_as::<_, LabStat>(
            "SELECT program, lab, admitted, rejected, first_choice, highest, lowest, average, median, note
             FROM report_lab_stats WHERE year = $1 ORDER BY program, lab",
        ).bind(selected_year).fetch_all(pool),
    )?;

    Ok(AnnualReportDetail {
        overview,
        school_tiers,
        schools,
        subjects,
        score_bands,
        labs,
    })
}

pub(crate) async fn get_latest_report(
    State(state): State<AppState>,
) -> Result<Json<AnnualReportDetail>, ApiError> {
    Ok(Json(fetch_report(&state.pool, None).await?))
}

pub(crate) async fn get_report(
    State(state): State<AppState>,
    Path(year): Path<i32>,
) -> Result<Json<AnnualReportDetail>, ApiError> {
    Ok(Json(fetch_report(&state.pool, Some(year)).await?))
}

pub(crate) async fn list_admin_reports(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<AnnualReport>>, ApiError> {
    authorized(&headers, &state)?;
    list_reports(State(state)).await
}

pub(crate) async fn upsert_report(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<AnnualReportInput>,
) -> Result<Json<AnnualReport>, ApiError> {
    authorized(&headers, &state)?;
    if !(2010..=2100).contains(&input.year)
        || input.title.trim().is_empty()
        || input.title.trim().chars().count() > 160
        || input.source_file.trim().is_empty()
        || input.source_note.trim().is_empty()
        || input
            .exam_source_coverage
            .is_some_and(|value| !(0.0..=100.0).contains(&value))
    {
        return Err(ApiError::BadRequest("年度报告字段不合法".into()));
    }
    let non_negative = [
        input.exam_applicants_min,
        input.national_total_cutoff,
        input.national_politics_english_cutoff,
        input.national_subject_cutoff,
        input.academic_cutoff,
        input.professional_cutoff,
        input.interviewed_total,
        input.admitted_total,
        input.academic_admitted,
        input.professional_admitted,
        input.recommendation_total,
        input.direct_phd,
        input.recommendation_academic,
        input.recommendation_professional,
        input.exam_source_sample,
    ];
    if non_negative.into_iter().flatten().any(|value| value < 0) {
        return Err(ApiError::BadRequest("年度报告人数与分数不能为负数".into()));
    }

    let report = sqlx::query_as::<_, AnnualReport>(
        "INSERT INTO annual_reports (
            year, title, exam_applicants_min, applicants_note, national_total_cutoff,
            national_politics_english_cutoff, national_subject_cutoff, academic_cutoff,
            professional_cutoff, interviewed_total, admitted_total, academic_admitted,
            professional_admitted, recommendation_total, direct_phd,
            recommendation_academic, recommendation_professional, exam_source_sample,
            exam_source_coverage, score_formula, source_file, source_note
         ) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11,$12,$13,$14,$15,$16,$17,$18,$19,$20,$21,$22)
         ON CONFLICT (year) DO UPDATE SET
            title=EXCLUDED.title, exam_applicants_min=EXCLUDED.exam_applicants_min,
            applicants_note=EXCLUDED.applicants_note, national_total_cutoff=EXCLUDED.national_total_cutoff,
            national_politics_english_cutoff=EXCLUDED.national_politics_english_cutoff,
            national_subject_cutoff=EXCLUDED.national_subject_cutoff, academic_cutoff=EXCLUDED.academic_cutoff,
            professional_cutoff=EXCLUDED.professional_cutoff, interviewed_total=EXCLUDED.interviewed_total,
            admitted_total=EXCLUDED.admitted_total, academic_admitted=EXCLUDED.academic_admitted,
            professional_admitted=EXCLUDED.professional_admitted, recommendation_total=EXCLUDED.recommendation_total,
            direct_phd=EXCLUDED.direct_phd, recommendation_academic=EXCLUDED.recommendation_academic,
            recommendation_professional=EXCLUDED.recommendation_professional, exam_source_sample=EXCLUDED.exam_source_sample,
            exam_source_coverage=EXCLUDED.exam_source_coverage, score_formula=EXCLUDED.score_formula,
            source_file=EXCLUDED.source_file, source_note=EXCLUDED.source_note, updated_at=now()
         RETURNING year, title, exam_applicants_min, applicants_note, national_total_cutoff,
            national_politics_english_cutoff, national_subject_cutoff, academic_cutoff,
            professional_cutoff, interviewed_total, admitted_total, academic_admitted,
            professional_admitted, recommendation_total, direct_phd,
            recommendation_academic, recommendation_professional, exam_source_sample,
            exam_source_coverage, score_formula, source_file, source_note, updated_at",
    )
    .bind(input.year)
    .bind(input.title.trim())
    .bind(input.exam_applicants_min)
    .bind(
        input
            .applicants_note
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty()),
    )
    .bind(input.national_total_cutoff)
    .bind(input.national_politics_english_cutoff)
    .bind(input.national_subject_cutoff)
    .bind(input.academic_cutoff)
    .bind(input.professional_cutoff)
    .bind(input.interviewed_total)
    .bind(input.admitted_total)
    .bind(input.academic_admitted)
    .bind(input.professional_admitted)
    .bind(input.recommendation_total)
    .bind(input.direct_phd)
    .bind(input.recommendation_academic)
    .bind(input.recommendation_professional)
    .bind(input.exam_source_sample)
    .bind(input.exam_source_coverage)
    .bind(
        input
            .score_formula
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty()),
    )
    .bind(input.source_file.trim())
    .bind(input.source_note.trim())
    .fetch_one(&state.pool)
    .await?;
    Ok(Json(report))
}

pub(crate) fn validate_report_key(year: i32) -> Result<(), ApiError> {
    if (2010..=2100).contains(&year) {
        Ok(())
    } else {
        Err(ApiError::BadRequest("年度不合法".into()))
    }
}

pub(crate) async fn upsert_school_tier(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<SchoolTierInput>,
) -> Result<Json<SchoolTierStat>, ApiError> {
    authorized(&headers, &state)?;
    validate_report_key(input.year)?;
    if !["recommendation", "exam"].contains(&input.track.as_str())
        || !["985", "211", "non_211"].contains(&input.tier.as_str())
        || input.admitted < 0
        || !(0.0..=100.0).contains(&input.percentage)
    {
        return Err(ApiError::BadRequest("生源层次字段不合法".into()));
    }
    let item = sqlx::query_as::<_, SchoolTierStat>("INSERT INTO report_school_tiers (year, track, tier, admitted, percentage) VALUES ($1,$2,$3,$4,$5) ON CONFLICT (year,track,tier) DO UPDATE SET admitted=EXCLUDED.admitted, percentage=EXCLUDED.percentage RETURNING track,tier,admitted,percentage")
        .bind(input.year).bind(input.track).bind(input.tier).bind(input.admitted).bind(input.percentage).fetch_one(&state.pool).await?;
    Ok(Json(item))
}

pub(crate) async fn delete_school_tier(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<SchoolTierInput>,
) -> Result<StatusCode, ApiError> {
    authorized(&headers, &state)?;
    let result =
        sqlx::query("DELETE FROM report_school_tiers WHERE year=$1 AND track=$2 AND tier=$3")
            .bind(input.year)
            .bind(input.track)
            .bind(input.tier)
            .execute(&state.pool)
            .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

pub(crate) async fn upsert_school(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<SchoolInput>,
) -> Result<Json<SchoolStat>, ApiError> {
    authorized(&headers, &state)?;
    validate_report_key(input.year)?;
    if !["recommendation", "exam"].contains(&input.track.as_str())
        || !["985", "211", "non_211"].contains(&input.tier.as_str())
        || input.school.trim().is_empty()
        || input.admitted <= 0
    {
        return Err(ApiError::BadRequest("院校字段不合法".into()));
    }
    let item = sqlx::query_as::<_, SchoolStat>("INSERT INTO report_schools (year,track,tier,school,admitted) VALUES ($1,$2,$3,$4,$5) ON CONFLICT (year,track,school) DO UPDATE SET tier=EXCLUDED.tier, admitted=EXCLUDED.admitted RETURNING track,tier,school,admitted")
        .bind(input.year).bind(input.track).bind(input.tier).bind(input.school.trim()).bind(input.admitted).fetch_one(&state.pool).await?;
    Ok(Json(item))
}

pub(crate) async fn delete_school(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<SchoolInput>,
) -> Result<StatusCode, ApiError> {
    authorized(&headers, &state)?;
    let result = sqlx::query("DELETE FROM report_schools WHERE year=$1 AND track=$2 AND school=$3")
        .bind(input.year)
        .bind(input.track)
        .bind(input.school)
        .execute(&state.pool)
        .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

pub(crate) async fn upsert_subject(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<SubjectInput>,
) -> Result<Json<SubjectStat>, ApiError> {
    authorized(&headers, &state)?;
    validate_report_key(input.year)?;
    if input.program.trim().is_empty()
        || !["initial_subject", "admitted_total"].contains(&input.phase.as_str())
        || input.subject.trim().is_empty()
        || [input.highest, input.lowest, input.average, input.median]
            .into_iter()
            .any(|v| v < 0.0)
    {
        return Err(ApiError::BadRequest("科目统计字段不合法".into()));
    }
    let item = sqlx::query_as::<_, SubjectStat>("INSERT INTO report_subject_stats (year,program,phase,subject,highest,lowest,average,median) VALUES ($1,$2,$3,$4,$5,$6,$7,$8) ON CONFLICT (year,program,phase,subject) DO UPDATE SET highest=EXCLUDED.highest,lowest=EXCLUDED.lowest,average=EXCLUDED.average,median=EXCLUDED.median RETURNING program,phase,subject,highest,lowest,average,median")
        .bind(input.year).bind(input.program.trim()).bind(input.phase).bind(input.subject.trim()).bind(input.highest).bind(input.lowest).bind(input.average).bind(input.median).fetch_one(&state.pool).await?;
    Ok(Json(item))
}

pub(crate) async fn delete_subject(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<SubjectInput>,
) -> Result<StatusCode, ApiError> {
    authorized(&headers, &state)?;
    let result = sqlx::query(
        "DELETE FROM report_subject_stats WHERE year=$1 AND program=$2 AND phase=$3 AND subject=$4",
    )
    .bind(input.year)
    .bind(input.program)
    .bind(input.phase)
    .bind(input.subject)
    .execute(&state.pool)
    .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

pub(crate) async fn upsert_score_band(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<ScoreBandInput>,
) -> Result<Json<ScoreBandStat>, ApiError> {
    authorized(&headers, &state)?;
    validate_report_key(input.year)?;
    if input.program.trim().is_empty()
        || input.band.trim().is_empty()
        || [
            input.band_order,
            input.interviewed,
            input.admitted,
            input.cumulative_interviewed,
            input.cumulative_admitted,
        ]
        .into_iter()
        .any(|v| v < 0)
    {
        return Err(ApiError::BadRequest("分数段字段不合法".into()));
    }
    let item = sqlx::query_as::<_, ScoreBandStat>("INSERT INTO report_score_bands (year,program,band,band_order,interviewed,admitted,cumulative_interviewed,cumulative_admitted,note) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9) ON CONFLICT (year,program,band) DO UPDATE SET band_order=EXCLUDED.band_order,interviewed=EXCLUDED.interviewed,admitted=EXCLUDED.admitted,cumulative_interviewed=EXCLUDED.cumulative_interviewed,cumulative_admitted=EXCLUDED.cumulative_admitted,note=EXCLUDED.note RETURNING program,band,band_order,interviewed,admitted,cumulative_interviewed,cumulative_admitted,note")
        .bind(input.year).bind(input.program.trim()).bind(input.band.trim()).bind(input.band_order).bind(input.interviewed).bind(input.admitted).bind(input.cumulative_interviewed).bind(input.cumulative_admitted).bind(input.note.map(|v| v.trim().to_owned()).filter(|v| !v.is_empty())).fetch_one(&state.pool).await?;
    Ok(Json(item))
}

pub(crate) async fn delete_score_band(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<ScoreBandInput>,
) -> Result<StatusCode, ApiError> {
    authorized(&headers, &state)?;
    let result =
        sqlx::query("DELETE FROM report_score_bands WHERE year=$1 AND program=$2 AND band=$3")
            .bind(input.year)
            .bind(input.program)
            .bind(input.band)
            .execute(&state.pool)
            .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

pub(crate) async fn upsert_lab(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<LabInput>,
) -> Result<Json<LabStat>, ApiError> {
    authorized(&headers, &state)?;
    validate_report_key(input.year)?;
    if input.program.trim().is_empty()
        || !(1..=20).contains(&input.lab)
        || [input.admitted, input.rejected, input.first_choice]
            .into_iter()
            .any(|v| v < 0)
        || [input.highest, input.lowest, input.average, input.median]
            .into_iter()
            .flatten()
            .any(|v| v < 0.0)
    {
        return Err(ApiError::BadRequest("科室统计字段不合法".into()));
    }
    let item = sqlx::query_as::<_, LabStat>("INSERT INTO report_lab_stats (year,program,lab,admitted,rejected,first_choice,highest,lowest,average,median,note) VALUES ($1,$2,$3,$4,$5,$6,$7,$8,$9,$10,$11) ON CONFLICT (year,program,lab) DO UPDATE SET admitted=EXCLUDED.admitted,rejected=EXCLUDED.rejected,first_choice=EXCLUDED.first_choice,highest=EXCLUDED.highest,lowest=EXCLUDED.lowest,average=EXCLUDED.average,median=EXCLUDED.median,note=EXCLUDED.note RETURNING program,lab,admitted,rejected,first_choice,highest,lowest,average,median,note")
        .bind(input.year).bind(input.program.trim()).bind(input.lab).bind(input.admitted).bind(input.rejected).bind(input.first_choice).bind(input.highest).bind(input.lowest).bind(input.average).bind(input.median).bind(input.note.map(|v| v.trim().to_owned()).filter(|v| !v.is_empty())).fetch_one(&state.pool).await?;
    Ok(Json(item))
}

pub(crate) async fn delete_lab(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<LabInput>,
) -> Result<StatusCode, ApiError> {
    authorized(&headers, &state)?;
    let result =
        sqlx::query("DELETE FROM report_lab_stats WHERE year=$1 AND program=$2 AND lab=$3")
            .bind(input.year)
            .bind(input.program)
            .bind(input.lab)
            .execute(&state.pool)
            .await?;
    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound);
    }
    Ok(StatusCode::NO_CONTENT)
}

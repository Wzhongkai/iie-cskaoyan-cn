use std::time::Duration;

use axum::{
    extract::DefaultBodyLimit,
    http::StatusCode,
    routing::{delete, get, patch, post},
    Router,
};
use tower_http::{
    cors::CorsLayer, limit::RequestBodyLimitLayer, services::ServeDir, timeout::TimeoutLayer,
    trace::TraceLayer,
};

use crate::{
    handlers::{
        articles::{
            create_article, delete_article, get_article, list_admin_articles, list_articles,
            update_article,
        },
        reports::{
            delete_lab, delete_school, delete_school_tier, delete_score_band, delete_subject,
            get_latest_report, get_report, list_admin_reports, list_reports, upsert_lab,
            upsert_report, upsert_school, upsert_school_tier, upsert_score_band, upsert_subject,
        },
        stats::{delete_stat, list_admin_stats, list_stats, upsert_stat},
        submissions::{create_submission, list_submissions, update_submission},
        system::health,
        uploads::upload_image,
    },
    state::AppState,
};

pub(crate) fn build(state: AppState) -> Router {
    let upload_service = ServeDir::new(state.upload_dir.clone());

    Router::new()
        .route("/api/health", get(health))
        .route("/api/v1/stats", get(list_stats))
        .route("/api/v1/reports", get(list_reports))
        .route("/api/v1/reports/latest", get(get_latest_report))
        .route("/api/v1/reports/{year}", get(get_report))
        .route("/api/v1/articles", get(list_articles))
        .route("/api/v1/articles/{slug}", get(get_article))
        .route("/api/v1/submissions", post(create_submission))
        .route("/api/v1/uploads", post(upload_image))
        .route("/api/v1/admin/submissions", get(list_submissions))
        .route("/api/v1/admin/submissions/{id}", patch(update_submission))
        .route(
            "/api/v1/admin/articles",
            get(list_admin_articles).post(create_article),
        )
        .route(
            "/api/v1/admin/articles/{id}",
            patch(update_article).delete(delete_article),
        )
        .route(
            "/api/v1/admin/stats",
            get(list_admin_stats).post(upsert_stat),
        )
        .route("/api/v1/admin/stats/{year}/{program}", delete(delete_stat))
        .route(
            "/api/v1/admin/reports",
            get(list_admin_reports).post(upsert_report),
        )
        .route(
            "/api/v1/admin/report-school-tiers",
            post(upsert_school_tier).delete(delete_school_tier),
        )
        .route(
            "/api/v1/admin/report-schools",
            post(upsert_school).delete(delete_school),
        )
        .route(
            "/api/v1/admin/report-subjects",
            post(upsert_subject).delete(delete_subject),
        )
        .route(
            "/api/v1/admin/report-score-bands",
            post(upsert_score_band).delete(delete_score_band),
        )
        .route(
            "/api/v1/admin/report-labs",
            post(upsert_lab).delete(delete_lab),
        )
        .nest_service("/uploads", upload_service)
        .with_state(state)
        .layer(DefaultBodyLimit::max(6 * 1024 * 1024))
        .layer(RequestBodyLimitLayer::new(6 * 1024 * 1024))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(10),
        ))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
}

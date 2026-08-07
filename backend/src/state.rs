use sqlx::PgPool;
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) pool: PgPool,
    pub(crate) admin_token: String,
    pub(crate) upload_dir: PathBuf,
}

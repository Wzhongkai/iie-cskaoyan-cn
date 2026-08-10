use sqlx::PgPool;
use std::path::PathBuf;

#[derive(Clone)]
pub(crate) struct GithubOAuthConfig {
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
    pub(crate) redirect_uri: String,
}

#[derive(Clone)]
pub(crate) struct AppState {
    pub(crate) pool: PgPool,
    pub(crate) admin_token: String,
    pub(crate) upload_dir: PathBuf,
    pub(crate) github_oauth: Option<GithubOAuthConfig>,
}

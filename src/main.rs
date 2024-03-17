use anyhow::Context;
use mhb_server::{config::AppConfig, startup::Application};
use shuttle_secrets::SecretStore;
use sqlx::PgPool;

#[shuttle_runtime::main]
async fn mhb_api(
    #[shuttle_shared_db::Postgres] pool: PgPool,
    #[shuttle_secrets::Secrets] secrets: SecretStore,
) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("failed to run migrations")?;

    let config = AppConfig::try_from(secrets)?;

    let Application(app) = Application::build(pool, config)?;

    Ok(app.into())
}

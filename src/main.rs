use sqlx::PgPool;
use mhb_server::startup::Application;
use anyhow::Context;

#[shuttle_runtime::main]
async fn mhb_api(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .context("failed to run migrations")?;

    let Application(app) = Application::build(pool).await?;

    Ok(app.into())
}

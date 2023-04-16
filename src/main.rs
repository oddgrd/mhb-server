pub mod graphql;
pub mod schema;
pub mod startup;

use shuttle_runtime::CustomError;
use sqlx::PgPool;
use startup::Application;

#[shuttle_runtime::main]
async fn mhb_api(#[shuttle_shared_db::Postgres] pool: PgPool) -> shuttle_axum::ShuttleAxum {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let Application(app) = Application::build(pool).await?;

    Ok(app.into())
}

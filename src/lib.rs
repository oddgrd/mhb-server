pub mod graphql;
pub mod schema;
pub mod startup;

use shuttle_service::error::CustomError;
use sqlx::PgPool;
use startup::Application;
use sync_wrapper::SyncWrapper;

#[shuttle_service::main]
async fn mhb_api(#[shared::Postgres] pool: PgPool) -> shuttle_service::ShuttleAxum {
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .map_err(CustomError::new)?;

    let Application(app) = Application::build(pool).await?;

    Ok(SyncWrapper::new(app))
}

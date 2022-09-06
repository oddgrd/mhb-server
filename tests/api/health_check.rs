use crate::helpers::spawn_app;

#[tokio::test]
async fn health_check() {
    let app = spawn_app().await;
    let response = app
        .api_client
        .get(format!("{}/health_check", app.address))
        .send()
        .await
        .unwrap();

    let status = response.status().as_u16();
    assert_eq!(status, 200);
}

use mhb_server::models::boulder::NewBoulder;

use reqwest::StatusCode;
use serde_json::Value;

use crate::helpers::spawn_app;

#[tokio::test]
async fn create_boulder_returns_a_200_and_boulder_data() {
    let app = spawn_app().await;

    let new_boulder = NewBoulder {
        title: "test-boulder".into(),
        suggested_grade: 3,
        published: false,
    };

    let response = app
        .api_client
        .post(format!("{}/boulders", app.address))
        .json(&new_boulder)
        .send()
        .await
        .unwrap();

    let status = response.status().as_u16();
    assert_eq!(status, 200);

    let boulder: Value = serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();

    assert_eq!(boulder["title"], "test-boulder");
    assert_eq!(boulder["suggested_grade"].as_i64().unwrap(), 3);

    // Verify that the new boulder isn't published
    assert_eq!(boulder["published"], false);
}

#[tokio::test]
async fn get_boulder_returns_200_and_boulder() {
    let app = spawn_app().await;

    // First, try to fetch a boulder that doesn't exist.
    let response = app
        .api_client
        .get(format!("{}/boulders/not-a-real-boulder-id", app.address))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let new_boulder = NewBoulder {
        title: "test-boulder".into(),
        suggested_grade: 3,
        published: false,
    };

    let response = app
        .api_client
        .post(format!("{}/boulders", app.address))
        .json(&new_boulder)
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let boulder: Value = serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();

    assert_eq!(boulder["title"], "test-boulder");
    assert_eq!(boulder["suggested_grade"].as_i64().unwrap(), 3);

    // We need to extract the boulder id so we can use it to fetch the boulder.
    let boulder_id = boulder["id"].as_str().unwrap();

    let response = app
        .api_client
        .get(format!("{}/boulders/{boulder_id}", app.address))
        .send()
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let persisted_boulder: Value =
        serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();

    assert_eq!(persisted_boulder["title"], "test-boulder");

    // Verify the boulder we created is identical to the one we retrieved from the DB
    assert_eq!(boulder, persisted_boulder);
}

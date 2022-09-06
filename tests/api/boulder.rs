use serde_json::{json, Value};

use crate::helpers::spawn_app;

/// Create boulder GraphQL mutation
const CREATE_BOULDER: &str = r#"
    mutation($input: CreateBoulderInput!) {
        createBoulder(input: $input) {
            boulder {
                id
                title
                grade
                published
                updatedAt
                createdAt
            }
        }
    }
"#;

/// Get boulder GraphQL query
const GET_BOULDER: &str = r#"
    query($id: String!) {
        getBoulder(id: $id) {
            id
            title
            grade
            published
            updatedAt
            createdAt
        }
    }
"#;

#[tokio::test]
async fn create_boulder_returns_a_200() {
    let app = spawn_app().await;
    let response = app
        .graphql_query(
            CREATE_BOULDER,
            json!({
                "input": {
                    "title": "hello world",
                    "grade": 5
                }
            }),
        )
        .await;

    let status = response.status().as_u16();
    let body: Value = serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();

    let boulder = &body["data"]["createBoulder"]["boulder"];

    assert_eq!(status, 200);
    assert_eq!(boulder["title"], "hello world");

    // Verify that the new boulder isn't published by default
    assert_eq!(boulder["published"], false);
}

#[tokio::test]
async fn create_boulder_persists_the_new_boulder() {
    let app = spawn_app().await;
    let response = app
        .graphql_query(
            CREATE_BOULDER,
            json!({
                "input": {
                    "title": "hello world",
                    "grade": 5
                }
            }),
        )
        .await;

    let status = response.status().as_u16();
    let body: Value = serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();

    let boulder = &body["data"]["createBoulder"]["boulder"];

    assert_eq!(status, 200);
    assert_eq!(boulder["title"], "hello world");

    let boulder_id = boulder["id"].as_str().unwrap();

    // Retrieve the boulder we just created from the database
    let response = app
        .graphql_query(GET_BOULDER, json!({ "id": boulder_id }))
        .await;

    let status = response.status().as_u16();
    assert_eq!(status, 200);

    let body: Value = serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();

    // Verify the boulder we created is identical to the one we retrieved from the DB
    let persisted_boulder = &body["data"]["getBoulder"];
    assert_eq!(boulder, persisted_boulder);
}

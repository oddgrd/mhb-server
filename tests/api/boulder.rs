// use serde_json::{json, Value};
//
// use crate::helpers::spawn_app;

#[tokio::test]
async fn create_boulder_returns_a_200_and_boulder_data() {
    // let app = spawn_app().await;

    // let status = response.status().as_u16();
    // assert_eq!(status, 200);
    //
    // let boulder: Value = serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();
    //
    // assert_eq!(boulder["title"], "hello world");
    // assert_eq!(boulder["grade"].as_i64().unwrap(), 5);
    //
    // // Verify that the new boulder isn't published by default
    // assert_eq!(boulder["published"], false);
}

#[tokio::test]
async fn create_boulder_persists_the_new_boulder() {
    // let app = spawn_app().await;

    // let response = todo!();
    // let status = response.status().as_u16();
    // assert_eq!(status, 200);
    //
    // let boulder: Value = serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();
    //
    // assert_eq!(boulder["title"], "hello world");
    //
    // let boulder_id = boulder["id"].as_str().unwrap();
    //
    // // Retrieve the boulder we just created from the database
    // let response = todo!();
    // let status = response.status().as_u16();
    // assert_eq!(status, 200);
    //
    // let boulder: Value = serde_json::from_slice(&response.bytes().await.unwrap()).unwrap();
    //
    // // Verify the boulder we created is identical to the one we retrieved from the DB
    // let persisted_boulder = todo!(); 
    // assert_eq!(boulder, persisted_boulder);
}

use super::*;
use axum::{
    body::Body,
    http::{self, Request, StatusCode},
};
use http_body_util::BodyExt; // for `collect`
use models::{CreateItem, UpdateItem};
use serde_json::Value;
use tower::ServiceExt; // for `call`, `oneshot`, and `ready`

#[tokio::test]
async fn it_creates_an_item() {
    let db = Db::default();
    let app = app(db);

    let new_item = CreateItem {
        name: "Item 1".to_string(),
        description: "Description 1".to_string(),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::POST)
                .uri("/items")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&new_item).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_string = String::from_utf8(body.to_vec()).unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();

    // println!("{:?}", body_string);
    println!("Returned body: {:?}", body_string);
    assert!(body["id"].is_string());
    assert_eq!(body["name"], new_item.name);
    assert_eq!(body["description"], new_item.description);

    // Check that ID is of type Uuid
    let id_str = body["id"].as_str().unwrap();
    Uuid::parse_str(id_str).unwrap();
}

#[tokio::test]
async fn it_returns_all_items() {
    let db = Db::default();
    let new_item_1 = Item {
        id: Uuid::new_v4(),
        name: "Item 1".to_string(),
        description: "Description 1".to_string(),
    };
    let new_item_2 = Item {
        id: Uuid::new_v4(),
        name: "Item 2".to_string(),
        description: "Description 2".to_string(),
    };

    db.write()
        .unwrap()
        .insert(new_item_1.id, new_item_1.clone());
    db.write()
        .unwrap()
        .insert(new_item_2.id, new_item_2.clone());

    let app = app(db);
    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::GET)
                .uri("/items")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body_string = String::from_utf8(body.to_vec()).unwrap();
    let mut body: Value = serde_json::from_slice(&body).unwrap();

    println!("Returned body: {:?}", body_string);
    assert!(body.is_array());
    assert_eq!(body.as_array().unwrap().len(), 2);

    let items = body.as_array_mut().unwrap();
    items.sort_by(|a, b| a["name"].as_str().unwrap().cmp(b["name"].as_str().unwrap()));

    assert_eq!(body[0]["id"], new_item_1.id.to_string());
    assert_eq!(body[0]["name"], new_item_1.name);
    assert_eq!(body[0]["description"], new_item_1.description);
    assert_eq!(body[1]["id"], new_item_2.id.to_string());
    assert_eq!(body[1]["name"], new_item_2.name);
    assert_eq!(body[1]["description"], new_item_2.description);
}

#[tokio::test]
async fn it_fails_updating_nonexistent_item() {
    let db = Db::default();
    let app = app(db);

    let update_item = UpdateItem {
        name: Some("Item 1".to_string()),
        description: Some("Description 1".to_string()),
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(http::Method::PATCH)
                .uri("/items/00000000-0000-0000-0000-000000000000")
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .body(Body::from(serde_json::to_string(&update_item).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();

    println!("{:?}", response);
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

use crate::{
    models::{CreateItem, Item, UpdateItem},
    Db,
};
use axum::{
    extract::{Json, Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use uuid::Uuid;

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn create_item(
    State(db): State<Db>,
    Json(payload): Json<CreateItem>,
) -> impl IntoResponse {
    let mut db = db.write().unwrap();
    let item = Item {
        id: Uuid::new_v4(),
        name: payload.name,
        description: payload.description,
    };
    db.insert(item.id, item.clone());
    (StatusCode::CREATED, Json(item))
}

pub async fn read_items(State(db): State<Db>) -> impl IntoResponse {
    let db = db.read().unwrap();
    let items: Vec<Item> = db.values().cloned().collect();
    Json(items)
}

pub async fn update_item(
    Path(id): Path<Uuid>,
    State(db): State<Db>,
    Json(payload): Json<UpdateItem>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut item = db
        .read()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(StatusCode::NOT_FOUND)?;

    if let Some(name) = payload.name {
        item.name = name;
    }

    if let Some(description) = payload.description {
        item.description = description;
    }

    db.write().unwrap().insert(item.id, item.clone());

    Ok(Json(item))
}

pub async fn delete_item(Path(id): Path<Uuid>, State(db): State<Db>) -> impl IntoResponse {
    let mut db = db.write().unwrap();
    if db.remove(&id).is_some() {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

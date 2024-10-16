use crate::models::Input;
use axum::Json;

pub async fn hello_world() -> &'static str {
    "Hello, world!"
}

pub async fn stringify_json(Json(payload): Json<Input>) -> String {
    println!("Hello, {}", payload.name);
    format!("Hello, {}", payload.name)
}

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: Uuid,
    pub product: String,
    pub quantity: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateItem {
    pub product: String,
    pub quantity: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateItem {
    pub product: Option<String>,
    pub quantity: Option<String>,
}

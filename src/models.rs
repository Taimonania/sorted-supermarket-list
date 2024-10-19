use serde::{Deserialize, Serialize};
use uuid::Uuid;

// #[derive(Serialize, Deserialize, Clone)]
// pub struct Supermarket {
//     pub id: u32,
//     pub name: String,
// }

// #[derive(Serialize, Deserialize, Clone)]
// pub struct List {
//     pub id: u32,
//     pub name: String,
//     pub supermarket: Supermarket,
//     pub items: Vec<Item>,
// }

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateItem {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateItem {
    pub name: Option<String>,
    pub description: Option<String>,
}

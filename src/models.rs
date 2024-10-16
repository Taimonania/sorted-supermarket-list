use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Input {
    pub name: String,
}

pub struct ShoppingItem {
    pub id: u32,
    pub name: String,
    pub description: String,
}

pub struct Supermarket {
    pub id: u32,
    pub name: String,
}

pub struct ShoppingList {
    pub id: u32,
    pub name: String,
    pub supermarket: Supermarket,
    pub items: Vec<ShoppingItem>,
}
use serde::{Deserialize, Serialize};

/// Represents an item in the inventory.
#[derive(Serialize, Deserialize, Debug)]
pub struct Item {

    pub id: i32,
    pub name: String,
    pub description: String,
    pub category: String,
    pub image: String,

    pub price: i32,
    pub rarity: i32,

}

impl Item {
    pub fn new(id: i32, name: String, description: String, category: String, image: String, price: i32, rarity: i32) -> Item {
        Item {
            id,
            name,
            description,
            category,
            image,
            price,
            rarity,
        }
    }
}
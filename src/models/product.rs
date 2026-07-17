use serde::Serialize;

#[derive(Serialize, Debug, Clone)]
pub struct ProductItem {
    pub id: u32,
    pub name: String,
    pub price_1: f32,
    pub price_2: f32,
    pub inventory: u32,
}

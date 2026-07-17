use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::product::ProductItem;

#[derive(Serialize, Debug, Clone)]
pub struct Sale {
    id: u32,
    items: Vec<ProductItem>,
    total_amount: f32,
    sale_date: DateTime<Utc>,
    transactio_time: DateTime<Utc>,
}

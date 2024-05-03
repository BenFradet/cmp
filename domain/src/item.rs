use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
// can't have lifetimes for function components
pub struct Item {
    pub name: String,
    pub provider: String,
    pub price: String,
    pub image_link: String,
    pub product_link: String,
    pub logo_link: String,
    pub time: String,
}
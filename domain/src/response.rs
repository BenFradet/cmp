use serde_derive::{Deserialize, Serialize};

use crate::item::Item;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    pub items: Vec<Item>,
}
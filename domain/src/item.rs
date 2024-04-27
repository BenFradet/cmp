#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub name: String,
    pub provider: String,
    pub price: String,
    pub image_link: String,
    pub link: String,
    pub time: String,
}
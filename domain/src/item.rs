#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub name: String,
    pub provider: &'static str,
    pub price: String,
    pub image_link: String,
    pub link: String,
    pub logo_link: &'static str,
    pub time: String,
}
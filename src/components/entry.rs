use yew::prelude::*;

use crate::domain::item::Item;

#[derive(PartialEq, Properties, Clone)]
pub struct EntryProps {
    pub item: Item,
}

#[function_component(Entry)]
pub fn entry(props: &EntryProps) -> Html {
    let price = &props.item.price;
    html! {
        <div class="cell">{price}</div>
    }
}
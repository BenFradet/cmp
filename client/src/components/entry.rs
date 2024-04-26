use domain::item::Item;
use yew::prelude::*;

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
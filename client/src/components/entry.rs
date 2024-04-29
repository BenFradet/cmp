use domain::item::Item;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct EntryProps {
    pub item: Item,
}

#[function_component(Entry)]
pub fn entry(props: &EntryProps) -> Html {
    let Item { name, provider, price, image_link, link, logo_link, time } = &props.item;
    html! {
        <div class="card">
            <header class="card-header">
                <img src={logo_link.clone()} alt={provider.clone()} />
                //<p class="card-header-title">{provider}</p>
                <button class="card-header-icon" aria-label="more options">
                    <span class="icon">
                        <i class="fas fa-angle-down" aria-hidden="true"></i>
                    </span>
                </button>
            </header>
            <div class="card-image">
                <figure class="image is-4by3">
                    <img src={image_link.clone()} alt="Product image" />
                </figure>
            </div>
            <div class="card-content">
                <div class="content">
                    <span class="heading has-text-grey">{format!("Price for {} on {}:", name, provider)}</span>
                    <h3 class="mt-2 mb-0">{price}</h3>
                    <p class="highlight number">{price}</p>
                    <time>{time}</time>
                </div>
            </div>
            <footer class="card-footer">
                <a href={link.clone()} class="card-footer-item">{format!("Go to {}!", provider)}</a>
            </footer>
        </div>
    }
}
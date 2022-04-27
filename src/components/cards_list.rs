use yew::{classes, function_component, html, Callback, Properties};

use crate::models::card_content::CardContent;

#[derive(Properties, PartialEq)]
pub struct CardsListProps {
    pub cards: Vec<CardContent>,
    pub on_click: Callback<CardContent>,
}

#[function_component(CardsList)]
pub fn cards_list(CardsListProps { cards, on_click }: &CardsListProps) -> Html {
    cards
        .iter()
        .map(|card| {
            let on_video_select = {
                let on_click = on_click.clone();
                let video = card.clone();
                Callback::from(move |_| on_click.emit(video.clone()))
            };

            html! {
                <p class={classes!("bg-grey-100")} onclick={on_video_select}>{card.original.clone()}</p>
            }
        })
        .collect()
}

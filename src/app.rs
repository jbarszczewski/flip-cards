use rand::{thread_rng, Rng};
use yew::MouseEvent;
use yew::{classes, function_component, html, use_state, Callback};

use crate::components::card_details::CardDetails;
use crate::components::cards_list::CardsList;
use crate::models::card_content::CardContent;

#[function_component(App)]
pub fn app() -> Html {
	let mut rng = rand::thread_rng();
	let cards = vec![
		CardContent {
			id: 1,
			original: "Something".to_string(),
			translation: "Qualcosa".to_string(),
		},
		CardContent {
			id: 2,
			original: "An apple".to_string(),
			translation: "Una Mela".to_string(),
		},
		CardContent {
			id: 3,
			original: "This is a test. That is slightly longer sentence. And one more.".to_string(),
			translation:
				"Questo è un test. Questa è una frase leggermente più lunga. E un altro ancora."
					.to_string(),
		},
	];
	let selected_card = use_state(|| None);
	let on_card_select = {
		let selected_card = selected_card.clone();
		Callback::from(move |video: CardContent| selected_card.set(Some(video)))
	};

	let onclick = {
		let selected_card = selected_card.clone();
		let random_card = cards[rng.gen_range(0..cards.len())].clone();
		Callback::from(move |_: MouseEvent| selected_card.set(Some(random_card.clone())))
	};

	let details = selected_card.as_ref().map(|card| {
		html! {
			<CardDetails card={card.clone()} />
		}
	});

	html! {
		<>
			<div class={classes!("flex", "mb-4")}>
				  <div class={classes!("w-full", "bg-gray-500", "h-12")}>
					  <h1 class={classes!("text-3xl")}>{ "Flip Cards!" }</h1>
				  </div>
			</div>
			<div class={classes!("flex", "mb-4")}>
				  <div class={classes!("w-3/4")}>
					<button {onclick} class={classes!("bg-blue-500", "hover:bg-blue-700", "text-white", "font-bold", "py-2", "px-4", "rounded")}>{"Give me random!"}</button>
					{for details}
				</div>
				  <div class={classes!("w-1/4", "bg-gray-400")}>
					<CardsList cards={cards} on_click={on_card_select.clone()}/>
				</div>
			</div>
		</>
	}
}

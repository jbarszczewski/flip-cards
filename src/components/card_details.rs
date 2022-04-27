use yew::{classes, function_component, html, Properties};

use crate::models::card_content::CardContent;

#[derive(Clone, Properties, PartialEq)]
pub struct CardDetailsProps {
	pub card: CardContent,
}

#[function_component(CardDetails)]
pub fn card_details(CardDetailsProps { card }: &CardDetailsProps) -> Html {
	html! {
		<div class={classes!("max-w-sm","rounded", "overflow-hidden" ,"shadow-lg")}>
			<div class={classes!("px-6", "py-6")}>
				<h3 class={classes!("text-blue-700", "font-bold")}>{ card.original.clone() }</h3>
			</div>
			<div class={classes!("px-6", "py-6")}>
				<h3 class={classes!("text-green-700", "font-bold")}>{ card.translation.clone() }</h3>
			</div>
		</div>
	}
}

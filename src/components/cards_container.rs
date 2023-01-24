use yew::prelude::*;

#[derive(PartialEq, Clone)]
pub struct Card {
    pub owner: String,
    pub text: String,
    pub date: String,
}

#[derive(Properties, PartialEq)]
pub struct CardsContainerProps {
    pub cards: Vec<Card>,
}

#[function_component(CardsContainer)]
pub fn cards_container(props: &CardsContainerProps) -> Html{
    let cloned_prop_cards = props.cards.clone();

    let html_cards = cloned_prop_cards.into_iter().map(|card| {
        html!{
            <>
            <p>{"Card Owner: "}{card.owner}</p>
            <p>{card.text}</p>
            <p>{"Date: "}{card.date}</p>
            </>     
        }
    }).collect::<Html>();


    let cards = html!{
        <div class="card">
            {html_cards}
        </div>
    };

    html!{
        <div class= "cards_container">
            {cards}
        </div>
    }
}

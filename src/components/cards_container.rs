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
            <div class="card">
                <p>{"Card Owner: "}{card.owner}</p>
                <p>{card.text}</p>
                <p>{"Date: "}{card.date}</p>
            </div>
            </>     
        }
    }).collect::<Html>();

    html!{
        <>
        <div class= "cards_container">
            {html_cards}
        </div>
        </>
    }
}

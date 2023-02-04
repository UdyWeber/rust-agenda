use std::ops::Deref;

use chrono::Utc;
use yew::prelude::*;
use crate::components::card_adder::CardAdder;
use gloo::console::log;

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

    let cards_state = use_state(|| cloned_prop_cards);

    let cloned_card_states = cards_state.clone();
    let handle_card_adder = Callback::from(move |_|{
        let new_card = Card{
            owner: "Adder Jaw".to_owned(),
            date: Utc::now().to_string(),
            text: "On the other hand, we denounce with righteous indignation and dislike men who are so beguiled and demoralized by the charms of pleasure of the moment, so blinded by desire, that they cannot foresee the pain and trouble that are bound to ensue; and equal blame belongs to those who fail in their duty through weakness of will, which is the same as saying through shrinking from toil and pain. These cases are perfectly simple and easy to distinguish. In a free hour, when our power of choice is untrammelled and when nothing prevents our being able to do what we like best, every pleasure is to be welcomed and every pain avoided. But in certain circumstances and owing to the claims of duty or the obligations of business it will frequently occur that pleasures have to be repudiated and annoyances accepted. The wise man therefore always holds in these matters to this principle of selection: he rejects pleasures to secure other greater pleasures, or else he endures pains to avoid worse pains.".to_owned(),
        };

        let mut cards = cloned_card_states
            .deref()
            .to_owned();
        
        cards.push(new_card);
        cloned_card_states.set(cards);
        
        log!("Button has been clicked")
    });

    let cards_to_used = cards_state.to_vec(); 

    let html_cards = cards_to_used.into_iter().map(|card| {
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
        <CardAdder handle_onclick={handle_card_adder}/>
        <div class= "cards_container">
            {html_cards}
        </div>
        </>
    }
}

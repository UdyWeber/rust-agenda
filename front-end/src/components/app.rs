use std::ops::Deref;

use chrono::Utc;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::components::text_input::TextInput;
use crate::components::cards_container::{CardsContainer, Card};
use crate::components::form_button::CardFormButton;


#[function_component(App)]
pub fn app() -> Html {
    let (owner_name, owner_callback) = get_name_from_owner_and_callback();
    let (a, b) = get_card_text_and_callback();
    let (handle_card_adder, cards) = get_add_card_callback_and_cards(owner_name, a);
    
    html! {
        <>
        <center>
            <div class="card_form">
                <br/>
                <TextInput input_name="card_owner_name" field_name="Card Owner Name" handle_onchange={owner_callback}/>
                <br/>
                <label for="texto">{"Card Text"}</label><br/>
                <textarea name="texto" onchange={b}/>
                <br/>
            </div>
        </center>
        <center><CardFormButton label={"Submit Form"} handle_onclick={handle_card_adder}/></center>
        <br/>
        <h1><center>{"CARDS"}</center></h1>
        <hr/>
        
        <CardsContainer cards={cards}/>
        </>
    }
}


fn get_name_from_owner_and_callback() -> (String, Callback<web_sys::Event>) {
    let owner_input_state = use_state(|| "".to_owned());

    let cloned_owner_input_state = owner_input_state.clone();
    let card_owner_handle_change = Callback::from(move |event: Event| {
        let input_value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        cloned_owner_input_state.set(input_value);
    });

    let owner_name = owner_input_state.to_string();

    return (owner_name, card_owner_handle_change)
}

fn get_card_text_and_callback() -> (String, Callback<web_sys::Event>) {
    let text_input_state = use_state(|| "".to_owned());

    let cloned_owner_input_state = text_input_state.clone();
    let text_handle_change = Callback::from(move |event: Event| {
        let input_value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();

        cloned_owner_input_state.set(input_value);
    });

    let owner_name = text_input_state.to_string();

    return (owner_name, text_handle_change)
}

fn get_add_card_callback_and_cards(owner_name: String, card_text: String) -> (yew::Callback<()>, Vec<Card>) {
    let cards = vec![];
    let cards_state = use_state(|| cards);

    let cloned_card_states = cards_state.clone();
    let handle_card_adder = Callback::from(move |_|{
        let new_card = Card{
            owner: owner_name.to_owned(),
            date: Utc::now().to_string(),
            text: card_text.to_owned()
        };

        let mut cards = cloned_card_states
            .deref()
            .to_owned();
        
        cards.push(new_card);
        cloned_card_states.set(cards);
        
        log!("Button has been clicked")
    });

    return (handle_card_adder, cards_state.to_vec())
}
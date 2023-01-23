use std::vec;

use chrono::Utc;
use gloo::console::log;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
struct InputProps {
    input_name: String,
    field_name: String,
    handle_onchange: Callback<String>,
}


#[function_component(TextInput)]
fn text_inputs(props: &InputProps) -> Html {
    let handle_on_change = props.handle_onchange.clone();
    let on_change = Callback::from(move |event: Event| {
        let input_value = event
            .target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        handle_on_change.emit(input_value);
    });
    
    html! {
        <>
        <label for={props.input_name.clone()}>{props.field_name.clone()}</label><br/>
        <input type="text" name={props.input_name.clone()} onchange={on_change}/><br/>
        </>
    }
}

#[derive(Properties, PartialEq)]
struct ButtonProps {
    label: String,
    handle_onclick: Callback<()>,
}


#[function_component(CardFormButton)]
fn card_form_button(props: &ButtonProps) -> Html{
    let onclick = props.handle_onclick.clone();
    let button_onclick = Callback::from(move |_| {
        onclick.emit(());
    });

    html! {
        <button onclick={button_onclick}>{&props.label}</button>
    }
}

#[function_component(CardForm)]
fn card_form() -> Html{
    let card_owner_state = use_state(|| "You need to pass in your name".to_owned());
    let button_click_count_state = use_state(|| 0_i32);

    let cloned_state = card_owner_state.clone(); 
    let card_owner_changed = Callback::from(move |card_owner|{
        cloned_state.set("Field full".to_owned());
        log!("Card Owner Has Changed!!", "Content:", card_owner);
        
    });

    
    let cloned_click_count_state = button_click_count_state.clone();
    let button_callback = Callback::from(move |_|{
        let count = * cloned_click_count_state;
        cloned_click_count_state.set(count + 1);
        log!("Button has been clicked")
    });

    html! {
        <>
        <div class="card_form">
            <TextInput input_name="fcard_owner" field_name="Card Owner" handle_onchange={card_owner_changed}/>
            <br/>
            <b>{"Card Checkup Status"}</b>
            <p>{"Card Owner: "}{&*card_owner_state}</p>

            <CardFormButton label="Submit Card" handle_onclick={button_callback}/>
            <br/>
            <p>{"Button Has been Clicked: "}{*button_click_count_state}{" Times"}</p>
        </div>
        </>
    }
}

#[derive(PartialEq, Clone)]
struct Card {
    owner: String,
    text: String,
    date: String,
}

#[derive(Properties, PartialEq)]
struct CardsContainerProps {
    cards: Vec<Card>,
}

#[function_component(CardsContainer)]
fn cards_container(props: &CardsContainerProps) -> Html{
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

#[function_component(App)]
fn app() -> Html {
    let cards = vec![
        Card{
            owner: "Jaw".to_owned(),
            date: Utc::now().to_string(),
            text: "Hoje estava na reuniao com o Will e o Aru e tiraram sarro de mim :`((((".to_owned(),
        },
        
    ];

    html! {
        <>
        <CardForm/>

        <h1><center>{"CARDS"}</center></h1>
        <hr/>
        
        <CardsContainer cards={cards}/>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}

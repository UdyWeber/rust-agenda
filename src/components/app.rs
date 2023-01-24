use chrono::Utc;
use yew::prelude::*;

use crate::components::cards_form::CardForm;
use crate::components::cards_container::{CardsContainer, Card};


#[function_component(App)]
pub fn app() -> Html {
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

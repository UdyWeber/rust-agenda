use yew::prelude::*;
use gloo::console::log;

use crate::components::text_input::TextInput;
use crate::components::form_button::CardFormButton;


#[function_component(CardForm)]
pub fn card_form() -> Html{
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

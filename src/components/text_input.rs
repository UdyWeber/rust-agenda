use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub input_name: String,
    pub field_name: String,
    pub handle_onchange: Callback<String>,
}


#[function_component(TextInput)]
pub fn text_inputs(props: &InputProps) -> Html {
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
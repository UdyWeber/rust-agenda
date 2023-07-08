use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct InputProps {
    pub input_name: String,
    pub field_name: String,
    pub handle_onchange: Callback<Event>
}


#[function_component(TextInput)]
pub fn text_inputs(props: &InputProps) -> Html {
    let on_change = props.handle_onchange.clone();
    html! {
        <>
        <label for={props.input_name.clone()}>{props.field_name.clone()}</label><br/>
        <input type="text" name={props.input_name.clone()} onchange={on_change}/><br/>
        </>
    }
}
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub label: String,
    pub handle_onclick: Callback<()>,
}


#[function_component(CardFormButton)]
pub fn card_form_button(props: &ButtonProps) -> Html{
    let onclick = props.handle_onclick.clone();
    let button_onclick = Callback::from(move |_| {
        onclick.emit(());
    });

    html! {
        <button onclick={button_onclick}>{&props.label}</button>
    }
}
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CardAdderProps {
    pub handle_onclick: Callback<()>,
}


#[function_component(CardAdder)]
pub fn card_adder(props: &CardAdderProps) -> Html{
    let onclick = props.handle_onclick.clone();
    let button_onclick = Callback::from(move |_| {
        onclick.emit(());
    });

    html! {
        <button onclick={button_onclick}>{"Adds a Card"}</button>
    }
}
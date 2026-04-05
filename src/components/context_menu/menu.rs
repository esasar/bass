use yew::{function_component, html, Children, Html, Properties};
use crate::components::sandbox::Position;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub pos: Position,
    pub children: Children
}

#[function_component(ContextMenu)]
pub fn context_menu(props: &Props) -> Html {
    html! {
        <menu>
            { props.children.iter().map(|item| { item }).collect::<Html>() }
        </menu>
    }
}
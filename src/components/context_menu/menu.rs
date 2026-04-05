use yew::{function_component, html, props, Children, Html, Properties};
use crate::components::sandbox::Position;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub pos: Position,
    pub children: Children
}

#[function_component(ContextMenu)]
pub fn context_menu(props: &Props) -> Html {
    let position = format!("position: absolute; left: {}px; top: {}px;", props.pos.x, props.pos.y);

    html! {
        <menu style={position}>
            { props.children.iter().map(|item| { item }).collect::<Html>() }
        </menu>
    }
}
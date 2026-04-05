use yew::{function_component, html, Children, Html, Properties};
use crate::components::sandbox::Position;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub pos: Position,
    pub children: Children
}

const MENU_WIDTH: f64 = 100.0; // ~approximate

#[function_component(ContextMenu)]
pub fn context_menu(props: &Props) -> Html {
    let window_width = web_sys::window()
        .unwrap()
        .inner_width()
        .unwrap()
        .as_f64()
        .unwrap();

    // adjust context-menu position if too close to the right side to prevent it from overflowing
    let adjusted_x = if props.pos.x + MENU_WIDTH > window_width {
        props.pos.x - MENU_WIDTH
    } else {
        props.pos.x
    };

    let position = format!("position: absolute; left: {}px; top: {}px;", adjusted_x, props.pos.y);

    html! {
        <menu class="context-menu" style={position}>
            { props.children.iter().map(|item| { item }).collect::<Html>() }
        </menu>
    }
}
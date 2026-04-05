use yew::{function_component, html, Html};

#[function_component(Sandbox)]
pub fn sandbox() -> Html {
    html! {
        <canvas>
        </canvas>
    }
}
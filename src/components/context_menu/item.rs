use yew::{function_component, html, Callback, Html, MouseEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub on_click: Callback<MouseEvent>,
}

#[function_component(ContextMenuItem)]
pub fn context_menu_item(props: &Props) -> Html {
    html! {
        <menuitem class="context-menu-item" onclick={&props.on_click}>
            {&props.label}
        </menuitem>
    }
}
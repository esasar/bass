use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
}

#[function_component(Header)]
pub fn header(props: &Props) -> Html {
    html! {
        <header>
            <h1>
                {&props.title}
            </h1>
        </header>
    }
}
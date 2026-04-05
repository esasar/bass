use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub accuracy: f64,
    pub runtime: f64,
}

#[function_component(Footer)]
pub fn footer(props: &Props) -> Html {
    html! {
        <footer>
            <div>
                {"Accuracy: "}
                <span class="footer-accuracy">
                    {format!("{:3.2}", 100.0 * props.accuracy)}
                </span>
                {"% | Runtime: "}
                <span class="footer-runtime">
                    {format!("{:.2}", props.runtime)}
                </span>
                {"mus"}
            </div>
        </footer>
    }
}
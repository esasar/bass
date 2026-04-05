mod components;
mod sim;

use yew::{function_component, html, use_state, Html};
use crate::components::{Header, Sandbox};

#[function_component(App)]
pub fn app() -> Html {
    let accuracies = use_state(Vec::<f64>::new);
    let runtimes = use_state(Vec::<f64>::new);

    html! {
        <>
            <Header title={"BAss"}/>
            <main>
                <section>
                    <Sandbox accuracies={accuracies.clone()} runtimes={runtimes}/>
                </section>
                <aside>{accuracies.last().map(|v| format!("{:.3}", v)).unwrap_or_default()}</aside>
            </main>
            <footer>{"Footer"}</footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
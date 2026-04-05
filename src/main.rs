mod components;
mod sim;

use yew::{function_component, html, use_state, Html};
use crate::components::{Footer, Header, Sandbox};

#[function_component(App)]
pub fn app() -> Html {
    let accuracies = use_state(Vec::<f64>::new);
    let runtimes = use_state(Vec::<f64>::new);

    html! {
        <>
            <Header title={"BAss"}/>
            <main>
                <section>
                    <Sandbox accuracies={accuracies.clone()} runtimes={runtimes.clone()}/>
                </section>
                <aside>
                    
                </aside>
            </main>
            <Footer
                accuracy={accuracies.last().copied().unwrap_or(0.0)}
                runtime={runtimes.last().copied().unwrap_or(0.0)}
            />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
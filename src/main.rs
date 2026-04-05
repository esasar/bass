mod components;

use yew::{function_component, html, Html};
use crate::components::{Header, Sandbox};

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <>
            <Header title={"BAss"}/>
            <main>
                <section>
                    <Sandbox/>
                </section>
                <aside>{"Aside content"}</aside>
            </main>
            <footer>{"Footer"}</footer>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
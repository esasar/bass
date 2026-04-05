use yew::{use_state, Callback};
use yew::{function_component, html, Html};
use crate::components::sandbox::controls::Controls;

#[function_component(Sandbox)]
pub fn sandbox() -> Html {
    let std = use_state(|| 1.0f64);
    let on_std_change = Callback::from({
        let std = std.clone();
        move |val: f64| std.set(val)
    });

    let iters = use_state(|| 10.0f64);
    let on_iters_change = Callback::from({
        let iters = iters.clone();
        move |val: f64| iters.set(val)
    });

    html! {
        <>
            <Controls
                std={*std}
                on_std_change={on_std_change}
                iters={*iters}
                on_iters_change={on_iters_change}
            />
            <canvas>
            </canvas>
        </>
    }
}
use std::str::FromStr;
use web_sys::HtmlInputElement;
use yew::{function_component, html, Callback, Html, InputEvent, Properties, TargetCast};
use crate::components::Slider;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub std: f64,
    pub on_std_change: Callback<f64>,
    // TODO: iters from f64 -> i64
    pub iters: f64,
    pub on_iters_change: Callback<f64>,
}

fn parse_input_event<T: FromStr>(e: InputEvent) -> Option<T> {
    e.target_unchecked_into::<HtmlInputElement>()
        .value()
        .parse()
        .ok()
}

fn input_callback<T: FromStr + 'static>(on_change: Callback<T>) -> Callback<InputEvent> {
    Callback::from(move |e: InputEvent| {
        if let Some(val) = parse_input_event::<T>(e) {
            on_change.emit(val);
        }
    })
}

#[function_component(Controls)]
pub fn controls(props: &Props) -> Html {
    let on_std_input = input_callback(props.on_std_change.clone());
    let on_iters_input = input_callback(props.on_iters_change.clone());

    html! {
        <div class="controls">
            <Slider
                label={"Std"}
                value={props.std}
                on_input={on_std_input}
                min={0.1}
                max={45.0}
                unit={"deg"}
                step={0.1}
            />
            <Slider
                label={"Iters"}
                value={props.iters}
                on_input={on_iters_input}
                min={1.0}
                max={100.0}
                unit={""}
                step={1.0}
            />
        </div>
    }
}
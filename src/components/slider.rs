use yew::{function_component, html, Callback, Html, InputEvent, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub label: String,
    pub on_input: Callback<InputEvent>,
    #[prop_or(0.0)]
    pub value: f64,
    #[prop_or(0.0)]
    pub min: f64,
    #[prop_or(100.0)]
    pub max: f64,
    #[prop_or(1.0)]
    pub step: f64,
    #[prop_or("".to_string())]
    pub unit: String,
}

#[function_component(Slider)]
pub fn slider(props: &Props) -> Html {
    let value_width = format!("width: {}ch", props.max.to_string().len() + 2);

    html! {
        <div class="slider">
            <label>
                {&props.label}{" ("}
                <span class="slider-value" style={value_width}>
                    {format!("{:.1}", props.value)}
                </span>{&props.unit}{")"}
            </label>
            <input
                type="range"
                min={props.min.to_string()}
                max={props.max.to_string()}
                value={props.value.to_string()}
                oninput={&props.on_input}
                step={props.step.to_string()}
            />
        </div>
    }
}
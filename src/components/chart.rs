use plotly::{Configuration, Layout, Plot, Scatter};
use plotly::common::Font;
use plotly::layout::Axis;
use web_sys::{js_sys};
use web_sys::wasm_bindgen::JsValue;
use web_sys::wasm_bindgen::prelude::wasm_bindgen;
use yew::{function_component, html, use_effect_with, use_memo, Html, Properties};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Plotly, js_name = newPlot)]
    fn new_plot(id: &str, data: &JsValue);
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub y_data: Vec<f64>,
}

const TRANSPARENT_COLOR: &str = "#00000000";

fn create_layout() -> Layout {
    let x_axis = Axis::new()
        .show_tick_labels(false)
        .show_grid(false)
        .zero_line(false);
    let y_axis = Axis::new()
        .range(vec![0.0, 100.0])
        .title("Association accuracy (%)");
    // TODO: get font/color from CSS?
    let font = Font::new()
        .family("'Courier New', monospace")
        .color("#e0e0e0");
    Layout::new()
        .auto_size(true)
        .x_axis(x_axis)
        .y_axis(y_axis)
        .paper_background_color(TRANSPARENT_COLOR)
        .plot_background_color(TRANSPARENT_COLOR)
        .font(font)
}

fn create_config() -> Configuration {
    Configuration::new().static_plot(true)
}

fn create_plot(y_data: &[f64], layout: &Layout, config: &Configuration) -> Plot {
    let y_data: Vec<f64> = y_data.iter().map(|a| a * 100.0).collect();
    let x_data = (0..y_data.len()).map(|i| i as f64).collect::<Vec<f64>>();
    let trace = Scatter::new(x_data, y_data);

    let mut plot = Plot::new();
    plot.set_layout(layout.clone());
    plot.set_configuration(config.clone());
    plot.add_trace(trace);

    plot
}
#[function_component(Chart)]
pub fn chart(props: &Props) -> Html {
    let id = "plot-div";

    let layout = use_memo((), |_| create_layout());
    let config = use_memo((), |_| create_config());

    use_effect_with(props.y_data.clone(), {
        let layout = layout.clone();
        let config = config.clone();

        move |y_data| {
            let plot = create_plot(y_data, &*layout, &*config);
            let data = js_sys::JSON::parse(&plot.to_json()).unwrap();

            new_plot("plot-div", &data);
            || ()
        }
    });

    html! {
        <div class="chart-wrapper">
            <div class="chart" id={id}></div>
        </div>
    }
}
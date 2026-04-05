use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
use web_sys::wasm_bindgen::JsCast;
use yew::{use_effect_with, use_node_ref, use_state, Callback};
use yew::{function_component, html, Html};
use crate::components::context_menu::{ContextMenu, ContextMenuItem};
use crate::components::sandbox::controls::Controls;
use crate::components::sandbox::model::{Entity, Kind, RenderState, Renderable, SceneState};
use crate::components::sandbox::Position;

fn client_to_canvas(canvas: &HtmlCanvasElement, client_pos: &Position) -> Position {
    let rect = canvas.get_bounding_client_rect();
    let scale_x = canvas.width() as f64 / rect.width();
    let scale_y = canvas.height() as f64 / rect.height();

    Position {
        x: (client_pos.x - rect.left()) * scale_x,
        y: (client_pos.y - rect.top()) * scale_y,
    }
}

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

    let canvas_ref = use_node_ref();

    let context_menu_pos = use_state(|| Option::<Position>::None);
    let on_canvas_context_menu = {
        let context_menu_pos = context_menu_pos.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            context_menu_pos.set(Some(Position{ x: e.client_x() as f64, y: e.client_y() as f64 }))
        })
    };

    let scene = use_state(SceneState::default);
    let id = use_state(|| 0usize);

    let on_add_target = {
        let scene = scene.clone();
        let canvas_ref = canvas_ref.clone();
        let context_menu_pos = context_menu_pos.clone();
        let id = id.clone();
        Callback::from(move |_: MouseEvent| {
            if let (Some(client_pos), Some(canvas)) = (
                *context_menu_pos,
                canvas_ref.cast::<HtmlCanvasElement>()
            ) {
                let canvas_pos = client_to_canvas(&canvas, &client_pos);
                let mut new_scene = (*scene).clone();
                let current_id = *id;
                new_scene.entities.insert(current_id, Entity {
                    id: current_id,
                    position: canvas_pos,
                    kind: Kind::Target,
                });
                scene.set(new_scene);
                id.set(current_id + 1);
                context_menu_pos.set(None);
            }
        })
    };

    use_effect_with((scene.clone(), canvas_ref.clone()), |(scene, canvas_ref)| {
        if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();
            ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
            for (_, entity) in scene.entities.iter() {
                entity.render(&ctx, &RenderState::default(), &SceneState::default());
            }
        }
    });

    html! {
        <>
            <Controls
                std={*std}
                on_std_change={on_std_change}
                iters={*iters}
                on_iters_change={on_iters_change}
            />
            <canvas
                ref={canvas_ref}
                oncontextmenu={on_canvas_context_menu}
            />
            if let Some(pos) = *context_menu_pos {
                <ContextMenu pos={pos}>
                    <ContextMenuItem label={"Clear all"} on_click={Callback::from(|_| ())}/>
                    <ContextMenuItem label={"Add target"} on_click={on_add_target}/>
                </ContextMenu>
            }
        </>
    }
}
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};
use web_sys::wasm_bindgen::JsCast;
use yew::{use_effect_with, use_node_ref, use_reducer, use_state, Callback, NodeRef, Properties, UseReducerHandle, UseStateHandle};
use yew::{function_component, html, Html};
use crate::components::context_menu::{ContextMenu, ContextMenuItem};
use crate::components::sandbox::controls::Controls;
use crate::components::sandbox::model::{Entity, Kind, Renderable, Scene, SceneAction, OBJECT_SIZE};
use crate::components::sandbox::Position;
use crate::sim::monte_carlo;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub accuracies: UseStateHandle<Vec<f64>>,
    pub runtimes: UseStateHandle<Vec<f64>>,
}


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
pub fn sandbox(props: &Props) -> Html {
    let std = use_state(|| 1.0f64);
    let on_std_change = Callback::from({
        let std = std.clone();
        move |val: f64| std.set(val)
    });

    // TODO: iterations f64 -> i64
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

    let scene = use_reducer(Scene::default);
    // TODO: running id -> something else?
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
                let current_id = *id;
                let new_entity = Entity {
                    id: current_id,
                    position: canvas_pos,
                    kind: Kind::Target,
                };
                scene.dispatch(SceneAction::Add(current_id, new_entity));
                id.set(current_id + 1);
                context_menu_pos.set(None);
            }
        })
    };

    let on_add_observer = {
        let scene = scene.clone();
        let canvas_ref = canvas_ref.clone();
        let context_menu_pos = context_menu_pos.clone();
        let id = id.clone();
        let std = std.clone();
        Callback::from(move |_: MouseEvent| {
            if let (Some(client_pos), Some(canvas)) = (
                *context_menu_pos,
                canvas_ref.cast::<HtmlCanvasElement>()
            ) {
                let canvas_pos = client_to_canvas(&canvas, &client_pos);
                let current_id = *id;
                let new_entity = Entity {
                    id: current_id,
                    position: canvas_pos,
                    kind: Kind::Observer { std: *std },
                };
                scene.dispatch(SceneAction::Add(current_id, new_entity));
                id.set(current_id + 1);
                context_menu_pos.set(None);
            }
        })
    };

    let on_clear_all = {
        let scene = scene.clone();
        let context_menu_pos = context_menu_pos.clone();
        Callback::from(move |_: MouseEvent| {
            scene.dispatch(SceneAction::ClearAll);
            context_menu_pos.set(None);
        })
    };

    let accuracies = props.accuracies.clone();
    let runtimes = props.runtimes.clone();

    use_effect_with((scene.clone(), canvas_ref.clone(), iters.clone()), {
        let accuracies = accuracies.clone();
        let runtimes = runtimes.clone();
        move |(scene, canvas_ref, iters): &(UseReducerHandle<Scene>, NodeRef, UseStateHandle<f64>)| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let ctx = canvas
                    .get_context("2d")
                    .unwrap()
                    .unwrap()
                    .dyn_into::<CanvasRenderingContext2d>()
                    .unwrap();
                ctx.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
                for (_, entity) in scene.entities.iter() {
                    entity.render(&ctx, scene);
                }
            }

            let targets: Vec<Entity> = scene.entities.values().filter(|e| matches!(e.kind, Kind::Target)).cloned().collect();
            let observers: Vec<Entity> = scene.entities.values().filter(|e| matches!(e.kind, Kind::Observer { std: _ })).cloned().collect();

            if !targets.is_empty() && !observers.is_empty() {
                let (acc, rt) = monte_carlo(&observers, &targets, **iters as usize);
                let mut new_accuracies = (*accuracies).clone();
                let mut new_runtimes = (*runtimes).clone();
                new_accuracies.push(acc);
                new_runtimes.push(rt);
                if new_accuracies.len() > 50 {
                    new_accuracies.remove(0);
                }
                if new_runtimes.len() > 50 {
                    new_runtimes.remove(0);
                }
                accuracies.set(new_accuracies);
                runtimes.set(new_runtimes);
            }
        }
    });

    let on_mouse_down = {
        let scene = scene.clone();
        let context_menu_pos = context_menu_pos.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(id) = scene.touched {
                scene.dispatch(SceneAction::Select(Some(id)));
            }
            context_menu_pos.set(None);
        })
    };

    let on_mouse_move = {
        let scene = scene.clone();
        let canvas_ref = canvas_ref.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(canvas) = canvas_ref.cast::<HtmlCanvasElement>() {
                let canvas_pos = client_to_canvas(&canvas, &Position { x: e.client_x() as f64, y: e.client_y() as f64 });
                if let Some(id) = scene.selected {
                    scene.dispatch(SceneAction::Move(id, canvas_pos));
                    return;
                }
                let mut touched_id = None;
                for (id, entity) in scene.entities.iter() {
                    if (canvas_pos.x - entity.position.x).abs() < OBJECT_SIZE / 2.0 && (canvas_pos.y - entity.position.y).abs() < OBJECT_SIZE / 2.0 {
                        touched_id = Some(*id);
                        break;
                    }
                }
                scene.dispatch(SceneAction::Touch(touched_id));
            }
        })
    };

    let on_mouse_up = {
        let scene = scene.clone();
        Callback::from(move |_: MouseEvent| {
            scene.dispatch(SceneAction::Select(None));
        })
    };

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
                onmousedown={on_mouse_down}
                onmouseup={on_mouse_up}
                onmousemove={on_mouse_move}
                oncontextmenu={on_canvas_context_menu}
            />
            if let Some(pos) = *context_menu_pos {
                <ContextMenu pos={pos}>
                    <ContextMenuItem label={"Clear all"} on_click={on_clear_all}/>
                    <ContextMenuItem label={"Add target"} on_click={on_add_target}/>
                    <ContextMenuItem label={"Add observer"} on_click={on_add_observer}/>
                </ContextMenu>
            }
        </>
    }
}
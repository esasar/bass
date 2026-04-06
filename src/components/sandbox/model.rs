use std::collections::HashMap;
use std::rc::Rc;
use rand_distr::num_traits::Pow;
use web_sys::js_sys::Math::atan2;
use web_sys::{CanvasRenderingContext2d};
use yew::Reducible;

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, PartialEq)]
pub enum Kind {
    Target {
        accuracy: f64,
    },
    Observer {
        std: f64,
    }
}

#[derive(Clone, PartialEq)]
pub struct Entity {
    pub id: usize,
    pub position: Position,
    pub kind: Kind,
}

pub trait Renderable {
    fn render(&self, ctx: &CanvasRenderingContext2d, scene_state: &Scene);
}

impl Renderable for Entity {
    // TODO: split this func
    fn render(&self, ctx: &CanvasRenderingContext2d, scene: &Scene) {
        let x = self.position.x - OBJECT_SIZE / 2.0;
        let y = self.position.y - OBJECT_SIZE / 2.0;

        let touched = Some(self.id) == scene.touched;
        let selected = scene.selected.contains(&self.id);

        if selected {
            ctx.set_stroke_style_str("red");
            ctx.set_line_width(2.0);
            ctx.stroke_rect(x - 2.0, y - 2.0, OBJECT_SIZE + 4.0, OBJECT_SIZE + 4.0);
        }

        match &self.kind {
            Kind::Target { accuracy }=> {
                // TODO: add colours to a config or css
                let color = if touched {
                    "red"
                } else {
                    let c = accuracy.pow(5);
                    let v = c * 255.0;
                    &format!("rgb({v}, {v}, {v})")
                };
                ctx.set_fill_style_str(color);
                ctx.fill_rect(x, y, OBJECT_SIZE, OBJECT_SIZE);
            }
            Kind::Observer { std } => {
                // TODO: add colours to a config or css
                let color = if touched{ "red" } else { OBSERVER_COLOR };
                ctx.set_fill_style_str(color);
                ctx.fill_rect(x, y, OBJECT_SIZE, OBJECT_SIZE);

                let targets = scene.entities.iter().filter(|(_, e)| matches!(e.kind, Kind::Target { .. }));
                for (_, t) in targets {
                    let dx = t.position.x - self.position.x;
                    let dy = t.position.y - self.position.y;
                    let az = atan2(dy, dx);

                    let left = az - std.to_radians();
                    let right = az + std.to_radians();

                    let left_x = self.position.x + left.cos() * FAR;
                    let left_y = self.position.y + left.sin() * FAR;

                    let right_x = self.position.x + right.cos() * FAR;
                    let right_y = self.position.y + right.sin() * FAR;

                    ctx.begin_path();
                    ctx.move_to(self.position.x, self.position.y);
                    ctx.line_to(left_x, left_y);
                    ctx.line_to(right_x, right_y);
                    ctx.close_path();
                    // TODO: add colours to a config or css
                    ctx.set_fill_style_str(BEAM_COLOR);
                    ctx.fill();
                }
            }
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct Scene {
    pub entities: HashMap<usize, Entity>,
    pub touched: Option<usize>,
    pub dragging: Option<usize>,
    pub selection_box: Option<(Position, Position)>,
    pub selected: Vec<usize>,
}

pub enum SceneAction {
    Add(usize, Entity),
    Remove(usize),
    ClearAll,
    Touch(Option<usize>),
    Select(Vec<usize>),
    Move(usize, Position),
    AdjustStd(usize, f64),
    StartDrag(usize),
    EndDrag,
    StartSelectionBox(Position),
    UpdateSelectionBox(Position),
    EndSelectionBox,
    SetEntities(HashMap<usize, Entity>),
}

fn in_selection(pos: Position, a: Position, b: Position) -> bool {
    let min_x = a.x.min(b.x);
    let max_x = a.x.max(b.x);
    let min_y = a.y.min(b.y);
    let max_y = a.y.max(b.y);

    pos.x >= min_x && pos.x <= max_x && pos.y >= min_y && pos.y <= max_y
}

impl Reducible for Scene {
    type Action = SceneAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            SceneAction::Add(id, entity) => {
                let mut entities = self.entities.clone();
                entities.insert(id, entity);
                Rc::new(Scene { entities, ..(*self).clone() })
            }
            SceneAction::Remove(id) => {
                let mut entities = self.entities.clone();
                entities.remove(&id);
                Rc::new(Scene { entities, ..(*self).clone() })
            }
            SceneAction::ClearAll => Rc::new(Scene {
                entities: HashMap::new(),
                ..(*self).clone()
            }),
            SceneAction::Touch(id) => Rc::new(Scene {
                touched: id,
                ..(*self).clone()
            }),
            SceneAction::Select(ids) => Rc::new(Scene {
                selected: ids,
                ..(*self).clone()
            }),
            SceneAction::Move(id, pos) => {
                let mut entities = self.entities.clone();
                if let Some(entity) = entities.get_mut(&id) {
                    entity.position = pos;
                }
                Rc::new(Scene {
                    entities,
                    ..(*self).clone()
                })
            }
            SceneAction::AdjustStd(id, new_std) => {
                let mut entities = self.entities.clone();
                if let Some(entity) = entities.get_mut(&id) {
                    if let Kind::Observer { ref mut std } = entity.kind {
                        *std = new_std;
                    }
                }
                Rc::new(Scene {
                    entities,
                    ..(*self).clone()
                })
            }
            SceneAction::StartDrag(id) => Rc::new(Scene {
                dragging: Some(id),
                ..(*self).clone()
            }),
            SceneAction::EndDrag => Rc::new(Scene {
                dragging: None,
                ..(*self).clone()
            }),
            SceneAction::StartSelectionBox(pos) => Rc::new(Scene {
               selection_box: Some((pos, pos)),
                ..(*self).clone()
            }),
            SceneAction::UpdateSelectionBox(pos) => {
                if let Some((anchor, _)) = self.selection_box {
                    let selected = self.entities.values()
                        .filter(|e| in_selection(e.position, anchor, pos))
                        .map(|e| e.id)
                        .collect();
                    Rc::new(Scene {
                        selection_box: Some((anchor, pos)),
                        selected,
                        ..(*self).clone()
                    })
                } else {
                    Rc::new((*self).clone())
                }
            },
            SceneAction::EndSelectionBox => Rc::new(Scene {
                selection_box: None,
                ..(*self).clone()
            }),
            SceneAction::SetEntities(entities) => Rc::new(Scene {
                entities,
                ..(*self).clone()
            })
        }
    }
}

pub const OBJECT_SIZE: f64 = 10.0;
pub const FAR: f64 = 10000.0;
pub const TARGET_COLOR: &str = "#e05c5c";
pub const OBSERVER_COLOR: &str = "#7eb8c9";
pub const BEAM_COLOR: &str = "rgba(255, 200, 80, 0.15)";
pub const TOUCH_HIT_SIZE: f64 = 48.0;
use std::collections::HashMap;
use std::rc::Rc;
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
    Target,
    Observer {
        std: f64
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
        let selected = Some(self.id) == scene.selected;

        if selected {
            ctx.set_stroke_style_str("red");
            ctx.set_line_width(2.0);
            ctx.stroke_rect(x - 2.0, y - 2.0, OBJECT_SIZE + 4.0, OBJECT_SIZE + 4.0);
        }

        match &self.kind {
            Kind::Target => {
                // TODO: add colours to a config or css
                let color = if touched { "red" } else { TARGET_COLOR };
                ctx.set_fill_style_str(color);
                ctx.fill_rect(x, y, OBJECT_SIZE, OBJECT_SIZE);
            }
            Kind::Observer { std } => {
                // TODO: add colours to a config or css
                let color = if touched{ "red" } else { OBSERVER_COLOR };
                ctx.set_fill_style_str(color);
                ctx.fill_rect(x, y, OBJECT_SIZE, OBJECT_SIZE);

                let targets = scene.entities.iter().filter(|(_, e)| matches!(e.kind, Kind::Target));
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
    pub selected: Option<usize>,
}

pub enum SceneAction {
    Add(usize, Entity),
    Remove(usize),
    ClearAll,
    Touch(Option<usize>),
    Select(Option<usize>),
    Move(usize, Position),
    AdjustStd(usize, f64),
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
            SceneAction::Select(id) => Rc::new(Scene {
                selected: id,
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
        }
    }
}

pub const OBJECT_SIZE: f64 = 10.0;
pub const FAR: f64 = 10000.0;
pub const TARGET_COLOR: &str = "#e05c5c";
pub const OBSERVER_COLOR: &str = "#7eb8c9";
pub const BEAM_COLOR: &str = "rgba(255, 200, 80, 0.15)";
pub const TOUCH_HIT_SIZE: f64 = 48.0;
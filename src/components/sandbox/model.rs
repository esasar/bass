use std::collections::HashMap;
use std::rc::Rc;
use web_sys::js_sys::Math::atan2;
use web_sys::{console, CanvasRenderingContext2d};
use yew::Reducible;
use crate::components::sandbox::model::Kind::Observer;

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
        match &self.kind {
            Kind::Target => {
                // TODO: add colours to a config or css
                let color = if Some(self.id) == scene.touched { "blue" } else { "red" };
                ctx.set_fill_style_str(color);
                ctx.fill_rect(self.position.x - OBJECT_SIZE / 2.0, self.position.y - OBJECT_SIZE / 2.0, OBJECT_SIZE, OBJECT_SIZE);
            }
            Kind::Observer { std } => {
                // TODO: add colours to a config or css
                let color = if Some(self.id) == scene.touched { "blue" } else { "green" };
                ctx.set_fill_style_str(color);
                ctx.fill_rect(self.position.x - OBJECT_SIZE / 2.0, self.position.y - OBJECT_SIZE / 2.0, OBJECT_SIZE, OBJECT_SIZE);

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
                    ctx.set_fill_style_str("rgba(255, 0, 0, 0.1)");
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
            // TODO: move entity
        }
    }
}

pub const OBJECT_SIZE: f64 = 10.0;
pub const FAR: f64 = 10000.0;
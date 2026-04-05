use std::collections::HashMap;
use std::rc::Rc;
use web_sys::js_sys::Math::atan2;
use web_sys::CanvasRenderingContext2d;
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

#[derive(Clone, PartialEq, Default)]
pub struct RenderState {
    pub selected: bool,
    pub touched: bool,
}

pub struct EntityView<'a> {
    pub entity: &'a Entity,
    pub render_state: &'a RenderState,
}

pub trait Renderable {
    fn render(&self, ctx: &CanvasRenderingContext2d, render_state: &RenderState, scene_state: &Scene);
}

impl Renderable for Entity {
    fn render(&self, ctx: &CanvasRenderingContext2d, render_state: &RenderState, scene_state: &Scene) {
        match &self.kind {
            Kind::Target => {
                ctx.set_fill_style_str("red");
                ctx.fill_rect(self.position.x - OBJECT_SIZE / 2.0, self.position.y - OBJECT_SIZE / 2.0, OBJECT_SIZE, OBJECT_SIZE);
            }
            Kind::Observer { std } => {
                ctx.set_fill_style_str("green");
                ctx.fill_rect(self.position.x - OBJECT_SIZE / 2.0, self.position.y - OBJECT_SIZE / 2.0, OBJECT_SIZE, OBJECT_SIZE);

                let targets = scene_state.entities.iter().filter(|(_, e)| matches!(e.kind, Kind::Target));
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
                    ctx.set_fill_style_str("red");
                    ctx.fill();
                }
            }
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct Scene {
    pub entities: HashMap<usize, Entity>,
    pub render_states: HashMap<usize, RenderState>,
}

pub enum SceneAction {
    Add(usize, Entity),
    Remove(usize),
    ClearAll,
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
            })
        }
    }
}

pub const OBJECT_SIZE: f64 = 10.0;
pub const FAR: f64 = 10000.0;
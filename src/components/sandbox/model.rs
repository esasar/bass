use std::collections::HashMap;
use web_sys::js_sys::Math::atan2;
use web_sys::CanvasRenderingContext2d;

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
    fn render(&self, ctx: &CanvasRenderingContext2d, render_state: &RenderState, scene_state: &SceneState);
}

impl Renderable for Entity {
    fn render(&self, ctx: &CanvasRenderingContext2d, render_state: &RenderState, scene_state: &SceneState) {
        let color = if render_state.selected {
            "green"
        } else if render_state.touched {
            "orange"
        } else {
            "black"
        };

        match &self.kind {
            Kind::Target => {
                ctx.set_fill_style_str(color);
                ctx.fill_rect(self.position.x - OBJECT_SIZE / 2.0, self.position.y - OBJECT_SIZE / 2.0, OBJECT_SIZE, OBJECT_SIZE);
            }
            Kind::Observer { std } => {
                ctx.set_fill_style_str(color);
                ctx.fill_rect(self.position.x - OBJECT_SIZE / 2.0, self.position.y - OBJECT_SIZE / 2.0, OBJECT_SIZE, OBJECT_SIZE);

                let targets = scene_state.entities.iter().filter(|(_, e)| matches!(e.kind, Kind::Target));
                for (_, t) in targets {
                    let dx = t.position.x - self.position.x;
                    let dy = t.position.y - self.position.y;
                    let az = atan2(dy, dx);

                    let left = az - std;
                    let right = az + std;

                    let left_x = self.position.x + left.cos() * FAR;
                    let left_y = self.position.y + left.sin() * FAR;

                    let right_x = self.position.x + right.cos() * FAR;
                    let right_y = self.position.y + right.sin() * FAR;

                    ctx.begin_path();
                    ctx.move_to(self.position.x, self.position.y);
                    ctx.line_to(left_x, left_y);
                    ctx.line_to(right_x, right_y);
                    ctx.close_path();
                    ctx.set_fill_style_str("rgba(255, 0, 0, 0.1)");
                    ctx.fill();
                }
            }
        }
    }
}

#[derive(Default, Clone, PartialEq)]
pub struct SceneState {
    pub entities: HashMap<usize, Entity>,
    pub render_states: HashMap<usize, RenderState>,
}

pub const OBJECT_SIZE: f64 = 10.0;
pub const FAR: f64 = 10000.0;
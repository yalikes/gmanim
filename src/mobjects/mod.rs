pub trait Mobject: Rotate + SimpleMove + Draw {}

use crate::{Color, Context, ContextType, GMFloat, SceneConfig};

use nalgebra::{point, Vector3};
pub mod svg_shape;
pub mod text;
pub mod formula;

pub trait SimpleMove {
    fn move_this(&mut self, movement: Vector3<GMFloat>) {}
}

pub trait Rotate {
    fn rotate(&mut self, axis: Vector3<GMFloat>, value: f32);
}

pub trait Draw {
    //draw shape without fill()
    fn draw(&self, ctx: &mut Context);
}

#[derive(Debug, Clone, Copy)]
pub struct DrawConfig {
    stoke_width: GMFloat,
    fill: bool,
    color: Color,
}

impl Default for DrawConfig {
    fn default() -> Self {
        DrawConfig {
            stoke_width: 0.25,
            fill: true,
            color: Default::default(),
        }
    }
}



pub struct Rectangle {
    pub p0: Vector3<GMFloat>,
    pub p1: Vector3<GMFloat>,
    pub p2: Vector3<GMFloat>,
    pub p3: Vector3<GMFloat>,
    pub draw_config: DrawConfig,
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            p0: Vector3::new(0.0, 0.0, 0.0),
            p1: Vector3::new(1.0, 0.0, 0.0),
            p2: Vector3::new(1.0, 1.0, 0.0),
            p3: Vector3::new(0.0, 1.0, 0.0),
            draw_config: DrawConfig::default(),
        }
    }
}

impl SimpleMove for Rectangle {
    fn move_this(&mut self, movement: Vector3<GMFloat>) {
        self.p0 = self.p0.clone() + movement.clone();
        self.p1 = self.p1.clone() + movement.clone();
        self.p2 = self.p2.clone() + movement.clone();
        self.p3 = self.p3.clone() + movement;
    }
}

impl Rotate for Rectangle {
    fn rotate(&mut self, axis: Vector3<GMFloat>, value: GMFloat) {}
}

impl Draw for Rectangle {
    fn draw(self: &Self, ctx: &mut Context) {
        match &mut ctx.ctx_type {
            ContextType::Raqote(dt) => {
                let scale_factor = ctx.scene_config.scale_factor;
                let mut pb = raqote::PathBuilder::new();
                let p0 = (
                    coordinate_change_x(self.p0[(0)], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.p0[(1)], ctx.scene_config.height) * scale_factor,
                );
                let p1 = (
                    coordinate_change_x(self.p1[(0)], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.p1[(1)], ctx.scene_config.height) * scale_factor,
                );
                let p2 = (
                    coordinate_change_x(self.p2[(0)], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.p2[(1)], ctx.scene_config.height) * scale_factor,
                );
                let p3 = (
                    coordinate_change_x(self.p3[(0)], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.p3[(1)], ctx.scene_config.height) * scale_factor,
                );
                pb.move_to(p0.0, p0.1);
                pb.line_to(p1.0, p1.1);
                pb.line_to(p2.0, p2.1);
                pb.line_to(p3.0, p3.1);
                pb.line_to(p0.0, p0.1);
                let path = pb.finish();
                dt.stroke(
                    &path,
                    &raqote::Source::Solid(self.draw_config.color.into()),
                    &raqote::StrokeStyle {
                        cap: raqote::LineCap::Round,
                        join: raqote::LineJoin::Round,
                        width: self.draw_config.stoke_width * scale_factor,
                        ..Default::default()
                    },
                    &raqote::DrawOptions::new(),
                );
            }
            _ => {}
        }
    }
}

impl Mobject for Rectangle {}



pub struct SimpleLine {
    pub p0: Vector3<GMFloat>,
    pub p1: Vector3<GMFloat>,
    pub draw_config: DrawConfig,
}

impl Default for SimpleLine {
    fn default() -> Self {
        SimpleLine {
            p0: Vector3::new(0.0, 0.0, 0.0),
            p1: Vector3::new(1.0, 0.0, 0.0),
            draw_config: DrawConfig::default(),
        }
    }
}

impl SimpleMove for SimpleLine {
    fn move_this(&mut self, movement: Vector3<GMFloat>) {
        self.p0 = self.p0.clone() + movement.clone();
        self.p1 = self.p1.clone() + movement.clone();
    }
}

impl Rotate for SimpleLine {
    fn rotate(&mut self, axis: Vector3<GMFloat>, value: f32) {}
}

impl Draw for SimpleLine {
    fn draw(self: &Self, ctx: &mut Context) {
        let scale_factor = ctx.scene_config.scale_factor;
        match &mut ctx.ctx_type {
            ContextType::Raqote(dt) => {
                let mut pb = raqote::PathBuilder::new();
                let p0 = (
                    coordinate_change_x(self.p0[(0)], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.p0[(1)], ctx.scene_config.height) * scale_factor,
                );
                let p1 = (
                    coordinate_change_x(self.p1[(0)], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.p1[(1)], ctx.scene_config.height) * scale_factor,
                );
                pb.move_to(p0.0, p0.1);
                pb.line_to(p1.0, p1.1);
                let path = pb.finish();
                dt.stroke(
                    &path,
                    &raqote::Source::Solid(self.draw_config.color.into()),
                    &raqote::StrokeStyle {
                        cap: raqote::LineCap::Round,
                        join: raqote::LineJoin::Round,
                        width: self.draw_config.stoke_width * scale_factor,
                        ..Default::default()
                    },
                    &raqote::DrawOptions::new(),
                );
            }
            _ => {}
        }
    }
}

impl Mobject for SimpleLine {}



pub struct PolyLine {
    pub points: Vec<Vector3<GMFloat>>,
    pub draw_config: DrawConfig,
}

impl Default for PolyLine {
    fn default() -> Self {
        PolyLine {
            points: vec![Vector3::new(0.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0)],
            draw_config: DrawConfig::default(),
        }
    }
}

impl SimpleMove for PolyLine {
    fn move_this(&mut self, movement: Vector3<GMFloat>) {
        for i in 0..self.points.len() {
            self.points[i] = self.points[i].clone() + movement.clone();
        }
    }
}

impl Rotate for PolyLine {
    fn rotate(&mut self, axis: Vector3<GMFloat>, value: GMFloat) {}
}

impl Draw for PolyLine {
    fn draw(self: &Self, ctx: &mut Context) {
        if self.points.len() < 2 {
            return;
        }

        let scale_factor = ctx.scene_config.scale_factor;

        match &mut ctx.ctx_type {
            ContextType::Raqote(dt) => {
                let mut pb = raqote::PathBuilder::new();
                let p0 = (
                    coordinate_change_x(self.points[0][(0)], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.points[0][(1)], ctx.scene_config.height)
                        * scale_factor,
                );
                pb.move_to(p0.0, p0.1);
                for p in self.points[1..].iter() {
                    let point = (
                        coordinate_change_x(p[(0)], ctx.scene_config.width) * scale_factor,
                        coordinate_change_y(p[(1)], ctx.scene_config.height) * scale_factor,
                    );
                    pb.line_to(point.0, point.1);
                }
                let path = pb.finish();
                dt.stroke(
                    &path,
                    &raqote::Source::Solid(self.draw_config.color.into()),
                    &raqote::StrokeStyle {
                        cap: raqote::LineCap::Round,
                        join: raqote::LineJoin::Round,
                        width: self.draw_config.stoke_width * scale_factor,
                        ..Default::default()
                    },
                    &raqote::DrawOptions::new(),
                );
            }
            _ => {}
        }
    }
}

impl Mobject for PolyLine {}



pub fn rotate_matrix(axis: Vector3<GMFloat>, theta: GMFloat) {
    //assume axis is a unit vector
}

#[inline]
pub fn coordinate_change_x(position_x: f32, scene_width: f32) -> f32 {
    scene_width / 2.0 + position_x
}

#[inline]
pub fn coordinate_change_y(position_y: f32, scene_height: f32) -> f32 {
    scene_height / 2.0 - position_y
}
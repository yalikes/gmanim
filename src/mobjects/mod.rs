pub trait Mobject: Rotate + SimpleMove + Draw {}

use crate::{Context, ContextType, SceneConfig, GMFloat};

use nalgebra::Vector3;

pub trait Draw {
    //draw shape without fill()
    fn draw(&self, ctx: &mut Context);
}

impl Rotate for PolyLine {
    fn rotate(&mut self, axis: Vector3<GMFloat>, value: GMFloat) {}
}

impl Rotate for Rectangle {
    fn rotate(&mut self, axis: Vector3<GMFloat>, value: GMFloat) {}
}

impl Mobject for PolyLine {}

impl Mobject for Rectangle {}



pub trait SimpleMove {
    fn move_this(&mut self, movement: Vector3<GMFloat>) {}
}

impl SimpleMove for SimpleLine {
    fn move_this(&mut self, movement: Vector3<GMFloat>) {
        self.p0 = self.p0.clone() + movement.clone();
        self.p1 = self.p1.clone() + movement.clone();
    }
}

impl SimpleMove for PolyLine {
    fn move_this(&mut self, movement: Vector3<GMFloat>) {
        for i in 0..self.points.len() {
            self.points[i] = self.points[i].clone() + movement.clone();
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

pub fn rotate_matrix(axis: Vector3<GMFloat>, theta: GMFloat){
    //assume axis is a unit vector
}
pub trait Rotate {
    fn rotate(&mut self, axis: Vector3<GMFloat>, value: f32);
}

impl Rotate for SimpleLine {
    fn rotate(&mut self, axis: Vector3<GMFloat>, value: f32) {}
}


impl Mobject for SimpleLine {}

pub struct PolyLine {
    pub stroke_width: f32,
    pub points: Vec<Vector3<GMFloat>>,
}

pub struct SimpleLine {
    pub stroke_width: f32,
    pub p0: Vector3<GMFloat>,
    pub p1: Vector3<GMFloat>,
}

pub struct Rectangle {
    pub stroke_width: f32,
    pub p0: Vector3<GMFloat>,
    pub p1: Vector3<GMFloat>,
    pub p2: Vector3<GMFloat>,
    pub p3: Vector3<GMFloat>,
}

#[inline]
pub fn coordinate_change_x(position_x: f32, scene_width: f32) -> f32 {
    scene_width / 2.0 + position_x
}

#[inline]
pub fn coordinate_change_y(position_y: f32, scene_height: f32) -> f32 {
    scene_height / 2.0 - position_y
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
                    &raqote::Source::Solid(raqote::SolidSource {
                        r: 0x0,
                        g: 0x0,
                        b: 0x80,
                        a: 0x80,
                    }),
                    &raqote::StrokeStyle {
                        cap: raqote::LineCap::Round,
                        join: raqote::LineJoin::Round,
                        width: self.stroke_width * scale_factor,
                        ..Default::default()
                    },
                    &raqote::DrawOptions::new(),
                );
            }
            _ => {}
        }
    }
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
                    &raqote::Source::Solid(raqote::SolidSource {
                        r: 0x0,
                        g: 0x0,
                        b: 0x80,
                        a: 0x80,
                    }),
                    &raqote::StrokeStyle {
                        cap: raqote::LineCap::Round,
                        join: raqote::LineJoin::Round,
                        width: self.stroke_width * scale_factor,
                        ..Default::default()
                    },
                    &raqote::DrawOptions::new(),
                );
            }
            _ => {}
        }
    }
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
                    &raqote::Source::Solid(raqote::SolidSource {
                        r: 0xff,
                        g: 0x0,
                        b: 0x0,
                        a: 0xff,
                    }),
                    &raqote::StrokeStyle {
                        cap: raqote::LineCap::Round,
                        join: raqote::LineJoin::Round,
                        width: self.stroke_width * scale_factor,
                        ..Default::default()
                    },
                    &raqote::DrawOptions::new(),
                );
            }
            _ => {}
        }
    }
}

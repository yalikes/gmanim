pub trait Mobject: Transform + Draw {}

use crate::{Color, Context, ContextType, GMFloat, SceneConfig};

use nalgebra::{point, Vector3, Point3, Point};
use tiny_skia::{LineCap, LineJoin, Paint, Stroke, StrokeDash};
pub mod formula;
pub mod group;
pub mod path;
pub mod svg_shape;
pub mod text;

pub trait Transform {
    fn transform(&mut self, transform: nalgebra::Transform3<GMFloat>) {}
}
pub trait SimpleMove {
    fn move_this(&mut self, movement: Vector3<GMFloat>) {}
}

pub trait Rotate {
    fn rotate(&mut self, axis: Vector3<GMFloat>, value: GMFloat);
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
    pub p0: Point3<GMFloat>,
    pub p1: Point3<GMFloat>,
    pub p2: Point3<GMFloat>,
    pub p3: Point3<GMFloat>,
    pub draw_config: DrawConfig,
}

impl Default for Rectangle {
    fn default() -> Self {
        Rectangle {
            p0: Point3::new(0.0, 0.0, 0.0),
            p1: Point3::new(1.0, 0.0, 0.0),
            p2: Point3::new(1.0, 1.0, 0.0),
            p3: Point3::new(0.0, 1.0, 0.0),
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

impl Transform for Rectangle {
    fn transform(&mut self, transform: nalgebra::Transform3<GMFloat>) {
        self.p0 = transform * self.p0;
        self.p1 = transform * self.p1;
        self.p2 = transform * self.p2;
        self.p3 = transform * self.p3;
    }
}

impl Draw for Rectangle {
    fn draw(self: &Self, ctx: &mut Context) {
        match &mut ctx.ctx_type {
            ContextType::TinySKIA(pixmap) => {
                let scale_factor = ctx.scene_config.scale_factor;
                let mut pb = tiny_skia::PathBuilder::new();
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
                pb.move_to(p0.0 as f32, p0.1 as f32);
                pb.line_to(p1.0 as f32, p1.1 as f32);
                pb.line_to(p2.0 as f32, p2.1 as f32);
                pb.line_to(p3.0 as f32, p3.1 as f32);
                pb.line_to(p0.0 as f32, p0.1 as f32);
                let path = pb.finish().unwrap();

                let mut stroke = Stroke::default();
                stroke.width = self.draw_config.stoke_width * scale_factor;
                stroke.line_cap = LineCap::Round;
                stroke.line_join = LineJoin::Round;
                let mut paint = Paint::default();
                paint.set_color(self.draw_config.color.into());

                paint.anti_alias = true;
                pixmap.stroke_path(
                    &path,
                    &paint,
                    &stroke,
                    tiny_skia::Transform::identity(),
                    None,
                );
            }
            _ => {}
        }
    }
}

impl Mobject for Rectangle {}

pub struct SimpleLine {
    pub p0: Point3<GMFloat>,
    pub p1: Point3<GMFloat>,
    pub draw_config: DrawConfig,
}

impl Default for SimpleLine {
    fn default() -> Self {
        SimpleLine {
            p0: Point3::new(0.0, 0.0, 0.0),
            p1: Point3::new(1.0, 0.0, 0.0),
            draw_config: DrawConfig::default(),
        }
    }
}


impl Transform for SimpleLine {
    fn transform(&mut self, transform: nalgebra::Transform3<GMFloat>) {
        
    }
}

impl Draw for SimpleLine {
    fn draw(self: &Self, ctx: &mut Context) {
        let scale_factor = ctx.scene_config.scale_factor;
        match &mut ctx.ctx_type {
            ContextType::TinySKIA(pixmap) => {
                let mut pb = tiny_skia::PathBuilder::new();
                let p0 = (
                    coordinate_change_x(self.p0[(0)], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.p0[(1)], ctx.scene_config.height) * scale_factor,
                );
                let p1 = (
                    coordinate_change_x(self.p1[(0)], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.p1[(1)], ctx.scene_config.height) * scale_factor,
                );
                pb.move_to(p0.0 as f32, p0.1 as f32);
                pb.line_to(p1.0 as f32, p1.1 as f32);
                let path = pb.finish().unwrap();

                let mut stroke = Stroke::default();
                stroke.width = self.draw_config.stoke_width * scale_factor;
                stroke.line_cap = LineCap::Round;
                stroke.line_join = LineJoin::Round;
                let mut paint = Paint::default();
                paint.set_color(self.draw_config.color.into());

                pixmap.stroke_path(
                    &path,
                    &paint,
                    &stroke,
                    tiny_skia::Transform::identity(),
                    None,
                );
            }
            _ => {}
        }
    }
}

impl Mobject for SimpleLine {}

pub struct PolyLine {
    pub points: Vec<Point3<GMFloat>>,
    pub draw_config: DrawConfig,
}

impl Default for PolyLine {
    fn default() -> Self {
        PolyLine {
            points: vec![Point3::new(0.0, 0.0, 0.0), Point3::new(0.0, 0.0, 0.0)],
            draw_config: DrawConfig::default(),
        }
    }
}


impl Transform for PolyLine {
    fn transform(&mut self, transform: nalgebra::Transform3<GMFloat>) {
        
    }
}

impl Draw for PolyLine {
    fn draw(self: &Self, ctx: &mut Context) {
        if self.points.len() < 2 {
            return;
        }

        let scale_factor = ctx.scene_config.scale_factor;

        match &mut ctx.ctx_type {
            ContextType::TinySKIA(pixmap) => {
                let mut pb = tiny_skia::PathBuilder::new();
                let p0 = (
                    coordinate_change_x(self.points[0][(0)], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.points[0][(1)], ctx.scene_config.height)
                        * scale_factor,
                );
                pb.move_to(p0.0 as f32, p0.1 as f32);
                for p in self.points[1..].iter() {
                    let point = (
                        coordinate_change_x(p[(0)], ctx.scene_config.width) * scale_factor,
                        coordinate_change_y(p[(1)], ctx.scene_config.height) * scale_factor,
                    );
                    pb.line_to(point.0 as f32, point.1 as f32);
                }
                let path = pb.finish().unwrap();

                let mut stroke = Stroke::default();
                stroke.width = self.draw_config.stoke_width * scale_factor;
                stroke.line_cap = LineCap::Round;
                stroke.line_join = LineJoin::Round;

                let mut paint = Paint::default();
                paint.set_color(self.draw_config.color.into());

                pixmap.stroke_path(
                    &path,
                    &paint,
                    &stroke,
                    tiny_skia::Transform::identity(),
                    None,
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
pub fn coordinate_change_x(position_x: GMFloat, scene_width: GMFloat) -> GMFloat {
    scene_width / 2.0 + position_x
}

#[inline]
pub fn coordinate_change_y(position_y: GMFloat, scene_height: GMFloat) -> GMFloat {
    scene_height / 2.0 - position_y
}

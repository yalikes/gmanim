#![allow(unused)]

mod video_backend;
struct Color {
    r: f32,
    g: f32,
    b: f32,
}
pub enum ContextType {
    Raqote(raqote::DrawTarget), // we always have cairo as a fallback
    VULKAN,
    CUDA,
    HIP,
}

pub struct SceneConfig {
    width: f32,
    height: f32,
    output_width: u32,
    output_height: u32,
    scale_factor: f32,
}

pub struct Context {
    ctx_type: ContextType,
    scene_config: SceneConfig,
}

impl Default for SceneConfig {
    fn default() -> Self {
        SceneConfig {
            width: 16.0,
            height: 9.0,
            output_width: 1920,
            output_height: 1080,
            scale_factor: 1920.0 / 16.0,
        }
    }
}

trait SimpleMove {
    fn move_this(&mut self, movement: ndarray::Array1<f32>) {}
}

impl SimpleMove for SimpleLine {
    fn move_this(&mut self, movement: ndarray::Array1<f32>) {
        self.p0 = self.p0.clone() + movement.clone();
        self.p1 = self.p1.clone() + movement.clone();
    }
}

impl SimpleMove for PolyLine {
    fn move_this(&mut self, movement: ndarray::Array1<f32>) {
        for i in 0..self.points.len() {
            self.points[i] = self.points[i].clone() + movement.clone();
        }
    }
}

impl SimpleMove for Rectangle {
    fn move_this(&mut self, movement: ndarray::Array1<f32>) {
        self.position = self.position.clone() + movement;
    }
}
trait Rotate {
    fn rotate(&mut self, axis: ndarray::Array1<f32>, value: f32);
}

impl Rotate for SimpleLine {
    fn rotate(&mut self, axis: ndarray::Array1<f32>, value: f32) {}
}

trait Mobject: Rotate + SimpleMove + Draw {}

impl Mobject for SimpleLine {}

#[derive(Default)]
struct Scene {
    mobjects: Vec<Box<dyn Mobject>>,
}

impl Scene {
    fn new() -> Self {
        Scene { mobjects: vec![] }
    }
    fn save_png(&self, ctx: &mut Context, file_path: &str) {
        ctx.clear_transparent();

        for m in self.mobjects.iter() {
            m.draw(ctx);
        }

        match &mut ctx.ctx_type {
            ContextType::Raqote(dt) => {
                dt.write_png(file_path);
            }
            _ => {}
        }
    }

    fn add(&mut self, mobject: Box<dyn Mobject>) {
        self.mobjects.push(mobject);
    }
}

impl Default for Context {
    fn default() -> Self {
        let scene_config = SceneConfig::default();
        let dt = raqote::DrawTarget::new(
            scene_config.output_width as i32,
            scene_config.output_height as i32,
        );
        Self {
            ctx_type: ContextType::Raqote(dt),
            scene_config,
        }
    }
}

impl Context {
    fn clear_transparent(&mut self) {
        match &mut self.ctx_type {
            ContextType::Raqote(dt) => {
                dt.clear(raqote::SolidSource {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                });
            }
            _ => {}
        }
    }

    fn image_bytes(&self) -> &[u8] {
        match &self.ctx_type {
            ContextType::Raqote(dt) => dt.get_data_u8(),
            _ => &[],
        }
    }
}
pub trait Draw {
    //draw shape without fill()
    fn draw(&self, ctx: &mut Context);
}

struct PolyLine {
    stroke_width: f32,
    points: Vec<ndarray::Array1<f32>>,
}

struct SimpleLine {
    stroke_width: f32,
    p0: ndarray::Array1<f32>,
    p1: ndarray::Array1<f32>,
}

struct Rectangle {
    stroke_width: f32,
    position: ndarray::Array1<f32>,
    width: f32,
    height: f32,
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
                    coordinate_change_x(self.p0[[0]], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.p0[[1]], ctx.scene_config.height) * scale_factor,
                );
                let p1 = (
                    coordinate_change_x(self.p1[[0]], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.p1[[1]], ctx.scene_config.height) * scale_factor,
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
                    coordinate_change_x(self.points[0][[0]], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.points[0][[1]], ctx.scene_config.height)
                        * scale_factor,
                );
                pb.move_to(p0.0, p0.1);
                for p in self.points[1..].iter() {
                    let point = (
                        coordinate_change_x(p[[0]], ctx.scene_config.width) * scale_factor,
                        coordinate_change_y(p[[1]], ctx.scene_config.height) * scale_factor,
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
                    coordinate_change_x(self.position[[0]], ctx.scene_config.width) * scale_factor,
                    coordinate_change_y(self.position[[1]], ctx.scene_config.height) * scale_factor,
                );
                let p1 = (
                    coordinate_change_x(self.position[[0]] + self.width, ctx.scene_config.width)
                        * scale_factor,
                    p0.1,
                );
                let p2 = (
                    p1.0,
                    coordinate_change_y(self.position[[1]] + self.height, ctx.scene_config.height)
                        * scale_factor,
                );
                let p3 = (p0.0, p2.1);
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

impl Rotate for PolyLine {
    fn rotate(&mut self, axis: ndarray::Array1<f32>, value: f32) {}
}

impl Rotate for Rectangle {
    fn rotate(&mut self, axis: ndarray::Array1<f32>, value: f32) {}
}

impl Mobject for PolyLine {}

impl Mobject for Rectangle {}
#[test]
fn test_simple_line_image() {
    let mut ctx = Context::default();
    let mut scene = Scene::new();
    let simple_line = SimpleLine {
        stroke_width: 0.2,
        p0: ndarray::arr1(&[0.0, 0.0, 0.0]),
        p1: ndarray::arr1(&[1.0, 1.0, 0.0]),
    };
    let simple_line2 = SimpleLine {
        stroke_width: 0.2,
        p0: ndarray::arr1(&[1.0, 1.0, 0.0]),
        p1: ndarray::arr1(&[5.0, 2.0, 0.0]),
    };
    scene.add(Box::new(simple_line));
    scene.add(Box::new(simple_line2));
    scene.save_png(&mut ctx, "simple_line.png");
}

#[test]
fn test_polyline_image() {
    let mut ctx = Context::default();
    let mut scene = Scene::new();
    let polyline = PolyLine {
        stroke_width: 0.2,
        points: vec![
            ndarray::arr1(&[0.0, 0.0]),
            ndarray::arr1(&[3.5, 1.0]),
            ndarray::arr1(&[3.5, 3.5]),
            ndarray::arr1(&[4.0, 4.0]),
            ndarray::arr1(&[6.0, 4.0]),
        ],
    };
    scene.add(Box::new(polyline));
    scene.save_png(&mut ctx, "poly_line.png");
}

#[test]
fn test_rectangle_image() {
    let mut ctx = Context::default();
    let mut scene = Scene::new();
    let rectangle = Rectangle {
        stroke_width: 0.2,
        position: ndarray::arr1(&[0.0, 0.0, 0.0]),
        width: 3.0,
        height: 3.0,
    };
    scene.add(Box::new(rectangle));
    scene.save_png(&mut ctx, "rectangle.png");
}

struct AnimationConfig {
    current_frame: u32,
    total_frames: u32,
}
struct Movement {
    displacement: ndarray::Array1<f32>,
    animation_config: AnimationConfig,
    mobject: Box<dyn Mobject>,
}

impl Iterator for Movement {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[test]
fn write_frame() {
    use std::sync::{Arc, Mutex};
    use std::thread;
    let mut ctx = Context::default();
    let mut scene = Scene::new();
    let rectangle = Rectangle {
        stroke_width: 0.2,
        position: ndarray::arr1(&[0.0, 0.0, 0.0]),
        width: 3.0,
        height: 3.0,
    };
    scene.add(Box::new(rectangle));

    use std::io::Write;
    use std::process::Command;
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};

    use video_backend::{FFMPEGBackend, FrameMessage, VideoBackend, VideoBackendType};

    let mut video_backend_var = VideoBackend {
        backend_type: VideoBackendType::FFMPEG(FFMPEGBackend::new()),
        video_config: video_backend::VideoConfig {
            filename: "output.mp4".to_owned(),
            framerate: 60,
            output_width: 1920,
            output_height: 1080,
        },
    };
    for _ in 0..480 {
        let now = std::time::Instant::now();
        scene.mobjects[0].move_this(ndarray::arr1(&[0.01, 0.0, 0.0]));
        ctx.clear_transparent();
        for m in scene.mobjects.iter() {
            m.draw(&mut ctx);
        }
        video_backend_var.write_frame(ctx.image_bytes());
    }
}

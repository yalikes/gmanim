#![allow(unused)]

mod mobjects;
mod video_backend;
mod math_utils;

use mobjects::Mobject;
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

#[test]
fn test_simple_line_image() {
    use mobjects::SimpleLine;
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
    use mobjects::PolyLine;
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
    use mobjects::Rectangle;
    let mut ctx = Context::default();
    let mut scene = Scene::new();
    let rectangle = Rectangle {
        stroke_width: 0.2,
        p0: ndarray::arr1(&[0.0, 0.0, 0.0]),
        p1: ndarray::arr1(&[3.0, 0.0, 0.0]),
        p2: ndarray::arr1(&[3.0, 3.0, 0.0]),
        p3: ndarray::arr1(&[0.0, 3.0, 0.0]),
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
    use mobjects::Rectangle;
    use std::sync::{Arc, Mutex};
    use std::thread;
    let mut ctx = Context::default();
    let mut scene = Scene::new();
    let rectangle = Rectangle {
        stroke_width: 0.2,
        p0: ndarray::arr1(&[0.0, 0.0, 0.0]),
        p1: ndarray::arr1(&[3.0, 0.0, 0.0]),
        p2: ndarray::arr1(&[3.0, 3.0, 0.0]),
        p3: ndarray::arr1(&[0.0, 3.0, 0.0]),
    };
    scene.add(Box::new(rectangle));

    use std::io::Write;
    use std::process::Command;
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};

    use video_backend::{
        BgraRAWBackend, FFMPEGBackend, FrameMessage, VideoBackend, VideoBackendType, VideoConfig,
    };

    let video_config = VideoConfig {
        filename: "output.mp4".to_owned(),
        framerate: 60,
        output_height: 1080,
        output_width: 1920,
    };
    let mut video_backend_var = VideoBackend {
        backend_type: VideoBackendType::FFMPEG(FFMPEGBackend::new(&video_config)),
    };
    for _ in 0..480 {
        let now = std::time::Instant::now();
        scene.mobjects[0].move_this(ndarray::arr1(&[0.01, 0.0, 0.0]));
        ctx.clear_transparent();
        for m in scene.mobjects.iter() {
            m.draw(&mut ctx);
        }
        video_backend_var.write_frame(ctx.image_bytes());
        // println!("takes {:?}", now.elapsed());
    }
}

#[test]
fn thread_frame_pass() {
    use mobjects::Rectangle;
    use std::sync::mpsc::channel;
    use std::sync::{Arc, Mutex};
    use std::thread;
    let mut ctx = Context::default();
    let mut scene = Scene::new();
    let rectangle = Rectangle {
        stroke_width: 0.2,
        p0: ndarray::arr1(&[0.0, 0.0, 0.0]),
        p1: ndarray::arr1(&[3.0, 0.0, 0.0]),
        p2: ndarray::arr1(&[3.0, 3.0, 0.0]),
        p3: ndarray::arr1(&[0.0, 3.0, 0.0]),
    };
    scene.add(Box::new(rectangle));

    use std::collections::VecDeque;
    use std::io::Write;
    use std::process::Command;
    use std::sync::mpsc;
    use std::sync::mpsc::{Receiver, Sender};

    use video_backend::{
        BgraRAWBackend, FFMPEGBackend, FrameMessage, VideoBackend, VideoBackendType, VideoConfig,
    };

    let video_config = VideoConfig {
        filename: "output.mp4".to_owned(),
        framerate: 60,
        output_height: 1080,
        output_width: 1920,
    };

    let mut video_backend_var = VideoBackend {
        backend_type: VideoBackendType::FFMPEG(FFMPEGBackend::new(&video_config)),
    };

    let (tx, rx) = channel::<video_backend::FrameMessage>();
    let queue = Arc::new(Mutex::new(VecDeque::<Vec<u8>>::new()));
    let queue_ref = queue.clone();

    let state = Arc::new(Mutex::new(video_backend::VideoBackendState::Running));
    let state_ref = state.clone();
    let thread_handler = thread::spawn(move || {
        video_backend_var.write_frame_background(rx, state_ref, queue_ref);
    });
    for _ in 0..480 {
        let now = std::time::Instant::now();
        scene.mobjects[0].move_this(ndarray::arr1(&[0.01, 0.0, 0.0]));
        ctx.clear_transparent();
        for m in scene.mobjects.iter() {
            m.draw(&mut ctx);
        }
        let data_bytes = ctx.image_bytes().to_vec();
        {
            let mut queue_guard = queue.lock().unwrap();
            queue_guard.push_back(data_bytes);
        } //release queue lock
        {
            let mut state_guard = state.lock().unwrap();
            match *state_guard {
                video_backend::VideoBackendState::Sleeping => {
                    *state_guard = video_backend::VideoBackendState::Running;
                    tx.send(FrameMessage::Frame);
                }
                _ => {}
            }
        } //release state lock
        tx.send(FrameMessage::End);
    }
    thread_handler.join();
}

#![allow(unused)]

struct Color {
    r: f32,
    g: f32,
    b: f32,
}
pub enum ContextType {
    CAIRO(cairo::Context), // we always have cairo as a fallback
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
    fn move_with<T>(self: &mut Self, movement: ndarray::Array1<f32>) {}
}

impl SimpleMove for SimpleLine {
    fn move_with<T>(self: &mut Self, movement: ndarray::Array1<f32>) {
        self.p0 = self.p0.clone() + movement.clone();
        self.p1 = self.p1.clone() + movement.clone();
    }
}

impl SimpleMove for PolyLine{
    fn move_with<T>(self: &mut Self, movement: ndarray::Array1<f32>) {
        for i in 0..self.points.len(){
            self.points[i] = self.points[i].clone() + movement.clone();
        }
    }
}

impl SimpleMove for Rectangle {
    fn move_with<T>(self: &mut Self, movement: ndarray::Array1<f32>) {
        self.position = self.position.clone() + movement;
    }
}

impl Default for Context {
    fn default() -> Self {
        let scene_config = SceneConfig::default();
        let image_surface = cairo::ImageSurface::create(
            cairo::Format::ARgb32,
            scene_config.output_width as i32,
            scene_config.output_height as i32,
        )
        .unwrap();
        let ctx_cr = cairo::Context::new(image_surface).unwrap();
        let ctx_type = ContextType::CAIRO(ctx_cr);
        Self {
            ctx_type: ctx_type,
            scene_config,
        }
    }
}

impl Context {
    pub fn save_png(self: &Self, file_path: &str) {
        match &self.ctx_type {
            ContextType::CAIRO(c) => {
                let mut f = std::fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(file_path)
                    .unwrap();
                c.target().write_to_png(&mut f);
            }
            _ => {}
        }
    }
}
pub trait Draw {
    //draw shape without fill
    fn draw(self: &Self, ctx: &Context);
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
    fn draw(self: &Self, ctx: &Context) {
        match &ctx.ctx_type {
            ContextType::CAIRO(c) => {
                let scale_factor = ctx.scene_config.scale_factor;
                c.set_line_width((self.stroke_width * scale_factor).into());
                c.set_line_cap(cairo::LineCap::Butt);
                c.set_source_rgba(1.0, 1.0, 0.5, 1.0);
                c.move_to(
                    (coordinate_change_x(self.p0[[0]], ctx.scene_config.width) * scale_factor)
                        .into(),
                    (coordinate_change_y(self.p0[[1]], ctx.scene_config.height) * scale_factor)
                        .into(),
                );
                c.line_to(
                    (coordinate_change_x(self.p1[[0]], ctx.scene_config.width) * scale_factor)
                        .into(),
                    (coordinate_change_y(self.p1[[1]], ctx.scene_config.height) * scale_factor)
                        .into(),
                );
                c.stroke().unwrap();
            }
            _ => {}
        }
    }
}

impl Draw for PolyLine {
    fn draw(self: &Self, ctx: &Context) {
        match &ctx.ctx_type {
            ContextType::CAIRO(c) => {
                let scale_factor = ctx.scene_config.scale_factor;
                if self.points.len() < 2 {
                    return;
                }
                c.set_line_width((self.stroke_width * scale_factor).into());
                c.set_line_cap(cairo::LineCap::Butt);
                c.set_line_join(cairo::LineJoin::Round);
                c.set_source_rgba(1.0, 1.0, 0.5, 1.0);
                c.move_to(
                    (coordinate_change_x(self.points[0][[0]], ctx.scene_config.width)
                        * scale_factor)
                        .into(),
                    (coordinate_change_y(self.points[0][[1]], ctx.scene_config.height)
                        * ctx.scene_config.scale_factor)
                        .into(),
                );
                for p in self.points[1..].iter() {
                    c.line_to(
                        (coordinate_change_x(p[[0]], ctx.scene_config.width) * scale_factor).into(),
                        (coordinate_change_y(p[[1]], ctx.scene_config.height) * scale_factor)
                            .into(),
                    );
                }
                c.stroke().unwrap();
            }
            _ => {}
        }
    }
}

impl Draw for Rectangle {
    fn draw(self: &Self, ctx: &Context) {
        match &ctx.ctx_type {
            ContextType::CAIRO(c) => {
                let scale_factor = ctx.scene_config.scale_factor;
                c.set_line_width((self.stroke_width * scale_factor).into());
                c.set_line_cap(cairo::LineCap::Butt);
                c.set_line_join(cairo::LineJoin::Round);
                c.set_source_rgba(1.0, 1.0, 0.5, 1.0);

                c.rectangle(
                    (coordinate_change_x(self.position[[0]], ctx.scene_config.width)
                        * scale_factor)
                        .into(),
                    (coordinate_change_x(self.position[[1]], ctx.scene_config.height)
                        * scale_factor)
                        .into(),
                    (self.width * scale_factor).into(),
                    (self.height * scale_factor).into(),
                );
                c.stroke().unwrap();
            }
            _ => {}
        }
    }
}

#[test]
fn test_simple_line_image() {
    let ctx = Context::default();
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
    simple_line.draw(&ctx);
    simple_line2.draw(&ctx);
    ctx.save_png("simple_line.png");
}

#[test]
fn test_polyline_image() {
    let ctx = Context::default();
    let polyline = PolyLine {
        stroke_width: 0.2,
        points: vec![
            ndarray::arr1(&[0.0, 0.0]),
            ndarray::arr1(&[3.5, 1.0]),
            ndarray::arr1(&[3.5, 3.5]),
            ndarray::arr1(&[4.0, 4.5]),
            ndarray::arr1(&[6.0, 4.5]),
        ],
    };
    polyline.draw(&ctx);
    ctx.save_png("polyline.png");
}

#[test]
fn test_rectangle_image() {
    let ctx = Context::default();
    let polyline = Rectangle {
        stroke_width: 0.2,
        position: ndarray::arr1(&[0.0, 0.0, 0.0]),
        width: 3.0,
        height: 3.0,
    };
    polyline.draw(&ctx);
    ctx.save_png("rectangle.png");
}

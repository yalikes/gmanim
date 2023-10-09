#![allow(unused)]
pub enum ContextType {
    CAIRO(cairo::Context), // we always have cairo as a fallback
    VULKAN,
    CUDA,
    HIP,
}
pub struct Context {
    ctx_type: ContextType,
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

type Vec2f = Vec2<f32>;
struct Vec2<T> {
    x: T,
    y: T,
}

struct PolyLine {
    stroke_width: f32,
    points: Vec<Vec2f>,
}

struct SimpleLine {
    stroke_width: f32,
    p0: Vec2f,
    p1: Vec2f,
}

struct Rectangle {
    stroke_width: f32,
    position: Vec2f,
    width: f32,
    height: f32,
}

impl Draw for SimpleLine {
    fn draw(self: &Self, ctx: &Context) {
        match &ctx.ctx_type {
            ContextType::CAIRO(c) => {
                c.set_line_width(self.stroke_width.into());
                c.set_line_cap(cairo::LineCap::Butt);
                c.set_source_rgba(1.0, 1.0, 0.5, 1.0);
                c.move_to(self.p0.x.into(), self.p0.y.into());
                c.line_to(self.p1.x.into(), self.p1.y.into());
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
                if self.points.len() < 2 {
                    return;
                }
                c.set_line_width(self.stroke_width.into());
                c.set_line_cap(cairo::LineCap::Butt);
                c.set_line_join(cairo::LineJoin::Round);
                c.set_source_rgba(1.0, 1.0, 0.5, 1.0);
                c.move_to(self.points[0].x.into(), self.points[0].y.into());
                for p in self.points[1..].iter() {
                    c.line_to(p.x.into(), p.y.into());
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
                c.set_line_width(self.stroke_width.into());
                c.set_line_cap(cairo::LineCap::Butt);
                c.set_line_join(cairo::LineJoin::Round);
                c.set_source_rgba(1.0, 1.0, 0.5, 1.0);

                c.rectangle(
                    self.position.x.into(),
                    self.position.y.into(),
                    self.width.into(),
                    self.height.into(),
                );

                c.stroke().unwrap();
            }
            _ => {}
        }
    }
}

#[test]
fn test_simple_line_image() {
    let image_surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 1024, 1024).unwrap();
    let cr_ctx = cairo::Context::new(image_surface).unwrap();
    let ctx = Context {
        ctx_type: ContextType::CAIRO(cr_ctx),
    };
    let simple_line = SimpleLine {
        stroke_width: 1.0,
        p0: Vec2 { x: 0.0, y: 0.0 },
        p1: Vec2 { x: 200.0, y: 200.0 },
    };
    let simple_line2 = SimpleLine {
        stroke_width: 1.0,
        p0: Vec2 { x: 200.0, y: 200.0 },
        p1: Vec2 { x: 500.0, y: 200.0 },
    };
    simple_line.draw(&ctx);
    simple_line2.draw(&ctx);
    ctx.save_png("simple_line.png");
}

#[test]
fn test_polyline_image() {
    let image_surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 1024, 1024).unwrap();
    let cr_ctx = cairo::Context::new(image_surface).unwrap();
    let ctx = Context {
        ctx_type: ContextType::CAIRO(cr_ctx),
    };
    let polyline = PolyLine {
        stroke_width: 16.0,
        points: vec![
            Vec2 { x: 100.0, y: 100.0 },
            Vec2 { x: 350.0, y: 100.0 },
            Vec2 { x: 350.0, y: 350.0 },
            Vec2 { x: 400.0, y: 450.0 },
            Vec2 { x: 600.0, y: 600.0 },
        ],
    };
    polyline.draw(&ctx);
    ctx.save_png("polyline.png");
}

#[test]
fn test_rectangle_image() {
    let image_surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 1024, 1024).unwrap();
    let cr_ctx = cairo::Context::new(image_surface).unwrap();
    let ctx = Context {
        ctx_type: ContextType::CAIRO(cr_ctx),
    };
    let polyline = Rectangle {
        stroke_width: 32.0,
        position: Vec2 { x: 500.0, y: 500.0 },
        width: 300.0,
        height: 300.0,
    };
    polyline.draw(&ctx);
    ctx.save_png("rectangle.png");
}

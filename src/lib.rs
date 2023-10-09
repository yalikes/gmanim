#![allow(unused)]
pub enum ContextType{
    CAIRO(cairo::Context), // we always have cairo as a fallback
    VULKAN, 
    CUDA,
    HIP,
}
pub struct Context {
    ctx_type: ContextType
}

impl Context {
    pub fn save_png(self: &Self, file_path: &str){
        match &self.ctx_type{
            ContextType::CAIRO(c) => {
                let mut f = std::fs::OpenOptions::new().write(true).create(true).open(file_path).unwrap();
                c.target().write_to_png(&mut f);
            },
            _ => {

            }
        }
    }
}
pub trait Draw {//draw shape without fill
    fn draw(self: &Self, ctx: &Context);
}

type Vec2f = Vec2<f32>;
struct Vec2<T> {
    x: T,
    y: T,
}

struct PolyLine {
    width: f32,
    points: Vec<Vec2f>,
}

struct SimpleLine{
    width: f32,
    p0: Vec2f,
    p1: Vec2f,
}

impl Draw for SimpleLine {
    fn draw(self: &Self, ctx: &Context) {
        match  &ctx.ctx_type {
            ContextType::CAIRO(c) => {
                c.set_line_width(self.width.into());
                c.set_source_rgba(1.0, 1.0, 0.5, 1.0);
                c.move_to(self.p0.x.into(), self.p0.y.into());
                c.line_to(self.p1.x.into(), self.p1.y.into());
                c.stroke().unwrap();
            },
            _ => {
            }
        }
    }
}

#[test]
fn test_write_image(){
    let image_surface = cairo::ImageSurface::create(cairo::Format::ARgb32, 1024, 1024).unwrap();
    let cr_ctx = cairo::Context::new(image_surface).unwrap();
    let ctx = Context{
        ctx_type: ContextType::CAIRO(cr_ctx)
    };
    let simple_line = SimpleLine{
        width: 1.0,
        p0: Vec2 { x: 0.0, y: 0.0 },
        p1: Vec2 { x: 200.0, y: 200.0 }
    };
    let simple_line2 = SimpleLine {
        width: 1.0,
        p0: Vec2 { x: 200.0, y: 200.0 },
        p1: Vec2 { x: 500.0, y: 200.0 }
    };
    simple_line.draw(&ctx);
    simple_line2.draw(&ctx);
    ctx.save_png("simple_line.png");
}
use std::fs;
use std::io::Read;

use log::info;
use rusttype::{point, Font, Scale};

use crate::mobjects::Draw;
use crate::{ContextType, GMFloat, log_utils};
use nalgebra::{Vector3, Point3};

use super::{coordinate_change_x, coordinate_change_y, DrawConfig, Mobject, Transform};

pub struct Text {
    pub text: String,
    pub position: Point3<GMFloat>,
    pub font_size: GMFloat,
    pub draw_config: DrawConfig,
}

pub enum FontConfig {
    Default,
    FontName(String),
    FontFile(String),
}

impl Transform for Text {
    fn transform(&mut self, transform: nalgebra::Transform3<GMFloat>) {
        
    }
}

impl Draw for Text {
    fn draw(&self, ctx: &mut crate::Context) {
        if self.text.len() == 0 {
            return; //this is no text to draw
        }
        let scale_factor = ctx.scene_config.scale_factor;
        match &mut ctx.ctx_type {
            ContextType::TinySKIA(pixmap) => {
                // let width;
                // let height;
                let mut f = fs::File::open("/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc")
                    .expect("can't open font file");
                let mut font_data_data = vec![];
                f.read_to_end(&mut font_data_data)
                    .expect("can't read font file");

                let font = Font::try_from_bytes(&font_data_data)
                    .expect("failed to parse font file content");
                let scale = Scale::uniform(self.font_size as f32);
                let v_metrics = font.v_metrics(scale);
                // to see why we make start at (0.0, v_metrics.ascent), take a look at documentation
                let glyphs: Vec<_> = font
                    .layout(&self.text, scale, point(0.0, 0.0 + v_metrics.ascent)) // maybe I need some padding here
                    .collect();

                let img_height = (v_metrics.ascent - v_metrics.descent).ceil() as usize;
                let (img_width, min_x) = {
                    let min_x = glyphs
                        .first()
                        .map(|g| g.pixel_bounding_box().unwrap().min.x)
                        .unwrap();
                    let max_x = glyphs
                        .last()
                        .map(|g| g.pixel_bounding_box().unwrap().max.x)
                        .unwrap();
                    ((max_x - min_x) as usize, min_x)
                }; // great, rusttype help me to calculate advance width and Kerning Pair
                let mut pixmap_new =
                    tiny_skia::Pixmap::new(img_width as u32, img_height as u32).unwrap();
                for glyph in glyphs {
                    if let Some(bounding_box) = glyph.pixel_bounding_box() {
                        glyph.draw(|x, y, v| {
                            let idx_x = (x + bounding_box.min.x as u32 - min_x as u32) as usize;
                            let idx_y = (y + bounding_box.min.y as u32 - min_x as u32) as usize;
                            let pixeles = pixmap_new.pixels_mut();
                            pixeles[idx_x + idx_y * img_width] =
                                tiny_skia::PremultipliedColorU8::from_rgba(
                                    (self.draw_config.color.r as f32 * v) as u8,
                                    (self.draw_config.color.g as f32 * v) as u8,
                                    (self.draw_config.color.b as f32 * v) as u8,
                                    (self.draw_config.color.a as f32 * v) as u8,
                                )
                                .unwrap();
                        });
                    }
                }

                let paint = tiny_skia::PixmapPaint::default();

                pixmap.draw_pixmap(
                    (coordinate_change_x(self.position.x, ctx.scene_config.width) * scale_factor)
                        as i32,
                    (coordinate_change_y(self.position.y, ctx.scene_config.height) * scale_factor)
                        as i32,
                    pixmap_new.as_ref(),
                    &paint,
                    tiny_skia::Transform::identity(),
                    None,
                );
            }
            _ => {}
        }
    }
}

impl Mobject for Text {}

#[test]
fn test_draw_text() {
    let mut ctx = crate::Context::default();
    let mut scene = crate::Scene::new();
    let text = Text {
        text: "测试文本".to_owned(),
        position: Point3::new(0.0, 0.0, 0.0),
        font_size: 600.0,
        draw_config: Default::default(),
    };
    text.draw(&mut ctx);
    match &mut ctx.ctx_type {
        ContextType::TinySKIA(pixmap) => {
            pixmap.save_png("text_render.png");
        }
        _ => {}
    }
}

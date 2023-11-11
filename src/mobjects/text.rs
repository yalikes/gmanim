use std::fs;
use std::io::Read;

use raqote::{Image, DrawOptions};
use rusttype::{point, Font, Scale};

use crate::mobjects::Draw;
use crate::{ContextType, GMFloat};
use nalgebra::Vector2;

pub struct Text {
    text: String,
    position: Vector2<GMFloat>,
    font_size: GMFloat,
}

pub enum FontConfig {
    Default,
    FontName(String),
    FontFile(String),
}

impl Draw for Text {
    fn draw(&self, ctx: &mut crate::Context) {
        if self.text.len() == 0 {
            return; //this is no text to draw
        }
        match &mut ctx.ctx_type {
            ContextType::Raqote(dt) => {
                // let width;
                // let height;
                let mut f = fs::File::open("/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc")
                    .expect("can't open font file");
                let mut font_data_data = vec![];
                f.read_to_end(&mut font_data_data)
                    .expect("can't read font file");

                let font = Font::try_from_bytes(&font_data_data)
                    .expect("failed to parse font file content");
                let scale = Scale::uniform(self.font_size);
                let v_metrics = font.v_metrics(scale);
                // to see why we make start at (0.0, v_metrics.ascent), take a look at documentation
                let glyphs: Vec<_> = font
                    .layout(&self.text, scale, point(0.0, 0.0 + v_metrics.ascent))
                    .collect();

                let img_height = (v_metrics.ascent - v_metrics.descent).ceil() as usize;
                let img_width = {
                    let min_x = glyphs
                        .first()
                        .map(|g| g.pixel_bounding_box().unwrap().min.x)
                        .unwrap();
                    let max_x = glyphs
                        .last()
                        .map(|g| g.pixel_bounding_box().unwrap().max.x)
                        .unwrap();
                    (max_x - min_x) as usize
                }; // great, rusttype help me to calculate advance width and Kerning Pair
                let mut data = vec![0 as u32; img_width * img_height];
                for glyph in glyphs {
                    if let Some(bounding_box) = glyph.pixel_bounding_box() {
                        glyph.draw(|x, y, v| {
                            let idx_x = (x + bounding_box.min.x as u32) as usize;
                            let idx_y = (y + bounding_box.min.y as u32) as usize;
                            data[idx_x + idx_y * img_width] = u32::from_be_bytes([
                                (255 as f32 * v) as u8,
                                (255 as f32 * v) as u8,
                                0,
                                0,
                            ]);
                        });
                    }
                }
                dt.draw_image_at(10.0, 10.0, &Image{
                    width: img_width as i32,
                    height: img_height as i32,
                    data: &data
                }, &DrawOptions::default())
            }
            _ => {}
        }
    }
}

#[test]
fn test_draw_text() {
    let mut ctx = crate::Context::default();
    let mut scene = crate::Scene::new();
    let text = Text {
        text: "我去,怎么回事".to_owned(),
        position: Vector2::new(0.0, 0.0),
        font_size: 600.0,
    };
    text.draw(&mut ctx);
    match &mut ctx.ctx_type {
        ContextType::Raqote(dt) => {
            dt.write_png("text_render.png");
        }
        _ => {}
    }
}

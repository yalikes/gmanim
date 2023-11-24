use std::{fs, io::Read};

use nalgebra::Vector2;
use usvg::{tiny_skia_path::PathSegment, Group, Node, NodeExt, NodeKind, TreeParsing};

use crate::{
    math_utils::{point2d_to_point3d, point3d_to_point2d},
    Context, ContextType, GMFloat, Scene,
};

use super::{
    coordinate_change_x, coordinate_change_y, group::MobjectGroup, Draw, DrawConfig, Mobject,
    Transform,
};

#[derive(Debug)]
pub enum PathElement {
    MoveTo(nalgebra::Point2<GMFloat>),
    LineTo(nalgebra::Point2<GMFloat>),
    QuadTo(nalgebra::Point2<GMFloat>, nalgebra::Point2<GMFloat>),
    CubicTo(
        nalgebra::Point2<GMFloat>,
        nalgebra::Point2<GMFloat>,
        nalgebra::Point2<GMFloat>,
    ),
    Close,
}

#[derive(Debug)]
struct SVGPath {
    elements: Vec<PathElement>,
    is_closed: bool,
    draw_config: DrawConfig,
}

impl SVGPath {
    fn new() -> Self {
        Self {
            elements: vec![],
            is_closed: false,
            draw_config: Default::default(),
        }
    }
    fn move_to_origin(&mut self) {
        if self.elements.len() == 0 {
            return;
        }
        let start = self.elements.first().unwrap();
        let start_pos;
        if let PathElement::MoveTo(p) = start {
            start_pos = p.clone();
        } else {
            return;
        }
        let start_displacement = nalgebra::Vector2::new(start_pos.x, start_pos.y);
        for e in &mut self.elements {
            match e {
                PathElement::MoveTo(p) => {
                    *p -= start_displacement;
                }
                PathElement::LineTo(p) => {
                    *p -= start_displacement;
                }
                PathElement::QuadTo(p1, p2) => {
                    *p1 -= start_displacement;
                    *p2 -= start_displacement;
                }
                PathElement::CubicTo(p1, p2, p3) => {
                    *p1 -= start_displacement;
                    *p2 -= start_displacement;
                    *p3 -= start_displacement;
                }
                PathElement::Close => {}
            }
        }
    }
    pub fn flip_y_coordinate(&mut self) {
        for e in &mut self.elements {
            match e {
                PathElement::MoveTo(p) => {
                    p.y = -p.y;
                }
                PathElement::LineTo(p) => {
                    p.y = -p.y;
                }
                PathElement::QuadTo(p1, p2) => {
                    p1.y = -p1.y;
                    p2.y = -p2.y;
                }
                PathElement::CubicTo(p1, p2, p3) => {
                    p1.y = -p1.y;
                    p2.y = -p2.y;
                    p3.y = -p3.y;
                }
                PathElement::Close => {}
            }
        }
    }
}

impl super::Transform for SVGPath {
    fn transform(&mut self, transform: nalgebra::Transform3<GMFloat>) {
        for e in &mut self.elements {
            match e {
                PathElement::MoveTo(p) => {
                    *p = point3d_to_point2d(transform * point2d_to_point3d(p.clone()));
                }
                PathElement::LineTo(p) => {
                    *p = point3d_to_point2d(transform * point2d_to_point3d(p.clone()));
                }
                PathElement::QuadTo(p1, p2) => {
                    *p1 = point3d_to_point2d(transform * point2d_to_point3d(p1.clone()));
                    *p2 = point3d_to_point2d(transform * point2d_to_point3d(p2.clone()));
                }
                PathElement::CubicTo(p1, p2, p3) => {
                    *p1 = point3d_to_point2d(transform * point2d_to_point3d(p1.clone()));
                    *p2 = point3d_to_point2d(transform * point2d_to_point3d(p2.clone()));
                    *p3 = point3d_to_point2d(transform * point2d_to_point3d(p3.clone()));
                }
                PathElement::Close => {}
            }
        }
    }
}

impl Draw for SVGPath {
    fn draw(&self, ctx: &mut crate::Context) {
        let scale_factor = ctx.scene_config.scale_factor;
        let scene_width = ctx.scene_config.width;
        let scene_height = ctx.scene_config.height;
        match &mut ctx.ctx_type {
            ContextType::TinySKIA(pixmap) => {
                let mut pb = tiny_skia::PathBuilder::new();
                for e in &self.elements {
                    match e {
                        PathElement::MoveTo(p) => {
                            pb.move_to(
                                coordinate_change_x(p.x as f32, ctx.scene_config.width as f32)
                                    * scale_factor as f32,
                                coordinate_change_y(p.y as f32, ctx.scene_config.height as f32)
                                    * scale_factor as f32,
                            );
                        }
                        PathElement::LineTo(p) => {
                            pb.line_to(
                                coordinate_change_x(p.x as f32, ctx.scene_config.width as f32)
                                    * scale_factor as f32,
                                coordinate_change_y(p.y as f32, ctx.scene_config.height as f32)
                                    * scale_factor as f32,
                            );
                        }
                        PathElement::QuadTo(p1, p2) => {
                            pb.quad_to(
                                coordinate_change_x(p1.x as f32, ctx.scene_config.width as f32)
                                    * scale_factor as f32,
                                coordinate_change_y(p1.y as f32, ctx.scene_config.height as f32)
                                    * scale_factor as f32,
                                coordinate_change_x(p2.x as f32, ctx.scene_config.width as f32)
                                    * scale_factor as f32,
                                coordinate_change_y(p2.y as f32, ctx.scene_config.height as f32)
                                    * scale_factor as f32,
                            );
                        }
                        PathElement::CubicTo(p1, p2, p3) => {
                            pb.cubic_to(
                                coordinate_change_x(p1.x as f32, scene_width as f32)
                                    * scale_factor as f32,
                                coordinate_change_y(p1.y as f32, scene_height as f32)
                                    * scale_factor as f32,
                                coordinate_change_x(p2.x as f32, scene_width as f32)
                                    * scale_factor as f32,
                                coordinate_change_y(p2.y as f32, scene_height as f32)
                                    * scale_factor as f32,
                                coordinate_change_x(p3.x as f32, scene_width as f32)
                                    * scale_factor as f32,
                                coordinate_change_y(p3.y as f32, scene_height as f32)
                                    * scale_factor as f32,
                            );
                        }
                        PathElement::Close => {
                            pb.close();
                        }
                    }
                }
                let path = pb.finish().unwrap();

                let mut stroke = tiny_skia::Stroke::default();
                stroke.width = self.draw_config.stoke_width * scale_factor;
                stroke.line_cap = tiny_skia::LineCap::Round;
                let mut paint = tiny_skia::Paint::default();
                paint.set_color(self.draw_config.color.into());
                pixmap.fill_path(
                    &path,
                    &paint,
                    Default::default(),
                    tiny_skia::Transform::identity(),
                    None,
                );
            }
            _ => {}
        }
    }
}

impl Mobject for SVGPath {}

pub fn open_svg_file(svg_filepath: &str) -> MobjectGroup {
    let mut svg_file = fs::File::options()
        .read(true)
        .open(svg_filepath)
        .expect("can't open svg file");
    let mut svg_str_buf = String::new();
    svg_file.read_to_string(&mut svg_str_buf);
    let tree = usvg::Tree::from_str(&svg_str_buf, &Default::default()).unwrap();
    let mut paths: Vec<SVGPath> = vec![];
    for node in tree.root.descendants() {
        let n = &*node.borrow();
        match n {
            NodeKind::Group(g) => {
                //we don't care for now
            }
            NodeKind::Image(img) => {
                //we don't care for now
            }
            NodeKind::Path(path) => {
                //apply transform
                let mut svg_path = SVGPath::new();
                let transform = node.abs_transform();
                let path_data = &path.data;
                for e in path_data.segments() {
                    let pe = process_path_element(e, transform);
                    svg_path.elements.push(pe);
                }
                svg_path.flip_y_coordinate();
                paths.push(svg_path);
            }
            NodeKind::Text(text) => {
                //we don't care for now
            }
        }
    }

    let mut grp_mobj = MobjectGroup {
        mobjects: paths
            .into_iter()
            .map(|p| Box::new(p) as Box<dyn Mobject>)
            .collect(),
    };

    let scaling_matrix = nalgebra::Matrix4::new_scaling(0.1);
    grp_mobj.transform(nalgebra::Transform::from_matrix_unchecked(scaling_matrix));

    grp_mobj
}

pub fn process_path_element(e: PathSegment, transform: tiny_skia::Transform) -> PathElement {
    match e {
        PathSegment::MoveTo(p) => {
            let mut new_p = p.clone();
            transform.map_point(&mut new_p);
            PathElement::MoveTo(nalgebra::Point2::new(
                new_p.x as GMFloat,
                new_p.y as GMFloat,
            ))
        }
        PathSegment::LineTo(p) => {
            let mut new_p = p.clone();
            transform.map_point(&mut new_p);
            PathElement::LineTo(nalgebra::Point2::new(
                new_p.x as GMFloat,
                new_p.y as GMFloat,
            ))
        }
        PathSegment::QuadTo(p1, p2) => {
            let mut new_p1 = p1.clone();
            let mut new_p2 = p2.clone();
            transform.map_point(&mut new_p1);
            transform.map_point(&mut new_p2);
            PathElement::QuadTo(
                nalgebra::Point2::new(new_p1.x as GMFloat, new_p1.y as GMFloat),
                nalgebra::Point2::new(new_p2.x as GMFloat, new_p2.y as GMFloat),
            )
        }
        PathSegment::CubicTo(p1, p2, p3) => {
            let mut new_p1 = p1.clone();
            let mut new_p2 = p2.clone();
            let mut new_p3 = p3.clone();
            transform.map_point(&mut new_p1);
            transform.map_point(&mut new_p2);
            transform.map_point(&mut new_p3);
            PathElement::CubicTo(
                nalgebra::Point2::new(new_p1.x as GMFloat, new_p1.y as GMFloat),
                nalgebra::Point2::new(new_p2.x as GMFloat, new_p2.y as GMFloat),
                nalgebra::Point2::new(new_p3.x as GMFloat, new_p3.y as GMFloat),
            )
        }
        PathSegment::Close => PathElement::Close,
    }
}

#[test]
fn test_parse_svg() {
    let mut m = open_svg_file("formula.svg");
    use super::Transform;
    let scaling_matrix = nalgebra::Matrix4::new_scaling(0.99);
    m.transform(nalgebra::Transform::from_matrix_unchecked(scaling_matrix));
    let mut scene = Scene::new();
    scene.add(Box::new(m));
    let mut ctx = Context::default();
    scene.save_png(&mut ctx, "formula.png");
}

#[test]
fn test_svg_transform() {
    use crate::video_backend::{
        BgraRAWBackend, ColorOrder, FFMPEGBackend, FrameMessage, VideoBackend, VideoBackendType,
        VideoConfig,
    };

    let video_config = VideoConfig {
        filename: "output.mp4".to_owned(),
        framerate: 60,
        output_height: 1080,
        output_width: 1920,
        color_order: ColorOrder::Rgba,
    };

    let mut video_backend_var = VideoBackend {
        backend_type: VideoBackendType::FFMPEG(FFMPEGBackend::new(&video_config)),
    };

    let mut m = open_svg_file("formula.svg");
    use super::Transform;
    let scaling_matrix = nalgebra::Matrix4::new_scaling(0.99);

    let mut scene = Scene::new();
    scene.add(Box::new(m));
    let mut ctx = Context::default();

    for _ in 0..480 {
        let now = std::time::Instant::now();

        scene.mobjects[0].borrow_mut().transform(nalgebra::Transform::from_matrix_unchecked(scaling_matrix));

        ctx.clear_transparent();
        for m in scene.mobjects.iter() {
            m.borrow().draw(&mut ctx);
        }
        video_backend_var.write_frame(ctx.image_bytes());
    }
}

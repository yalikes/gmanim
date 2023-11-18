use std::{fs, io::Read};

use nalgebra::Vector2;
use tiny_skia::Transform;
use usvg::{tiny_skia_path::PathSegment, Group, Node, NodeExt, NodeKind, TreeParsing};

use crate::{
    math_utils::{point2d_to_point3d, point3d_to_point2d},
    Context, ContextType, GMFloat, Scene,
};

use super::{
    coordinate_change_x, coordinate_change_y, group::MobjectGroup, Draw, DrawConfig, Mobject,
    Rotate, SimpleMove,
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
                                coordinate_change_x(p2.x as f32, ctx.scene_config.height as f32)
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
                println!("{:?}", self.draw_config.color);
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
                //we don't care for nowSVGPath { elements: [MoveTo([[5.764, 0.0]]), LineTo([[2.134, 0.0]]), CubicTo([[1.6793333, 0.0]], [[1.2906666, 0.17233372]], [[0.968, 0.5170002]]), CubicTo([[0.81211305, 0.68402195]], [[0.297, 1.3838289]], [[0.297, 1.5289998]]), CubicTo([[0.338111, 1.6112216]], [[0.3357954, 1.6719995]], [[0.473, 1.6719995]]), CubicTo([[0.5463334, 1.6719995]], [[0.616, 1.6243331]], [[0.682, 1.5289996]]), CubicTo([[1.034, 0.98633313]], [[1.4813334, 0.7149997]], [[2.024, 0.7149997]]), LineTo([[2.585, 0.7149997]]), CubicTo([[2.3283334, 1.6756659]], [[1.87, 2.852666]], [[1.21, 4.2459993]]), CubicTo([[1.1513333, 4.3779993]], [[1.122, 4.473333]], [[1.122, 4.531999]]), CubicTo([[1.122, 4.7519994]], [[1.2393334, 4.8619995]], [[1.474, 4.8619995]]), CubicTo([[1.6866666, 4.8619995]], [[1.8406667, 4.744666]], [[1.936, 4.5099993]]), CubicTo([[2.134, 3.8793328]], [[2.2733333, 3.3989992]], [[2.354, 3.0689993]]), LineTo([[2.9589999, 0.7149997]]), LineTo([[4.092, 0.7149997]]), CubicTo([[3.7913334, 2.0203328]], [[3.641, 2.933333]], [[3.641, 3.4539995]]), CubicTo([[3.641, 3.9883802]], [[3.7588787, 4.8619995]], [[4.169, 4.8619995]]), CubicTo([[4.4002156, 4.8619995]], [[4.653, 4.643087]], [[4.653, 4.4109993]]), CubicTo([[4.653, 4.352333]], [[4.631, 4.2679996]], [[4.587, 4.1579995]]), CubicTo([[4.381666, 3.6446662]], [[4.279, 3.0543327]], [[4.279, 2.3869996]]), CubicTo([[4.279, 1.866333]], [[4.3413334, 1.3089995]], [[4.466, 0.7149997]]), LineTo([[5.665, 0.7149997]]), CubicTo([[6.0463333, 0.7149997]], [[6.237, 0.5793333]], [[6.237, 0.3079996]]), CubicTo([[6.237, 0.055641174]], [[6.0421696, -4.7683716e-7]], [[5.764, -4.7683716e-7]]), Close], is_closed: false, draw_config: DrawConfig { stoke_width: 0.25, fill: true, color: Color { r: 51, g: 204, b: 255, a: 255 } } }
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

    MobjectGroup {
        mobjects: paths
            .into_iter()
            .map(|p| Box::new(p) as Box<dyn Mobject>)
            .collect(),
    }
}

pub fn process_path_element(e: PathSegment, transform: Transform) -> PathElement {
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
    println!("{:?}", scaling_matrix);
    m.transform(nalgebra::Transform::from_matrix_unchecked(
        scaling_matrix
    ));
    let mut scene = Scene::new();
    scene.add(Box::new(m));
    let mut ctx = Context::default();
    scene.save_png(&mut ctx, "formula.png");
}

#[test]
fn test_svg_transform() {
    use crate::video_backend::{
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



    let mut m = open_svg_file("formula.svg");
    use super::Transform;
    let scaling_matrix = nalgebra::Matrix4::new_scaling(0.99);


    let mut scene = Scene::new();
    scene.add(Box::new(m));
    let mut ctx = Context::default();

    for _ in 0..480 {
        let now = std::time::Instant::now();

        scene.mobjects[0].transform(nalgebra::Transform::from_matrix_unchecked(
            scaling_matrix
        ));

        ctx.clear_transparent();
        for m in scene.mobjects.iter() {
            m.draw(&mut ctx);
        }
        video_backend_var.write_frame(ctx.image_bytes());
        println!("takes {:?}", now.elapsed());
    }


}
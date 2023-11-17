use std::{fs, io::Read};

use nalgebra::Vector2;
use usvg::{TreeParsing, Node, NodeKind};

use crate::GMFloat;

use super::{group::MobjectGroup, Draw, Mobject};

enum PathElement {
    MoveTo(Vector2<GMFloat>),
    LineTo(Vector2<GMFloat>),
    CurveTo(Vec<GMFloat>),
}

struct SVGPath {
    elements: Vec<PathElement>,
    is_close: bool,
}

impl Draw for SVGPath {
    fn draw(&self, ctx: &mut crate::Context) {}
}

pub fn open_svg_file(svg_filepath: &str) -> MobjectGroup {
    let mut svg_file = fs::File::options()
        .read(true)
        .open(svg_filepath)
        .expect("can't open svg file");
    let mut svg_str_buf = String::new();
    svg_file.read_to_string(&mut svg_str_buf);
    let tree = usvg::Tree::from_str(&svg_str_buf, &Default::default()).unwrap();
    let mut tem_mobjects: Vec<Box<dyn Mobject>> = vec![];
    for c in tree.root.children() {
        let d = &*c.borrow();
        match d {
            NodeKind::Group(g) =>{
                
            }
            NodeKind::Image(img) => {

            }
            NodeKind::Path(p) => {

            }
            NodeKind::Text(t) => {

            }
        }
    }
    let mut mobjects = vec![];
    MobjectGroup { mobjects: mobjects }
}

#[test]
fn test_parse_svg() {
    open_svg_file("formula.svg");
}

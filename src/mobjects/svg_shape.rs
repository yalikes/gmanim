use std::{fs, io::Read};

use usvg::TreeParsing;

use super::group::MobjectGroup;

pub fn open_svg_file(svg_filepath: &str) -> MobjectGroup{
    let mut svg_file  = fs::File::options().read(true).open(svg_filepath).expect("can't open svg file");
    let mut svg_str_buf = String::new();
    svg_file.read_to_string(&mut svg_str_buf);
    let tree = usvg::Tree::from_str(&svg_str_buf, &Default::default()).unwrap();
    for c in tree.root.traverse(){
        println!("{:?}", c);
    }
    let mut mobjects = vec![];
    MobjectGroup { mobjects:  mobjects}
}

#[test]
fn test_parse_svg(){
    open_svg_file("formula.svg");
}
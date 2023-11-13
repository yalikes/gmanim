use std::fs;
use std::process::{Command, Stdio};

pub const TYPST_HEADER: &str = r#"#set page(
    width: auto,
    height: auto,
    margin: (x: 0cm, y: 0cm)
)"#;
pub struct Formula {
    formula_text: String,
}

impl Formula {
    fn write_to_typst(&self, output_typst_file_path: &str) {
        fs::write(
            output_typst_file_path,
            format!("{TYPST_HEADER}\n$ {} $", self.formula_text),
        );
    }
}

pub fn compile_to_svg(typst_file_path: &str, svg_file_path: &str) {
    let mut c = Command::new("typst")
        .args(["compile", "-f", "svg", typst_file_path, svg_file_path])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("can't compile to svg");
    c.wait();
}

#[test]
fn test_write_svg() {
    let f = Formula {
        formula_text: "pi".to_owned(),
    };
    let typst_path = "formula.typst";
    f.write_to_typst(typst_path);
    let svg_path = "formula.svg";
    compile_to_svg(typst_path, svg_path);
    assert!(std::path::Path::new(svg_path).exists());
}

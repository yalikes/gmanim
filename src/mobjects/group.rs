use crate::GMFloat;

use super::{Draw, Mobject, Rotate, SimpleMove};

pub struct MobjectGroup {
    pub mobjects: Vec<Box<dyn Mobject>>,
}

impl super::Transform for MobjectGroup {
    fn transform(&mut self, transform: nalgebra::Transform3<GMFloat>) {
        for m in &mut self.mobjects {
            m.transform(transform);
        }
    }
}

impl Draw for MobjectGroup {
    fn draw(&self, ctx: &mut crate::Context) {
        for m in &self.mobjects {
            m.draw(ctx);
        }
    }
}

impl Mobject for MobjectGroup {}

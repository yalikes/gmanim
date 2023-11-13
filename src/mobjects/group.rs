use super::{Draw, Mobject, Rotate, SimpleMove};

pub struct MobjectGroup {
    pub mobjects: Vec<Box<dyn Mobject>>,
}

impl SimpleMove for MobjectGroup {
    fn move_this(&mut self, movement: nalgebra::Vector3<crate::GMFloat>) {
        for m in &mut self.mobjects {
            m.move_this(movement);
        }
    }
}

impl Rotate for MobjectGroup {
    fn rotate(&mut self, axis: nalgebra::Vector3<crate::GMFloat>, value: f32) {
        for m in &mut self.mobjects {
            m.rotate(axis, value);
        }
    }
}

impl Draw for MobjectGroup {
    fn draw(&self, ctx: &mut crate::Context) {
        for m in &self.mobjects{
            m.draw(ctx);
        }
    }
}

impl Mobject for MobjectGroup {}

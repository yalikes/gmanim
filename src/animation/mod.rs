use std::{cell::RefCell, rc::Rc};

use nalgebra::Vector3;

use crate::{mobjects::Mobject, GMFloat, Scene};

trait Animation: Iterator<Item = Vec<u8>> {}

struct AnimationConfig {
    total_frame: u32,
    current_frame: u32,
    rate_function: fn(GMFloat) -> GMFloat,
}
struct Movement {
    displacement: Vector3<GMFloat>,
    scene: Scene,
    m: Rc<RefCell<Box<dyn Mobject>>>,
    animation_config: AnimationConfig,
}

impl Iterator for Movement {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        let current_frame = self.animation_config.current_frame;
        let total_frame = self.animation_config.total_frame;
        None
    }
}

impl Animation for Movement {}

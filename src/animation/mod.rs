use std::{cell::RefCell, rc::Rc};

use nalgebra::Vector3;

use crate::{mobjects::Mobject, GMFloat, Scene};

trait Animation: Iterator<Item = Vec<u8>> {}

struct AnimationConfig {
    total_frame: u32,
    current_frame: u32,
    rate_function: fn(GMFloat) -> GMFloat,
}
pub struct Movement {
    displacement: Vector3<GMFloat>,
    scene: Scene,
    m: Rc<RefCell<Box<dyn Mobject>>>,
    m_start_state: Box<dyn Mobject>,
    animation_config: AnimationConfig,
}

impl Iterator for Movement {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        let current_frame = self.animation_config.current_frame;
        let total_frame = self.animation_config.total_frame;
        if current_frame>total_frame{
            return None
        }
        let progress = (self.animation_config.rate_function)(
            current_frame as GMFloat / total_frame as GMFloat,
        );
        self.animation_config.current_frame += 1;

        None
    }
}

impl Animation for Movement {}

impl Movement {
    pub fn new(){

    }
}

use nalgebra::Vector3;

use crate::{mobjects::Mobject, GMFloat, Scene};

trait Animation: Iterator<Item = Vec<u8>> {}

struct AnimationConfig {
    total_frame: u32,
    current_frame: u32,
    rate_function: (),//maybe FnMut or else
}
struct Movement {
    displacement: Vector3<GMFloat>,
    scene: Scene,
    m: Box<dyn Mobject>,
    animation_config: AnimationConfig,
}

impl Iterator for Movement {
    type Item = Vec<u8>;
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

impl Animation for Movement {}

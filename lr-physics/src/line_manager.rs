use std::collections::HashMap;

use crate::line::hitbox::Hitbox;

type LineId = u32;
type CollidableObject = Box<dyn Hitbox>;

pub struct PhysicsLineManager {
    lines: HashMap<u32, CollidableObject>,
}

impl PhysicsLineManager {
    pub fn get_line(&self, id: LineId) -> Option<&CollidableObject> {
        self.lines.get(&id)
    }

    pub fn get_line_mut(&mut self, id: LineId) -> Option<&mut CollidableObject> {
        self.lines.get_mut(&id)
    }
}

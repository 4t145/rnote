use bevy::math::prelude::*;
use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Region {
    pub rect: Rect,
}

impl Region {
    pub fn from_point(point: Vec2) -> Self {
        Self {
            rect: Rect {
                max: point,
                min: point,
            },
        }
    }
    pub fn new(rect: Rect) -> Self {
        Region { rect }
    }
}

use bevy::prelude::*;
use std::time::Instant;
#[derive(Debug, Component)]
pub struct LastUpdate(pub Instant);

impl LastUpdate {
    pub fn now() -> Self {
        LastUpdate(Instant::now())
    }
    pub fn update(&mut self) {
        self.0 = Instant::now();
    }
}

impl std::ops::Deref for LastUpdate {
    type Target = Instant;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for LastUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

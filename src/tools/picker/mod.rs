use bevy::prelude::Entity;
#[derive(Debug, Default)]
pub struct Picker {
    pub selected: Option<Entity>,
}

impl Picker {
    #[inline]
    pub fn picked(&self) -> bool {
        self.selected.is_some()
    }
}

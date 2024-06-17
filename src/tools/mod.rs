pub mod brush;
pub mod picker;
use bevy::input::keyboard::KeyboardInput;
pub use bevy::prelude::*;
pub struct ToolPlugin;

impl Plugin for ToolPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToolBox>()
            .add_systems(Update, switch_tool)
            .add_systems(Update, picker::pick_unit_system);
    }
}
#[derive(Debug)]
pub enum Tool {
    Picker(picker::Picker),
    Brush {},
}

#[allow(clippy::derivable_impls)]
impl Default for Tool {
    fn default() -> Self {
        Tool::Picker(Default::default())
    }
}

#[derive(Resource)]
pub struct ToolBox {
    pub current_tool_index: usize,
    pub tools: Vec<Tool>,
}

impl Default for ToolBox {
    fn default() -> Self {
        Self {
            current_tool_index: 0,
            tools: vec![Tool::default(), Tool::Brush {}],
        }
    }
}

impl ToolBox {
    pub fn next_tool(&mut self) {
        if self.tools.is_empty() {
            return;
        };
        self.current_tool_index = (self.current_tool_index + 1) % self.tools.len()
    }
    pub fn prev_tool(&mut self) {
        if self.tools.is_empty() {
            return;
        };
        self.current_tool_index =
            (self.tools.len() + self.current_tool_index - 1) % self.tools.len()
    }
    pub fn current_tool(&self) -> Option<&Tool> {
        if self.tools.is_empty() {
            return None;
        }
        self.tools.get(self.current_tool_index)
    }
    pub fn current_tool_mut(&mut self) -> Option<&mut Tool> {
        if self.tools.is_empty() {
            return None;
        }
        self.tools.get_mut(self.current_tool_index)
    }
}

fn switch_tool(
    // these will panic if the resources don't exist
    mut tool_box: ResMut<ToolBox>,
    mut evtr: EventReader<KeyboardInput>,
) {
    for evt in evtr.read() {
        if evt.state.is_pressed() {
            match evt.key_code {
                KeyCode::KeyR => tool_box.prev_tool(),
                KeyCode::KeyT => tool_box.next_tool(),
                _ => {}
            }
        }
        info!("Current tool: {:?}", tool_box.current_tool());
    }
}

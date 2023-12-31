use bevy::app::{App, Plugin, Update};

use super::input::button_input;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (button_input, bevy::window::close_on_esc));
    }
}

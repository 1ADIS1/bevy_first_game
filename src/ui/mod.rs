use bevy::prelude::*;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_ui);
    }
}

pub fn init_ui() {
    println!("Hello, world, from ui!");
}

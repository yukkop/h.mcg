use bevy::{
    prelude::*,
    winit::WinitSettings, input::mouse::MouseMotion,
};

mod map;
use bevy_mod_picking::{PickingCamera, PickingCameraBundle, InteractablePickingPlugin};
use map::MapPlugin;
mod menu;
use menu::MenuPlugin;

#[derive(Component)]
struct Hoverable;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(InteractablePickingPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(MapPlugin)
        .add_startup_system(setup)
        .run();
}


fn setup(
    commands: &mut Commands,
) {
    commands.spawn(Camera2dBundle::default());
}

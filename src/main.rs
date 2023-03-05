use bevy::{prelude::*, sprite::MaterialMesh2dBundle, winit::WinitSettings};
use bevy_mod_raycast::{
    DefaultRaycastingPlugin, Intersection, RaycastMesh, RaycastMethod, RaycastSource, RaycastSystem,
};

mod graph;
mod map;
use map::MapPlugin;
mod menu;
use menu::MenuPlugin;

#[derive(Component)]
struct Hoverable;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // raycasts for Node type
        .add_plugin(DefaultRaycastingPlugin::<graph::structs::Node>::default())
        .add_system_to_stage(
            CoreStage::First,
            update_raycast_with_cursor.before(RaycastSystem::BuildRays::<graph::structs::Node>),
        )
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(MenuPlugin)
        .add_plugin(MapPlugin)
        .add_startup_system(setup)
        .add_system(intersection)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(RaycastSource::<graph::structs::Node>::new());
}

fn update_raycast_with_cursor(
    mut cursor: EventReader<CursorMoved>,
    mut query: Query<&mut RaycastSource<graph::structs::Node>>,
) {
    // Grab the most recent cursor event if it exists:
    let cursor_position = match cursor.iter().last() {
        Some(cursor_moved) => cursor_moved.position,
        None => return,
    };

    for mut pick_source in &mut query {
        pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
    }
}

fn intersection(query: Query<&Intersection<graph::structs::Node>>) {
    for intersection in &query {
        info!(
            "Distance {:?}, Position {:?}",
            intersection.distance(),
            intersection.position()
        );
    }
}

// use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
// use bevy_mod_raycast::{
//     DefaultRaycastingPlugin, Intersection, RaycastMesh, RaycastMethod, RaycastSource, RaycastSystem,
// };

// fn main() {
//     App::new()
//         .add_plugins(DefaultPlugins)
//         .add_plugin(DefaultRaycastingPlugin::<MyRaycastSet>::default())
//         .add_system_to_stage(
//             CoreStage::First,
//             update_raycast_with_cursor.before(RaycastSystem::BuildRays::<MyRaycastSet>),
//         )
//         .add_system(intersection)
//         .add_startup_system(setup)
//         .run();
// }

// struct MyRaycastSet;

// // Update our `RaycastSource` with the current cursor position every frame.
// fn update_raycast_with_cursor(
//     mut cursor: EventReader<CursorMoved>,
//     mut query: Query<&mut RaycastSource<MyRaycastSet>>,
// ) {
//     // Grab the most recent cursor event if it exists:
//     let cursor_position = match cursor.iter().last() {
//         Some(cursor_moved) => cursor_moved.position,
//         None => return,
//     };

//     for mut pick_source in &mut query {
//         pick_source.cast_method = RaycastMethod::Screenspace(cursor_position);
//     }
// }

// /// Report intersections
// fn intersection(query: Query<&Intersection<MyRaycastSet>>) {
//     for intersection in &query {
//         info!(
//             "Distance {:?}, Position {:?}",
//             intersection.distance(),
//             intersection.position()
//         );
//     }
// }

// fn setup(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     commands
//         .spawn(Camera2dBundle::default())
//         .insert(RaycastSource::<MyRaycastSet>::new()); // Designate the camera as our source;
//     commands
//         .spawn(MaterialMesh2dBundle {
//             mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
//             transform: Transform::default().with_scale(Vec3::splat(128.)),
//             material: materials.add(ColorMaterial::from(Color::PURPLE)),
//             ..default()
//         })
//         .insert(RaycastMesh::<MyRaycastSet>::default()); // Make this mesh ray cast-able;
// }

use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        MaterialMesh2dBundle,
    },
    winit::WinitSettings,
};
mod menu;
use menu::MenuPlugin;

#[derive(Component, Deref)]
struct Size(Vec2);

#[derive(Component)]
struct Map;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(MenuPlugin)
        .add_startup_system(setup_map)
        .run();
}

//map chanking
fn setup_map(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pos1 = Vec2::new(100., 200.);
    let pos2 = Vec2::new(-500., -100.);


    let map = spawn_map(&mut commands, Vec2::ZERO, Size(Vec2::new(1000., 1000.)), 0);

    let dot_deps = &mut (&mut commands, &mut meshes, &mut materials);
    let dot1 = spawn_dot(dot_deps, pos1, 1);
    let dot2 = spawn_dot(dot_deps, pos2, 1);

    let line = spawn_line(&mut commands, pos1, pos2, 5., 2);

    commands.entity(map).add_child(dot1);
    commands.entity(map).add_child(dot2);
    commands.entity(map).add_child(line);

}

fn spawn_map(
    commands: &mut Commands,
    position: Vec2,
    size: Size,
    layer: u32,
) ->  Entity {
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(*size),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(position.x, position.y, layer as f32)),
        ..default()
    }).id()
}

fn spawn_line(
    commands: &mut Commands,
    target_1: Vec2,
    target_2: Vec2,
    width: f32,
    layer: u32,
) -> Entity {
    let vec_ = target_2 - target_1;
    let middle_point = (vec_ / 2.) + target_1;
    let length = vec_.length();

    let angle = Vec2::angle_between(Vec2::new(1., 0.), vec_);
    println!("{}", angle);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::hex("e60c0c").unwrap(),
            custom_size: Some(Vec2::new(length, width)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(
            middle_point.x,
            middle_point.y,
            layer as f32,
        ))
        .with_rotation(Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle)),
        ..default()
    }).id()
}

fn spawn_dot(
    (commands, meshes, materials): &mut (
        &mut Commands,
        &mut ResMut<Assets<Mesh>>,
        &mut ResMut<Assets<ColorMaterial>>,
    ),
    position: Vec2,
    layer: u32,
) -> Entity {
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(5.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::hex("e60c0c").unwrap())),
        transform: Transform::from_translation(Vec3::new(position.x, position.y, layer as f32)),
        ..default()
    }).id()
}

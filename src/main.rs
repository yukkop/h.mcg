use bevy::{prelude::*, winit::WinitSettings, sprite::{collide_aabb::{collide, Collision}, MaterialMesh2dBundle}};
mod menu;
use menu::MenuPlugin;

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
    mut meshes:  ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pos1 = Vec2::new(100., 200.);
    let pos2 = Vec2::new(-500., -100.);
    spawn_dot(&mut commands, &mut meshes, &mut materials, pos1);
    spawn_dot(&mut commands, &mut meshes, &mut materials, pos2);
    spawn_line(&mut commands, pos1, pos2, 5.);
}

fn spawn_line(
    commands: &mut Commands,
    target_1: Vec2,
    target_2: Vec2,
    width: f32,
){
    let vec_ = target_2 - target_1;
    let middle_point = (vec_ / 2.) + target_1; 
    let length = vec_.length();

    let angle = Vec2::angle_between(Vec2::new(1., 0.), vec_);
    println!("{}", angle);

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(1.0, 1.0, 1.0),
            custom_size: Some(Vec2::new(length, width)),
            ..default()
        },
        transform: Transform
            ::from_translation(Vec3::new(middle_point.x, middle_point.y, 0.))
            .with_rotation(Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle)),
        ..default()
    });
}

fn spawn_dot(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    position: Vec2,
){
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(5.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::hex("e60c0c").unwrap())),
            transform: Transform::from_translation(Vec3::new(position.x, position.y, 0.)),
            ..default()
        });
}

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

#[derive(Component)]
struct Dot;

#[derive(Component)]
struct Line;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Only run the app when there is user input. This will significantly reduce CPU/GPU use.
        .insert_resource(WinitSettings::desktop_app())
        .add_plugin(MenuPlugin)
        .run();
}

//map chanking
fn setup_map(
    commands: &mut Commands,
) {
    let pos1 = Vec2::new(100., 200.);
    let pos2 = Vec2::new(-500., -100.);

    let map = MapBundle::new(Vec2::ZERO, Size(Vec2::new(1000., 1000.)), 0);

    // dot_deps.0.entity(map).add_child(dot1);
    // dot_deps.0.entity(map).add_child(dot2);
    // dot_deps.0.entity(map).add_child(line);
    commands.spawn(map).with_children(|child_builder| {
        child_builder.spawn(DotBundle::new(pos1, 1));
        child_builder.spawn(DotBundle::new(pos2, 1));
        child_builder.spawn(LineBundle::new(pos1, pos2, 5., 2));
    });
}

#[derive(Bundle)]
struct MapBundle {
    sprite: SpriteBundle,
    map: Map,
}

impl MapBundle {
    fn new(position: Vec2, size: Size, layer: u32) -> Self {
        Self {
            map: Map,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    custom_size: Some(*size),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    position.x,
                    position.y,
                    layer as f32,
                )),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
struct LineBundle {
    sprite: SpriteBundle,
    line: Line,
}

impl LineBundle {
    fn new(start_pos: Vec2, end_pos: Vec2, width: f32, layer: u32) -> Self {
        let vec_ = end_pos - start_pos;
        let middle_point = (vec_ / 2.) + start_pos;
        let length = vec_.length();

        let angle = Vec2::angle_between(Vec2::new(1., 0.), vec_);

        Self {
            line: Line,
            sprite: SpriteBundle {
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
            },
        }
    }
}

#[derive(Bundle)]
struct DotBundle {
    sprite: SpriteBundle,
    dot: Dot,
}

impl DotBundle {
    fn new(position: Vec2, layer: u32) -> Self {
        Self {
            dot: Dot,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::hex("e60c0c").unwrap(),
                    custom_size: Some(Vec2::new(8., 8.)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    position.x,
                    position.y,
                    layer as f32,
                )),
                ..default()
            },
        }
    }
}

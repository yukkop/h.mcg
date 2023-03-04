use bevy::{prelude::{Bundle, Color, Transform, Component, Deref, default}, sprite::{SpriteBundle, Sprite, MaterialMesh2dBundle}};
use bevy_math::{Vec2, Vec3, Quat};
pub(crate) use bevy::prelude::*;
use bevy_mod_picking::RaycastMesh;

#[derive(Component)]
pub struct OldNode {
    pub dot: Dot,
    pub connections: Vec<Dot>,
}

#[derive(Component, Deref)]
pub struct Size(pub Vec2);

#[derive(Component, Deref)]
pub struct Frame(pub Vec2);

#[derive(Component, Deref, Clone, Copy)]
pub struct Dot(pub Vec2);

#[derive(Component)]
pub struct Line;

#[derive(Bundle)]
pub struct LineBundle {
    pub sprite: SpriteBundle,
    pub line: Line,
}

impl LineBundle {
    pub fn new(start_pos: Vec2, end_pos: Vec2, width: f32, layer: u32) -> Self {
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

#[derive(Component, Clone, Copy, Deref)]
pub struct Node {
    pub dot: Dot,
}

#[derive(Bundle)]
pub struct NodeBundle {
    pub node: Node,
    pub sprite: SpriteBundle,
    pub r_mesh: RaycastMesh::<Node>
}
// TODO 
impl NodeBundle {
    pub fn new(position: Vec2, layer: u32) -> Self {
        Self {
            node: Node {
                dot: Dot(position)
            }, 
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::hex("ffffff").unwrap(),
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
            r_mesh: RaycastMesh::<Node>::default()
        }
    }
}

#[derive(Bundle)]
pub struct DotBundle {
    pub sprite: SpriteBundle,
    pub dot: Dot,
}

impl DotBundle {
    pub fn new(position: Vec2, layer: u32) -> Self {
        Self {
            dot: Dot(position),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::hex("ffffff").unwrap(),
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
